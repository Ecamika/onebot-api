#[cfg(feature = "selector")]
use tynavi::Selector;

use crate::message::receive_segment::ReceiveSegment;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs};

#[cfg(feature = "quick_operation")]
use crate::api::APISender;
#[cfg(feature = "quick_operation")]
use crate::error::{APIRequestError, APIResult};
#[cfg(feature = "quick_operation")]
use crate::message::send_segment::{AtData, SendSegment};
#[cfg(feature = "quick_operation")]
use crate::message::utils::AtType;
#[cfg(feature = "quick_operation")]
use crate::quick_operation::{QuickBan, QuickDeleteMsg, QuickKick, QuickReplyAt, QuickSendMsg};
#[cfg(feature = "quick_operation")]
use async_trait::async_trait;

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum Sex {
	#[serde(rename = "male")]
	Male,
	#[serde(rename = "female")]
	Female,
	#[serde(rename = "unknown")]
	Unknown,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PrivateMessageSender {
	pub user_id: Option<i64>,
	pub nickname: Option<String>,
	pub sex: Option<Sex>,
	pub age: Option<i64>,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for PrivateMessageSender {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i64> {
		api
			.send_private_msg(
				self.user_id.ok_or(APIRequestError::MissingParameters)?,
				msg,
				auto_escape,
			)
			.await
	}
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum PrivateMessageSubType {
	#[serde(rename = "friend")]
	Friend,
	#[serde(rename = "group")]
	Group,
	#[serde(rename = "other")]
	Other,
}

#[derive(Deserialize, Serialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GroupMessageAnonymous {
	id: i64,
	name: String,
	flag: String,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GroupMessageSender {
	pub user_id: Option<i64>,
	pub nickname: Option<String>,
	pub card: Option<String>,
	pub sex: Option<Sex>,
	pub age: Option<i64>,
	pub area: Option<String>,
	pub level: Option<String>,
	pub role: Option<GroupMessageSenderRole>,
	pub title: Option<String>,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for GroupMessageSender {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i64> {
		api
			.send_private_msg(
				self.user_id.ok_or(APIRequestError::MissingParameters)?,
				msg,
				auto_escape,
			)
			.await
	}
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupMessageSenderRole {
	#[serde(rename = "owner")]
	Owner,
	#[serde(rename = "admin")]
	Admin,
	#[serde(rename = "member")]
	Member,
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupMessageSubType {
	#[serde(rename = "normal")]
	Normal,
	#[serde(rename = "anonymous")]
	Anonymous,
	#[serde(rename = "notice")]
	Notice,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone)]
pub struct MessageEventPrivate {
	#[cfg_attr(feature = "selector", selector(variants(friend, group, other)))]
	pub sub_type: PrivateMessageSubType,
	pub message_id: i64,
	pub user_id: i64,
	pub message: Vec<ReceiveSegment>,
	pub raw_message: String,
	pub font: i64,
	pub sender: PrivateMessageSender,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for MessageEventPrivate {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i64> {
		api.send_private_msg(self.user_id, msg, auto_escape).await
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickDeleteMsg<T> for MessageEventPrivate {
	async fn delete_msg(&self, api: &T) -> APIResult<()> {
		api.delete_msg(self.message_id).await
	}
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone)]
pub struct MessageEventGroup {
	#[cfg_attr(feature = "selector", selector(variants(normal, anonymous, notice)))]
	pub sub_type: GroupMessageSubType,
	pub message_id: i64,
	pub group_id: i64,
	pub user_id: i64,
	pub anonymous: Option<GroupMessageAnonymous>,
	pub message: Vec<ReceiveSegment>,
	pub raw_message: String,
	pub font: i64,
	pub sender: GroupMessageSender,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for MessageEventGroup {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i64> {
		api.send_group_msg(self.group_id, msg, auto_escape).await
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickReplyAt<T> for MessageEventGroup {
	async fn reply_at(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i64> {
		let mut full_msg = vec![SendSegment::At(AtData {
			qq: AtType::Id(self.user_id.to_string()),
		})];
		full_msg.extend(msg);
		api
			.send_group_msg(self.group_id, full_msg, auto_escape)
			.await
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickDeleteMsg<T> for MessageEventGroup {
	async fn delete_msg(&self, api: &T) -> APIResult<()> {
		api.delete_msg(self.message_id).await
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickKick<T> for MessageEventGroup {
	async fn kick(&self, api: &T, reject_add_request: Option<bool>) -> APIResult<()> {
		api
			.set_group_kick(
				self.group_id as i64,
				self.user_id as i64,
				reject_add_request,
			)
			.await
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickBan<T> for MessageEventGroup {
	async fn ban(&self, api: &T, duration: Option<i64>) -> APIResult<()> {
		api
			.set_group_ban(self.group_id as i64, self.user_id as i64, duration)
			.await
	}
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[cfg_attr(not(feature = "selector"), derive(EnumIs))]
#[derive(Deserialize, Debug, Clone, Display)]
#[serde(tag = "message_type")]
pub enum MessageEvent {
	#[serde(rename = "private")]
	Private(MessageEventPrivate),

	#[serde(rename = "group")]
	Group(MessageEventGroup),
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for MessageEvent {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i64> {
		match self {
			Self::Group(data) => data.send_msg(api, msg, auto_escape),
			Self::Private(data) => data.send_msg(api, msg, auto_escape),
		}
		.await
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickDeleteMsg<T> for MessageEvent {
	async fn delete_msg(&self, api: &T) -> APIResult<()> {
		match self {
			Self::Group(data) => data.delete_msg(api).await,
			Self::Private(data) => data.delete_msg(api).await,
		}
	}
}
