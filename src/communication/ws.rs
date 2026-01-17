use super::communication_utils::*;
use anyhow::anyhow;
use async_trait::async_trait;
use flume::{Receiver, Sender};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use reqwest::IntoUrl;
use tokio::net::TcpStream;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use url::Url;

pub struct WsService {
	close_sender: broadcast::Sender<()>,
	api_receiver: Option<Receiver<String>>,
	msg_sender: Option<Sender<String>>,
	url: Url,
}

impl Drop for WsService {
	fn drop(&mut self) {
		let _ = self.close_sender.send(());
	}
}

impl WsService {
	pub fn new(url: impl IntoUrl) -> reqwest::Result<Self> {
		let (close_sender, _) = broadcast::channel(4);
		Ok(Self {
			close_sender,
			api_receiver: None,
			msg_sender: None,
			url: url.into_url()?,
		})
	}

	pub fn new_with_token(url: impl IntoUrl, token: Option<String>) -> reqwest::Result<Self> {
		if let Some(token) = token {
			let mut url = url.into_url()?;
			url.set_query(Some(&format!("access_token={}", token)));
			Self::new(url)
		} else {
			Self::new(url)
		}
	}
}

impl WsService {
	async fn connect(&self) -> anyhow::Result<()> {
		if self.msg_sender.is_none() || self.api_receiver.is_none() {
			return Err(anyhow!("msg_sender or api_receiver is none"));
		}
		let api_receiver = self.api_receiver.clone().unwrap();
		let msg_sender = self.msg_sender.clone().unwrap();
		let (ws_stream, _) = connect_async(self.url.as_str()).await?;
		let (write_half, read_half) = ws_stream.split();
		let write_half_close_receiver = self.close_sender.subscribe();
		let read_half_close_receiver = self.close_sender.subscribe();
		tokio::spawn(async move {
			Self::ws_stream_writer(write_half_close_receiver, api_receiver, write_half).await
		});
		tokio::spawn(async move {
			Self::ws_stream_reader(read_half_close_receiver, msg_sender, read_half).await
		});
		Ok(())
	}

	async fn ws_stream_writer(
		mut close_receiver: broadcast::Receiver<()>,
		api_receiver: Receiver<String>,
		mut write_half: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
	) {
		loop {
			tokio::select! {
				msg = api_receiver.recv_async() => {
					if let Ok(msg) = msg {
						let _ = write_half.send(Message::Text(msg.into())).await;
					}
				}
				_ = close_receiver.recv() => {
					return
				}
			}
		}
	}

	async fn ws_stream_reader(
		mut close_receiver: broadcast::Receiver<()>,
		msg_sender: Sender<String>,
		mut read_half: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
	) {
		loop {
			tokio::select! {
				msg = read_half.next() => {
					if let Some(Ok(Message::Text(data))) = msg {
						let _ = msg_sender.send_async(data.to_string()).await;
					}
				}
				_ = close_receiver.recv() => {
					return
				}
			}
		}
	}
}

#[async_trait]
impl CommunicationService for WsService {
	fn register_api_receiver(&mut self, api_receiver: Receiver<String>) {
		self.api_receiver = Some(api_receiver)
	}

	fn register_msg_sender(&mut self, msg_sender: Sender<String>) {
		self.msg_sender = Some(msg_sender)
	}

	async fn start(&self) -> anyhow::Result<()> {
		self.connect().await
	}
}
