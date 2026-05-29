use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use bytes::Bytes;
use eventsource_stream::{EventStream, Eventsource};
use futures::{Stream, StreamExt};
use reqwest::IntoUrl;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::task::JoinHandle;
use url::Url;

#[derive(Debug)]
pub struct SseService {
	url: Url,
	access_token: Option<String>,
	event_sender: Option<InternalEventSender>,
	task_handle: Option<JoinHandle<()>>,
	is_running: Arc<AtomicBool>,
	// auto_reconnect: bool,
	// reconnect_interval: Duration,
	// reconnect_signal_sender: broadcast::Sender<()>
}

impl Clone for SseService {
	fn clone(&self) -> Self {
		Self {
			url: self.url.clone(),
			access_token: self.access_token.clone(),
			event_sender: self.event_sender.clone(),
			task_handle: None,
			is_running: self.is_running.clone(),
		}
	}
}

impl Drop for SseService {
	fn drop(&mut self) {
		self.uninstall();
	}
}

impl SseService {
	pub fn new(
		url: impl IntoUrl,
		access_token: Option<String>,
		// auto_reconnect: Option<bool>,
		// reconnect_interval: Option<Duration>,
	) -> reqwest::Result<Self> {
		Ok(Self {
			url: url.into_url()?,
			access_token,
			event_sender: None,
			task_handle: None,
			is_running: Arc::new(AtomicBool::new(false)),
			// auto_reconnect: auto_reconnect.unwrap_or(true),
			// reconnect_interval: reconnect_interval.unwrap_or(Duration::from_secs(10)),
			// reconnect_signal_sender
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
		let mut es = self.eventsource().await?;
		let event_sender = self.event_sender.clone().unwrap();
		loop {
			match es.next().await {
				Some(Ok(es_event)) => {
					let event = serde_json::from_str(&es_event.data);
					if event.is_err() {
						continue;
					}
					let _ = event_sender.send_async(event?).await;
				}
				_ => return Err(anyhow::anyhow!("eventsource ended")),
			}
		}
	}

	// async fn reconnect_processor(self) -> anyhow::Result<()> {
	// 	let mut close_signal = self.close_signal_sender.subscribe();
	// 	let mut reconnect_signal = self.reconnect_signal_sender.subscribe();
	//
	// 	loop {
	// 		select! {
	// 			_ = close_signal.recv() => return Err(anyhow::anyhow!("close")),
	// 			_ = reconnect_signal.recv() => {
	//
	// 			}
	// 		}
	// 	}
	// }
}

#[async_trait]
impl CommunicationService for SseService {
	fn install(&mut self, _api_receiver: InternalAPIReceiver, event_sender: InternalEventSender) {
		self.event_sender = Some(event_sender);
	}

	fn uninstall(&mut self) {
		self.stop();
		self.event_sender = None;
	}

	fn stop(&mut self) {
		if let Some(handle) = self.task_handle.take() {
			handle.abort();
		}
		self.is_running.store(false, Ordering::Relaxed);
	}

	async fn start(&mut self) -> ServiceStartResult<()> {
		if self.is_running.load(Ordering::Relaxed) {
			return Err(ServiceStartError::TaskIsRunning);
		}

		if self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjectedEventSender);
		}

		self.is_running.store(true, Ordering::Relaxed);
		let service = self.clone();
		self.task_handle = Some(tokio::spawn(async move {
			service.eventsource_listener().await.ok();
		}));

		Ok(())
	}
}
