#[cfg(feature = "selector")]
use crate::selector::Selector;
use serde::Deserialize;
use strum::{Display, EnumIs};

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct GroupFile {
	pub id: String,
	pub name: String,
	pub size: i64,
	pub busid: i64,
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupAdminType {
	#[serde(rename = "set")]
	Set,
	#[serde(rename = "unset")]
	Unset,
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupDecreaseType {
	#[serde(rename = "leave")]
	Leave,
	#[serde(rename = "kick")]
	Kick,
	#[serde(rename = "kick_me")]
	KickMe,
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupIncreaseType {
	#[serde(rename = "approve")]
	Approve,
	#[serde(rename = "invite")]
	Invite,
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupBanType {
	#[serde(rename = "ban")]
	Ban,
	#[serde(rename = "lift_ban")]
	LiftBan,
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
#[serde(tag = "sub_type")]
pub enum NotifyType {
	#[serde(rename = "poke")]
	Poke { target_id: i64 },
	#[serde(rename = "lucky_king")]
	LuckyKing { target_id: i64 },
	#[serde(rename = "honor")]
	Honor { honor_type: HonorType },
}

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum HonorType {
	#[serde(rename = "talkative")]
	Talkative,
	#[serde(rename = "performer")]
	Performer,
	#[serde(rename = "emotion")]
	Emotion,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupUpload {
	pub group_id: i64,
	pub user_id: i64,
	pub file: GroupFile,
}

impl NoticeEventGroupUpload {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventGroupUpload> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventGroupUpload) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventGroupUpload) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventGroupUpload) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEventGroupUpload) -> bool,
	) -> Self {
		self.filter_async(f).await;
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

	pub fn filter_file(&mut self, f: impl FnOnce(&GroupFile) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.file)
		{
			self.data = None
		}
	}

	pub fn and_filter_file(mut self, f: impl FnOnce(&GroupFile) -> bool) -> Self {
		self.filter_file(f);
		self
	}

	pub async fn filter_file_async(&mut self, f: impl AsyncFnOnce(&GroupFile) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.file).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_file_async(mut self, f: impl AsyncFnOnce(&GroupFile) -> bool) -> Self {
		self.filter_file_async(f).await;
		self
	}
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupAdmin {
	sub_type: GroupAdminType,
	group_id: i64,
	user_id: i64,
}

impl NoticeEventGroupAdmin {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventGroupAdmin> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventGroupAdmin) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventGroupAdmin) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventGroupAdmin) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEventGroupAdmin) -> bool,
	) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_sub_type(&mut self, f: impl FnOnce(GroupAdminType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type)
		{
			self.data = None
		}
	}

	pub fn and_filter_sub_type(mut self, f: impl FnOnce(GroupAdminType) -> bool) -> Self {
		self.filter_sub_type(f);
		self
	}

	pub async fn filter_sub_type_async(&mut self, f: impl AsyncFnOnce(GroupAdminType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sub_type_async(
		mut self,
		f: impl AsyncFnOnce(GroupAdminType) -> bool,
	) -> Self {
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

	pub fn set(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_set()
		{
			self.data = None
		}
	}

	pub fn and_set(mut self) -> Self {
		self.set();
		self
	}

	pub fn not_set(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_set()
		{
			self.data = None
		}
	}

	pub fn and_not_set(mut self) -> Self {
		self.not_set();
		self
	}

	pub fn unset(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_unset()
		{
			self.data = None
		}
	}

	pub fn and_unset(mut self) -> Self {
		self.unset();
		self
	}

	pub fn not_unset(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_unset()
		{
			self.data = None
		}
	}

	pub fn and_not_unset(mut self) -> Self {
		self.not_unset();
		self
	}
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupDecrease {
	sub_type: GroupDecreaseType,
	operator_id: i64,
	user_id: i64,
}

impl NoticeEventGroupDecrease {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventGroupDecrease> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventGroupDecrease) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventGroupDecrease) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventGroupDecrease) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEventGroupDecrease) -> bool,
	) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_sub_type(&mut self, f: impl FnOnce(GroupDecreaseType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type)
		{
			self.data = None
		}
	}

	pub fn and_filter_sub_type(mut self, f: impl FnOnce(GroupDecreaseType) -> bool) -> Self {
		self.filter_sub_type(f);
		self
	}

	pub async fn filter_sub_type_async(&mut self, f: impl AsyncFnOnce(GroupDecreaseType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sub_type_async(
		mut self,
		f: impl AsyncFnOnce(GroupDecreaseType) -> bool,
	) -> Self {
		self.filter_sub_type_async(f).await;
		self
	}

	pub fn filter_operator_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.operator_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_operator_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_operator_id(f);
		self
	}

	pub async fn filter_operator_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.operator_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_operator_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_operator_id_async(f).await;
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

	pub fn leave(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_leave()
		{
			self.data = None
		}
	}

	pub fn and_leave(mut self) -> Self {
		self.leave();
		self
	}

	pub fn not_leave(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_leave()
		{
			self.data = None
		}
	}

	pub fn and_not_leave(mut self) -> Self {
		self.not_leave();
		self
	}

	pub fn kick(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_kick()
		{
			self.data = None
		}
	}

	pub fn and_kick(mut self) -> Self {
		self.kick();
		self
	}

	pub fn not_kick(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_kick()
		{
			self.data = None
		}
	}

	pub fn and_not_kick(mut self) -> Self {
		self.not_kick();
		self
	}

	pub fn kick_me(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_kick_me()
		{
			self.data = None
		}
	}

	pub fn and_kick_me(mut self) -> Self {
		self.kick_me();
		self
	}

	pub fn not_kick_me(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_kick_me()
		{
			self.data = None
		}
	}

	pub fn and_not_kick_me(mut self) -> Self {
		self.not_kick_me();
		self
	}
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupIncrease {
	sub_type: GroupIncreaseType,
	group_id: i64,
	operator_id: i64,
	user_id: i64,
}

impl NoticeEventGroupIncrease {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventGroupIncrease> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventGroupIncrease) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventGroupIncrease) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventGroupIncrease) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEventGroupIncrease) -> bool,
	) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_sub_type(&mut self, f: impl FnOnce(GroupIncreaseType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type)
		{
			self.data = None
		}
	}

	pub fn and_filter_sub_type(mut self, f: impl FnOnce(GroupIncreaseType) -> bool) -> Self {
		self.filter_sub_type(f);
		self
	}

	pub async fn filter_sub_type_async(&mut self, f: impl AsyncFnOnce(GroupIncreaseType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sub_type_async(
		mut self,
		f: impl AsyncFnOnce(GroupIncreaseType) -> bool,
	) -> Self {
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

	pub fn filter_operator_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.operator_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_operator_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_operator_id(f);
		self
	}

	pub async fn filter_operator_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.operator_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_operator_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_operator_id_async(f).await;
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

	pub fn approve(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_approve()
		{
			self.data = None
		}
	}

	pub fn and_approve(mut self) -> Self {
		self.approve();
		self
	}

	pub fn not_approve(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_approve()
		{
			self.data = None
		}
	}

	pub fn and_not_approve(mut self) -> Self {
		self.not_approve();
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

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupBan {
	sub_type: GroupBanType,
	group_id: i64,
	operator_id: i64,
	user_id: i64,
	duration: i64,
}

impl NoticeEventGroupBan {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventGroupBan> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventGroupBan) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventGroupBan) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventGroupBan) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEventGroupBan) -> bool,
	) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_sub_type(&mut self, f: impl FnOnce(GroupBanType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type)
		{
			self.data = None
		}
	}

	pub fn and_filter_sub_type(mut self, f: impl FnOnce(GroupBanType) -> bool) -> Self {
		self.filter_sub_type(f);
		self
	}

	pub async fn filter_sub_type_async(&mut self, f: impl AsyncFnOnce(GroupBanType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sub_type_async(
		mut self,
		f: impl AsyncFnOnce(GroupBanType) -> bool,
	) -> Self {
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

	pub fn filter_operator_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.operator_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_operator_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_operator_id(f);
		self
	}

	pub async fn filter_operator_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.operator_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_operator_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_operator_id_async(f).await;
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

	pub fn filter_duration(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.duration)
		{
			self.data = None
		}
	}

	pub fn and_filter_duration(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_duration(f);
		self
	}

	pub async fn filter_duration_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.duration).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_duration_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_duration_async(f).await;
		self
	}

	pub fn ban(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_ban()
		{
			self.data = None
		}
	}

	pub fn and_ban(mut self) -> Self {
		self.ban();
		self
	}

	pub fn not_ban(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_ban()
		{
			self.data = None
		}
	}

	pub fn and_not_ban(mut self) -> Self {
		self.not_ban();
		self
	}

	pub fn lift_ban(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_lift_ban()
		{
			self.data = None
		}
	}

	pub fn and_lift_ban(mut self) -> Self {
		self.lift_ban();
		self
	}

	pub fn not_lift_ban(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_lift_ban()
		{
			self.data = None
		}
	}

	pub fn and_not_lift_ban(mut self) -> Self {
		self.not_lift_ban();
		self
	}
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventFriendAdd {
	user_id: i64,
}

impl NoticeEventFriendAdd {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventFriendAdd> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventFriendAdd) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventFriendAdd) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventFriendAdd) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEventFriendAdd) -> bool,
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
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupRecall {
	group_id: i64,
	user_id: i64,
	operator_id: i64,
	message_id: i64,
}

impl NoticeEventGroupRecall {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventGroupRecall> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventGroupRecall) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventGroupRecall) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventGroupRecall) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEventGroupRecall) -> bool,
	) -> Self {
		self.filter_async(f).await;
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

	pub fn filter_operator_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.operator_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_operator_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_operator_id(f);
		self
	}

	pub async fn filter_operator_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.operator_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_operator_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_operator_id_async(f).await;
		self
	}

	pub fn filter_message_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.message_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_message_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_message_id(f);
		self
	}

	pub async fn filter_message_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.message_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_message_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_message_id_async(f).await;
		self
	}
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventFriendRecall {
	user_id: i64,
	message_id: i64,
}

impl NoticeEventFriendRecall {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventFriendRecall> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventFriendRecall) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventFriendRecall) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventFriendRecall) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEventFriendRecall) -> bool,
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

	pub fn filter_message_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.message_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_message_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_message_id(f);
		self
	}

	pub async fn filter_message_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.message_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_message_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_message_id_async(f).await;
		self
	}
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventNotify {
	group_id: i64,
	user_id: i64,
	#[serde(flatten)]
	data: NotifyType,
}

impl NoticeEventNotify {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEventNotify> {
	pub fn filter(&mut self, f: impl FnOnce(&NoticeEventNotify) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&NoticeEventNotify) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&NoticeEventNotify) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&NoticeEventNotify) -> bool) -> Self {
		self.filter_async(f).await;
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

	pub fn filter_data(&mut self, f: impl FnOnce(&NotifyType) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data)
		{
			self.data = None
		}
	}

	pub fn and_filter_data(mut self, f: impl FnOnce(&NotifyType) -> bool) -> Self {
		self.filter_data(f);
		self
	}

	pub async fn filter_data_async(&mut self, f: impl AsyncFnOnce(&NotifyType) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_data_async(mut self, f: impl AsyncFnOnce(&NotifyType) -> bool) -> Self {
		self.filter_data_async(f).await;
		self
	}
}

#[derive(Deserialize, Debug, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
#[serde(tag = "notice_type")]
pub enum NoticeEvent {
	#[serde(rename = "group_upload")]
	GroupUpload(NoticeEventGroupUpload),

	#[serde(rename = "group_admin")]
	GroupAdmin(NoticeEventGroupAdmin),

	#[serde(rename = "group_decrease")]
	GroupDecrease(NoticeEventGroupDecrease),

	#[serde(rename = "group_increase")]
	GroupIncrease(NoticeEventGroupIncrease),

	#[serde(rename = "group_ban")]
	GroupBan(NoticeEventGroupBan),

	#[serde(rename = "friend_add")]
	FriendAdd(NoticeEventFriendAdd),

	#[serde(rename = "group_recall")]
	GroupRecall(NoticeEventGroupRecall),

	#[serde(rename = "friend_recall")]
	FriendRecall(NoticeEventFriendRecall),

	#[serde(rename = "notify")]
	Notify(NoticeEventNotify),
}

impl NoticeEvent {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}

	pub fn match_group_upload(&self) -> Option<&NoticeEventGroupUpload> {
		if let Self::GroupUpload(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_group_upload<T>(
		&self,
		handler: impl FnOnce(&NoticeEventGroupUpload) -> T,
	) -> Option<T> {
		if let Self::GroupUpload(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_upload_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventGroupUpload) -> T,
	) -> Option<T> {
		if let Self::GroupUpload(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_group_admin(&self) -> Option<&NoticeEventGroupAdmin> {
		if let Self::GroupAdmin(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_group_admin<T>(&self, handler: impl FnOnce(&NoticeEventGroupAdmin) -> T) -> Option<T> {
		if let Self::GroupAdmin(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_admin_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventGroupAdmin) -> T,
	) -> Option<T> {
		if let Self::GroupAdmin(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_group_decrease(&self) -> Option<&NoticeEventGroupDecrease> {
		if let Self::GroupDecrease(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_group_decrease<T>(
		&self,
		handler: impl FnOnce(&NoticeEventGroupDecrease) -> T,
	) -> Option<T> {
		if let Self::GroupDecrease(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_decrease_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventGroupDecrease) -> T,
	) -> Option<T> {
		if let Self::GroupDecrease(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_group_increase(&self) -> Option<&NoticeEventGroupIncrease> {
		if let Self::GroupIncrease(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_group_increase<T>(
		&self,
		handler: impl FnOnce(&NoticeEventGroupIncrease) -> T,
	) -> Option<T> {
		if let Self::GroupIncrease(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_increase_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventGroupIncrease) -> T,
	) -> Option<T> {
		if let Self::GroupIncrease(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_group_ban(&self) -> Option<&NoticeEventGroupBan> {
		if let Self::GroupBan(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_group_ban<T>(&self, handler: impl FnOnce(&NoticeEventGroupBan) -> T) -> Option<T> {
		if let Self::GroupBan(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_ban_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventGroupBan) -> T,
	) -> Option<T> {
		if let Self::GroupBan(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_friend_add(&self) -> Option<&NoticeEventFriendAdd> {
		if let Self::FriendAdd(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_friend_add<T>(&self, handler: impl FnOnce(&NoticeEventFriendAdd) -> T) -> Option<T> {
		if let Self::FriendAdd(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_friend_add_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventFriendAdd) -> T,
	) -> Option<T> {
		if let Self::FriendAdd(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_group_recall(&self) -> Option<&NoticeEventGroupRecall> {
		if let Self::GroupRecall(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_group_recall<T>(
		&self,
		handler: impl FnOnce(&NoticeEventGroupRecall) -> T,
	) -> Option<T> {
		if let Self::GroupRecall(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_group_recall_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventGroupRecall) -> T,
	) -> Option<T> {
		if let Self::GroupRecall(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_friend_recall(&self) -> Option<&NoticeEventFriendRecall> {
		if let Self::FriendRecall(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_friend_recall<T>(
		&self,
		handler: impl FnOnce(&NoticeEventFriendRecall) -> T,
	) -> Option<T> {
		if let Self::FriendRecall(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_friend_recall_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventFriendRecall) -> T,
	) -> Option<T> {
		if let Self::FriendRecall(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_notify(&self) -> Option<&NoticeEventNotify> {
		if let Self::Notify(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_notify<T>(&self, handler: impl FnOnce(&NoticeEventNotify) -> T) -> Option<T> {
		if let Self::Notify(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_notify_async<T>(
		&self,
		handler: impl AsyncFnOnce(&NoticeEventNotify) -> T,
	) -> Option<T> {
		if let Self::Notify(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, NoticeEvent> {
	pub fn group_upload(&self) -> Selector<'a, NoticeEventGroupUpload> {
		Selector {
			data: self.data.and_then(|d| d.match_group_upload()),
		}
	}

	pub fn group_admin(&self) -> Selector<'a, NoticeEventGroupAdmin> {
		Selector {
			data: self.data.and_then(|d| d.match_group_admin()),
		}
	}

	pub fn group_decrease(&self) -> Selector<'a, NoticeEventGroupDecrease> {
		Selector {
			data: self.data.and_then(|d| d.match_group_decrease()),
		}
	}

	pub fn group_increase(&self) -> Selector<'a, NoticeEventGroupIncrease> {
		Selector {
			data: self.data.and_then(|d| d.match_group_increase()),
		}
	}

	pub fn group_ban(&self) -> Selector<'a, NoticeEventGroupBan> {
		Selector {
			data: self.data.and_then(|d| d.match_group_ban()),
		}
	}

	pub fn friend_add(&self) -> Selector<'a, NoticeEventFriendAdd> {
		Selector {
			data: self.data.and_then(|d| d.match_friend_add()),
		}
	}

	pub fn group_recall(&self) -> Selector<'a, NoticeEventGroupRecall> {
		Selector {
			data: self.data.and_then(|d| d.match_group_recall()),
		}
	}

	pub fn friend_recall(&self) -> Selector<'a, NoticeEventFriendRecall> {
		Selector {
			data: self.data.and_then(|d| d.match_friend_recall()),
		}
	}

	pub fn notify(&self) -> Selector<'a, NoticeEventNotify> {
		Selector {
			data: self.data.and_then(|d| d.match_notify()),
		}
	}
}
