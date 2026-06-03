use crate::api::arg_type::MessageType;
use crate::communication::utils::Client;
use crate::error::APIResult;
use crate::message::send_segment::SendSegment;
use async_trait::async_trait;
use onebot_api_macros::api_sender;
use return_type::*;
use serde_json::Value;

#[path = "return_type.rs"]
pub mod return_type;

/// NapCat 扩展 API 发送者 trait。
///
/// 该 trait 定义了 NapCat 在 OneBot V11 标准接口之外补充的扩展 API，
/// 实现者负责将 API 请求序列化为 JSON 并通过底层通信通道发送。
///
/// 所有方法均为异步，返回 [`APIResult`] 以处理可能的网络或协议错误。
///
/// # 参考
/// - [NapCat 扩展 API 文档](https://napneko.github.io/develop/api/doc)
#[async_trait]
pub trait NapCatAPISender {
	/// 群签到。
	///
	/// # 参数
	/// - `group_id` — 群号。
	async fn set_group_sign(&self, group_id: i64) -> APIResult<()>;

	/// 群聊戳一戳。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `user_id` — 对方 QQ 号。
	async fn group_poke(&self, group_id: i64, user_id: i64) -> APIResult<()>;

	/// 私聊戳一戳。
	///
	/// # 参数
	/// - `user_id` — 对方 QQ 号。
	async fn friend_poke(&self, user_id: i64) -> APIResult<()>;

	/// 获取推荐好友/群聊卡片。
	///
	/// # 参数
	/// - `user_id` — 对方 QQ 号，与 `group_id` 二选一。
	/// - `phone_number` — 对方手机号。
	/// - `group_id` — 群号，与 `user_id` 二选一。
	///
	/// # 返回值
	/// 返回包含错误码、错误信息与 `arkJson` 的响应对象。
	async fn ark_share_peer(
		&self,
		user_id: Option<i64>,
		phone_number: Option<String>,
		group_id: Option<i64>,
	) -> APIResult<ArkSharePeerResponse>;

	/// 获取推荐群聊卡片。
	///
	/// # 参数
	/// - `group_id` — 群号。
	///
	/// # 返回值
	/// 返回卡片 JSON 字符串。
	async fn ark_share_group(&self, group_id: i64) -> APIResult<String>;

	/// 获取机器人账号范围。
	///
	/// # 返回值
	/// 返回账号范围列表。
	async fn get_robot_uin_range(&self) -> APIResult<Vec<GetRobotUinRangeResponse>>;

	/// 设置在线状态。
	///
	/// # 参数
	/// - `status` — 在线状态。
	/// - `ext_status` — 扩展在线状态。
	/// - `battery_status` — 电量。
	async fn set_online_status(
		&self,
		status: i64,
		ext_status: Option<i64>,
		battery_status: Option<i64>,
	) -> APIResult<()>;

	/// 获取分类的好友列表。
	///
	/// # 返回值
	/// 返回按分类组织的好友列表。
	async fn get_friends_with_category(&self) -> APIResult<Vec<GetFriendsWithCategoryResponse>>;

	/// 设置 QQ 头像。
	///
	/// # 参数
	/// - `file` — 图片路径或链接。
	async fn set_qq_avatar(&self, file: String) -> APIResult<()>;

	/// 获取文件信息。
	///
	/// # 参数
	/// - `file_id` — 文件 ID。
	///
	/// # 返回值
	/// 返回文件路径、链接、文件名等响应数据。
	async fn get_file(&self, file_id: String) -> APIResult<GetFileResponse>;

	/// 转发单条消息到私聊。
	///
	/// # 参数
	/// - `message_id` — 消息 ID。
	/// - `user_id` — 目标 QQ 号。
	async fn forward_friend_single_msg(&self, message_id: i64, user_id: i64) -> APIResult<()>;

	/// 转发单条消息到群聊。
	///
	/// # 参数
	/// - `message_id` — 消息 ID。
	/// - `group_id` — 目标群号。
	async fn forward_group_single_msg(&self, message_id: i64, group_id: i64) -> APIResult<()>;

	/// 英译中。
	///
	/// # 参数
	/// - `words` — 英文单词或短语数组。
	///
	/// # 返回值
	/// 返回对应的中文翻译数组。
	async fn translate_en2zh(&self, words: Vec<String>) -> APIResult<Vec<String>>;

	/// 设置消息表情回复。
	///
	/// # 参数
	/// - `message_id` — 消息 ID。
	/// - `emoji_id` — 表情 ID。
	async fn set_msg_emoji_like(&self, message_id: i64, emoji_id: String) -> APIResult<()>;

	/// 发送合并转发消息。
	///
	/// # 参数
	/// - `message_type` — 消息类型，支持 `private` 与 `group`。
	/// - `user_id` — 私聊目标 QQ 号。
	/// - `group_id` — 群聊目标群号。
	/// - `messages` — `node` 类型消息数组。
	///
	/// # 返回值
	/// 返回包含 `message_id` 与 `res_id` 的响应。
	async fn send_forward_msg(
		&self,
		message_type: Option<MessageType>,
		user_id: i64,
		group_id: i64,
		messages: Vec<SendSegment>,
	) -> APIResult<SendForwardMsgResponse>;

	/// 设置私聊已读。
	///
	/// # 参数
	/// - `user_id` — QQ 号。
	async fn mark_private_msg_as_read(&self, user_id: i64) -> APIResult<()>;

	/// 设置群聊已读。
	///
	/// # 参数
	/// - `group_id` — 群号。
	async fn mark_group_msg_as_read(&self, group_id: i64) -> APIResult<()>;

	/// 获取私聊历史记录。
	///
	/// # 参数
	/// - `user_id` — QQ 号。
	/// - `message_seq` — 起始消息序号。
	/// - `count` — 获取数量。
	/// - `reverse_order` — 是否倒序。
	///
	/// # 返回值
	/// 返回包含 `messages` 的响应对象。
	async fn get_friend_msg_history(
		&self,
		user_id: i64,
		message_seq: Option<String>,
		count: Option<i64>,
		reverse_order: Option<bool>,
	) -> APIResult<GetFriendMsgHistoryResponse>;

	/// 创建收藏。
	///
	/// NapCat 文档当前页未明确列出参数与返回结构，先使用原始 JSON 载荷占位。
	async fn create_collection(&self, payload: Value) -> APIResult<Value>;

	/// 获取收藏列表。
	///
	/// NapCat 文档当前页未明确列出响应结构，先返回原始 JSON。
	async fn get_collection_list(&self) -> APIResult<Value>;

	/// 设置个性签名。
	///
	/// # 参数
	/// - `long_nick` — 签名内容。
	///
	/// # 返回值
	/// 返回包含 `result` 与 `errMsg` 的响应对象。
	async fn set_self_longnick(&self, long_nick: String) -> APIResult<SetSelfLongnickResponse>;

	/// 获取最近联系人。
	///
	/// # 参数
	/// - `count` — 获取数量。
	///
	/// # 返回值
	/// 返回最近联系人列表。
	async fn get_recent_contact(
		&self,
		count: Option<i64>,
	) -> APIResult<Vec<GetRecentContactResponse>>;

	/// 标记全部已读。
	///
	/// 对应 NapCat action `_mark_all_as_read`。
	async fn mark_all_as_read(&self) -> APIResult<()>;

	/// 获取自身点赞列表。
	///
	/// # 返回值
	/// 返回点赞统计与用户列表。
	async fn get_profile_like(&self) -> APIResult<GetProfileLikeResponse>;

	/// 获取自定义表情。
	///
	/// # 参数
	/// - `count` — 获取数量。
	///
	/// # 返回值
	/// 返回表情标识数组。
	async fn fetch_custom_face(&self, count: Option<i64>) -> APIResult<Vec<String>>;

	/// AI 文字转语音。
	///
	/// # 参数
	/// - `character` — AI 角色编号。
	/// - `group_id` — 群号。
	/// - `text` — 需要转语音的文字。
	///
	/// # 返回值
	/// 返回转换后的语音链接。
	async fn get_ai_record(
		&self,
		character: String,
		group_id: i64,
		text: String,
	) -> APIResult<String>;

	/// 获取 AI 语音角色列表。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `chat_type` — 聊天类型。
	///
	/// # 返回值
	/// 返回角色分类与角色列表。
	async fn get_ai_characters(
		&self,
		group_id: i64,
		chat_type: Option<i64>,
	) -> APIResult<GetAiCharactersResponse>;

	/// 发送群聊 AI 语音。
	///
	/// # 参数
	/// - `character` — AI 角色编号。
	/// - `group_id` — 群号。
	/// - `text` — 需要转语音的文字。
	///
	/// # 返回值
	/// 返回发送后的消息 ID。
	async fn send_group_ai_record(
		&self,
		character: String,
		group_id: i64,
		text: String,
	) -> APIResult<String>;

	/// 群聊或私聊戳一戳。
	///
	/// # 参数
	/// - `group_id` — 传入时按群聊戳一戳处理，否则按私聊戳一戳处理。
	/// - `user_id` — 对方 QQ 号。
	async fn send_poke(&self, group_id: Option<i64>, user_id: i64) -> APIResult<()>;
}

#[api_sender]
#[async_trait]
impl NapCatAPISender for Client {
	#[api(discard = true)]
	async fn set_group_sign(&self, group_id: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn group_poke(&self, group_id: i64, user_id: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn friend_poke(&self, user_id: i64) -> APIResult<()> {}

	#[api(action = "ArkSharePeer", map(phone_number = "phoneNumber"))]
	async fn ark_share_peer(
		&self,
		user_id: Option<i64>,
		phone_number: Option<String>,
		group_id: Option<i64>,
	) -> APIResult<ArkSharePeerResponse> {
	}

	#[api(action = "ArkShareGroup")]
	async fn ark_share_group(&self, group_id: i64) -> APIResult<String> {}

	async fn get_robot_uin_range(&self) -> APIResult<Vec<GetRobotUinRangeResponse>> {}

	#[api(discard = true)]
	async fn set_online_status(
		&self,
		status: i64,
		ext_status: Option<i64>,
		battery_status: Option<i64>,
	) -> APIResult<()> {
	}

	async fn get_friends_with_category(&self) -> APIResult<Vec<GetFriendsWithCategoryResponse>> {}

	#[api(discard = true)]
	async fn set_qq_avatar(&self, file: String) -> APIResult<()> {}

	async fn get_file(&self, file_id: String) -> APIResult<GetFileResponse> {}

	#[api(discard = true)]
	async fn forward_friend_single_msg(&self, message_id: i64, user_id: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn forward_group_single_msg(&self, message_id: i64, group_id: i64) -> APIResult<()> {}

	async fn translate_en2zh(&self, words: Vec<String>) -> APIResult<Vec<String>> {}

	#[api(discard = true)]
	async fn set_msg_emoji_like(&self, message_id: i64, emoji_id: String) -> APIResult<()> {}

	async fn send_forward_msg(
		&self,
		message_type: Option<MessageType>,
		user_id: i64,
		group_id: i64,
		messages: Vec<SendSegment>,
	) -> APIResult<SendForwardMsgResponse> {
	}

	#[api(discard = true)]
	async fn mark_private_msg_as_read(&self, user_id: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn mark_group_msg_as_read(&self, group_id: i64) -> APIResult<()> {}

	#[api(map(reverse_order = "reverseOrder"))]
	async fn get_friend_msg_history(
		&self,
		user_id: i64,
		message_seq: Option<String>,
		count: Option<i64>,
		reverse_order: Option<bool>,
	) -> APIResult<GetFriendMsgHistoryResponse> {
	}

	async fn create_collection(&self, payload: Value) -> APIResult<Value> {}

	async fn get_collection_list(&self) -> APIResult<Value> {}

	#[api(map(long_nick = "longNick"))]
	async fn set_self_longnick(&self, long_nick: String) -> APIResult<SetSelfLongnickResponse> {}

	async fn get_recent_contact(
		&self,
		count: Option<i64>,
	) -> APIResult<Vec<GetRecentContactResponse>> {
	}

	#[api(action = "_mark_all_as_read", discard = true)]
	async fn mark_all_as_read(&self) -> APIResult<()> {}

	async fn get_profile_like(&self) -> APIResult<GetProfileLikeResponse> {}

	async fn fetch_custom_face(&self, count: Option<i64>) -> APIResult<Vec<String>> {}

	#[api(response = DataStringResponse, extract = "data")]
	async fn get_ai_record(
		&self,
		character: String,
		group_id: i64,
		text: String,
	) -> APIResult<String> {
	}

	async fn get_ai_characters(
		&self,
		group_id: i64,
		chat_type: Option<i64>,
	) -> APIResult<GetAiCharactersResponse> {
	}

	#[api(response = SendGroupAiRecordResponse, extract = "message_id")]
	async fn send_group_ai_record(
		&self,
		character: String,
		group_id: i64,
		text: String,
	) -> APIResult<String> {
	}

	#[api(discard = true)]
	async fn send_poke(&self, group_id: Option<i64>, user_id: i64) -> APIResult<()> {}
}
