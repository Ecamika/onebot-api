use crate::communication::utils::*;
use crate::error::{
	ServiceRuntimeError, ServiceRuntimeResult, ServiceStartError, ServiceStartResult,
};
use async_trait::async_trait;
use tokio::task::JoinHandle;

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
/// let client = Client::new_with_options(combiner, Duration::from_secs(5), None, None);
/// client.start_service().await.unwrap();
/// ```
pub struct SplitCombiner<S: CommunicationService, R: CommunicationService> {
	send_side: S,
	read_side: R,
	event_process_sender: InternalEventSender,
	event_process_receiver: InternalEventReceiver,
	event_sender: Option<InternalEventSender>,
	task_handle: Option<JoinHandle<()>>,
}

impl<S: CommunicationService, R: CommunicationService> Drop for SplitCombiner<S, R> {
	fn drop(&mut self) {
		self.uninstall();
	}
}

impl<S: CommunicationService, R: CommunicationService> SplitCombiner<S, R> {
	pub fn new(send_side: S, read_side: R) -> Self {
		let (event_process_sender, event_process_receiver) = flume::bounded(16);
		Self {
			send_side,
			read_side,
			event_process_sender,
			event_process_receiver,
			event_sender: None,
			task_handle: None,
		}
	}
}

#[async_trait]
impl<S: CommunicationService, R: CommunicationService> CommunicationService
	for SplitCombiner<S, R>
{
	fn install(&mut self, api_receiver: InternalAPIReceiver, event_sender: InternalEventSender) {
		let (_, empty_api_receiver) = flume::bounded(1);
		self
			.send_side
			.install(api_receiver, self.event_process_sender.clone());
		self
			.read_side
			.install(empty_api_receiver, event_sender.clone());
		self.event_sender = Some(event_sender);
	}

	fn uninstall(&mut self) {
		self.stop();
		self.read_side.uninstall();
		self.send_side.uninstall();
		self.event_sender = None;
	}

	fn stop(&mut self) {
		if let Some(handle) = self.task_handle.take() {
			handle.abort();
		}
		self.send_side.stop();
		self.read_side.stop();
	}

	async fn start(&mut self) -> ServiceStartResult<()> {
		async fn processor(
			event_process_receiver: InternalEventReceiver,
			event_sender: InternalEventSender,
		) -> ServiceRuntimeResult<()> {
			loop {
				match event_process_receiver.recv_async().await {
					Ok(data) => {
						if data.is_api_response() {
							let _ = event_sender.send(data);
						}
					}
					Err(_) => return Err(ServiceRuntimeError::ChannelClosed),
				}
			}
		}

		if self.event_sender.is_none() {
			return Err(ServiceStartError::NotInjectedEventSender);
		}
		let event_sender = self.event_sender.clone().unwrap();
		let event_process_receiver = self.event_process_receiver.clone();

		self.task_handle = Some(tokio::spawn(async move {
			processor(event_process_receiver, event_sender).await.ok();
		}));

		futures::try_join!(self.send_side.start(), self.read_side.start())?;
		Ok(())
	}

	async fn restart(&mut self) -> ServiceStartResult<()> {
		futures::try_join!(self.send_side.restart(), self.read_side.restart())?;
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
	fn install(&mut self, api_receiver: InternalAPIReceiver, event_sender: InternalEventSender) {
		let (_, empty_api_receiver) = flume::bounded(1);
		self.send_side.install(api_receiver, event_sender.clone());
		self.read_side.install(empty_api_receiver, event_sender);
	}

	fn uninstall(&mut self) {
		self.send_side.uninstall();
		self.read_side.uninstall();
	}

	fn stop(&mut self) {
		self.send_side.stop();
		self.read_side.stop();
	}

	async fn start(&mut self) -> ServiceStartResult<()> {
		futures::try_join!(self.send_side.start(), self.read_side.start())?;
		Ok(())
	}

	async fn restart(&mut self) -> ServiceStartResult<()> {
		futures::try_join!(self.send_side.restart(), self.read_side.restart())?;
		Ok(())
	}
}
