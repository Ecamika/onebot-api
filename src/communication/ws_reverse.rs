use super::utils::*;
use crate::error::{
	ServiceRuntimeError, ServiceRuntimeResult, ServiceStartError, ServiceStartResult,
};
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
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::JoinHandle;

pub struct WsReverseService<T: ToSocketAddrs + Clone + Send + Sync> {
	api_receiver: Option<InternalAPIReceiver>,
	event_sender: Option<InternalEventSender>,
	serve_handle: Option<JoinHandle<()>>,
	app_state: Option<Arc<AppState>>,
	access_token: Option<String>,
	addr: T,
	is_running: Arc<AtomicBool>,
}

impl<T: ToSocketAddrs + Clone + Send + Sync> Drop for WsReverseService<T> {
	fn drop(&mut self) {
		self.uninstall();
	}
}

impl<T: ToSocketAddrs + Clone + Send + Sync> WsReverseService<T> {
	pub fn new(addr: T, access_token: Option<String>) -> Self {
		Self {
			api_receiver: None,
			event_sender: None,
			serve_handle: None,
			app_state: None,
			access_token,
			addr,
			is_running: Arc::new(AtomicBool::new(false)),
		}
	}
}

struct AppState {
	access_token: Option<String>,
	api_receiver: InternalAPIReceiver,
	event_sender: InternalEventSender,
	connected: Arc<AtomicBool>,
	connection_handle: Arc<Mutex<Option<JoinHandle<ServiceRuntimeResult<()>>>>>,
}

async fn send_processor(
	mut send_side: SplitSink<WebSocket, Message>,
	api_receiver: InternalAPIReceiver,
) -> ServiceRuntimeResult<()> {
	loop {
		match api_receiver.recv_async().await {
			Ok(data) => {
				let str = serde_json::to_string(&data);
				if str.is_err() {
					continue;
				}
				let _ = send_side.send(Message::Text(str?.into())).await;
			}
			Err(_) => return Err(ServiceRuntimeError::ChannelClosed),
		}
	}
}

async fn read_processor(
	mut read_side: SplitStream<WebSocket>,
	event_sender: InternalEventSender,
) -> ServiceRuntimeResult<()> {
	loop {
		match read_side.next().await {
			Some(Ok(msg)) => match msg {
				Message::Text(data) => {
					let str = data.as_str();
					let event = serde_json::from_str::<DeserializedEvent>(str);
					if event.is_err() {
						continue;
					}
					let _ = event_sender.send_async(event?).await;
				}
				Message::Close(_) => {
					return Err(ServiceRuntimeError::WebSocketClosedByPeer);
				}
				_ => (),
			},
			Some(Err(_)) => return Err(ServiceRuntimeError::WebSocketError),
			None => return Err(ServiceRuntimeError::WebSocketStreamEnded),
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
		let api_receiver = state.api_receiver.clone();
		let event_sender = state.event_sender.clone();
		state.connected.store(true, Ordering::Relaxed);
		let handle = tokio::spawn(async move {
			tokio::select! {
				r = send_processor(send_side, api_receiver) => r,
				r = read_processor(read_side, event_sender) => r,
			}
		});
		*state.connection_handle.lock().unwrap() = Some(handle);
	})
}

#[async_trait]
impl<T: ToSocketAddrs + Clone + Send + Sync> CommunicationService for WsReverseService<T> {
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
		if let Some(state) = self.app_state.as_ref()
			&& let Some(handle) = state.connection_handle.lock().unwrap().take()
		{
			handle.abort();
		}
		if let Some(handle) = self.serve_handle.take() {
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

		let api_receiver = self.api_receiver.clone().unwrap();
		let event_sender = self.event_sender.clone().unwrap();

		let state = Arc::new(AppState {
			access_token: self.access_token.clone(),
			api_receiver,
			event_sender,
			connected: Arc::new(AtomicBool::new(false)),
			connection_handle: Arc::new(Mutex::new(None)),
		});
		self.app_state = Some(state.clone());

		let listener = TcpListener::bind(self.addr.clone()).await?;
		let router = Router::new().fallback(any(handler)).with_state(state);

		self.is_running.store(true, Ordering::Relaxed);
		self.serve_handle = Some(tokio::spawn(async move {
			axum::serve(listener, router).await.ok();
		}));

		Ok(())
	}
}
