use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use reqwest::IntoUrl;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::task::JoinHandle;
use url::Url;

#[derive(Debug)]
pub struct HttpService {
	url: Url,
	access_token: Option<String>,
	api_receiver: Option<InternalAPIReceiver>,
	event_sender: Option<InternalEventSender>,
	task_handle: Option<JoinHandle<()>>,
	is_running: Arc<AtomicBool>,
}

impl Clone for HttpService {
	fn clone(&self) -> Self {
		Self {
			url: self.url.clone(),
			access_token: self.access_token.clone(),
			api_receiver: self.api_receiver.clone(),
			event_sender: self.event_sender.clone(),
			task_handle: None,
			is_running: self.is_running.clone(),
		}
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct HttpResponse {
	status: String,
	retcode: i32,
	data: JsonValue,
}

impl HttpService {
	pub fn new(url: impl IntoUrl, access_token: Option<String>) -> reqwest::Result<Self> {
		Ok(Self {
			url: url.into_url()?,
			access_token,
			api_receiver: None,
			event_sender: None,
			task_handle: None,
			is_running: Arc::new(AtomicBool::new(false)),
		})
	}

	async fn api_processor(self) -> anyhow::Result<()> {
		let api_receiver = self.api_receiver.clone().unwrap();
		let event_sender = self.event_sender.clone().unwrap();
		let client = reqwest::Client::new();

		loop {
			match api_receiver.recv_async().await {
				Ok(data) => {
					let response = self.send_api_request(&client, &data).await;
					if response.is_err() {
						continue;
					}
					let event = self.response_parser(data.echo, response?).await;
					if event.is_err() {
						continue;
					}
					let _ = event_sender.send_async(event?).await;
				}
				Err(_) => return Err(anyhow::anyhow!("api receiver closed")),
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
	) -> anyhow::Result<DeserializedEvent> {
		let status = response.status();
		if !status.is_success() {
			let res = APIResponse {
				echo,
				data: JsonValue::Null,
				retcode: status.as_u16() as i32,
				status: "failed".to_string(),
			};
			Ok(DeserializedEvent::APIResponse(res))
		} else {
			let http_res: HttpResponse = response.json().await?;
			let res = APIResponse {
				echo,
				data: http_res.data,
				status: http_res.status,
				retcode: http_res.retcode,
			};
			Ok(DeserializedEvent::APIResponse(res))
		}
	}
}

impl Drop for HttpService {
	fn drop(&mut self) {
		self.uninstall();
	}
}

#[async_trait]
impl CommunicationService for HttpService {
	fn install(&mut self, api_receiver: InternalAPIReceiver, event_sender: InternalEventSender) {
		self.api_receiver = Some(api_receiver);
		self.event_sender = Some(event_sender);
	}

	fn uninstall(&mut self) {
		self.stop();
		self.api_receiver = None;
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

		if self.api_receiver.is_none() && self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjected);
		} else if self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjectedEventSender);
		} else if self.api_receiver.is_none() {
			return Err(ServiceStartError::NotInjectedAPIReceiver);
		}

		self.is_running.store(true, Ordering::Relaxed);
		let service = self.clone();
		self.task_handle = Some(tokio::spawn(async move {
			service.api_processor().await.ok();
		}));

		Ok(())
	}
}
