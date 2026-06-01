use super::utils::*;
use crate::error::{
	ServiceRuntimeError, ServiceRuntimeResult, ServiceStartError, ServiceStartResult,
};
use async_trait::async_trait;
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::any;
use hmac::{Hmac, KeyInit, Mac};
use http::{HeaderMap, StatusCode};
use sha1::Sha1;
use std::sync::Arc;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::JoinHandle;

type HmacSha1 = Hmac<Sha1>;

pub struct HttpPostService<T: ToSocketAddrs + Clone + Send + Sync> {
	addr: T,
	hmac: Option<HmacSha1>,
	event_sender: Option<InternalEventSender>,
	task_handle: Option<JoinHandle<ServiceRuntimeResult<()>>>,
	prefix: String,
	task_state: Arc<ServiceTaskState>,
}

impl<T: ToSocketAddrs + Clone + Send + Sync> Drop for HttpPostService<T> {
	fn drop(&mut self) {
		self.uninstall();
	}
}

impl<T: ToSocketAddrs + Clone + Send + Sync> HttpPostService<T> {
	pub fn new(addr: T, prefix: Option<String>, secret: Option<String>) -> ServiceStartResult<Self> {
		let hmac = if let Some(secret) = secret {
			Some(HmacSha1::new_from_slice(secret.as_ref())?)
		} else {
			None
		};
		let mut prefix = prefix.unwrap_or("/".to_string());
		if !prefix.starts_with("/") {
			prefix = "/".to_string() + &prefix;
		}
		Ok(Self {
			addr,
			hmac,
			event_sender: None,
			task_handle: None,
			prefix,
			task_state: Arc::new(ServiceTaskState::new()),
		})
	}
}

struct AppState {
	hmac: Option<HmacSha1>,
	event_sender: InternalEventSender,
}

pub fn get_sig(mut hmac: HmacSha1, content: &[u8]) -> String {
	hmac.update(content);
	let result = hmac.finalize().into_bytes();
	hex::encode(result)
}

async fn processor(
	headers: HeaderMap,
	State(state): State<Arc<AppState>>,
	body: String,
) -> impl IntoResponse {
	if let Some(hmac) = state.hmac.clone() {
		let Some(received_sig) = headers.get("X-Signature") else {
			return StatusCode::UNAUTHORIZED;
		};
		let Ok(received_sig) = received_sig.to_str() else {
			return StatusCode::BAD_REQUEST;
		};
		let sig = get_sig(hmac, body.as_ref());
		if received_sig != "sha1=".to_string() + sig.as_str() {
			return StatusCode::FORBIDDEN;
		}
	}
	let Ok(event) = serde_json::from_str(&body) else {
		return StatusCode::BAD_REQUEST;
	};
	let _ = state.event_sender.send_async(event).await;
	StatusCode::NO_CONTENT
}

#[async_trait]
impl<T: ToSocketAddrs + Clone + Send + Sync> CommunicationService for HttpPostService<T> {
	fn install(&mut self, _api_receiver: InternalAPIReceiver, event_sender: InternalEventSender) {
		self.event_sender = Some(event_sender);
	}

	fn uninstall(&mut self) {
		self.stop();
		self.event_sender = None;
	}

	fn stop(&mut self) {
		if let Some(handle) = self.task_handle.take() {
			handle.abort();
		}
		self.task_state.stop();
	}

	async fn start(&mut self) -> ServiceStartResult<()> {
		if !self.task_state.try_start() {
			return Err(ServiceStartError::TaskIsRunning);
		}

		if self.event_sender.is_none() {
			self.task_state.stop();
			return Err(ServiceStartError::NotInjectedEventSender);
		}

		self.task_state.clear_runtime_error();
		let event_sender = self.event_sender.clone().unwrap();

		let state = Arc::new(AppState {
			event_sender,
			hmac: self.hmac.clone(),
		});

		let listener = match TcpListener::bind(self.addr.clone()).await {
			Ok(listener) => listener,
			Err(err) => {
				self.task_state.stop();
				return Err(err.into());
			}
		};
		let router = Router::new()
			.route(&self.prefix, any(processor))
			.with_state(state);

		let task_state = self.task_state.clone();
		self.task_handle = Some(tokio::spawn(async move {
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
