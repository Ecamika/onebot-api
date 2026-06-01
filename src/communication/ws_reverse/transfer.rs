use std::pin::Pin;
use std::task::{Context, Poll, ready};

use axum::extract::ws::{Message, WebSocket};
use flume::r#async::RecvStream;
use futures::{Sink, Stream};

use crate::communication::utils::{
	APIRequest, DeserializedEvent, InternalAPIReceiver, InternalEventSender,
};
use crate::error::{ServiceRuntimeError, ServiceRuntimeResult};

pub(super) struct ReverseWsTransfer<'a, 'b> {
	ws: WebSocket,
	api_stream: RecvStream<'a, APIRequest>,
	event_sender: &'b InternalEventSender,
	upload_state: UploadState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UploadState {
	AwaitingEvent,
	Flushing,
	ClosingByLocal,
	ClosedByLocal,
	ClosedByPeer,
}

impl<'a, 'b> ReverseWsTransfer<'a, 'b> {
	pub fn new(
		ws: WebSocket,
		api_receiver: &'a InternalAPIReceiver,
		event_sender: &'b InternalEventSender,
	) -> Self {
		Self {
			ws,
			api_stream: api_receiver.stream(),
			event_sender,
			upload_state: UploadState::AwaitingEvent,
		}
	}

	fn initiate_close(&mut self) {
		if self.upload_state != UploadState::ClosedByLocal {
			self.upload_state = UploadState::ClosingByLocal;
		}
	}

	fn poll_upload_one_event(&mut self, cx: &mut Context<'_>) -> Poll<ServiceRuntimeResult<()>> {
		let mut ws = Pin::new(&mut self.ws);
		ready!(ws.as_mut().poll_ready(cx)).map_err(|_| ServiceRuntimeError::WebSocketError)?;
		let api_stream = Pin::new(&mut self.api_stream);
		match ready!(api_stream.poll_next(cx)) {
			Some(event) => {
				let Ok(msg) = serde_json::to_string(&event) else {
					return Poll::Ready(Ok(()));
				};
				ws.as_mut()
					.start_send(Message::Text(msg.into()))
					.map_err(|_| ServiceRuntimeError::WebSocketError)?;
				self.upload_state = UploadState::Flushing;
				Poll::Ready(Ok(()))
			}
			None => {
				self.initiate_close();
				Poll::Ready(Ok(()))
			}
		}
	}

	fn poll_progress(&mut self, cx: &mut Context<'_>) -> Poll<ServiceRuntimeResult<()>> {
		loop {
			let ws = Pin::new(&mut self.ws);
			match self.upload_state {
				UploadState::AwaitingEvent => {
					if self.poll_upload_one_event(cx)?.is_ready() {
						continue;
					}
				}
				UploadState::Flushing => {
					ready!(ws.poll_flush(cx)).map_err(|_| ServiceRuntimeError::WebSocketError)?;
					self.upload_state = UploadState::AwaitingEvent;
					continue;
				}
				UploadState::ClosingByLocal => {
					ready!(ws.poll_close(cx)).map_err(|_| ServiceRuntimeError::WebSocketError)?;
					self.upload_state = UploadState::ClosedByLocal;
					return Poll::Ready(Ok(()));
				}
				UploadState::ClosedByLocal => return Poll::Ready(Ok(())),
				UploadState::ClosedByPeer => {
					ready!(ws.poll_close(cx)).map_err(|_| ServiceRuntimeError::WebSocketError)?;
					return Poll::Ready(Err(ServiceRuntimeError::WebSocketClosedByPeer));
				}
			}

			let ws = Pin::new(&mut self.ws);
			match ready!(ws.poll_next(cx)) {
				Some(Ok(Message::Text(msg))) => {
					let Ok(event) = serde_json::from_str::<DeserializedEvent>(msg.as_str()) else {
						continue;
					};
					self.event_sender.send(event).ok();
				}
				Some(Ok(Message::Close(_))) | None => self.upload_state = UploadState::ClosedByPeer,
				Some(Err(_)) => return Poll::Ready(Err(ServiceRuntimeError::WebSocketError)),
				Some(Ok(_)) => (),
			}
		}
	}
}

impl<'a, 'b> Future for ReverseWsTransfer<'a, 'b> {
	type Output = ServiceRuntimeResult<()>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.poll_progress(cx)
	}
}
