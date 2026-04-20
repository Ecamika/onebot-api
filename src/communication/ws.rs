use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use futures::future::{Either, select};
use std::ops::ControlFlow;
use std::pin::pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast;
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

#[derive(Clone, Debug)]
pub struct WsService {
	url: Url,
	api_receiver: Option<InternalAPIReceiver>,
	event_sender: Option<InternalEventSender>,
	close_signal_sender: broadcast::Sender<()>,
	reconnect_interval: Duration,
	is_running: Arc<AtomicBool>,
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
		let (close_signal_sender, _) = broadcast::channel(1);
		if let Some(access_token) = access_token {
			Self::url_concat_access_token(&mut url, &access_token);
		}
		Self {
			url,
			api_receiver: None,
			event_sender: None,
			close_signal_sender,
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
		close_signal: &mut broadcast::Receiver<()>,
		ws: WebSocketStream<impl AsyncRead + AsyncWrite + Unpin>,
	) -> WebSocketResult<ControlFlow<()>> {
		let mut transfer = ws_transfer::WsTransfer::new(ws, api_receiver, event_sender);
		select! {
			biased;

			_ = close_signal.recv() => {
				transfer.initiate_close();
				// 等 WebSocket 挥手完成后再返回
				transfer.await
			}
			transfer_res = &mut transfer => transfer_res,
		}
	}

	pub async fn spawn_processor(&self) -> ServiceStartResult<()> {
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

		let mut close_signal = self.close_signal_sender.subscribe();
		let reconnect_interval = self.reconnect_interval;
		tokio::spawn(async move {
			let _is_running_guard = is_running_guard;

			'handle_connection: loop {
				let result =
					Self::handle_connection(&api_receiver, &event_sender, &mut close_signal, ws).await;
				if let Ok(ControlFlow::Break(())) = result {
					break;
				}
				loop {
					let close_signal_future = pin!(close_signal.recv());
					let reconnect_future = pin!(async {
						tokio::time::sleep(reconnect_interval).await;
						Self::connect(&url).await
					});
					let result = select(close_signal_future, reconnect_future).await;
					match result {
						Either::Left(_) => {
							break 'handle_connection;
						}
						Either::Right((Ok(new_ws), _)) => {
							ws = new_ws;
							continue 'handle_connection;
						}
						Either::Right((Err(_), _)) => {
							continue;
						}
					}
				}
			}
		});

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

	fn stop(&self) {
		let _ = self.close_signal_sender.send(());
		self.is_running.store(false, Ordering::Relaxed);
	}

	async fn start(&self) -> ServiceStartResult<()> {
		if self.is_running.swap(true, Ordering::Relaxed) {
			return Err(ServiceStartError::TaskIsRunning);
		}

		self.spawn_processor().await?;
		Ok(())
	}
}
