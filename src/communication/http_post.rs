use super::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use axum::Router;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::any;
use hmac::{Hmac, KeyInit, Mac};
use http::{HeaderMap, StatusCode};
use sha1::Sha1;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::sync::broadcast;

type HmacSha1 = Hmac<Sha1>;

pub struct HttpPostService<T: ToSocketAddrs + Clone + Send + Sync> {
	addr: T,
	hmac: Option<HmacSha1>,
	event_sender: Option<InternalEventSender>,
	close_signal_sender: broadcast::Sender<()>,
	prefix: String,
	is_running: Arc<AtomicBool>,
}

impl<T: ToSocketAddrs + Clone + Send + Sync> Drop for HttpPostService<T> {
	fn drop(&mut self) {
		self.uninstall();
	}
}

impl<T: ToSocketAddrs + Clone + Send + Sync> HttpPostService<T> {
	pub fn new(addr: T, prefix: Option<String>, secret: Option<String>) -> anyhow::Result<Self> {
		let (close_signal_sender, _) = broadcast::channel(1);
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
			close_signal_sender,
			prefix,
			is_running: Arc::new(AtomicBool::new(false)),
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
	if state.hmac.is_some() {
		let received_sig = headers.get("X-Signature").map(|v| v.to_str().unwrap());
		if received_sig.is_none() {
			return StatusCode::UNAUTHORIZED;
		}
		let received_sig = received_sig.unwrap();
		let hmac = state.hmac.clone().unwrap();
		let sig = get_sig(hmac, body.as_ref());
		if received_sig != "sha1=".to_string() + sig.as_str() {
			return StatusCode::FORBIDDEN;
		}
	}
	let event = serde_json::from_str(&body).unwrap();
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

	fn stop(&self) {
		let _ = self.close_signal_sender.send(());
		self.is_running.store(false, Ordering::Relaxed);
	}

	async fn start(&self) -> ServiceStartResult<()> {
		if self.is_running.load(Ordering::Relaxed) {
			return Err(ServiceStartError::TaskIsRunning);
		}

		if self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjectedEventSender);
		}

		let event_sender = self.event_sender.clone().unwrap();

		let state = Arc::new(AppState {
			event_sender,
			hmac: self.hmac.clone(),
		});

		let listener = TcpListener::bind(self.addr.clone()).await?;
		let router = Router::new()
			.route(&self.prefix, any(processor))
			.with_state(state);
		let mut close_signal = self.close_signal_sender.subscribe();

		self.is_running.store(true, Ordering::Relaxed);
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
