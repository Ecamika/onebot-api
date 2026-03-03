use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use reqwest::IntoUrl;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use tokio::select;
use tokio::sync::broadcast;
use url::Url;

#[derive(Clone, Debug)]
pub struct HttpService {
	url: Url,
	access_token: Option<String>,
	api_receiver: Option<APIReceiver>,
	event_sender: Option<EventSender>,
	close_signal_sender: broadcast::Sender<()>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HttpResponse {
	status: String,
	retcode: i32,
	data: JsonValue,
}

impl HttpService {
	pub fn new(url: impl IntoUrl, access_token: Option<String>) -> reqwest::Result<Self> {
		let (close_signal_sender, _) = broadcast::channel(1);
		Ok(Self {
			url: url.into_url()?,
			access_token,
			api_receiver: None,
			event_sender: None,
			close_signal_sender,
		})
	}

	async fn api_processor(self) -> anyhow::Result<()> {
		let mut close_signal = self.close_signal_sender.subscribe();
		let api_receiver = self.api_receiver.clone().unwrap();
		let event_sender = self.event_sender.clone().unwrap();
		let client = reqwest::Client::new();

		loop {
			select! {
				_ = close_signal.recv() => return Err(anyhow::anyhow!("close")),
				Ok(data) = api_receiver.recv_async() => {
					let response = self.send_api_request(&client, &data).await;
					if response.is_err() {
						continue
					}
					let event = self.response_parser(data.echo, response?).await;
					if event.is_err() {
						continue
					}
					let _ = event_sender.send(Arc::new(event?));
				}
			}
		}
	}

	pub async fn send_api_request(
		&self,
		client: &reqwest::Client,
		api_request: &APIRequest,
	) -> anyhow::Result<reqwest::Response> {
		let mut url = self.url.clone();
		let mut path_segments = url
			.path_segments_mut()
			.map_err(|_| anyhow::anyhow!("URL is cannot-be-a-base"))?;
		path_segments.push(&api_request.action);
		drop(path_segments);
		let mut post_req = client.post(url);
		if let Some(token) = &self.access_token {
			post_req = post_req.header("Authorization", "Bearer ".to_string() + token);
		}
		let res = post_req
			.body(serde_json::to_string(&api_request.params)?)
			.send()
			.await?;
		Ok(res)
	}

	pub async fn response_parser(
		&self,
		echo: Option<String>,
		response: reqwest::Response,
	) -> anyhow::Result<Event> {
		let status = response.status();
		if !status.is_success() {
			let res = APIResponse {
				echo,
				data: JsonValue::Null,
				retcode: status.as_u16() as i32,
				status: "failed".to_string(),
			};
			Ok(Event::APIResponse(res))
		} else {
			let http_res: HttpResponse = response.json().await?;
			let res = APIResponse {
				echo,
				data: http_res.data,
				status: http_res.status,
				retcode: http_res.retcode,
			};
			Ok(Event::APIResponse(res))
		}
	}
}

impl Drop for HttpService {
	fn drop(&mut self) {
		let _ = self.close_signal_sender.send(());
	}
}

#[async_trait]
impl CommunicationService for HttpService {
	fn inject(&mut self, api_receiver: APIReceiver, event_sender: EventSender) {
		self.api_receiver = Some(api_receiver);
		self.event_sender = Some(event_sender);
	}

	async fn start_service(&self) -> ServiceStartResult<()> {
		if self.api_receiver.is_none() && self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjected);
		} else if self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjectedEventSender);
		} else if self.api_receiver.is_none() {
			return Err(ServiceStartError::NotInjectedAPIReceiver);
		}

		tokio::spawn(Self::api_processor(self.clone()));

		Ok(())
	}
}
