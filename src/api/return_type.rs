use crate::event::message::{
	GroupMessageSender, GroupMessageSenderRole, PrivateMessageSender, Sex,
};
use crate::message::receive_segment::ReceiveSegment;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// [`get_msg`](super::APISender::get_msg) 的响应数据。
///
/// 包含消息的发送时间、消息 ID、发送人信息及消息内容。
#[derive(Deserialize)]
pub struct GetMsgResponse {
	/// 发送时间戳。
	pub time: i32,
	/// 消息 ID。
	pub message_id: i32,
	/// 消息真实 ID。
	pub real_id: i32,
	/// 发送人信息。
	pub sender: Sender,
	/// 消息内容。
	pub message: Vec<ReceiveSegment>,
}

/// 消息发送人信息，根据消息类型区分私聊或群聊发送人。
#[derive(Deserialize)]
#[serde(tag = "message_type")]
pub enum Sender {
	/// 私聊消息发送人。
	#[serde(rename = "private")]
	Private {
		/// 私聊发送人详细信息。
		#[serde(flatten)]
		inner: PrivateMessageSender,
	},
	/// 群消息发送人。
	#[serde(rename = "group")]
	Group {
		/// 群消息发送人详细信息。
		#[serde(flatten)]
		inner: GroupMessageSender,
	},
}

/// [`get_login_info`](super::APISender::get_login_info) 的响应数据。
#[derive(Deserialize)]
pub struct GetLoginInfoResponse {
	/// QQ 号。
	pub user_id: i32,
	/// QQ 昵称。
	pub nickname: String,
}

/// [`get_forward_msg`](super::APISender::get_forward_msg) 的响应数据。
#[derive(Deserialize)]
pub struct GetForwardMsgResponse {
	/// 合并转发消息内容，数组中每个消息段均为 `node` 类型。
	pub message: Vec<ReceiveSegment>,
}

/// [`get_stranger_info`](super::APISender::get_stranger_info) 的响应数据。
#[derive(Deserialize)]
pub struct GetStrangerInfoResponse {
	/// QQ 号。
	pub user_id: i32,
	/// 昵称。
	pub nickname: String,
	/// 性别。
	pub sex: Sex,
	/// 年龄。
	pub age: i32,
}

/// [`get_friend_list`](super::APISender::get_friend_list) 的响应数组元素。
#[derive(Deserialize)]
pub struct GetFriendListResponse {
	/// QQ 号。
	pub user_id: i32,
	/// 昵称。
	pub nickname: String,
	/// 备注名。
	pub remark: String,
}

/// [`get_group_info`](super::APISender::get_group_info) 与 [`get_group_list`](super::APISender::get_group_list) 的响应数据。
#[derive(Deserialize, Debug)]
pub struct GetGroupInfoResponse {
	/// 群号。
	pub group_id: i64,
	/// 群名称。
	pub group_name: String,
	/// 成员数。
	pub member_count: i64,
	/// 最大成员数（群容量）。
	pub max_member_count: i64,
}

/// [`get_group_member_info`](super::APISender::get_group_member_info) 与 [`get_group_member_list`](super::APISender::get_group_member_list) 的响应数据。
///
/// 注意：通过 [`get_group_member_list`](super::APISender::get_group_member_list) 获取列表时，
/// 某些字段（如 `area`、`title` 等）可能缺失，具体应以单独获取的成员信息为准。
#[derive(Deserialize)]
pub struct GetGroupMemberInfoResponse {
	/// 群号。
	pub group_id: i32,
	/// QQ 号。
	pub user_id: i32,
	/// 昵称。
	pub nickname: String,
	/// 群名片／备注。
	pub card: String,
	/// 性别。
	pub sex: Sex,
	/// 年龄。
	pub age: i32,
	/// 地区。
	pub area: String,
	/// 加群时间戳。
	pub join_time: i32,
	/// 最后发言时间戳。
	pub last_sent_time: i32,
	/// 成员等级。
	pub level: String,
	/// 角色（`owner`、`admin` 或 `member`）。
	pub role: GroupMessageSenderRole,
	/// 是否不良记录成员。
	pub unfriendly: bool,
	/// 专属头衔。
	pub title: String,
	/// 专属头衔过期时间戳。
	pub title_expire_time: i32,
	/// 是否允许修改群名片。
	pub card_changeable: bool,
}

/// [`send_private_msg`](super::APISender::send_private_msg)、[`send_group_msg`](super::APISender::send_group_msg) 及 [`send_msg`](super::APISender::send_msg) 的响应数据。
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct SendMsgResponse {
	/// 消息 ID。
	pub message_id: i32,
}

/// [`can_send_image`](super::APISender::can_send_image) 与 [`can_send_record`](super::APISender::can_send_record) 的响应数据。
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct CanSendResponse {
	/// 是否可以发送。
	pub yes: bool,
}

/// [`get_group_honor_info`](super::APISender::get_group_honor_info) 的响应数据。
#[derive(Deserialize, Debug, Clone)]
pub struct GetGroupHonorInfoResponse {
	/// 群号。
	pub group_id: i64,
	/// 当前龙王（仅 `type` 为 `talkative` 或 `all` 时有数据）。
	pub current_talkative: Option<CurrentTalkative>,
	/// 历史龙王列表（仅 `type` 为 `talkative` 或 `all` 时有数据）。
	pub talkative_list: Option<Vec<HonorInfoListData>>,
	/// 群聊之火列表（仅 `type` 为 `performer` 或 `all` 时有数据）。
	pub performer_list: Option<Vec<HonorInfoListData>>,
	/// 群聊炽焰列表（仅 `type` 为 `legend` 或 `all` 时有数据）。
	pub legend_list: Option<Vec<HonorInfoListData>>,
	/// 冒尖小春笋列表（仅 `type` 为 `strong_newbie` 或 `all` 时有数据）。
	pub strong_newbie_list: Option<Vec<HonorInfoListData>>,
	/// 快乐之源列表（仅 `type` 为 `emotion` 或 `all` 时有数据）。
	pub emotion_list: Option<Vec<HonorInfoListData>>,
}

/// 群荣誉信息 — 当前龙王。
#[derive(Deserialize, Debug, Clone)]
pub struct CurrentTalkative {
	/// QQ 号。
	pub user_id: i64,
	/// 昵称。
	pub nickname: String,
	/// 头像 URL。
	pub avatar: String,
	/// 持续天数。
	pub day_count: i32,
}

/// 群荣誉信息 — 历史荣誉列表元素。
#[derive(Deserialize, Debug, Clone)]
pub struct HonorInfoListData {
	/// QQ 号。
	pub user_id: i64,
	/// 昵称。
	pub nickname: String,
	/// 头像 URL。
	pub avatar: String,
	/// 荣誉描述。
	pub description: String,
}

/// [`get_cookies`](super::APISender::get_cookies) 的响应数据。
#[derive(Deserialize, Debug, Clone)]
pub struct GetCookiesResponse {
	/// Cookies。
	pub cookies: String,
}

/// [`get_csrf_token`](super::APISender::get_csrf_token) 的响应数据。
#[derive(Deserialize, Debug, Clone)]
pub struct GetCsrfTokenResponse {
	/// CSRF Token。
	pub token: i32,
}

/// [`get_credentials`](super::APISender::get_credentials) 的响应数据。
///
/// 合并了 [`get_cookies`](super::APISender::get_cookies) 和 [`get_csrf_token`](super::APISender::get_csrf_token) 的结果。
#[derive(Deserialize, Debug, Clone)]
pub struct GetCredentialsResponse {
	/// Cookies。
	pub cookies: String,
	/// CSRF Token。
	pub csrf_token: i32,
}

/// [`get_record`](super::APISender::get_record) 与 [`get_image`](super::APISender::get_image) 的响应数据。
#[derive(Deserialize, Debug, Clone)]
pub struct GetDataResponse {
	/// 转换/下载后的文件路径。
	pub file: String,
}

/// [`get_status`](super::APISender::get_status) 的响应数据。
///
/// 建议仅使用 `online` 和 `good` 两个字段判断运行状态，因为根据 OneBot 实现的不同，
/// 其它字段可能完全不同。
#[derive(Deserialize, Debug, Clone)]
pub struct GetStatusResponse {
	/// 当前 QQ 是否在线；`null` 表示无法查询到在线状态。
	pub online: bool,
	/// 状态是否符合预期（各模块正常运行、功能正常且 QQ 在线）。
	pub good: bool,
	/// OneBot 实现自行添加的其它内容。
	#[serde(flatten)]
	pub data: HashMap<String, Value>,
}

/// [`get_version_info`](super::APISender::get_version_info) 的响应数据。
#[derive(Deserialize, Debug, Clone)]
pub struct GetVersionInfoResponse {
	/// 应用标识，如 `mirai-native`。
	pub app_name: String,
	/// 应用版本，如 `1.2.3`。
	pub app_version: String,
	/// OneBot 标准版本，如 `v11`。
	pub protocol_version: String,
	/// OneBot 实现自行添加的其它内容。
	#[serde(flatten)]
	pub data: HashMap<String, Value>,
}
