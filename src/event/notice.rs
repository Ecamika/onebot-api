use serde::Deserialize;
use strum::{Display, EnumIs};
#[cfg(feature = "selector")]
use tynavi::Selector;

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

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupUpload {
	pub group_id: i64,
	pub user_id: i64,
	pub file: GroupFile,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupAdmin {
	#[cfg_attr(feature = "selector", selector(variants(set, unset)))]
	pub sub_type: GroupAdminType,
	pub group_id: i64,
	pub user_id: i64,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupDecrease {
	#[cfg_attr(feature = "selector", selector(variants(leave, kick, kick_me)))]
	pub sub_type: GroupDecreaseType,
	pub group_id: i64,
	pub operator_id: i64,
	pub user_id: i64,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupIncrease {
	#[cfg_attr(feature = "selector", selector(variants(approve, invite)))]
	pub sub_type: GroupIncreaseType,
	pub group_id: i64,
	pub operator_id: i64,
	pub user_id: i64,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupBan {
	#[cfg_attr(feature = "selector", selector(variants(ban, lift_ban)))]
	pub sub_type: GroupBanType,
	pub group_id: i64,
	pub operator_id: i64,
	pub user_id: i64,
	pub duration: i64,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventFriendAdd {
	pub user_id: i64,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupRecall {
	pub group_id: i64,
	pub user_id: i64,
	pub operator_id: i64,
	pub message_id: i64,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventFriendRecall {
	pub user_id: i64,
	pub message_id: i64,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventNotify {
	pub group_id: i64,
	pub user_id: i64,
	#[serde(flatten)]
	pub data: NotifyType,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[cfg_attr(not(feature = "selector"), derive(EnumIs))]
#[derive(Deserialize, Debug, Clone, Display, Ord, PartialOrd, Eq, PartialEq)]
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
