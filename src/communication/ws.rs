use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use std::ops::ControlFlow;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Result as WebSocketResult;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use url::Url;

mod ws_transfer;

pub struct WsServiceBuilder {
	url: Url,
	access_token: Option<String>,
	reconnect_interval: Option<Duration>,
}

impl WsServiceBuilder {
	pub fn new(url: &str) -> Result<Self, url::ParseError> {
		let url = Url::parse(url)?;
		Ok(Self {
			url,
			access_token: None,
			reconnect_interval: None,
		})
	}

	pub fn new_with_url(url: Url) -> Self {
		Self {
			url,
			access_token: None,
			reconnect_interval: None,
		}
	}

	pub fn build(self) -> WsService {
		WsService::new_with_options(self.url, self.access_token, self.reconnect_interval)
	}

	pub fn access_token(mut self, access_token: String) -> Self {
		self.access_token = Some(access_token);
		self
	}

	pub fn reconnect_interval(mut self, reconnect_interval: Duration) -> Self {
		self.reconnect_interval = Some(reconnect_interval);
		self
	}
}

#[derive(Debug)]
pub struct WsService {
	url: Url,
	api_receiver: Option<InternalAPIReceiver>,
	event_sender: Option<InternalEventSender>,
	task_handle: Option<JoinHandle<()>>,
	reconnect_interval: Duration,
	is_running: Arc<AtomicBool>,
}

impl Clone for WsService {
	fn clone(&self) -> Self {
		Self {
			url: self.url.clone(),
			api_receiver: self.api_receiver.clone(),
			event_sender: self.event_sender.clone(),
			task_handle: None,
			reconnect_interval: self.reconnect_interval,
			is_running: self.is_running.clone(),
		}
	}
}

impl Drop for WsService {
	fn drop(&mut self) {
		self.uninstall();
	}
}

impl WsService {
	pub fn new(url: Url, access_token: Option<String>) -> Self {
		Self::new_with_options(url, access_token, None)
	}

	pub fn new_with_options(
		mut url: Url,
		access_token: Option<String>,
		reconnect_interval: Option<Duration>,
	) -> Self {
		if let Some(access_token) = access_token {
			Self::url_concat_access_token(&mut url, &access_token);
		}
		Self {
			url,
			api_receiver: None,
			event_sender: None,
			task_handle: None,
			reconnect_interval: reconnect_interval.unwrap_or(Duration::from_secs(10)),
			is_running: Arc::new(AtomicBool::new(false)),
		}
	}

	pub fn builder(url: &str) -> Result<WsServiceBuilder, url::ParseError> {
		WsServiceBuilder::new(url)
	}
}

impl WsService {
	pub fn url_concat_access_token(url: &mut Url, access_token: &str) {
		let mut query_pairs = url.query_pairs_mut();
		query_pairs.append_pair("access_token", access_token);
	}

	pub fn get_url(&self) -> &Url {
		&self.url
	}

	async fn connect(
		url: impl IntoClientRequest + Unpin,
	) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Error> {
		let (stream, _) = tokio_tungstenite::connect_async(url).await?;
		Ok(stream)
	}

	async fn handle_connection(
		api_receiver: &InternalAPIReceiver,
		event_sender: &InternalEventSender,
		ws: WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>,
	) -> WebSocketResult<ControlFlow<()>> {
		let transfer = ws_transfer::WsTransfer::new(ws, api_receiver, event_sender);
		transfer.await
	}

	pub async fn spawn_processor(&mut self) -> ServiceStartResult<()> {
		struct IsRunningGuard(Arc<AtomicBool>);
		impl Drop for IsRunningGuard {
			fn drop(&mut self) {
				self.0.store(false, Ordering::Relaxed);
			}
		}
		let is_running_guard = IsRunningGuard(self.is_running.clone());

		let (api_receiver, event_sender) = match (&self.api_receiver, &self.event_sender) {
			(Some(api_receiver), Some(event_sender)) => (api_receiver.clone(), event_sender.clone()),
			(None, None) => return Err(ServiceStartError::NotInjected),
			(None, Some(_)) => return Err(ServiceStartError::NotInjectedAPIReceiver),
			(Some(_), None) => return Err(ServiceStartError::NotInjectedEventSender),
		};

		let url = self.get_url().to_string();
		let mut ws = Self::connect(&url).await?;

		let reconnect_interval = self.reconnect_interval;
		self.task_handle = Some(tokio::spawn(async move {
			let _is_running_guard = is_running_guard;

			'handle_connection: loop {
				let result = Self::handle_connection(&api_receiver, &event_sender, ws).await;
				if let Ok(ControlFlow::Break(())) = result {
					break;
				}
				loop {
					tokio::time::sleep(reconnect_interval).await;
					match Self::connect(&url).await {
						Ok(new_ws) => {
							ws = new_ws;
							continue 'handle_connection;
						}
						Err(_) => continue,
					}
				}
			}
		}));

		Ok(())
	}
}

#[async_trait]
impl CommunicationService for WsService {
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
		if self.is_running.swap(true, Ordering::Relaxed) {
			return Err(ServiceStartError::TaskIsRunning);
		}

		self.spawn_processor().await?;
		Ok(())
	}
}
