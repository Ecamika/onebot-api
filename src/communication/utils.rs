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
use flume::SendError;
pub use flume::Sender as FlumeSender;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};
use std::sync::Arc;
use std::time::Duration;
use strum_macros::EnumIs;
use tokio::sync::broadcast;
pub use tokio::sync::broadcast::Receiver as BroadcastReceiver;
pub use tokio::sync::broadcast::Sender as BroadcastSender;

pub type APISender = FlumeSender<APIRequest>;
pub type APIReceiver = FlumeReceiver<APIRequest>;

pub type InternalEventSender = FlumeSender<DeserializedEvent>;
pub type InternalEventReceiver = FlumeReceiver<DeserializedEvent>;

pub type ServiceRuntimeResult<T> = Result<T, ServiceRuntimeError>;

pub type EventSender = BroadcastSender<Arc<Event>>;
pub type EventReceiver = BroadcastReceiver<Arc<Event>>;

pub type ArcServiceRuntimeError = Arc<ServiceRuntimeError>;
pub type ArcAPIResponse = Arc<APIResponse>;
pub type ArcNormalEvent = Arc<NormalEvent>;
pub type ArcDeserializedEvent = Arc<DeserializedEvent>;

#[derive(Deserialize, Clone, Debug, EnumIs)]
#[serde(untagged)]
pub enum DeserializedEvent {
	APIResponse(APIResponse),
	Event(JsonValue),
}

#[derive(Deserialize, Clone, Debug, EnumIs)]
#[serde(untagged)]
pub enum Event {
	APIResponse(APIResponse),
	Event(NormalEvent),
}

#[derive(Serialize, Clone, Debug)]
pub struct APIRequest {
	pub action: String,
	pub params: JsonValue,
	pub echo: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
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

	pub fn parse_data<T: DeserializeOwned>(self) -> APIResult<T> {
		if !self.verify() {
			return Err(APIRequestError::HttpError { code: self.retcode });
		}
		Ok(serde_json::from_value(self.data)?)
	}
}

impl EventTrait for Event {}
impl EventTrait for Arc<Event> {}

#[async_trait]
pub trait APIResponseListener {
	async fn listen(&mut self, echo: String, timeout: Option<Duration>) -> Option<Arc<Event>>;
	async fn listen_without_timeout(&mut self, echo: String) -> Option<Arc<Event>>;
	async fn listen_with_timeout(&mut self, echo: String, timeout: Duration) -> Option<Arc<Event>>;
}

#[async_trait]
impl APIResponseListener for BroadcastReceiver<Arc<Event>> {
	async fn listen(&mut self, echo: String, timeout: Option<Duration>) -> Option<Arc<Event>> {
		match timeout {
			Some(timeout) => self.listen_with_timeout(echo, timeout).await,
			None => self.listen_without_timeout(echo).await,
		}
	}
	async fn listen_without_timeout(&mut self, echo: String) -> Option<Arc<Event>> {
		loop {
			if let Ok(event) = self.recv().await // 获取Event
				&& let Event::APIResponse(response) = &*event // 判断是否为APIResponse
				&& response
					.echo
					.as_ref()
					.map(|target_echo| target_echo == &echo) // 判断echo是否一致
					.unwrap_or(false)
			// 若APIResponse不存在echo则默认false
			{
				return Some(Arc::clone(&event));
			}
			if self.is_closed() {
				return None;
			}
		}
	}

	async fn listen_with_timeout(&mut self, echo: String, timeout: Duration) -> Option<Arc<Event>> {
		tokio::time::timeout(timeout, self.listen_without_timeout(echo))
			.await
			.ok()?
	}
}

#[async_trait]
pub trait CommunicationService: Sync + Send {
	fn inject(&mut self, api_receiver: APIReceiver, event_sender: EventSender);
	async fn start_service(&self) -> ServiceStartResult<()>;
}

#[async_trait]
impl CommunicationService for Box<dyn CommunicationService> {
	fn inject(&mut self, api_receiver: APIReceiver, event_sender: EventSender) {
		(**self).inject(api_receiver, event_sender);
	}

	async fn start_service(&self) -> ServiceStartResult<()> {
		(**self).start_service().await
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
/// let ws_service = WsService::new("wss://example.com", Some("example_token".to_string())).unwrap();
/// let client = Client::new(ws_service, Duration::from_secs(5), None, None);
/// client.start_service().await.unwrap();
/// ```
pub struct Client {
	service: Box<dyn CommunicationService>,
	api_sender: APISender,
	api_receiver: APIReceiver,
	event_sender: EventSender,
	timeout: Option<Duration>,
}

impl EventReceiverTrait<Arc<Event>> for Client {
	fn get_receiver(&self) -> EventReceiver {
		self.event_sender.subscribe()
	}
}

impl Client {
	/// 创建一个 [`Client`] 实例
	///
	/// # Params
	/// - `service` 实现 [`IntoService`] 特征或 [`CommunicationService`] 特征的对象
	/// - `timeout` API请求超时时间，若为 `None` 则一直等待
	/// - `api_channel_cap` API请求消息通道的容量，默认为`16`
	/// - `event_channel_cap` Event消息通道的容量，默认为`16`
	pub fn new(
		service: impl IntoService,
		timeout: Option<Duration>,
		api_channel_cap: Option<usize>,
		event_channel_cap: Option<usize>,
	) -> Self {
		let mut service = Box::new(service.into());
		let (api_sender, api_receiver) = flume::bounded(api_channel_cap.unwrap_or(16));
		let (event_sender, _) = broadcast::channel(event_channel_cap.unwrap_or(16));
		service.inject(api_receiver.clone(), event_sender.clone());
		Self {
			service,
			api_receiver,
			api_sender,
			event_sender,
			timeout,
		}
	}
}

impl Client {
	/// 启动服务
	/// 在 `Client` 实例构造完成或调用 `change_service` 后都需要调用该方法启动服务
	pub async fn start_service(&self) -> ServiceStartResult<()> {
		self.service.start_service().await
	}

	/// 更换服务
	/// 即使在原服务启动后也可以更换服务
	/// 但需保证原服务被drop，即调用原服务的析构函数
	///
	/// # Examples
	/// ```rust
	/// use std::time::Duration;
	/// use onebot_api::communication::utils::Client;
	/// use onebot_api::communication::ws::WsService;
	/// use onebot_api::communication::ws_reverse::WsReverseService;
	///
	/// let ws_service = WsService::new("wss://example.com", Some("example_token".to_string())).unwrap();
	/// let mut client = Client::new(ws_service, Duration::from_secs(5), None, None);
	/// client.start_service().await.unwrap();
	///
	/// let ws_reverse_service = WsReverseService::new("0.0.0.0:8080", Some("example_token".to_string()));
	/// client.change_service(ws_reverse_service);
	/// client.start_service().await.unwrap();
	/// ```
	pub fn change_service(&mut self, service: impl IntoService) -> Box<dyn CommunicationService> {
		let mut service = Box::new(service.into());
		service.inject(self.api_receiver.clone(), self.event_sender.clone());
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
	pub fn generate_id() -> String {
		uuid::Uuid::new_v4().to_string()
	}

	/// 自动获取 `receiver` 并监听带有指定 `echo` 的API调用结果
	///
	/// # Returns
	/// - `Some(api_response)` 成功获取API调用结果
	/// - `None` 监听过程中事件通道被关闭或监听超时
	pub async fn get_response(&self, echo: String) -> Option<APIResponse> {
		let mut receiver = self.get_receiver();
		if let Event::APIResponse(res) = &*(receiver.listen(echo, self.timeout).await?) {
			Some(res.clone())
		} else {
			None
		}
	}

	pub fn parse_response<T: DeserializeOwned>(response: APIResponse) -> APIResult<T> {
		response.parse_data()
	}

	pub async fn send_request(
		&self,
		action: String,
		params: JsonValue,
		echo: String,
	) -> Result<(), SendError<APIRequest>> {
		self
			.api_sender
			.send_async(APIRequest {
				action,
				params,
				echo: Some(echo),
			})
			.await
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
	/// let client: Client = Client::new(WsService::new("ws://localhost:8080", None).unwrap(), Some(Duration::from_secs(5)), None, None);
	/// let response: Value =  client.send_and_parse("send_like", json!({})).await.unwrap();
	/// ```
	pub async fn send_and_parse<T: DeserializeOwned>(
		&self,
		action: impl ToString,
		params: JsonValue,
	) -> APIResult<T> {
		let echo = Self::generate_id();
		self
			.send_request(action.to_string(), params, echo.clone())
			.await?;
		let response = self.get_response(echo).await;
		if response.is_none() {
			return Err(APIRequestError::Timeout);
		}
		let response = response.unwrap();
		Self::parse_response(response)
	}
}

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
