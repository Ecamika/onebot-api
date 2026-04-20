use crate::api::APISender as APISenderTrait;
use crate::api::arg_type::MessageType;
use crate::api::return_type::*;
use crate::error::{APIRequestError, APIResult, ServiceRuntimeError, ServiceStartResult};
pub use crate::event::Event as NormalEvent;
use crate::event::EventReceiver as EventReceiverTrait;
use crate::event::EventTrait;
use crate::event::message::GroupMessageAnonymous;
use crate::message::receive_segment::ReceiveSegment;
use crate::message::send_segment::SendSegment;
use async_trait::async_trait;
pub use flume::Receiver as FlumeReceiver;
pub use flume::Sender as FlumeSender;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use strum::EnumIs;
use tokio::select;
use tokio::sync::broadcast;
pub use tokio::sync::broadcast::Receiver as BroadcastReceiver;
pub use tokio::sync::broadcast::Sender as BroadcastSender;

/// `Client` 与具体 `CommunicationService` 中  
/// API请求的发送通道  
/// 应由 `Client` 持有
pub type InternalAPISender = FlumeSender<APIRequest>;
/// `Client` 与具体 `CommunicationService` 中  
/// API请求的接收通道  
/// 应由 `CommunicationService` 持有
pub type InternalAPIReceiver = FlumeReceiver<APIRequest>;

/// `Client` 与具体 `CommunicationService` 中  
/// 原始事件的发送通道  
/// 应由 `CommunicationService` 持有
pub type InternalEventSender = FlumeSender<DeserializedEvent>;
/// `Client` 与具体 `CommunicationService` 中  
/// 原始事件的接收通道  
/// 应由 `Client` 持有
pub type InternalEventReceiver = FlumeReceiver<DeserializedEvent>;
/// `Client` 与具体 `RawEventProcessor` 中  
/// API响应的发送通道  
/// 应由 `Client` 持有
type InternalAPIResponseSender = tokio::sync::oneshot::Sender<Arc<APIResponse>>;

/// `Client` 与具体使用者之间  
/// API响应的发送通道  
/// 应由 `Client` 持有
pub type PublicAPIResponseSender = BroadcastSender<ArcAPIResponse>;

/// `Client` 与具体使用者之间  
/// 事件（除去API响应）的发送通道  
/// 应由 `Client` 持有
pub type PublicEventSender = BroadcastSender<ArcNormalEvent>;
/// `Client` 与具体使用者之间  
/// 事件（除去API响应）的接收通道  
/// 公开，任何人都可持有
pub type PublicEventReceiver = BroadcastReceiver<ArcNormalEvent>;

pub type ServiceRuntimeResult<T> = Result<T, ServiceRuntimeError>;

pub type ArcServiceRuntimeError = Arc<ServiceRuntimeError>;
pub type ArcAPIResponse = Arc<APIResponse>;
type ArcAPIRequestRegistry = Arc<Mutex<BTreeMap<String, InternalAPIResponseSender>>>;
pub type ArcNormalEvent = Arc<NormalEvent>;

pub const DEFAULT_CHANNEL_CAP: usize = 16;

impl EventTrait for APIResponse {}
impl EventTrait for ArcAPIResponse {}
impl EventTrait for ArcNormalEvent {}

#[derive(Deserialize, Clone, Debug, EnumIs)]
#[serde(untagged)]
/// 经由 `CommunicationService` 初步反序列化后的原始事件
pub enum DeserializedEvent {
	APIResponse(APIResponse),
	Event(JsonValue),
}

#[derive(Serialize, Clone, Debug)]
/// 由 `Client` 发出的API请求
pub struct APIRequest {
	pub action: String,
	pub params: JsonValue,
	pub echo: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
/// 由 `CommunicationService` 返回的API响应
pub struct APIResponse {
	pub status: String,
	pub retcode: i32,
	pub data: JsonValue,
	pub echo: Option<String>,
}

impl APIResponse {
	pub fn verify(&self) -> bool {
		self.status == "ok"
	}

	pub fn parse_data<T: DeserializeOwned>(&self) -> APIResult<T> {
		if !self.verify() {
			return Err(APIRequestError::HttpError { code: self.retcode });
		}
		Ok(serde_json::from_value(self.data.clone())?)
	}
}

#[async_trait]
pub trait CommunicationService: Sync + Send {
	/// 安装服务  
	/// 该方法仅进行服务的依赖注入，并不真正创建任务  
	/// **应具备幂等性**
	fn install(&mut self, api_receiver: InternalAPIReceiver, event_sender: InternalEventSender);

	/// 卸载服务  
	/// 服务应回收安装后产生的一切副作用和安装时注入的依赖  
	/// **应具备幂等性**
	fn uninstall(&mut self);

	/// 停止服务  
	/// 不同于 `uninstall` 方法 ，`stop` 方法仅需要回收安装后产生的副作用，并不需要回收安装时注入的依赖  
	/// **应具备幂等性**
	fn stop(&self);

	/// 开始服务  
	/// 服务内应保证任务在服务的生命周期内最多存在一个  
	/// 由于在服务生命周期内最多存在一个任务，所以该方法 **应具备幂等性**
	async fn start(&self) -> ServiceStartResult<()>;

	/// 重启服务  
	/// **不具备幂等性**
	async fn restart(&self) -> ServiceStartResult<()> {
		self.stop();
		self.start().await
	}
}

#[async_trait]
impl CommunicationService for Box<dyn CommunicationService> {
	fn install(&mut self, api_receiver: InternalAPIReceiver, event_sender: InternalEventSender) {
		(**self).install(api_receiver, event_sender);
	}

	fn uninstall(&mut self) {
		(**self).uninstall();
	}

	fn stop(&self) {
		(**self).stop();
	}

	async fn start(&self) -> ServiceStartResult<()> {
		(**self).start().await
	}

	async fn restart(&self) -> ServiceStartResult<()> {
		(**self).restart().await
	}
}

pub trait IntoService {
	fn into(self) -> impl CommunicationService + 'static;
}

impl<T: CommunicationService + 'static> IntoService for T {
	fn into(self) -> impl CommunicationService + 'static {
		self
	}
}

/// 对于Onebot V11协议API调用和事件接收的高层抽象  
/// 需要具体实现 [`CommunicationService`] 的底层服务支持
///
/// # Examples
/// ```rust
/// use std::time::Duration;
/// use onebot_api::communication::utils::Client;
/// use onebot_api::communication::ws::WsService;
///
/// let ws_service = WsService::new_with_options("wss://example.com", Some("example_token".to_string())).unwrap();
/// let client = Client::new_with_options(ws_service, Duration::from_secs(5), None, None);
/// client.start_service().await.unwrap();
/// ```
pub struct Client {
	service: Box<dyn CommunicationService>,
	internal_api_sender: InternalAPISender,
	internal_api_receiver: InternalAPIReceiver,
	internal_event_sender: InternalEventSender,
	// internal_event_receiver: InternalEventReceiver,
	api_request_registry: ArcAPIRequestRegistry,
	public_event_sender: PublicEventSender,
	timeout: Option<Duration>,
	echo_generator: Box<dyn Fn() -> String + Send + Sync>,
	close_signal_sender: broadcast::Sender<()>,
}

pub struct ClientBuilder {
	service: Box<dyn CommunicationService>,
	timeout: Option<Duration>,
	public_event_channel_cap: Option<usize>,
	internal_api_channel_cap: Option<usize>,
	internal_event_channel_cap: Option<usize>,
	echo_generator: Option<Box<dyn Fn() -> String + Send + Sync>>,
}

impl ClientBuilder {
	pub fn new(service: impl IntoService) -> Self {
		Self {
			service: Box::new(service.into()),
			timeout: None,
			public_event_channel_cap: None,
			internal_api_channel_cap: None,
			internal_event_channel_cap: None,
			echo_generator: None,
		}
	}

	pub fn build(self) -> Client {
		Client::new_with_options(
			self.service,
			self.timeout,
			self.public_event_channel_cap,
			self.internal_api_channel_cap,
			self.internal_event_channel_cap,
			self.echo_generator,
		)
	}

	pub fn timeout(mut self, timeout: Duration) -> Self {
		self.timeout = Some(timeout);
		self
	}

	pub fn public_event_channel_cap(mut self, cap: usize) -> Self {
		self.public_event_channel_cap = Some(cap);
		self
	}

	pub fn internal_event_channel_cap(mut self, cap: usize) -> Self {
		self.internal_event_channel_cap = Some(cap);
		self
	}

	pub fn internal_api_channel_cap(mut self, cap: usize) -> Self {
		self.internal_api_channel_cap = Some(cap);
		self
	}

	pub fn union_channel_cap(self, cap: usize) -> Self {
		self
			.public_event_channel_cap(cap)
			.internal_event_channel_cap(cap)
			.internal_api_channel_cap(cap)
	}

	pub fn public_union_channel_cap(self, cap: usize) -> Self {
		self.public_event_channel_cap(cap)
	}

	pub fn internal_union_channel_cap(self, cap: usize) -> Self {
		self
			.internal_event_channel_cap(cap)
			.internal_api_channel_cap(cap)
	}

	pub fn echo_generator(mut self, generator: Box<dyn Fn() -> String + Send + Sync>) -> Self {
		self.echo_generator = Some(generator);
		self
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		self.service.uninstall();
		let _ = self.close_signal_sender.send(());
	}
}

impl Client {
	pub fn subscribe_normal_event(&self) -> PublicEventReceiver {
		self.public_event_sender.subscribe()
	}
}

impl EventReceiverTrait<ArcNormalEvent> for Client {
	fn subscribe(&self) -> PublicEventReceiver {
		self.subscribe_normal_event()
	}
}

impl<T: IntoService> From<T> for Client {
	fn from(value: T) -> Self {
		Self::new(value)
	}
}

impl Client {
	/// 创建一个 [`Client`] 实例
	///
	/// # Params
	/// - `service` 实现 [`IntoService`] 特征或 [`CommunicationService`] 特征的对象
	/// - `timeout` API请求超时时间，若为 `None` 则一直等待
	/// - `public_api_response_channel_cap` API响应通道容量，默认为`16`
	/// - `public_event_channel_cap` Event事件通道容量，默认为`16`
	/// - `internal_api_channel_cap` API请求通道容量，默认为`16`
	/// - `internal_event_channel_cap` 原始事件通道容量，默认为`16`
	pub fn new_with_options(
		service: impl IntoService,
		timeout: Option<Duration>,
		public_event_channel_cap: Option<usize>,
		internal_api_channel_cap: Option<usize>,
		internal_event_channel_cap: Option<usize>,
		echo_generator: Option<Box<dyn Fn() -> String + Send + Sync>>,
	) -> Self {
		let get_cap = |v: Option<usize>| v.unwrap_or(DEFAULT_CHANNEL_CAP);

		let mut service = Box::new(service.into());
		let (internal_api_sender, internal_api_receiver) =
			flume::bounded(get_cap(internal_api_channel_cap));
		let (internal_event_sender, internal_event_receiver) =
			flume::bounded(get_cap(internal_event_channel_cap));
		let (public_event_sender, _) = broadcast::channel(get_cap(public_event_channel_cap));
		service.install(internal_api_receiver.clone(), internal_event_sender.clone());

		let (close_signal_sender, _) = broadcast::channel(1);
		let api_request_registry = ArcAPIRequestRegistry::default();

		tokio::spawn(Self::raw_event_processor(
			close_signal_sender.subscribe(),
			internal_event_receiver.clone(),
			api_request_registry.clone(),
			public_event_sender.clone(),
		));

		Self {
			service,
			timeout,
			internal_api_receiver,
			internal_api_sender,
			// internal_event_receiver,
			internal_event_sender,
			public_event_sender,
			echo_generator: echo_generator.unwrap_or(Box::new(Self::generate_id)),
			close_signal_sender,
			api_request_registry,
		}
	}

	pub fn new_with_union_channel_cap(
		service: impl IntoService,
		timeout: Option<Duration>,
		channel_cap: Option<usize>,
	) -> Self {
		Self::new_with_options(
			service,
			timeout,
			channel_cap,
			channel_cap,
			channel_cap,
			None,
		)
	}

	pub fn new_with_timeout(service: impl IntoService, timeout: Option<Duration>) -> Self {
		Self::new_with_union_channel_cap(service, timeout, None)
	}

	pub fn new(service: impl IntoService) -> Self {
		Self::new_with_timeout(service, Some(Duration::from_secs(5)))
	}

	pub fn builder(service: impl IntoService) -> ClientBuilder {
		ClientBuilder::new(service)
	}
}

impl Client {
	async fn raw_event_processor(
		mut close_signal: broadcast::Receiver<()>,
		internal_event_receiver: InternalEventReceiver,
		api_request_registry: ArcAPIRequestRegistry,
		public_event_sender: PublicEventSender,
	) -> anyhow::Result<()> {
		loop {
			select! {
				_ = close_signal.recv() => return Err(anyhow::anyhow!("close")),
				event = internal_event_receiver.recv_async() => {
					match event {
						Ok(DeserializedEvent::APIResponse(v)) => {
							let Some(response_channel) = v.echo.as_ref().and_then(|echo| {
								let mut registry = api_request_registry.lock().unwrap();
								registry.remove(echo)
							}) else {
								continue
							};
							response_channel.send(Arc::new(v)).ok();
						},
						Ok(DeserializedEvent::Event(v)) => {
							let v = serde_json::from_value(v);
							if v.is_err() {
								continue
							}
							let _ = public_event_sender.send(Arc::new(v?));
						},
						Err(_) => return Err(anyhow::anyhow!("internal event channel closed")),
					}
				}
			}
		}
	}

	/// 启动服务  
	/// 在 `Client` 实例构造完成或调用 `change_service` 后都需要调用该方法启动服务
	pub async fn start_service(&self) -> ServiceStartResult<()> {
		self.service.start().await
	}

	pub fn stop_service(&self) {
		self.service.stop();
	}

	pub async fn restart_service(&self) -> ServiceStartResult<()> {
		self.service.restart().await
	}

	/// 更换服务  
	/// 即使在原服务启动后也可以更换服务
	///
	/// # Examples
	/// ```rust
	/// use std::time::Duration;
	/// use onebot_api::communication::utils::Client;
	/// use onebot_api::communication::ws::WsService;
	/// use onebot_api::communication::ws_reverse::WsReverseService;
	///
	/// let ws_service = WsService::new_with_options("wss://example.com", Some("example_token".to_string())).unwrap();
	/// let mut client = Client::new_with_options(ws_service, Duration::from_secs(5), None, None);
	/// client.start_service().await.unwrap();
	///
	/// let ws_reverse_service = WsReverseService::new("0.0.0.0:8080", Some("example_token".to_string()));
	/// client.change_service(ws_reverse_service);
	/// client.start_service().await.unwrap();
	/// ```
	pub fn change_service(&mut self, service: impl IntoService) -> Box<dyn CommunicationService> {
		let mut service = Box::new(service.into());
		service.install(
			self.internal_api_receiver.clone(),
			self.internal_event_sender.clone(),
		);
		self.service.uninstall();
		std::mem::replace(&mut self.service, service)
	}

	/// 获取当前服务的引用
	pub fn get_service(&self) -> &dyn CommunicationService {
		&*self.service
	}

	/// 获取当前服务的可变引用
	pub fn get_service_mut(&mut self) -> &mut dyn CommunicationService {
		&mut *self.service
	}
}

impl Client {
	/// 随机生成uuid格式的id  
	/// 用于echo的生成  
	/// ---
	/// 不要说用uuid有几率冲突  
	/// 一个请求就那么点时间  
	/// 这么点时间能产生一个uuid冲突建议你直接去买彩票
	pub fn generate_id() -> String {
		uuid::Uuid::new_v4().to_string()
	}

	pub fn parse_response<T: DeserializeOwned>(response: ArcAPIResponse) -> APIResult<T> {
		response.parse_data()
	}

	pub async fn send_request(
		&self,
		action: String,
		params: JsonValue,
		echo: String,
	) -> APIResult<ArcAPIResponse> {
		let (response_tx, response_rx) = tokio::sync::oneshot::channel();
		{
			let echo = echo.clone();
			let mut registry = self.api_request_registry.lock().unwrap();
			registry.insert(echo, response_tx);
		}

		struct RequestGuard<'a> {
			echo: String,
			registry: &'a ArcAPIRequestRegistry,
		}
		impl<'a> Drop for RequestGuard<'a> {
			fn drop(&mut self) {
				let mut registry = self.registry.lock().unwrap();
				registry.remove(&self.echo);
			}
		}
		let _request_guard = RequestGuard {
			echo: echo.clone(),
			registry: &self.api_request_registry,
		};

		self
			.internal_api_sender
			.send_async(APIRequest {
				action,
				params,
				echo: Some(echo),
			})
			.await?;
		let response_future = async { response_rx.await.ok() };
		if let Some(timeout) = self.timeout {
			tokio::time::timeout(timeout, response_future)
				.await
				.ok()
				.flatten()
		} else {
			response_future.await
		}
		.ok_or(APIRequestError::Timeout)
	}

	/// 生成echo并发送API请求  
	/// 同时等待API响应并自动解析
	///
	/// # Params
	/// - `action` 要调用的 `action` 的名称
	/// - `params` 调用 `action` 所需要的参数
	///
	/// # Examples
	/// ```rust
	/// use std::time::Duration;
	/// use serde_json::{json, Value};
	/// use onebot_api::communication::utils::Client;
	/// use onebot_api::communication::ws::WsService;
	///
	/// let client: Client = Client::new_with_options(WsService::new_with_options("ws://localhost:8080", None).unwrap(), Some(Duration::from_secs(5)), None, None);
	/// let response: Value =  client.send_and_parse("send_like", json!({})).await.unwrap();
	/// ```
	pub async fn send_and_parse<T: DeserializeOwned>(
		&self,
		action: impl ToString,
		params: JsonValue,
	) -> APIResult<T> {
		let echo = (self.echo_generator)();
		let response = self
			.send_request(action.to_string(), params, echo.clone())
			.await?;
		Self::parse_response(response)
	}
}

// APISender START

#[async_trait]
impl APISenderTrait for Client {
	async fn send_private_msg(
		&self,
		user_id: i64,
		message: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		let params = json!({
			"user_id": user_id,
			"message": message,
			"auto_escape": auto_escape
		});
		let response: SendMsgResponse = self.send_and_parse("send_private_msg", params).await?;
		Ok(response.message_id)
	}

	async fn send_group_msg(
		&self,
		group_id: i64,
		message: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		let params = json!({
			"group_id": group_id,
			"message": message,
			"auto_escape": auto_escape
		});
		let response: SendMsgResponse = self.send_and_parse("send_group_msg", params).await?;
		Ok(response.message_id)
	}

	async fn send_msg(
		&self,
		message_type: Option<MessageType>,
		user_id: i64,
		group_id: i64,
		message: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		let params = json!({
			"message_type": message_type,
			"user_id": user_id,
			"group_id": group_id,
			"message": message,
			"auto_escape": auto_escape
		});
		let response: SendMsgResponse = self.send_and_parse("send_msg", params).await?;
		Ok(response.message_id)
	}

	async fn delete_msg(&self, message_id: i32) -> APIResult<()> {
		let params = json!({
			"message_id": message_id
		});
		self.send_and_parse("delete_msg", params).await
	}

	async fn get_msg(&self, message_id: i32) -> APIResult<GetMsgResponse> {
		let params = json!({
			"message_id": message_id
		});
		self.send_and_parse("get_msg", params).await
	}

	async fn get_forward_msg(&self, id: String) -> APIResult<Vec<ReceiveSegment>> {
		let params = json!({
			"id": id
		});
		let response: GetForwardMsgResponse = self.send_and_parse("get_forward_msg", params).await?;
		Ok(response.message)
	}

	async fn send_like(&self, user_id: i64, times: Option<i32>) -> APIResult<()> {
		let params = json!({
			"user_id": user_id,
			"times": times
		});
		self.send_and_parse("send_like", params).await
	}

	async fn set_group_kick(
		&self,
		group_id: i32,
		user_id: i32,
		reject_add_request: Option<bool>,
	) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"user_id": user_id,
			"reject_add_request": reject_add_request
		});
		self.send_and_parse("set_group_kick", params).await
	}

	async fn set_group_ban(
		&self,
		group_id: i32,
		user_id: i32,
		duration: Option<i32>,
	) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"user_id": user_id,
			"duration": duration
		});
		self.send_and_parse("set_group_ban", params).await
	}

	async fn set_group_anonymous_ban(
		&self,
		group_id: i32,
		anonymous: Option<GroupMessageAnonymous>,
		flag: Option<String>,
		duration: Option<i32>,
	) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"anonymous": anonymous,
			"flag": flag,
			"duration": duration
		});
		self.send_and_parse("set_group_anonymous_ban", params).await
	}

	async fn set_group_whole_ban(&self, group_id: i32, enable: Option<bool>) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"enable": enable
		});
		self.send_and_parse("set_group_whole_ban", params).await
	}

	async fn set_group_admin(
		&self,
		group_id: i32,
		user_id: i32,
		enable: Option<bool>,
	) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"user_id": user_id,
			"enable": enable
		});
		self.send_and_parse("set_group_admin", params).await
	}

	async fn set_group_anonymous(&self, group_id: i32, enable: Option<bool>) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"enable": enable
		});
		self.send_and_parse("set_group_anonymous", params).await
	}

	async fn set_group_card(
		&self,
		group_id: i32,
		user_id: i32,
		card: Option<String>,
	) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"user_id": user_id,
			"card": card
		});
		self.send_and_parse("set_group_card", params).await
	}

	async fn set_group_name(&self, group_id: i32, group_name: String) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"group_name": group_name
		});
		self.send_and_parse("set_group_name", params).await
	}

	async fn set_group_leave(&self, group_id: i32, is_dismiss: Option<bool>) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"is_dismiss": is_dismiss
		});
		self.send_and_parse("set_group_leave", params).await
	}

	async fn set_group_special_title(
		&self,
		group_id: i32,
		user_id: i32,
		special_title: Option<String>,
		duration: Option<i32>,
	) -> APIResult<()> {
		let params = json!({
			"group_id": group_id,
			"user_id": user_id,
			"special_title": special_title,
			"duration": duration
		});
		self.send_and_parse("set_group_special_title", params).await
	}

	async fn set_friend_add_request(
		&self,
		flag: String,
		approve: Option<bool>,
		remark: Option<String>,
	) -> APIResult<()> {
		let params = json!({
			"flag": flag,
			"approve": approve,
			"remark": remark
		});
		self.send_and_parse("set_friend_add_request", params).await
	}

	async fn set_group_add_request(
		&self,
		flag: String,
		sub_type: String,
		approve: Option<bool>,
		reason: Option<String>,
	) -> APIResult<()> {
		let params = json!({
			"flag": flag,
			"sub_type": sub_type,
			"approve": approve,
			"reason": reason
		});
		self.send_and_parse("set_group_add_request", params).await
	}

	async fn get_login_info(&self) -> APIResult<GetLoginInfoResponse> {
		let params = json!({});
		self.send_and_parse("get_login_info", params).await
	}

	async fn get_stranger_info(
		&self,
		user_id: i32,
		no_cache: Option<bool>,
	) -> APIResult<GetStrangerInfoResponse> {
		let params = json!({
			"user_id": user_id,
			"no_cache": no_cache
		});
		self.send_and_parse("get_stranger_info", params).await
	}

	async fn get_friend_list(&self) -> APIResult<Vec<GetFriendListResponse>> {
		let params = json!({});
		self.send_and_parse("get_friend_list", params).await
	}

	async fn get_group_info(
		&self,
		group_id: i32,
		no_cache: Option<bool>,
	) -> APIResult<GetGroupInfoResponse> {
		let params = json!({
			"group_id": group_id,
			"no_cache": no_cache
		});
		self.send_and_parse("get_group_info", params).await
	}

	async fn get_group_list(&self) -> APIResult<Vec<GetGroupInfoResponse>> {
		let params = json!({});
		self.send_and_parse("get_group_list", params).await
	}

	async fn get_group_member_info(
		&self,
		group_id: i32,
		user_id: i32,
		no_cache: Option<bool>,
	) -> APIResult<GetGroupMemberInfoResponse> {
		let params = json!({
			"group_id": group_id,
			"user_id": user_id,
			"no_cache": no_cache
		});
		self.send_and_parse("get_group_member_info", params).await
	}

	async fn get_group_member_list(
		&self,
		group_id: i32,
	) -> APIResult<Vec<GetGroupMemberInfoResponse>> {
		let params = json!({
			"group_id": group_id
		});
		self.send_and_parse("get_group_member_list", params).await
	}

	async fn get_group_honor_info(
		&self,
		group_id: i64,
		honor_type: String,
	) -> APIResult<GetGroupMemberInfoResponse> {
		let params = json!({
			"group_id": group_id,
			"type": honor_type
		});
		self.send_and_parse("get_group_honor_info", params).await
	}

	async fn get_cookies(&self, domain: Option<String>) -> APIResult<String> {
		let params = json!({
			"domain": domain
		});
		let response: GetCookiesResponse = self.send_and_parse("get_cookies", params).await?;
		Ok(response.cookies)
	}

	async fn get_csrf_token(&self) -> APIResult<i32> {
		let params = json!({});
		let response: GetCsrfTokenResponse = self.send_and_parse("get_csrf_token", params).await?;
		Ok(response.token)
	}

	async fn get_credentials(&self, domain: Option<String>) -> APIResult<GetCredentialsResponse> {
		let params = json!({
			"domain": domain
		});
		self.send_and_parse("get_credentials", params).await
	}

	async fn get_record(&self, file: String, out_format: String) -> APIResult<String> {
		let params = json!({
			"file": file,
			"out_format": out_format
		});
		let response: GetDataResponse = self.send_and_parse("get_record", params).await?;
		Ok(response.file)
	}

	async fn get_image(&self, file: String) -> APIResult<String> {
		let params = json!({
			"file": file
		});
		let response: GetDataResponse = self.send_and_parse("get_image", params).await?;
		Ok(response.file)
	}

	async fn can_send_image(&self) -> APIResult<bool> {
		let params = json!({});
		let response: CanSendResponse = self.send_and_parse("can_send_image", params).await?;
		Ok(response.yes)
	}

	async fn can_send_record(&self) -> APIResult<bool> {
		let params = json!({});
		let response: CanSendResponse = self.send_and_parse("can_send_record", params).await?;
		Ok(response.yes)
	}

	async fn get_status(&self) -> APIResult<GetStatusResponse> {
		let params = json!({});
		self.send_and_parse("get_status", params).await
	}

	async fn get_version_info(&self) -> APIResult<GetVersionInfoResponse> {
		let params = json!({});
		self.send_and_parse("get_version_info", params).await
	}

	async fn set_restart(&self, delay: Option<i32>) -> APIResult<()> {
		let params = json!({
			"delay": delay
		});
		self.send_and_parse("set_restart", params).await
	}

	async fn clean_cache(&self) -> APIResult<()> {
		let params = json!({});
		self.send_and_parse("clean_cache", params).await
	}
}
