use crate::error::APIResult;
use crate::event::message::GroupMessageAnonymous;
use crate::message::receive_segment::ReceiveSegment;
use crate::message::send_segment::SendSegment;
use arg_type::*;
use async_trait::async_trait;
use return_type::*;

pub mod arg_type;
pub mod return_type;

/// OneBot V11 协议 API 发送者 trait。
///
/// 该 trait 定义了 OneBot V11 标准公开 API 的完整接口，实现者负责将 API 请求
/// 序列化为 JSON 并通过底层通信通道（WebSocket、HTTP 等）发送。
///
/// 所有方法均为异步，返回 [`APIResult`] 以处理可能的网络或协议错误。
///
/// # 参考
/// - [OneBot V11 API 文档](https://github.com/botuniverse/onebot-11/blob/master/api/public.md)
#[async_trait]
pub trait APISender {
	/// 发送私聊消息。
	///
	/// # 参数
	/// - `user_id` — 对方 QQ 号。
	/// - `message` — 要发送的消息内容。
	/// - `auto_escape` — 消息内容是否作为纯文本发送（不解析 CQ 码），仅在 `message` 为字符串时有效。
	///
	/// # 返回值
	/// 返回发送成功的消息 ID（`i32`）。
	///
	/// # 参考
	/// - [`send_private_msg`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#send_private_msg-%E5%8F%91%E9%80%81%E7%A7%81%E8%81%8A%E6%B6%88%E6%81%AF)
	async fn send_private_msg(
		&self,
		user_id: i64,
		message: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32>;

	/// 发送群消息。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `message` — 要发送的消息内容。
	/// - `auto_escape` — 消息内容是否作为纯文本发送（不解析 CQ 码），仅在 `message` 为字符串时有效。
	///
	/// # 返回值
	/// 返回发送成功的消息 ID（`i32`）。
	///
	/// # 参考
	/// - [`send_group_msg`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#send_group_msg-%E5%8F%91%E9%80%81%E7%BE%A4%E6%B6%88%E6%81%AF)
	async fn send_group_msg(
		&self,
		group_id: i64,
		message: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32>;

	/// 发送消息（自动判断私聊/群聊）。
	///
	/// # 参数
	/// - `message_type` — 消息类型，`private` 或 `group`；如不传入则根据 `user_id`/`group_id` 判断。
	/// - `user_id` — 对方 QQ 号（私聊时需要）。
	/// - `group_id` — 群号（群聊时需要）。
	/// - `message` — 要发送的消息内容。
	/// - `auto_escape` — 消息内容是否作为纯文本发送（不解析 CQ 码），仅在 `message` 为字符串时有效。
	///
	/// # 返回值
	/// 返回发送成功的消息 ID（`i32`）。
	///
	/// # 参考
	/// - [`send_msg`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#send_msg-%E5%8F%91%E9%80%81%E6%B6%88%E6%81%AF)
	async fn send_msg(
		&self,
		message_type: Option<MessageType>,
		user_id: i64,
		group_id: i64,
		message: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32>;

	/// 撤回消息。
	///
	/// # 参数
	/// - `message_id` — 要撤回的消息 ID。
	///
	/// # 参考
	/// - [`delete_msg`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#delete_msg-%E6%92%A4%E5%9B%9E%E6%B6%88%E6%81%AF)
	async fn delete_msg(&self, message_id: i32) -> APIResult<()>;

	/// 获取消息详情。
	///
	/// # 参数
	/// - `message_id` — 消息 ID。
	///
	/// # 返回值
	/// 返回包含发送时间、消息类型、发送人信息及消息内容的 [`GetMsgResponse`]。
	///
	/// # 参考
	/// - [`get_msg`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_msg-%E8%8E%B7%E5%8F%96%E6%B6%88%E6%81%AF)
	async fn get_msg(&self, message_id: i32) -> APIResult<GetMsgResponse>;

	/// 获取合并转发消息内容。
	///
	/// # 参数
	/// - `id` — 合并转发消息 ID。
	///
	/// # 返回值
	/// 返回消息段数组，其中每个消息段均为 `node` 类型。
	///
	/// # 参考
	/// - [`get_forward_msg`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_forward_msg-%E8%8E%B7%E5%8F%96%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91%E6%B6%88%E6%81%AF)
	async fn get_forward_msg(&self, id: String) -> APIResult<Vec<ReceiveSegment>>;

	/// 发送好友赞。
	///
	/// # 参数
	/// - `user_id` — 对方 QQ 号。
	/// - `times` — 赞的次数，每个好友每天最多 10 次。
	///
	/// # 参考
	/// - [`send_like`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#send_like-%E5%8F%91%E9%80%81%E5%A5%BD%E5%8F%8B%E8%B5%9E)
	async fn send_like(&self, user_id: i64, times: Option<i32>) -> APIResult<()>;

	/// 群组踢人。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `user_id` — 要踢的 QQ 号。
	/// - `reject_add_request` — 是否拒绝此人后续的加群请求。
	///
	/// # 参考
	/// - [`set_group_kick`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_kick-%E7%BE%A4%E7%BB%84%E8%B8%A2%E4%BA%BA)
	async fn set_group_kick(
		&self,
		group_id: i32,
		user_id: i32,
		reject_add_request: Option<bool>,
	) -> APIResult<()>;

	/// 群组单人禁言。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `user_id` — 要禁言的 QQ 号。
	/// - `duration` — 禁言时长（秒），`0` 表示取消禁言。
	///
	/// # 参考
	/// - [`set_group_ban`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_ban-%E7%BE%A4%E7%BB%84%E5%8D%95%E4%BA%BA%E7%A6%81%E8%A8%80)
	async fn set_group_ban(
		&self,
		group_id: i32,
		user_id: i32,
		duration: Option<i32>,
	) -> APIResult<()>;

	/// 群组匿名用户禁言。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `anonymous` — 要禁言的匿名用户对象（群消息上报的 `anonymous` 字段）。
	/// - `flag` — 要禁言的匿名用户 flag（需从群消息上报数据中获得）。
	/// - `duration` — 禁言时长（秒），无法取消匿名用户禁言。
	///
	/// # 说明
	/// `anonymous` 和 `flag` 两者任选其一传入即可；若都传入则使用 `anonymous`。
	///
	/// # 参考
	/// - [`set_group_anonymous_ban`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_anonymous_ban-%E7%BE%A4%E7%BB%84%E5%8C%BF%E5%90%8D%E7%94%A8%E6%88%B7%E7%A6%81%E8%A8%80)
	async fn set_group_anonymous_ban(
		&self,
		group_id: i32,
		anonymous: Option<GroupMessageAnonymous>,
		flag: Option<String>,
		duration: Option<i32>,
	) -> APIResult<()>;

	/// 群组全员禁言。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `enable` — `true` 开启全员禁言，`false` 关闭。
	///
	/// # 参考
	/// - [`set_group_whole_ban`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_whole_ban-%E7%BE%A4%E7%BB%84%E5%85%A8%E5%91%98%E7%A6%81%E8%A8%80)
	async fn set_group_whole_ban(&self, group_id: i32, enable: Option<bool>) -> APIResult<()>;

	/// 群组设置管理员。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `user_id` — 要设置的 QQ 号。
	/// - `enable` — `true` 设置为管理员，`false` 取消管理员。
	///
	/// # 参考
	/// - [`set_group_admin`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_admin-%E7%BE%A4%E7%BB%84%E8%AE%BE%E7%BD%AE%E7%AE%A1%E7%90%86%E5%91%98)
	async fn set_group_admin(
		&self,
		group_id: i32,
		user_id: i32,
		enable: Option<bool>,
	) -> APIResult<()>;

	/// 设置群组匿名聊天。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `enable` — `true` 允许匿名聊天，`false` 禁止。
	///
	/// # 参考
	/// - [`set_group_anonymous`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_anonymous-%E7%BE%A4%E7%BB%84%E5%8C%BF%E5%90%8D)
	async fn set_group_anonymous(&self, group_id: i32, enable: Option<bool>) -> APIResult<()>;

	/// 设置群名片（群备注）。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `user_id` — 要设置的 QQ 号。
	/// - `card` — 群名片内容，不填或空字符串表示删除群名片。
	///
	/// # 参考
	/// - [`set_group_card`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_card-%E8%AE%BE%E7%BD%AE%E7%BE%A4%E5%90%8D%E7%89%87%E7%BE%A4%E5%A4%87%E6%B3%A8)
	async fn set_group_card(
		&self,
		group_id: i32,
		user_id: i32,
		card: Option<String>,
	) -> APIResult<()>;

	/// 设置群名。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `group_name` — 新群名。
	///
	/// # 参考
	/// - [`set_group_name`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_name-%E8%AE%BE%E7%BD%AE%E7%BE%A4%E5%90%8D)
	async fn set_group_name(&self, group_id: i32, group_name: String) -> APIResult<()>;

	/// 退出群组。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `is_dismiss` — 是否解散；若登录号为群主，仅在此项为 `true` 时能够解散。
	///
	/// # 参考
	/// - [`set_group_leave`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_leave-%E9%80%80%E5%87%BA%E7%BE%A4%E7%BB%84)
	async fn set_group_leave(&self, group_id: i32, is_dismiss: Option<bool>) -> APIResult<()>;

	/// 设置群组专属头衔。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `user_id` — 要设置的 QQ 号。
	/// - `special_title` — 专属头衔，不填或空字符串表示删除。
	/// - `duration` — 有效期（秒），`-1` 表示永久（效果取决于实现）。
	///
	/// # 参考
	/// - [`set_group_special_title`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_special_title-%E8%AE%BE%E7%BD%AE%E7%BE%A4%E7%BB%84%E4%B8%93%E5%B1%9E%E5%A4%B4%E8%A1%94)
	async fn set_group_special_title(
		&self,
		group_id: i32,
		user_id: i32,
		special_title: Option<String>,
		duration: Option<i32>,
	) -> APIResult<()>;

	/// 处理加好友请求。
	///
	/// # 参数
	/// - `flag` — 加好友请求的 flag（需从上报数据中获得）。
	/// - `approve` — 是否同意请求。
	/// - `remark` — 添加后的好友备注（仅在同意时有效）。
	///
	/// # 参考
	/// - [`set_friend_add_request`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_friend_add_request-%E5%A4%84%E7%90%86%E5%8A%A0%E5%A5%BD%E5%8F%8B%E8%AF%B7%E6%B1%82)
	async fn set_friend_add_request(
		&self,
		flag: String,
		approve: Option<bool>,
		remark: Option<String>,
	) -> APIResult<()>;

	/// 处理加群请求或邀请。
	///
	/// # 参数
	/// - `flag` — 加群请求的 flag（需从上报数据中获得）。
	/// - `sub_type` — 请求类型，`add` 或 `invite`，需与上报消息中的 `sub_type` 相符。
	/// - `approve` — 是否同意请求/邀请。
	/// - `reason` — 拒绝理由（仅在拒绝时有效）。
	///
	/// # 参考
	/// - [`set_group_add_request`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_group_add_request-%E5%A4%84%E7%90%86%E5%8A%A0%E7%BE%A4%E8%AF%B7%E6%B1%82%E9%82%80%E8%AF%B7)
	async fn set_group_add_request(
		&self,
		flag: String,
		sub_type: String,
		approve: Option<bool>,
		reason: Option<String>,
	) -> APIResult<()>;

	/// 获取登录号信息。
	///
	/// # 返回值
	/// 返回当前登录账号的 QQ 号及昵称。
	///
	/// # 参考
	/// - [`get_login_info`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_login_info-%E8%8E%B7%E5%8F%96%E7%99%BB%E5%BD%95%E5%8F%B7%E4%BF%A1%E6%81%AF)
	async fn get_login_info(&self) -> APIResult<GetLoginInfoResponse>;

	/// 获取陌生人信息。
	///
	/// # 参数
	/// - `user_id` — QQ 号。
	/// - `no_cache` — 是否不使用缓存；使用缓存响应更快但可能更新不及时。
	///
	/// # 返回值
	/// 返回 QQ 号、昵称、性别及年龄。
	///
	/// # 参考
	/// - [`get_stranger_info`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_stranger_info-%E8%8E%B7%E5%8F%96%E9%99%8C%E7%94%9F%E4%BA%BA%E4%BF%A1%E6%81%AF)
	async fn get_stranger_info(
		&self,
		user_id: i32,
		no_cache: Option<bool>,
	) -> APIResult<GetStrangerInfoResponse>;

	/// 获取好友列表。
	///
	/// # 返回值
	/// 返回好友信息数组，包含 QQ 号、昵称和备注名。
	///
	/// # 参考
	/// - [`get_friend_list`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_friend_list-%E8%8E%B7%E5%8F%96%E5%A5%BD%E5%8F%8B%E5%88%97%E8%A1%A8)
	async fn get_friend_list(&self) -> APIResult<Vec<GetFriendListResponse>>;

	/// 获取群信息。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `no_cache` — 是否不使用缓存；使用缓存响应更快但可能更新不及时。
	///
	/// # 返回值
	/// 返回群号、群名称、成员数及最大成员数。
	///
	/// # 参考
	/// - [`get_group_info`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_group_info-%E8%8E%B7%E5%8F%96%E7%BE%A4%E4%BF%A1%E6%81%AF)
	async fn get_group_info(
		&self,
		group_id: i32,
		no_cache: Option<bool>,
	) -> APIResult<GetGroupInfoResponse>;

	/// 获取群列表。
	///
	/// # 返回值
	/// 返回群信息数组，元素与 [`get_group_info`](Self::get_group_info) 接口相同。
	///
	/// # 参考
	/// - [`get_group_list`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_group_list-%E8%8E%B7%E5%8F%96%E7%BE%A4%E5%88%97%E8%A1%A8)
	async fn get_group_list(&self) -> APIResult<Vec<GetGroupInfoResponse>>;

	/// 获取群成员信息。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `user_id` — QQ 号。
	/// - `no_cache` — 是否不使用缓存；使用缓存响应更快但可能更新不及时。
	///
	/// # 返回值
	/// 返回群成员详细信息，包含昵称、群名片、性别、年龄、角色等。
	///
	/// # 参考
	/// - [`get_group_member_info`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_group_member_info-%E8%8E%B7%E5%8F%96%E7%BE%A4%E6%88%90%E5%91%98%E4%BF%A1%E6%81%AF)
	async fn get_group_member_info(
		&self,
		group_id: i32,
		user_id: i32,
		no_cache: Option<bool>,
	) -> APIResult<GetGroupMemberInfoResponse>;

	/// 获取群成员列表。
	///
	/// # 参数
	/// - `group_id` — 群号。
	///
	/// # 返回值
	/// 返回群成员信息数组。注意：获取列表时某些字段（如 `area`、`title`）可能缺失，
	/// 具体应以单独调用 [`get_group_member_info`](Self::get_group_member_info) 的结果为准。
	///
	/// # 参考
	/// - [`get_group_member_list`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_group_member_list-%E8%8E%B7%E5%8F%96%E7%BE%A4%E6%88%90%E5%91%98%E5%88%97%E8%A1%A8)
	async fn get_group_member_list(
		&self,
		group_id: i32,
	) -> APIResult<Vec<GetGroupMemberInfoResponse>>;

	/// 获取群荣誉信息。
	///
	/// # 参数
	/// - `group_id` — 群号。
	/// - `honor_type` — 要获取的群荣誉类型，可选 `talkative`、`performer`、`legend`、
	///   `strong_newbie`、`emotion`，或传入 `all` 获取所有数据。
	///
	/// # 返回值
	/// 返回群荣誉信息，包含当前龙王及各历史荣誉列表。
	///
	/// # 参考
	/// - [`get_group_honor_info`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_group_honor_info-%E8%8E%B7%E5%8F%96%E7%BE%A4%E8%8D%A3%E8%AA%89%E4%BF%A1%E6%81%AF)
	async fn get_group_honor_info(
		&self,
		group_id: i64,
		honor_type: String,
	) -> APIResult<GetGroupHonorInfoResponse>;

	/// 获取 Cookies。
	///
	/// # 参数
	/// - `domain` — 需要获取 cookies 的域名。
	///
	/// # 返回值
	/// 返回指定域名的 Cookies 字符串。
	///
	/// # 参考
	/// - [`get_cookies`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_cookies-%E8%8E%B7%E5%8F%96-cookies)
	async fn get_cookies(&self, domain: Option<String>) -> APIResult<String>;

	/// 获取 CSRF Token。
	///
	/// # 返回值
	/// 返回 CSRF Token（`i32`）。
	///
	/// # 参考
	/// - [`get_csrf_token`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_csrf_token-%E8%8E%B7%E5%8F%96-csrf-token)
	async fn get_csrf_token(&self) -> APIResult<i32>;

	/// 获取 QQ 相关接口凭证（Cookies + CSRF Token）。
	///
	/// # 参数
	/// - `domain` — 需要获取 cookies 的域名。
	///
	/// # 返回值
	/// 返回包含 Cookies 和 CSRF Token 的 [`GetCredentialsResponse`]。
	///
	/// # 参考
	/// - [`get_credentials`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_credentials-%E8%8E%B7%E5%8F%96-qq-%E7%9B%B8%E5%85%B3%E6%8E%A5%E5%8F%A3%E5%87%AD%E8%AF%81)
	async fn get_credentials(&self, domain: Option<String>) -> APIResult<GetCredentialsResponse>;

	/// 获取语音文件。
	///
	/// 通常需要安装 ffmpeg，具体请参考 OneBot 实现的相关说明。
	///
	/// # 参数
	/// - `file` — 收到的语音文件名（消息段的 `file` 参数）。
	/// - `out_format` — 要转换到的格式，支持 `mp3`、`amr`、`wma`、`m4a`、`spx`、`ogg`、`wav`、`flac`。
	///
	/// # 返回值
	/// 返回转换后的语音文件路径。
	///
	/// # 参考
	/// - [`get_record`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_record-%E8%8E%B7%E5%8F%96%E8%AF%AD%E9%9F%B3)
	async fn get_record(&self, file: String, out_format: String) -> APIResult<String>;

	/// 获取图片文件。
	///
	/// # 参数
	/// - `file` — 收到的图片文件名（消息段的 `file` 参数）。
	///
	/// # 返回值
	/// 返回下载后的图片文件路径。
	///
	/// # 参考
	/// - [`get_image`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_image-%E8%8E%B7%E5%8F%96%E5%9B%BE%E7%89%87)
	async fn get_image(&self, file: String) -> APIResult<String>;

	/// 检查是否可以发送图片。
	///
	/// # 返回值
	/// 返回 `true` 表示可以发送图片。
	///
	/// # 参考
	/// - [`can_send_image`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#can_send_image-%E6%A3%80%E6%9F%A5%E6%98%AF%E5%90%A6%E5%8F%AF%E4%BB%A5%E5%8F%91%E9%80%81%E5%9B%BE%E7%89%87)
	async fn can_send_image(&self) -> APIResult<bool>;

	/// 检查是否可以发送语音。
	///
	/// # 返回值
	/// 返回 `true` 表示可以发送语音。
	///
	/// # 参考
	/// - [`can_send_record`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#can_send_record-%E6%A3%80%E6%9F%A5%E6%98%AF%E5%90%A6%E5%8F%AF%E4%BB%A5%E5%8F%91%E9%80%81%E8%AF%AD%E9%9F%B3)
	async fn can_send_record(&self) -> APIResult<bool>;

	/// 获取运行状态。
	///
	/// # 返回值
	/// 返回包含 `online`（QQ 是否在线）和 `good`（状态是否符合预期）的 [`GetStatusResponse`]。
	///
	/// # 参考
	/// - [`get_status`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_status-%E8%8E%B7%E5%8F%96%E8%BF%90%E8%A1%8C%E7%8A%B6%E6%80%81)
	async fn get_status(&self) -> APIResult<GetStatusResponse>;

	/// 获取版本信息。
	///
	/// # 返回值
	/// 返回应用标识、应用版本及 OneBot 标准版本等信息。
	///
	/// # 参考
	/// - [`get_version_info`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_version_info-%E8%8E%B7%E5%8F%96%E7%89%88%E6%9C%AC%E4%BF%A1%E6%81%AF)
	async fn get_version_info(&self) -> APIResult<GetVersionInfoResponse>;

	/// 重启 OneBot 实现。
	///
	/// 由于重启会中断当前 API 服务，此操作是异步执行的，接口返回的 `status` 为 `async`。
	///
	/// # 参数
	/// - `delay` — 延迟重启的毫秒数，若默认情况下无法重启可尝试设置为 `2000` 左右。
	///
	/// # 参考
	/// - [`set_restart`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#set_restart-%E9%87%8D%E5%90%AF-onebot-%E5%AE%9E%E7%8E%B0)
	async fn set_restart(&self, delay: Option<i32>) -> APIResult<()>;

	/// 清理缓存。
	///
	/// 用于清理积攒过多的缓存文件。
	///
	/// # 参考
	/// - [`clean_cache`](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#clean_cache-%E6%B8%85%E7%90%86%E7%BC%93%E5%AD%98)
	async fn clean_cache(&self) -> APIResult<()>;
}
