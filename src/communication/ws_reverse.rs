use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::any;
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use http::{HeaderMap, StatusCode};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::select;
use tokio::sync::broadcast;

pub struct WsReverseService<T: ToSocketAddrs + Clone + Send + Sync> {
	api_receiver: Option<APIReceiver>,
	event_sender: Option<EventSender>,
	close_signal_sender: broadcast::Sender<()>,
	access_token: Option<String>,
	addr: T,
}

impl<T: ToSocketAddrs + Clone + Send + Sync> Drop for WsReverseService<T> {
	fn drop(&mut self) {
		let _ = self.close_signal_sender.send(());
	}
}

impl<T: ToSocketAddrs + Clone + Send + Sync> WsReverseService<T> {
	pub fn new(addr: T, access_token: Option<String>) -> Self {
		let (close_signal_sender, _) = broadcast::channel(1);
		Self {
			api_receiver: None,
			event_sender: None,
			close_signal_sender,
			access_token,
			addr,
		}
	}
}

struct AppState {
	access_token: Option<String>,
	api_receiver: APIReceiver,
	event_sender: EventSender,
	close_signal_sender: broadcast::Sender<()>,
	connected: Arc<AtomicBool>,
}

async fn send_processor(
	mut send_side: SplitSink<WebSocket, Message>,
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

async fn read_processor(
	mut read_side: SplitStream<WebSocket>,
	event_sender: EventSender,
	mut close_signal: broadcast::Receiver<()>,
	connection_close_signal_sender: broadcast::Sender<()>,
	connected: Arc<AtomicBool>,
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
						connected.store(false, Ordering::Relaxed);
						return Err(anyhow::anyhow!("close"));
					},
					_ => ()
				}
			}
		}
	}
}

async fn handler(
	headers: HeaderMap,
	State(state): State<Arc<AppState>>,
	ws: WebSocketUpgrade,
) -> Response {
	if state.connected.load(Ordering::Relaxed) {
		return Response::builder()
			.status(StatusCode::FORBIDDEN)
			.body(Body::from(""))
			.unwrap();
	}
	if state.access_token.is_some() {
		let received_token = headers.get("Authorization").map(|v| v.to_str().unwrap());
		if received_token.is_none() {
			return Response::builder()
				.status(StatusCode::UNAUTHORIZED)
				.body(Body::from(""))
				.unwrap();
		}
		let received_token = received_token.unwrap();
		let access_token = state.access_token.clone().unwrap();
		if received_token != "Bearer ".to_string() + &access_token {
			return Response::builder()
				.status(StatusCode::FORBIDDEN)
				.body(Body::from(""))
				.unwrap();
		}
	}
	ws.on_upgrade(async move |socket: WebSocket| {
		let (send_side, read_side) = socket.split();
		let (connection_close_signal_sender, connection_close_signal) = broadcast::channel(1);
		let api_receiver = state.api_receiver.clone();
		let event_sender = state.event_sender.clone();
		state.connected.store(true, Ordering::Relaxed);
		let send_task = tokio::spawn(send_processor(
			send_side,
			api_receiver,
			state.close_signal_sender.subscribe(),
			connection_close_signal,
		));
		let read_task = tokio::spawn(read_processor(
			read_side,
			event_sender,
			state.close_signal_sender.subscribe(),
			connection_close_signal_sender,
			Arc::clone(&state.connected),
		));
		let (r1, r2) = futures::try_join!(send_task, read_task).unwrap();
		r1.and(r2).unwrap();
	})
}

#[async_trait]
impl<T: ToSocketAddrs + Clone + Send + Sync> CommunicationService for WsReverseService<T> {
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

		let api_receiver = self.api_receiver.clone().unwrap();
		let event_sender = self.event_sender.clone().unwrap();

		let state = Arc::new(AppState {
			access_token: self.access_token.clone(),
			api_receiver,
			event_sender,
			close_signal_sender: self.close_signal_sender.clone(),
			connected: Arc::new(AtomicBool::new(false)),
		});

		let listener = TcpListener::bind(self.addr.clone()).await?;
		let router = Router::new().fallback(any(handler)).with_state(state);
		let mut close_signal = self.close_signal_sender.subscribe();

		tokio::spawn(
			axum::serve(listener, router)
				.with_graceful_shutdown(async move {
					let _ = close_signal.recv().await;
				})
				.into_future(),
		);

		Ok(())
	}
}
