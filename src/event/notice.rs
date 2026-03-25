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

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupAdmin {
	sub_type: GroupAdminType,
	group_id: i64,
	user_id: i64,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupDecrease {
	sub_type: GroupDecreaseType,
	operator_id: i64,
	user_id: i64,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupIncrease {
	sub_type: GroupIncreaseType,
	group_id: i64,
	operator_id: i64,
	user_id: i64,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupBan {
	sub_type: GroupBanType,
	group_id: i64,
	operator_id: i64,
	user_id: i64,
	duration: i64,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventFriendAdd {
	user_id: i64,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventGroupRecall {
	group_id: i64,
	user_id: i64,
	operator_id: i64,
	message_id: i64,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventFriendRecall {
	user_id: i64,
	message_id: i64,
}

#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NoticeEventNotify {
	group_id: i64,
	user_id: i64,
	#[serde(flatten)]
	data: NotifyType,
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
