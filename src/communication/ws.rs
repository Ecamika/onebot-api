use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use reqwest::IntoUrl;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use url::Url;

#[derive(Clone, Debug)]
pub struct WsService {
	url: Url,
	access_token: Option<String>,
	api_receiver: Option<APIReceiver>,
	event_sender: Option<EventSender>,
	close_signal_sender: broadcast::Sender<()>,
	connection_close_signal_sender: broadcast::Sender<()>,
	auto_reconnect: bool,
	reconnect_interval: Duration,
	max_reconnect_times: u32,
}

impl Drop for WsService {
	fn drop(&mut self) {
		let _ = self.close_signal_sender.send(());
	}
}

impl WsService {
	pub fn new(
		url: impl IntoUrl,
		access_token: Option<String>,
		auto_reconnect: Option<bool>,
		reconnect_interval: Option<Duration>,
		max_reconnect_times: Option<u32>,
	) -> reqwest::Result<Self> {
		let (close_signal_sender, _) = broadcast::channel(1);
		let (connection_close_signal_sender, _) = broadcast::channel(1);
		Ok(Self {
			url: url.into_url()?,
			access_token,
			api_receiver: None,
			event_sender: None,
			close_signal_sender,
			connection_close_signal_sender,
			auto_reconnect: auto_reconnect.unwrap_or(true),
			reconnect_interval: reconnect_interval.unwrap_or(Duration::from_secs(10)),
			max_reconnect_times: max_reconnect_times.unwrap_or(5),
		})
	}
}

impl WsService {
	pub fn get_url(&self) -> Url {
		let mut url = self.url.clone();
		if let Some(token) = &self.access_token {
			let mut query_pairs = url.query_pairs_mut();
			query_pairs.append_pair("access_token", token);
		}
		url
	}

	pub async fn connect(
		url: impl ToString,
	) -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, tokio_tungstenite::tungstenite::Error> {
		let (stream, _) = tokio_tungstenite::connect_async(url.to_string()).await?;
		Ok(stream)
	}

	pub async fn send_processor(
		mut send_side: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
		api_receiver: APIReceiver,
		mut close_signal: broadcast::Receiver<()>,
		mut connection_close_signal: broadcast::Receiver<()>,
	) -> anyhow::Result<()> {
		loop {
			select! {
				_ = close_signal.recv() => return Err(anyhow::anyhow!("close")),
				_ = connection_close_signal.recv() => return Err(anyhow::anyhow!("close")),
				Ok(data) = api_receiver.recv_async() => {
					let str = serde_json::to_string(&data);
					if str.is_err() {
						continue
					}
					let _ = send_side.send(Message::Text(str?.into())).await;
				}
			}
		}
	}

	pub async fn read_processor(
		mut read_side: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
		event_sender: EventSender,
		mut close_signal: broadcast::Receiver<()>,
		connection_close_signal_sender: broadcast::Sender<()>,
	) -> anyhow::Result<()> {
		loop {
			select! {
				_ = close_signal.recv() => return Err(anyhow::anyhow!("close")),
				Some(Ok(msg)) = read_side.next() => {
					match msg {
						Message::Text(data) => {
							let str = data.as_str();
							let event = serde_json::from_str::<Event>(str);
							if event.is_err() {
								continue
							}
							let event = Arc::new(event?);
							let _ = event_sender.send(event);
						},
						Message::Close(_) => {
							let _ = connection_close_signal_sender.send(());
							return Err(anyhow::anyhow!("close"));
						},
						_ => ()
					}
				}
			}
		}
	}

	pub async fn spawn_processor(&self) -> ServiceStartResult<()> {
		if self.api_receiver.is_none() && self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjected);
		} else if self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjectedEventSender);
		} else if self.api_receiver.is_none() {
			return Err(ServiceStartError::NotInjectedAPIReceiver);
		}

		let api_receiver = self.api_receiver.clone().unwrap();
		let event_sender = self.event_sender.clone().unwrap();

		let (send_side, read_side) = Self::connect(self.get_url()).await?.split();

		tokio::spawn(Self::read_processor(
			read_side,
			event_sender,
			self.close_signal_sender.subscribe(),
			self.connection_close_signal_sender.clone(),
		));
		tokio::spawn(Self::send_processor(
			send_side,
			api_receiver,
			self.close_signal_sender.subscribe(),
			self.connection_close_signal_sender.subscribe(),
		));
		Ok(())
	}

	pub async fn reconnect(&self, reconnect_times: u32) -> anyhow::Result<()> {
		if reconnect_times > self.max_reconnect_times {
			return Err(anyhow::anyhow!("over max reconnect times"));
		}
		tokio::time::sleep(self.reconnect_interval).await;
		if self.spawn_processor().await.is_err() {
			Box::pin(self.reconnect(reconnect_times + 1)).await
		} else {
			Ok(())
		}
	}

	pub async fn reconnect_processor(self) -> anyhow::Result<()> {
		let mut close_signal = self.close_signal_sender.subscribe();
		let mut connection_close_signal = self.connection_close_signal_sender.subscribe();
		loop {
			select! {
				_ = close_signal.recv() => return Err(anyhow::anyhow!("close")),
				_ = connection_close_signal.recv() => {
					tokio::time::sleep(self.reconnect_interval).await;
					self.reconnect(1).await?;
				}
			}
		}
	}
}

#[async_trait]
impl CommunicationService for WsService {
	fn inject(&mut self, api_receiver: APIReceiver, event_sender: EventSender) {
		self.api_receiver = Some(api_receiver);
		self.event_sender = Some(event_sender);
	}

	async fn start_service(&self) -> ServiceStartResult<()> {
		self.spawn_processor().await?;
		if self.auto_reconnect {
			tokio::spawn(Self::reconnect_processor(self.clone()));
		}
		Ok(())
	}
}
