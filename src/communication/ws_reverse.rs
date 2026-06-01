use super::utils::*;
use crate::error::{ServiceRuntimeResult, ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use axum::Router;
use axum::body::Body;
use axum::extract::ws::WebSocket;
use axum::extract::{State, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::any;
use http::{HeaderMap, StatusCode};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::{AbortHandle, JoinHandle};

mod transfer;

use transfer::ReverseWsTransfer;

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
	connection_handle: Arc<Mutex<Option<AbortHandle>>>,
}

fn empty_response(status: StatusCode) -> Response {
	Response::builder()
		.status(status)
		.body(Body::from(""))
		.unwrap()
}

async fn run_connection(state: Arc<AppState>, socket: WebSocket) -> ServiceRuntimeResult<()> {
	struct ConnectionGuard {
		connected: Arc<AtomicBool>,
		connection_handle: Arc<Mutex<Option<AbortHandle>>>,
	}

	impl Drop for ConnectionGuard {
		fn drop(&mut self) {
			*self.connection_handle.lock().unwrap() = None;
			self.connected.store(false, Ordering::Release);
		}
	}

	let _guard = ConnectionGuard {
		connected: state.connected.clone(),
		connection_handle: state.connection_handle.clone(),
	};

	let api_receiver = state.api_receiver.clone();
	let event_sender = state.event_sender.clone();
	let transfer = ReverseWsTransfer::new(socket, &api_receiver, &event_sender);
	transfer.await
}

async fn handler(
	headers: HeaderMap,
	State(state): State<Arc<AppState>>,
	ws: WebSocketUpgrade,
) -> Response {
	if state.access_token.is_some() {
		let received_token = headers.get("Authorization").and_then(|v| v.to_str().ok());
		if received_token.is_none() {
			return empty_response(StatusCode::UNAUTHORIZED);
		}
		let received_token = received_token.unwrap();
		let access_token = state.access_token.clone().unwrap();
		if received_token != "Bearer ".to_string() + &access_token {
			return empty_response(StatusCode::FORBIDDEN);
		}
	}
	if state
		.connected
		.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
		.is_err()
	{
		return empty_response(StatusCode::FORBIDDEN);
	}

	let failed_state = state.clone();
	ws.on_failed_upgrade(move |_error| {
		*failed_state.connection_handle.lock().unwrap() = None;
		failed_state.connected.store(false, Ordering::Release);
	})
	.on_upgrade(move |socket: WebSocket| async move {
		let handle = tokio::spawn(run_connection(state.clone(), socket));
		*state.connection_handle.lock().unwrap() = Some(handle.abort_handle());
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
			state.connected.store(false, Ordering::Release);
		}
		if let Some(handle) = self.serve_handle.take() {
			handle.abort();
		}
		self.is_running.store(false, Ordering::Release);
	}

	async fn start(&mut self) -> ServiceStartResult<()> {
		if self.is_running.load(Ordering::Acquire) {
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

		self.is_running.store(true, Ordering::Release);
		self.serve_handle = Some(tokio::spawn(async move {
			axum::serve(listener, router).await.ok();
		}));

		Ok(())
	}
}
