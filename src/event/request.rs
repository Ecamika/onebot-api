#[cfg(feature = "selector")]
use crate::selector::Selector;
use serde::Deserialize;
use strum::{Display, EnumIs};

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupType {
	#[serde(rename = "add")]
	Add,
	#[serde(rename = "invite")]
	Invite,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RequestEventFriend {
	user_id: i64,
	comment: String,
	flag: String,
}

impl RequestEventFriend {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, RequestEventFriend> {
	pub fn filter(&mut self, f: impl FnOnce(&RequestEventFriend) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&RequestEventFriend) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&RequestEventFriend) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&RequestEventFriend) -> bool,
	) -> Self {
		self.filter_async(f).await;
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

	pub fn filter_comment(&mut self, f: impl FnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.comment)
		{
			self.data = None
		}
	}

	pub fn and_filter_comment(mut self, f: impl FnOnce(&str) -> bool) -> Self {
		self.filter_comment(f);
		self
	}

	pub async fn filter_comment_async(&mut self, f: impl AsyncFnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.comment).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_comment_async(mut self, f: impl AsyncFnOnce(&str) -> bool) -> Self {
		self.filter_comment_async(f).await;
		self
	}

	pub fn filter_flag(&mut self, f: impl FnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.flag)
		{
			self.data = None
		}
	}

	pub fn and_filter_flag(mut self, f: impl FnOnce(&str) -> bool) -> Self {
		self.filter_flag(f);
		self
	}

	pub async fn filter_flag_async(&mut self, f: impl AsyncFnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.flag).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_flag_async(mut self, f: impl AsyncFnOnce(&str) -> bool) -> Self {
		self.filter_flag_async(f).await;
		self
	}
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RequestEventGroup {
	sub_type: GroupType,
	group_id: i64,
	user_id: i64,
	comment: String,
	flag: String,
}

impl RequestEventGroup {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, RequestEventGroup> {
	pub fn filter(&mut self, f: impl FnOnce(&RequestEventGroup) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&RequestEventGroup) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&RequestEventGroup) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&RequestEventGroup) -> bool) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_sub_type(&mut self, f: impl FnOnce(GroupType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type)
		{
			self.data = None
		}
	}

	pub fn and_filter_sub_type(mut self, f: impl FnOnce(GroupType) -> bool) -> Self {
		self.filter_sub_type(f);
		self
	}

	pub async fn filter_sub_type_async(&mut self, f: impl AsyncFnOnce(GroupType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sub_type_async(mut self, f: impl AsyncFnOnce(GroupType) -> bool) -> Self {
		self.filter_sub_type_async(f).await;
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

	pub fn filter_comment(&mut self, f: impl FnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.comment)
		{
			self.data = None
		}
	}

	pub fn and_filter_comment(mut self, f: impl FnOnce(&str) -> bool) -> Self {
		self.filter_comment(f);
		self
	}

	pub async fn filter_comment_async(&mut self, f: impl AsyncFnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.comment).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_comment_async(mut self, f: impl AsyncFnOnce(&str) -> bool) -> Self {
		self.filter_comment_async(f).await;
		self
	}

	pub fn filter_flag(&mut self, f: impl FnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.flag)
		{
			self.data = None
		}
	}

	pub fn and_filter_flag(mut self, f: impl FnOnce(&str) -> bool) -> Self {
		self.filter_flag(f);
		self
	}

	pub async fn filter_flag_async(&mut self, f: impl AsyncFnOnce(&str) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.flag).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_flag_async(mut self, f: impl AsyncFnOnce(&str) -> bool) -> Self {
		self.filter_flag_async(f).await;
		self
	}

	pub fn add(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_add()
		{
			self.data = None
		}
	}

	pub fn and_add(mut self) -> Self {
		self.add();
		self
	}

	pub fn not_add(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_add()
		{
			self.data = None
		}
	}

	pub fn and_not_add(mut self) -> Self {
		self.not_add();
		self
	}

	pub fn invite(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_invite()
		{
			self.data = None
		}
	}

	pub fn and_invite(mut self) -> Self {
		self.invite();
		self
	}

	pub fn not_invite(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_invite()
		{
			self.data = None
		}
	}

	pub fn and_not_invite(mut self) -> Self {
		self.not_invite();
		self
	}
}

#[derive(Deserialize, Debug, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
#[serde(tag = "request_type")]
pub enum RequestEvent {
	#[serde(rename = "friend")]
	Friend(RequestEventFriend),

	#[serde(rename = "group")]
	Group(RequestEventGroup),
}

impl RequestEvent {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}

	pub fn match_friend(&self) -> Option<&RequestEventFriend> {
		if let Self::Friend(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_friend<T>(&self, handler: impl FnOnce(&RequestEventFriend) -> T) -> Option<T> {
		if let Self::Friend(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_friend_async<T>(
		&self,
		handler: impl AsyncFnOnce(&RequestEventFriend) -> T,
	) -> Option<T> {
		if let Self::Friend(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_group(&self) -> Option<&RequestEventGroup> {
		if let Self::Group(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_group<T>(&self, handler: impl FnOnce(&RequestEventGroup) -> T) -> Option<T> {
		if let Self::Group(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_async<T>(
		&self,
		handler: impl AsyncFnOnce(&RequestEventGroup) -> T,
	) -> Option<T> {
		if let Self::Group(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, RequestEvent> {
	pub fn friend(&self) -> Selector<'a, RequestEventFriend> {
		Selector {
			data: self.data.and_then(|d| d.match_friend()),
		}
	}

	pub fn group(&self) -> Selector<'a, RequestEventGroup> {
		Selector {
			data: self.data.and_then(|d| d.match_group()),
		}
	}
}
