use std::ops::ControlFlow;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

use flume::r#async::RecvStream;
use futures::future::FusedFuture;
use futures::stream::FusedStream;
use futures::{Sink, Stream};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::{Message, Result as WebSocketResult};

use crate::communication::utils::{
	APIRequest, DeserializedEvent, InternalAPIReceiver, InternalEventSender,
};

pub(super) struct WsTransfer<'a, 'b, S> {
	ws: WebSocketStream<S>,
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

impl<'a, 'b, S: AsyncRead + AsyncWrite + Unpin> WsTransfer<'a, 'b, S> {
	pub fn new(
		ws: WebSocketStream<S>,
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

	fn poll_upload_one_event(&mut self, cx: &mut Context<'_>) -> Poll<WebSocketResult<()>> {
		let mut ws = Pin::new(&mut self.ws);
		ready!(ws.as_mut().poll_ready(cx)?);
		let api_stream = Pin::new(&mut self.api_stream);
		match ready!(api_stream.poll_next(cx)) {
			Some(event) => {
				let Ok(msg) = serde_json::to_string(&event) else {
					return Poll::Ready(Ok(()));
				};
				ws.as_mut().start_send(Message::Text(msg.into()))?;
				self.upload_state = UploadState::Flushing;
				Poll::Ready(Ok(()))
			}
			// api_receiver 对面已经关了，但是 event_sender 还不一定。
			// 这时我们不应该立刻关闭连接，而是继续等待 event_sender 也断开。
			None => {
				if self.event_sender.is_disconnected() {
					self.initiate_close();
				}
				Poll::Pending
			}
		}
	}

	fn poll_progress(&mut self, cx: &mut Context<'_>) -> Poll<WebSocketResult<ControlFlow<()>>> {
		loop {
			let ws = Pin::new(&mut self.ws);
			match self.upload_state {
				UploadState::AwaitingEvent => {
					if self.poll_upload_one_event(cx)?.is_ready() {
						continue;
					}
				}
				UploadState::Flushing => {
					if ws.poll_flush(cx)?.is_ready() {
						self.upload_state = UploadState::AwaitingEvent;
						continue;
					}
				}
				UploadState::ClosingByLocal => {
					let close_result = ready!(ws.poll_close(cx));
					self.upload_state = UploadState::ClosedByLocal;
					if close_result.is_err() {
						return Poll::Ready(Ok(ControlFlow::Break(())));
					}
				}
				UploadState::ClosedByLocal => match ready!(ws.poll_next(cx)) {
					None | Some(Ok(Message::Close(_))) | Some(Err(_)) => {
						return Poll::Ready(Ok(ControlFlow::Break(())));
					}
					Some(Ok(_)) => continue,
				},
				UploadState::ClosedByPeer => {
					ready!(ws.poll_close(cx)?);
					return Poll::Ready(Ok(ControlFlow::Continue(())));
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
				None | Some(Ok(Message::Close(_))) => self.upload_state = UploadState::ClosedByPeer,
				Some(Err(e)) => return Poll::Ready(Err(e)),
				Some(_) => (),
			}
		}
	}

	pub fn initiate_close(&mut self) {
		if self.upload_state != UploadState::ClosedByLocal {
			self.upload_state = UploadState::ClosingByLocal;
		}
	}
}

impl<'a, 'b, S: AsyncRead + AsyncWrite + Unpin> Future for WsTransfer<'a, 'b, S> {
	type Output = WebSocketResult<ControlFlow<()>>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.poll_progress(cx)
	}
}

impl<'a, 'b, S: AsyncRead + AsyncWrite + Unpin> FusedFuture for WsTransfer<'a, 'b, S> {
	fn is_terminated(&self) -> bool {
		self.ws.is_terminated()
	}
}
