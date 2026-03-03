use crate::communication::utils::*;
use crate::error::{ServiceStartError, ServiceStartResult};
use async_trait::async_trait;
use tokio::select;
use tokio::sync::broadcast;

/// 将事件接收与API发送分为两个不同服务实现  
/// 服务分为 `send_side` 与 `read_side`  
/// 其中，`send_side` 负责API发送服务，`read_side` 负责事件接收服务  
/// `send_side` 的事件通道由一个 processor task 负责  
/// processor 将 `send_side` 的API响应事件并入原事件通道，其余事件丢弃
/// # Examples
/// ```rust
/// use std::time::Duration;
/// use onebot_api::communication::http::HttpService;
/// use onebot_api::communication::sse::SseService;
/// use onebot_api::communication::combiner::SplitCombiner;
/// use onebot_api::communication::utils::Client;
///
/// let http_service = HttpService::new("https://example.com", Some("example_token".to_string())).unwrap();
/// let sse_service = SseService::new("https://example.com/_events", Some("example_token".to_string())).unwrap();
/// let combiner = SplitCombiner::new(http_service, sse_service);
/// let client = Client::new(combiner, Duration::from_secs(5), None, None);
/// client.start_service().await.unwrap();
/// ```
pub struct SplitCombiner<S: CommunicationService, R: CommunicationService> {
	send_side: S,
	read_side: R,
	event_process_sender: EventSender,
	event_sender: Option<EventSender>,
	close_signal_sender: broadcast::Sender<()>,
}

impl<S: CommunicationService, R: CommunicationService> Drop for SplitCombiner<S, R> {
	fn drop(&mut self) {
		let _ = self.close_signal_sender.send(());
	}
}

impl<S: CommunicationService, R: CommunicationService> SplitCombiner<S, R> {
	pub fn new(send_side: S, read_side: R) -> Self {
		let (event_process_sender, _) = broadcast::channel(16);
		let (close_signal_sender, _) = broadcast::channel(1);
		Self {
			send_side,
			read_side,
			event_process_sender,
			event_sender: None,
			close_signal_sender,
		}
	}
}

#[async_trait]
impl<S: CommunicationService, R: CommunicationService> CommunicationService
	for SplitCombiner<S, R>
{
	fn inject(&mut self, api_receiver: APIReceiver, event_sender: EventSender) {
		let (_, empty_api_receiver) = flume::bounded(1);
		self
			.send_side
			.inject(api_receiver, self.event_process_sender.clone());
		self
			.read_side
			.inject(empty_api_receiver, event_sender.clone());
		self.event_sender = Some(event_sender);
	}

	async fn start_service(&self) -> ServiceStartResult<()> {
		async fn processor(
			mut close_signal: broadcast::Receiver<()>,
			mut event_process_receiver: EventReceiver,
			event_sender: EventSender,
		) -> anyhow::Result<()> {
			loop {
				select! {
					_ = close_signal.recv() => return Err(anyhow::anyhow!("close")),
					Ok(data) = event_process_receiver.recv() => {
						if data.is_api_response() {
							let _ = event_sender.send(data);
						}
					}
				}
			}
		}

		if self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjectedEventSender);
		}
		let event_sender = self.event_sender.clone().unwrap();

		tokio::spawn(processor(
			self.close_signal_sender.subscribe(),
			self.event_process_sender.subscribe(),
			event_sender,
		));

		futures::try_join!(
			self.send_side.start_service(),
			self.read_side.start_service()
		)?;
		Ok(())
	}
}

/// 详见 [`SplitCombiner`]  
/// 与 `SplitCombiner` 的区别在于  
/// `BothEventCombiner` 会将 `send_side` 的所有事件均并入原事件通道  
/// 因此，`BothEventCombiner` 不存在 processor task
pub struct BothEventCombiner<S: CommunicationService, R: CommunicationService> {
	send_side: S,
	read_side: R,
}

impl<S: CommunicationService, R: CommunicationService> BothEventCombiner<S, R> {
	pub fn new(send_side: S, read_side: R) -> Self {
		Self {
			send_side,
			read_side,
		}
	}
}

impl<S: CommunicationService, R: CommunicationService> Drop for BothEventCombiner<S, R> {
	fn drop(&mut self) {}
}

#[async_trait]
impl<S: CommunicationService, R: CommunicationService> CommunicationService
	for BothEventCombiner<S, R>
{
	fn inject(&mut self, api_receiver: APIReceiver, event_sender: EventSender) {
		let (_, empty_api_receiver) = flume::bounded(1);
		self.send_side.inject(api_receiver, event_sender.clone());
		self.read_side.inject(empty_api_receiver, event_sender);
	}

	async fn start_service(&self) -> ServiceStartResult<()> {
		futures::try_join!(
			self.send_side.start_service(),
			self.read_side.start_service()
		)?;
		Ok(())
	}
}
