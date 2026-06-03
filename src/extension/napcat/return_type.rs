use crate::api::return_type::GetMsgResponse;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct ArkSharePeerResponse {
	#[serde(rename = "errCode")]
	pub err_code: i64,
	#[serde(rename = "errMsg")]
	pub err_msg: String,
	#[serde(rename = "arkJson")]
	pub ark_json: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetRobotUinRangeResponse {
	#[serde(rename = "minUin")]
	pub min_uin: i64,
	#[serde(rename = "maxUin")]
	pub max_uin: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetFriendsWithCategoryResponse {
	#[serde(rename = "categoryId")]
	pub category_id: i64,
	#[serde(rename = "categorySortId")]
	pub category_sort_id: i64,
	#[serde(rename = "categoryName")]
	pub category_name: String,
	#[serde(rename = "categoryMbCount")]
	pub category_mb_count: i64,
	#[serde(rename = "onlineCount")]
	pub online_count: i64,
	#[serde(rename = "buddyList")]
	pub buddy_list: Vec<GetFriendsWithCategoryBuddyResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetFriendsWithCategoryBuddyResponse {
	pub qid: String,
	#[serde(rename = "longNick")]
	pub long_nick: String,
	pub birthday_year: i64,
	pub birthday_month: i64,
	pub birthday_day: i64,
	pub age: i64,
	pub sex: String,
	#[serde(rename = "eMail")]
	pub email: String,
	#[serde(rename = "phoneNum")]
	pub phone_num: String,
	#[serde(rename = "categoryId")]
	pub category_id: i64,
	#[serde(rename = "richTime")]
	pub rich_time: i64,
	#[serde(rename = "richBuffer")]
	pub rich_buffer: HashMap<String, i64>,
	pub uid: String,
	pub uin: String,
	pub nick: String,
	pub remark: String,
	pub user_id: i64,
	pub nickname: String,
	pub level: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetFileResponse {
	pub file: String,
	pub url: String,
	pub file_size: String,
	pub file_name: String,
	pub base64: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SendForwardMsgResponse {
	pub message_id: i64,
	pub res_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DataStringResponse {
	pub data: String,
}

#[derive(Deserialize)]
pub struct GetFriendMsgHistoryResponse {
	pub messages: Vec<GetMsgResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SetSelfLongnickResponse {
	pub result: i64,
	#[serde(rename = "errMsg")]
	pub err_msg: String,
}

#[derive(Deserialize)]
pub struct GetRecentContactResponse {
	#[serde(rename = "lastestMsg")]
	pub latest_msg: GetMsgResponse,
	#[serde(rename = "peerUin")]
	pub peer_uin: i64,
	pub remark: String,
	#[serde(rename = "msgTime")]
	pub msg_time: String,
	#[serde(rename = "chatType")]
	pub chat_type: i64,
	#[serde(rename = "msgId")]
	pub msg_id: String,
	#[serde(rename = "sendNickName")]
	pub send_nick_name: String,
	#[serde(rename = "sendMemberName")]
	pub send_member_name: String,
	#[serde(rename = "peerName")]
	pub peer_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetProfileLikeResponse {
	pub total_count: i64,
	pub new_count: i64,
	pub new_nearby_count: i64,
	pub last_visit_time: i64,
	#[serde(rename = "userInfos")]
	pub user_infos: Vec<GetProfileLikeUserInfoResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetProfileLikeUserInfoResponse {
	pub uid: String,
	pub src: i64,
	#[serde(rename = "latestTime")]
	pub latest_time: i64,
	pub count: i64,
	#[serde(rename = "giftCount")]
	pub gift_count: i64,
	#[serde(rename = "customId")]
	pub custom_id: i64,
	#[serde(rename = "lastCharged")]
	pub last_charged: i64,
	#[serde(rename = "bAvailableCnt")]
	pub b_available_cnt: i64,
	#[serde(rename = "bTodayVotedCnt")]
	pub b_today_voted_cnt: i64,
	pub nick: String,
	pub gender: i64,
	pub age: i64,
	#[serde(rename = "isFriend")]
	pub is_friend: bool,
	pub isvip: bool,
	#[serde(rename = "isSvip")]
	pub is_svip: bool,
	pub uin: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetAiCharactersResponse {
	#[serde(rename = "type")]
	pub kind: String,
	pub characters: Vec<GetAiCharacterResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetAiCharacterResponse {
	pub character_id: String,
	pub character_name: String,
	pub preview_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SendGroupAiRecordResponse {
	pub message_id: String,
}
