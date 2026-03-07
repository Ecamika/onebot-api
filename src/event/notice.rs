use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "notice_type")]
pub enum NoticeEvent {
	#[serde(rename = "group_upload")]
	GroupUpload {
		group_id: i64,
		user_id: i64,
		file: GroupFile,
	},

	#[serde(rename = "group_admin")]
	GroupAdmin {
		sub_type: GroupAdminType,
		group_id: i64,
		user_id: i64,
	},

	#[serde(rename = "group_decrease")]
	GroupDecrease {
		sub_type: GroupDecreaseType,
		group_id: i64,
		operator_id: i64,
		user_id: i64,
	},

	#[serde(rename = "group_increase")]
	GroupIncrease {
		sub_type: GroupIncreaseType,
		group_id: i64,
		operator_id: i64,
		user_id: i64,
	},

	#[serde(rename = "group_ban")]
	GroupBan {
		sub_type: GroupBanType,
		group_id: i64,
		operator_id: i64,
		user_id: i64,
		duration: i64,
	},

	#[serde(rename = "friend_add")]
	FriendAdd { user_id: i64 },

	#[serde(rename = "group_recall")]
	GroupRecall {
		group_id: i64,
		user_id: i64,
		operator_id: i64,
		message_id: i64,
	},

	#[serde(rename = "friend_recall")]
	FriendRecall { user_id: i64, message_id: i64 },

	#[serde(rename = "notify")]
	Notify {
		group_id: i64,
		user_id: i64,
		#[serde(flatten)]
		data: NotifyType,
	},
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupFile {
	pub id: String,
	pub name: String,
	pub size: i64,
	pub busid: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub enum GroupAdminType {
	#[serde(rename = "set")]
	Set,
	#[serde(rename = "unset")]
	Unset,
}

#[derive(Deserialize, Debug, Clone)]
pub enum GroupDecreaseType {
	#[serde(rename = "leave")]
	Leave,
	#[serde(rename = "kick")]
	Kick,
	#[serde(rename = "kick_me")]
	KickMe,
}

#[derive(Deserialize, Debug, Clone)]
pub enum GroupIncreaseType {
	#[serde(rename = "approve")]
	Approve,
	#[serde(rename = "invite")]
	Invite,
}

#[derive(Deserialize, Debug, Clone)]
pub enum GroupBanType {
	#[serde(rename = "ban")]
	Ban,
	#[serde(rename = "lift_ban")]
	LiftBan,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "sub_type")]
pub enum NotifyType {
	#[serde(rename = "poke")]
	Poke { target_id: i64 },
	#[serde(rename = "lucky_king")]
	LuckyKing { target_id: i64 },
	#[serde(rename = "honor")]
	Honor { honor_type: HonorType },
}

#[derive(Deserialize, Debug, Clone)]
pub enum HonorType {
	#[serde(rename = "talkative")]
	Talkative,
	#[serde(rename = "performer")]
	Performer,
	#[serde(rename = "emotion")]
	Emotion,
}
