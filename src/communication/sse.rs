use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use bytes::Bytes;
use eventsource_stream::{EventStream, Eventsource};
use futures::{Stream, StreamExt};
use reqwest::IntoUrl;
use std::sync::Arc;
use tokio::select;
use tokio::sync::broadcast;
use url::Url;

#[derive(Debug, Clone)]
pub struct SseService {
	url: Url,
	access_token: Option<String>,
	event_sender: Option<EventSender>,
	close_signal_sender: broadcast::Sender<()>,
}

impl Drop for SseService {
	fn drop(&mut self) {
		let _ = self.close_signal_sender.send(());
	}
}

impl SseService {
	pub fn new(url: impl IntoUrl, access_token: Option<String>) -> reqwest::Result<Self> {
		let (close_signal_sender, _) = broadcast::channel(1);
		Ok(Self {
			url: url.into_url()?,
			access_token,
			event_sender: None,
			close_signal_sender,
		})
	}

	pub async fn eventsource(
		&self,
	) -> anyhow::Result<EventStream<impl Stream<Item = reqwest::Result<Bytes>>>> {
		let client = reqwest::Client::new();
		let mut req = client.get(self.url.clone());
		if let Some(token) = &self.access_token {
			req = req.header("Authorization", "Bearer ".to_string() + token);
		}
		let eventsource = req.send().await?.bytes_stream().eventsource();
		Ok(eventsource)
	}

	async fn eventsource_listener(self) -> anyhow::Result<()> {
		let mut close_signal = self.close_signal_sender.subscribe();
		let mut es = self.eventsource().await?;
		let event_sender = self.event_sender.clone().unwrap();
		loop {
			select! {
				_ = close_signal.recv() => return Err(anyhow::anyhow!("close")),
				Some(Ok(es_event)) = es.next() => {
					let event = serde_json::from_str(&es_event.data);
					if event.is_err() {
						continue
					}
					let _ = event_sender.send(Arc::new(event?));
				}
			}
		}
	}
}

#[async_trait]
impl CommunicationService for SseService {
	fn inject(&mut self, _api_receiver: APIReceiver, event_sender: EventSender) {
		self.event_sender = Some(event_sender);
	}

	async fn start_service(&self) -> ServiceStartResult<()> {
		if self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjectedEventSender);
		}

		tokio::spawn(Self::eventsource_listener(self.clone()));

		Ok(())
	}
}
