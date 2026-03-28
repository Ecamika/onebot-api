use crate::api::APISender;
use crate::error::{APIRequestError, APIResult};
use crate::message::receive_segment::ReceiveSegment;
use crate::message::send_segment::SendSegment;
#[cfg(feature = "selector")]
use crate::selector::Selector;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs};

#[cfg(feature = "quick_operation")]
use crate::quick_operation::QuickSendMsg;

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
	pub age: Option<i32>,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for PrivateMessageSender {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
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
	pub age: Option<i32>,
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
	) -> APIResult<i32> {
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

#[derive(Deserialize, Debug, Clone)]
pub struct MessageEventPrivate {
	pub sub_type: PrivateMessageSubType,
	pub message_id: i32,
	pub user_id: i64,
	pub message: Vec<ReceiveSegment>,
	pub raw_message: String,
	pub font: i32,
	pub sender: PrivateMessageSender,
}

impl MessageEventPrivate {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, MessageEventPrivate> {
	pub fn filter(&mut self, f: impl FnOnce(&MessageEventPrivate) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&MessageEventPrivate) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&MessageEventPrivate) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&MessageEventPrivate) -> bool,
	) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_sub_type(&mut self, f: impl FnOnce(PrivateMessageSubType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type)
		{
			self.data = None
		}
	}

	pub fn and_filter_sub_type(mut self, f: impl FnOnce(PrivateMessageSubType) -> bool) -> Self {
		self.filter_sub_type(f);
		self
	}

	pub async fn filter_sub_type_async(
		&mut self,
		f: impl AsyncFnOnce(PrivateMessageSubType) -> bool,
	) {
		if let Some(data) = self.data
			&& !f(data.sub_type).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sub_type_async(
		mut self,
		f: impl AsyncFnOnce(PrivateMessageSubType) -> bool,
	) -> Self {
		self.filter_sub_type_async(f).await;
		self
	}

	pub fn filter_message_id(&mut self, f: impl FnOnce(i32) -> bool) {
		if let Some(data) = self.data
			&& !f(data.message_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_message_id(mut self, f: impl FnOnce(i32) -> bool) -> Self {
		self.filter_message_id(f);
		self
	}

	pub async fn filter_message_id_async(&mut self, f: impl AsyncFnOnce(i32) -> bool) {
		if let Some(data) = self.data
			&& !f(data.message_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_message_id_async(mut self, f: impl AsyncFnOnce(i32) -> bool) -> Self {
		self.filter_message_id_async(f).await;
		self
	}

	pub fn filter_user_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.user_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_user_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_user_id(f);
		self
	}

	pub async fn filter_user_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.user_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_user_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_user_id_async(f).await;
		self
	}

	pub fn filter_message(&mut self, f: impl FnOnce(&Vec<ReceiveSegment>) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.message)
		{
			self.data = None
		}
	}

	pub fn and_filter_message(mut self, f: impl FnOnce(&Vec<ReceiveSegment>) -> bool) -> Self {
		self.filter_message(f);
		self
	}

	pub async fn filter_message_async(&mut self, f: impl AsyncFnOnce(&Vec<ReceiveSegment>) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.message).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_message_async(
		mut self,
		f: impl AsyncFnOnce(&Vec<ReceiveSegment>) -> bool,
	) -> Self {
		self.filter_message_async(f).await;
		self
	}

	pub fn filter_raw_message(&mut self, f: impl FnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.raw_message)
		{
			self.data = None
		}
	}

	pub fn and_filter_raw_message(mut self, f: impl FnOnce(&str) -> bool) -> Self {
		self.filter_raw_message(f);
		self
	}

	pub async fn filter_raw_message_async(&mut self, f: impl AsyncFnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.raw_message).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_raw_message_async(mut self, f: impl AsyncFnOnce(&str) -> bool) -> Self {
		self.filter_raw_message_async(f).await;
		self
	}

	pub fn filter_font(&mut self, f: impl FnOnce(i32) -> bool) {
		if let Some(data) = self.data
			&& !f(data.font)
		{
			self.data = None
		}
	}

	pub fn and_filter_font(mut self, f: impl FnOnce(i32) -> bool) -> Self {
		self.filter_font(f);
		self
	}

	pub async fn filter_font_async(&mut self, f: impl AsyncFnOnce(i32) -> bool) {
		if let Some(data) = self.data
			&& !f(data.font).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_font_async(mut self, f: impl AsyncFnOnce(i32) -> bool) -> Self {
		self.filter_font_async(f).await;
		self
	}

	pub fn filter_sender(&mut self, f: impl FnOnce(&PrivateMessageSender) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.sender)
		{
			self.data = None
		}
	}

	pub fn and_filter_sender(mut self, f: impl FnOnce(&PrivateMessageSender) -> bool) -> Self {
		self.filter_sender(f);
		self
	}

	pub async fn filter_sender_async(&mut self, f: impl AsyncFnOnce(&PrivateMessageSender) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.sender).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sender_async(
		mut self,
		f: impl AsyncFnOnce(&PrivateMessageSender) -> bool,
	) -> Self {
		self.filter_sender_async(f).await;
		self
	}

	pub fn friend(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_friend()
		{
			self.data = None
		}
	}

	pub fn and_friend(mut self) -> Self {
		self.friend();
		self
	}

	pub fn not_friend(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_friend()
		{
			self.data = None
		}
	}

	pub fn and_not_friend(mut self) -> Self {
		self.not_friend();
		self
	}

	pub fn group(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_group()
		{
			self.data = None
		}
	}

	pub fn and_group(mut self) -> Self {
		self.group();
		self
	}

	pub fn not_group(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_group()
		{
			self.data = None
		}
	}

	pub fn and_not_group(mut self) -> Self {
		self.not_group();
		self
	}

	pub fn other(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_other()
		{
			self.data = None
		}
	}

	pub fn and_other(mut self) -> Self {
		self.other();
		self
	}

	pub fn not_other(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_other()
		{
			self.data = None
		}
	}

	pub fn and_not_other(mut self) -> Self {
		self.not_other();
		self
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for MessageEventPrivate {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		api.send_private_msg(self.user_id, msg, auto_escape).await
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageEventGroup {
	pub sub_type: GroupMessageSubType,
	pub message_id: i32,
	pub group_id: i64,
	pub user_id: i64,
	pub anonymous: Option<GroupMessageAnonymous>,
	pub message: Vec<ReceiveSegment>,
	pub raw_message: String,
	pub font: i32,
	pub sender: GroupMessageSender,
}

impl MessageEventGroup {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, MessageEventGroup> {
	pub fn filter(&mut self, f: impl FnOnce(&MessageEventGroup) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&MessageEventGroup) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&MessageEventGroup) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&MessageEventGroup) -> bool) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_sub_type(&mut self, f: impl FnOnce(GroupMessageSubType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type)
		{
			self.data = None
		}
	}

	pub fn and_filter_sub_type(mut self, f: impl FnOnce(GroupMessageSubType) -> bool) -> Self {
		self.filter_sub_type(f);
		self
	}

	pub async fn filter_sub_type_async(&mut self, f: impl AsyncFnOnce(GroupMessageSubType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sub_type_async(
		mut self,
		f: impl AsyncFnOnce(GroupMessageSubType) -> bool,
	) -> Self {
		self.filter_sub_type_async(f).await;
		self
	}

	pub fn filter_message_id(&mut self, f: impl FnOnce(i32) -> bool) {
		if let Some(data) = self.data
			&& !f(data.message_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_message_id(mut self, f: impl FnOnce(i32) -> bool) -> Self {
		self.filter_message_id(f);
		self
	}

	pub async fn filter_message_id_async(&mut self, f: impl AsyncFnOnce(i32) -> bool) {
		if let Some(data) = self.data
			&& !f(data.message_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_message_id_async(mut self, f: impl AsyncFnOnce(i32) -> bool) -> Self {
		self.filter_message_id_async(f).await;
		self
	}

	pub fn filter_group_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.group_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_group_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_group_id(f);
		self
	}

	pub async fn filter_group_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.group_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_group_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_group_id_async(f).await;
		self
	}

	pub fn filter_user_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.user_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_user_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_user_id(f);
		self
	}

	pub async fn filter_user_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.user_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_user_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_user_id_async(f).await;
		self
	}

	pub fn filter_anonymous(&mut self, f: impl FnOnce(&Option<GroupMessageAnonymous>) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.anonymous)
		{
			self.data = None
		}
	}

	pub fn and_filter_anonymous(
		mut self,
		f: impl FnOnce(&Option<GroupMessageAnonymous>) -> bool,
	) -> Self {
		self.filter_anonymous(f);
		self
	}

	pub async fn filter_anonymous_async(
		&mut self,
		f: impl AsyncFnOnce(&Option<GroupMessageAnonymous>) -> bool,
	) {
		if let Some(data) = self.data
			&& !f(&data.anonymous).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_anonymous_async(
		mut self,
		f: impl AsyncFnOnce(&Option<GroupMessageAnonymous>) -> bool,
	) -> Self {
		self.filter_anonymous_async(f).await;
		self
	}

	pub fn filter_message(&mut self, f: impl FnOnce(&Vec<ReceiveSegment>) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.message)
		{
			self.data = None
		}
	}

	pub fn and_filter_message(mut self, f: impl FnOnce(&Vec<ReceiveSegment>) -> bool) -> Self {
		self.filter_message(f);
		self
	}

	pub async fn filter_message_async(&mut self, f: impl AsyncFnOnce(&Vec<ReceiveSegment>) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.message).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_message_async(
		mut self,
		f: impl AsyncFnOnce(&Vec<ReceiveSegment>) -> bool,
	) -> Self {
		self.filter_message_async(f).await;
		self
	}

	pub fn filter_raw_message(&mut self, f: impl FnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.raw_message)
		{
			self.data = None
		}
	}

	pub fn and_filter_raw_message(mut self, f: impl FnOnce(&str) -> bool) -> Self {
		self.filter_raw_message(f);
		self
	}

	pub async fn filter_raw_message_async(&mut self, f: impl AsyncFnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.raw_message).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_raw_message_async(mut self, f: impl AsyncFnOnce(&str) -> bool) -> Self {
		self.filter_raw_message_async(f).await;
		self
	}

	pub fn filter_font(&mut self, f: impl FnOnce(i32) -> bool) {
		if let Some(data) = self.data
			&& !f(data.font)
		{
			self.data = None
		}
	}

	pub fn and_filter_font(mut self, f: impl FnOnce(i32) -> bool) -> Self {
		self.filter_font(f);
		self
	}

	pub async fn filter_font_async(&mut self, f: impl AsyncFnOnce(i32) -> bool) {
		if let Some(data) = self.data
			&& !f(data.font).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_font_async(mut self, f: impl AsyncFnOnce(i32) -> bool) -> Self {
		self.filter_font_async(f).await;
		self
	}

	pub fn filter_sender(&mut self, f: impl FnOnce(&GroupMessageSender) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.sender)
		{
			self.data = None
		}
	}

	pub fn and_filter_sender(mut self, f: impl FnOnce(&GroupMessageSender) -> bool) -> Self {
		self.filter_sender(f);
		self
	}

	pub async fn filter_sender_async(&mut self, f: impl AsyncFnOnce(&GroupMessageSender) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.sender).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sender_async(
		mut self,
		f: impl AsyncFnOnce(&GroupMessageSender) -> bool,
	) -> Self {
		self.filter_sender_async(f).await;
		self
	}

	pub fn normal(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_normal()
		{
			self.data = None
		}
	}

	pub fn and_normal(mut self) -> Self {
		self.normal();
		self
	}

	pub fn not_normal(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_normal()
		{
			self.data = None
		}
	}

	pub fn and_not_normal(mut self) -> Self {
		self.not_normal();
		self
	}

	pub fn anonymous(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_anonymous()
		{
			self.data = None
		}
	}

	pub fn and_anonymous(mut self) -> Self {
		self.anonymous();
		self
	}

	pub fn not_anonymous(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_anonymous()
		{
			self.data = None
		}
	}

	pub fn and_not_anonymous(mut self) -> Self {
		self.not_anonymous();
		self
	}

	pub fn notice(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_notice()
		{
			self.data = None
		}
	}

	pub fn and_notice(mut self) -> Self {
		self.notice();
		self
	}

	pub fn not_notice(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_notice()
		{
			self.data = None
		}
	}

	pub fn and_not_notice(mut self) -> Self {
		self.not_notice();
		self
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickSendMsg<T> for MessageEventGroup {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32> {
		api.send_group_msg(self.group_id, msg, auto_escape).await
	}
}

#[derive(Deserialize, Debug, Clone, Display, EnumIs)]
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
	) -> APIResult<i32> {
		match self {
			Self::Group(data) => data.send_msg(api, msg, auto_escape),
			Self::Private(data) => data.send_msg(api, msg, auto_escape),
		}
		.await
	}
}

impl MessageEvent {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}

	pub fn match_private(&self) -> Option<&MessageEventPrivate> {
		if let Self::Private(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_private<T>(&self, handler: impl FnOnce(&MessageEventPrivate) -> T) -> Option<T> {
		if let Self::Private(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_private_async<T>(
		&self,
		handler: impl AsyncFnOnce(&MessageEventPrivate) -> T,
	) -> Option<T> {
		if let Self::Private(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_group(&self) -> Option<&MessageEventGroup> {
		if let Self::Group(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_group<T>(&self, handler: impl FnOnce(&MessageEventGroup) -> T) -> Option<T> {
		if let Self::Group(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_async<T>(
		&self,
		handler: impl AsyncFnOnce(&MessageEventGroup) -> T,
	) -> Option<T> {
		if let Self::Group(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, MessageEvent> {
	pub fn private(&self) -> Selector<'a, MessageEventPrivate> {
		Selector {
			data: self.data.and_then(|d| d.match_private()),
		}
	}

	pub fn group(&self) -> Selector<'a, MessageEventGroup> {
		Selector {
			data: self.data.and_then(|d| d.match_group()),
		}
	}
}
