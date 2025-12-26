use crate::api::APISender;
use crate::api::arg_type::MessageType;
use crate::api::return_type::{
	CanSendResponse, GetCookiesResponse, GetCredentialsResponse, GetCsrfTokenResponse,
	GetDataResponse, GetFriendListResponse, GetGroupInfoResponse, GetGroupMemberInfoResponse,
	GetLoginInfoResponse, GetMsgResponse, GetStatusResponse, GetStrangerInfoResponse,
	GetVersionInfoResponse, SendMsgResponse,
};
use crate::event::message::GroupMessageAnonymous;
use crate::event::{Event, EventReceiver, EventTrait};
use crate::message::Segment;
use anyhow::anyhow;
use async_trait::async_trait;
use flume::{Receiver, Sender};
use macros::generate_json;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct APICallResponse {
	pub status: String,
	pub retcode: u32,
	#[serde(default)]
	pub data: Value,
	pub echo: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum WsEvent {
	Event(Event),
	Response(APICallResponse),
}

impl EventTrait for WsEvent {}

#[async_trait]
pub trait WebSocketService: Send + Sync {
	fn register_api_receiver(&mut self, api_receiver: Receiver<String>);
	fn register_msg_sender(&mut self, msg_sender: Sender<String>);
	async fn start(&self) -> anyhow::Result<()>;
}

pub struct WsClient {
	service: Box<dyn WebSocketService>, // 使用DI解耦
	msg_sender: Sender<String>,
	// msg_receiver: Receiver<String>,
	api_sender: Sender<String>,
	api_receiver: Receiver<String>,
	broadcast_sender: Arc<broadcast::Sender<WsEvent>>,
	close_sender: Sender<()>,
	// close_receiver: Receiver<()>,
	max_waiting_times: Option<i32>,
}

impl WsClient {
	pub fn with_service(mut service: Box<dyn WebSocketService>) -> Self {
		let (msg_sender, msg_receiver) = flume::unbounded();
		let (api_sender, api_receiver) = flume::unbounded();
		let (close_sender, close_receiver) = flume::unbounded();
		service.register_api_receiver(api_receiver.clone());
		service.register_msg_sender(msg_sender.clone());
		let (broadcast_sender, _) = broadcast::channel(16);
		let broadcast_sender = Arc::new(broadcast_sender);
		Self::spawn_event_listener(
			msg_receiver.clone(),
			Arc::clone(&broadcast_sender),
			close_receiver.clone(),
		);
		Self {
			service,
			msg_sender,
			// msg_receiver,
			api_sender,
			api_receiver,
			broadcast_sender,
			close_sender,
			// close_receiver,
			max_waiting_times: Some(10),
		}
	}

	pub async fn start_service(&self) -> anyhow::Result<()> {
		self.service.start().await
	}

	pub fn change_service(&mut self, mut service: Box<dyn WebSocketService>) {
		service.register_api_receiver(self.api_receiver.clone());
		service.register_msg_sender(self.msg_sender.clone());
		self.service = service;
	}

	pub fn get_service(&self) -> &dyn WebSocketService {
		&*self.service
	}

	pub fn get_service_mut(&mut self) -> &mut dyn WebSocketService {
		&mut *self.service
	}
}

impl WsClient {
	fn spawn_event_listener(
		msg_receiver: Receiver<String>,
		broadcast_sender: Arc<broadcast::Sender<WsEvent>>,
		close_receiver: Receiver<()>,
	) {
		tokio::spawn(async move {
			Self::ws_stream_handler(&msg_receiver, &broadcast_sender, &close_receiver).await;
		});
	}

	async fn ws_stream_handler(
		msg_receiver: &Receiver<String>,
		broadcast_sender: &broadcast::Sender<WsEvent>,
		close_receiver: &Receiver<()>,
	) {
		loop {
			tokio::select! {
				msg = msg_receiver.recv_async() => {
					if let Ok(msg) = msg {
						Self::msg_handler(msg, broadcast_sender);
					}
				}
				_ = close_receiver.recv_async() => {
					return
				}
			}
		}
	}

	fn msg_handler(msg: String, broadcast_sender: &broadcast::Sender<WsEvent>) {
		if let Ok(event) = serde_json::from_str(&msg) {
			let _ = broadcast_sender.send(event);
		}
	}

	fn generate_api_call_json(
		action: String,
		params: HashMap<String, Value>,
		echo: String,
	) -> String {
		serde_json::to_string(&json!({
			"action": action,
			"params": params,
			"echo": echo
		}))
		.unwrap()
	}

	async fn wait_for_echo(
		rx: &mut broadcast::Receiver<WsEvent>,
		echo: String,
		max: Option<i32>,
	) -> Option<APICallResponse> {
		let mut count = 0;
		let target_echo = Some(echo.clone());
		while let Ok(event) = rx.recv().await {
			if let WsEvent::Response(res) = event
				&& res.echo == target_echo
			{
				return Some(res);
			}
			count += 1;
			if let Some(max) = max
				&& count >= max
			{
				return None;
			}
		}
		None
	}

	async fn send_json(&self, json: String, echo: String) -> anyhow::Result<APICallResponse> {
		let receiver = self.get_receiver();
		let max_waiting_times = self.max_waiting_times;
		let task = tokio::spawn(async move {
			let mut receiver = receiver;
			Self::wait_for_echo(&mut receiver, echo, max_waiting_times).await
		});

		self.api_sender.send_async(json).await?;
		let res = task.await?;
		if let Some(data) = res {
			Ok(data)
		} else {
			Err(anyhow!("No response!"))
		}
	}

	fn verify_response(res: APICallResponse) -> anyhow::Result<Value> {
		if res.status == "ok" {
			Ok(res.data)
		} else {
			Err(anyhow!("the request failed with code: {}", res.retcode))
		}
	}

	async fn get_request_value<T: DeserializeOwned>(
		&self,
		json: String,
		echo: String,
	) -> anyhow::Result<T> {
		let res = self.send_json(json, echo).await?;
		let value = Self::verify_response(res)?;
		let data = serde_json::from_value::<T>(value)?;
		Ok(data)
	}
}

impl Drop for WsClient {
	fn drop(&mut self) {
		let _ = self.close_sender.send(());
	}
}

#[async_trait]
impl EventReceiver<WsEvent> for WsClient {
	fn get_receiver(&self) -> broadcast::Receiver<WsEvent> {
		self.broadcast_sender.subscribe()
	}
}

#[allow(unused_variables)]
#[async_trait]
impl APISender for WsClient {
	#[generate_json]
	async fn send_private_msg(
		&self,
		user_id: i64,
		message: Segment,
		auto_escape: Option<bool>,
	) -> anyhow::Result<i32> {
		let res: SendMsgResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.message_id)
	}

	#[generate_json]
	async fn send_group_msg(
		&self,
		group_id: i64,
		message: Segment,
		auto_escape: Option<bool>,
	) -> anyhow::Result<i32> {
		let res: SendMsgResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.message_id)
	}

	#[generate_json]
	async fn send_msg(
		&self,
		message_type: Option<MessageType>,
		user_id: i64,
		group_id: i64,
		message: Segment,
		auto_escape: Option<bool>,
	) -> anyhow::Result<i32> {
		let res: SendMsgResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.message_id)
	}

	#[generate_json]
	async fn delete_msg(&self, message_id: i32) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn get_msg(&self, message_id: i32) -> anyhow::Result<GetMsgResponse> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_forward_msg(&self, id: String) -> anyhow::Result<Vec<Segment>> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn send_like(&self, user_id: i64, times: Option<i32>) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_kick(
		&self,
		group_id: i32,
		user_id: i32,
		reject_add_request: Option<bool>,
	) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_ban(
		&self,
		group_id: i32,
		user_id: i32,
		duration: Option<i32>,
	) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_anonymous_ban(
		&self,
		group_id: i32,
		anonymous: Option<GroupMessageAnonymous>,
		flag: Option<String>,
		duration: Option<i32>,
	) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_whole_ban(&self, group_id: i32, enable: Option<bool>) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_admin(
		&self,
		group_id: i32,
		user_id: i32,
		enable: Option<bool>,
	) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_anonymous(&self, group_id: i32, enable: Option<bool>) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_card(
		&self,
		group_id: i32,
		user_id: i32,
		card: Option<String>,
	) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_name(&self, group_id: i32, group_name: String) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_leave(&self, group_id: i32, is_dismiss: Option<bool>) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_special_title(
		&self,
		group_id: i32,
		user_id: i32,
		special_title: Option<String>,
		duration: Option<i32>,
	) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_friend_add_request(
		&self,
		flag: String,
		approve: Option<bool>,
		remark: Option<String>,
	) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn set_group_add_request(
		&self,
		flag: String,
		sub_type: String,
		approve: Option<bool>,
		reason: Option<String>,
	) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn get_login_info(&self) -> anyhow::Result<GetLoginInfoResponse> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_stranger_info(
		&self,
		user_id: i32,
		no_cache: Option<bool>,
	) -> anyhow::Result<GetStrangerInfoResponse> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_friend_list(&self) -> anyhow::Result<Vec<GetFriendListResponse>> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_group_info(
		&self,
		group_id: i32,
		no_cache: Option<bool>,
	) -> anyhow::Result<GetGroupInfoResponse> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_group_list(&self) -> anyhow::Result<Vec<GetGroupInfoResponse>> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_group_member_info(
		&self,
		group_id: i32,
		user_id: i32,
		no_cache: Option<bool>,
	) -> anyhow::Result<GetGroupMemberInfoResponse> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_group_member_list(
		&self,
		group_id: i32,
	) -> anyhow::Result<Vec<GetGroupMemberInfoResponse>> {
		self.get_request_value(__json, __echo).await
	}

	async fn get_group_honor_info(
		&self,
		group_id: i64,
		honor_type: String,
	) -> anyhow::Result<GetGroupMemberInfoResponse> {
		let echo = Uuid::new_v4().to_string();
		let mut map = HashMap::new();
		map.insert("group_id".to_string(), serde_json::to_value(group_id)?);
		map.insert("type".to_string(), serde_json::to_value(honor_type)?);
		let json = Self::generate_api_call_json("get_group_honor_info".to_string(), map, echo.clone());
		self.get_request_value(json, echo).await
	}

	#[generate_json]
	async fn get_cookies(&self, domain: Option<String>) -> anyhow::Result<String> {
		let res: GetCookiesResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.cookies)
	}

	#[generate_json]
	async fn get_csrf_token(&self) -> anyhow::Result<i32> {
		let res: GetCsrfTokenResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.token)
	}

	#[generate_json]
	async fn get_credentials(
		&self,
		domain: Option<String>,
	) -> anyhow::Result<GetCredentialsResponse> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_record(&self, file: String, out_format: String) -> anyhow::Result<String> {
		let res: GetDataResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.file)
	}

	#[generate_json]
	async fn get_image(&self, file: String) -> anyhow::Result<String> {
		let res: GetDataResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.file)
	}

	#[generate_json]
	async fn can_send_image(&self) -> anyhow::Result<bool> {
		let res: CanSendResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.yes)
	}

	#[generate_json]
	async fn can_send_record(&self) -> anyhow::Result<bool> {
		let res: CanSendResponse = self.get_request_value(__json, __echo).await?;
		Ok(res.yes)
	}

	#[generate_json]
	async fn get_status(&self) -> anyhow::Result<GetStatusResponse> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn get_version_info(&self) -> anyhow::Result<GetVersionInfoResponse> {
		self.get_request_value(__json, __echo).await
	}

	#[generate_json]
	async fn set_restart(&self, delay: Option<i32>) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}

	#[generate_json]
	async fn clean_cache(&self) -> anyhow::Result<()> {
		self.get_request_value::<Value>(__json, __echo).await?;
		Ok(())
	}
}
