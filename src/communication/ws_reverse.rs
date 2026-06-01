use super::utils::*;
use crate::error::{
	ServiceRuntimeError, ServiceRuntimeResult, ServiceStartError, ServiceStartResult,
};
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
use std::sync::atomic::Ordering;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::{AbortHandle, JoinHandle};

mod transfer;

use transfer::ReverseWsTransfer;

pub struct WsReverseService<T: ToSocketAddrs + Clone + Send + Sync> {
	api_receiver: Option<InternalAPIReceiver>,
	event_sender: Option<InternalEventSender>,
	serve_handle: Option<JoinHandle<ServiceRuntimeResult<()>>>,
	app_state: Option<Arc<AppState>>,
	access_token: Option<String>,
	addr: T,
	task_state: Arc<ServiceTaskState>,
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
			task_state: Arc::new(ServiceTaskState::new()),
		}
	}
}

struct AppState {
	access_token: Option<String>,
	api_receiver: InternalAPIReceiver,
	event_sender: InternalEventSender,
	connected: Arc<std::sync::atomic::AtomicBool>,
	connection_handle: Arc<Mutex<Option<AbortHandle>>>,
	task_state: Arc<ServiceTaskState>,
}

fn empty_response(status: StatusCode) -> Response {
	Response::builder()
		.status(status)
		.body(Body::from(""))
		.unwrap()
}

async fn run_connection(state: Arc<AppState>, socket: WebSocket) -> ServiceRuntimeResult<()> {
	struct ConnectionGuard {
		connected: Arc<std::sync::atomic::AtomicBool>,
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
		let Some(access_token) = state.access_token.as_ref() else {
			return empty_response(StatusCode::INTERNAL_SERVER_ERROR);
		};
		let Some(received_token) = headers.get("Authorization") else {
			return empty_response(StatusCode::UNAUTHORIZED);
		};
		let Ok(received_token) = received_token.to_str() else {
			return empty_response(StatusCode::BAD_REQUEST);
		};
		if received_token != "Bearer ".to_string() + access_token {
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
		let task_state = state.task_state.clone();
		let handle = tokio::spawn(run_connection(state.clone(), socket));
		*state.connection_handle.lock().unwrap() = Some(handle.abort_handle());
		tokio::spawn(async move {
			if let Ok(Err(err)) = handle.await {
				task_state.record_runtime_error(err);
			}
		});
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
		self.task_state.stop();
	}

	async fn start(&mut self) -> ServiceStartResult<()> {
		if !self.task_state.try_start() {
			return Err(ServiceStartError::TaskIsRunning);
		}

		if self.api_receiver.is_none() && self.event_sender.is_none() {
			self.task_state.stop();
			return Err(ServiceStartError::NotInjected);
		} else if self.event_sender.is_none() {
			self.task_state.stop();
			return Err(ServiceStartError::NotInjectedEventSender);
		} else if self.api_receiver.is_none() {
			self.task_state.stop();
			return Err(ServiceStartError::NotInjectedAPIReceiver);
		}

		self.task_state.clear_runtime_error();
		let api_receiver = self.api_receiver.clone().unwrap();
		let event_sender = self.event_sender.clone().unwrap();

		let state = Arc::new(AppState {
			access_token: self.access_token.clone(),
			api_receiver,
			event_sender,
			connected: Arc::new(std::sync::atomic::AtomicBool::new(false)),
			connection_handle: Arc::new(Mutex::new(None)),
			task_state: self.task_state.clone(),
		});
		self.app_state = Some(state.clone());

		let listener = match TcpListener::bind(self.addr.clone()).await {
			Ok(listener) => listener,
			Err(err) => {
				self.task_state.stop();
				return Err(err.into());
			}
		};
		let router = Router::new().fallback(any(handler)).with_state(state);

		let task_state = self.task_state.clone();
		self.serve_handle = Some(tokio::spawn(async move {
			let _guard = ServiceTaskGuard::new(task_state.clone());
			match axum::serve(listener, router)
				.await
				.map_err(ServiceRuntimeError::from)
			{
				Ok(()) => Ok(()),
				Err(err) => {
					task_state.record_runtime_error(err);
					Ok(())
				}
			}
		}));

		Ok(())
	}

	fn is_running(&self) -> bool {
		self.task_state.is_running()
	}

	fn take_runtime_error(&mut self) -> Option<ServiceRuntimeError> {
		self.task_state.take_runtime_error()
	}
}
