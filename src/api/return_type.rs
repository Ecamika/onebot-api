use super::arg_type::*;
use crate::event::message::{
	GroupMessageSender, GroupMessageSenderRole, PrivateMessageSender, Sex,
};
use crate::message::Segment;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct GetMsgResponse {
	pub time: i32,
	pub message_id: i32,
	pub real_id: i32,
	pub sender: Sender,
	pub message: Vec<Segment>,
}

#[derive(Deserialize)]
#[serde(tag = "message_type")]
pub enum Sender {
	#[serde(rename = "private")]
	Private {
		#[serde(flatten)]
		inner: PrivateMessageSender,
	},
	#[serde(rename = "group")]
	Group {
		#[serde(flatten)]
		inner: GroupMessageSender,
	},
}

#[derive(Deserialize)]
pub struct GetLoginInfoResponse {
	pub user_id: i32,
	pub nickname: String,
}

#[derive(Deserialize)]
pub struct GetStrangerInfoResponse {
	pub user_id: i32,
	pub nickname: String,
	pub sex: Sex,
	pub age: i32,
}

#[derive(Deserialize)]
pub struct GetFriendListResponse {
	pub user_id: i32,
	pub nickname: String,
	pub remark: String,
}

#[derive(Deserialize, Debug)]
pub struct GetGroupInfoResponse {
	pub group_id: i64,
	pub group_name: String,
	pub member_count: i64,
	pub max_member_count: i64,
}

#[derive(Deserialize)]
pub struct GetGroupMemberInfoResponse {
	pub group_id: i32,
	pub user_id: i32,
	pub nickname: String,
	pub card: String,
	pub sex: Sex,
	pub age: i32,
	pub area: String,
	pub join_time: i32,
	pub last_sent_time: i32,
	pub level: String,
	pub role: GroupMessageSenderRole,
	pub unfriendly: bool,
	pub title: String,
	pub title_expire_time: i32,
	pub card_changeable: bool,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct SendMsgResponse {
	pub message_id: i32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct CanSendResponse {
	pub yes: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetGroupHonorInfoResponse {
	pub group_id: i64,
	pub current_talkative: Option<CurrentTalkative>,
	pub talkative_list: Option<Vec<HonorInfoListData>>,
	pub performer_list: Option<Vec<HonorInfoListData>>,
	pub legend_list: Option<Vec<HonorInfoListData>>,
	pub strong_newbie_list: Option<Vec<HonorInfoListData>>,
	pub emotion_list: Option<Vec<HonorInfoListData>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CurrentTalkative {
	pub user_id: i64,
	pub nickname: String,
	pub avatar: String,
	pub day_count: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HonorInfoListData {
	pub user_id: i64,
	pub nickname: String,
	pub avatar: String,
	pub description: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetCookiesResponse {
	pub cookies: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetCsrfTokenResponse {
	pub token: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetCredentialsResponse {
	pub cookies: String,
	pub csrf_token: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetDataResponse {
	pub file: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetStatusResponse {
	pub online: bool,
	pub good: bool,
	#[serde(flatten)]
	pub data: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetVersionInfoResponse {
	pub app_name: String,
	pub app_version: String,
	pub protocol_version: String,
	#[serde(flatten)]
	pub data: HashMap<String, Value>,
}
