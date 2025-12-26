use crate::event::message::GroupMessageAnonymous;
use crate::message::Segment;
use arg_type::*;
use async_trait::async_trait;
use return_type::*;

pub mod arg_type;
pub mod return_type;

pub(crate) trait APIArg {}

#[async_trait]
pub trait APISender {
	async fn send_private_msg(
		&self,
		user_id: i64,
		message: Segment,
		auto_escape: Option<bool>,
	) -> anyhow::Result<i32>;
	async fn send_group_msg(
		&self,
		group_id: i64,
		message: Segment,
		auto_escape: Option<bool>,
	) -> anyhow::Result<i32>;
	async fn send_msg(
		&self,
		message_type: Option<MessageType>,
		user_id: i64,
		group_id: i64,
		message: Segment,
		auto_escape: Option<bool>,
	) -> anyhow::Result<i32>;
	async fn delete_msg(&self, message_id: i32) -> anyhow::Result<()>;
	async fn get_msg(&self, message_id: i32) -> anyhow::Result<GetMsgResponse>;
	async fn get_forward_msg(&self, id: String) -> anyhow::Result<Vec<Segment>>;
	async fn send_like(&self, user_id: i64, times: Option<i32>) -> anyhow::Result<()>;
	async fn set_group_kick(
		&self,
		group_id: i32,
		user_id: i32,
		reject_add_request: Option<bool>,
	) -> anyhow::Result<()>;
	async fn set_group_ban(
		&self,
		group_id: i32,
		user_id: i32,
		duration: Option<i32>,
	) -> anyhow::Result<()>;
	async fn set_group_anonymous_ban(
		&self,
		group_id: i32,
		anonymous: Option<GroupMessageAnonymous>,
		flag: Option<String>,
		duration: Option<i32>,
	) -> anyhow::Result<()>;
	async fn set_group_whole_ban(&self, group_id: i32, enable: Option<bool>) -> anyhow::Result<()>;
	async fn set_group_admin(
		&self,
		group_id: i32,
		user_id: i32,
		enable: Option<bool>,
	) -> anyhow::Result<()>;
	async fn set_group_anonymous(&self, group_id: i32, enable: Option<bool>) -> anyhow::Result<()>;
	async fn set_group_card(
		&self,
		group_id: i32,
		user_id: i32,
		card: Option<String>,
	) -> anyhow::Result<()>;
	async fn set_group_name(&self, group_id: i32, group_name: String) -> anyhow::Result<()>;
	async fn set_group_leave(&self, group_id: i32, is_dismiss: Option<bool>) -> anyhow::Result<()>;
	async fn set_group_special_title(
		&self,
		group_id: i32,
		user_id: i32,
		special_title: Option<String>,
		duration: Option<i32>,
	) -> anyhow::Result<()>;
	async fn set_friend_add_request(
		&self,
		flag: String,
		approve: Option<bool>,
		remark: Option<String>,
	) -> anyhow::Result<()>;
	async fn set_group_add_request(
		&self,
		flag: String,
		sub_type: String,
		approve: Option<bool>,
		reason: Option<String>,
	) -> anyhow::Result<()>;
	async fn get_login_info(&self) -> anyhow::Result<GetLoginInfoResponse>;
	async fn get_stranger_info(
		&self,
		user_id: i32,
		no_cache: Option<bool>,
	) -> anyhow::Result<GetStrangerInfoResponse>;
	async fn get_friend_list(&self) -> anyhow::Result<Vec<GetFriendListResponse>>;
	async fn get_group_info(
		&self,
		group_id: i32,
		no_cache: Option<bool>,
	) -> anyhow::Result<GetGroupInfoResponse>;
	async fn get_group_list(&self) -> anyhow::Result<Vec<GetGroupInfoResponse>>;
	async fn get_group_member_info(
		&self,
		group_id: i32,
		user_id: i32,
		no_cache: Option<bool>,
	) -> anyhow::Result<GetGroupMemberInfoResponse>;
	async fn get_group_member_list(
		&self,
		group_id: i32,
	) -> anyhow::Result<Vec<GetGroupMemberInfoResponse>>;
	async fn get_group_honor_info(
		&self,
		group_id: i64,
		honor_type: String,
	) -> anyhow::Result<GetGroupMemberInfoResponse>;
	async fn get_cookies(&self, domain: Option<String>) -> anyhow::Result<String>;
	async fn get_csrf_token(&self) -> anyhow::Result<i32>;
	async fn get_credentials(&self, domain: Option<String>)
	-> anyhow::Result<GetCredentialsResponse>;
	async fn get_record(&self, file: String, out_format: String) -> anyhow::Result<String>;
	async fn get_image(&self, file: String) -> anyhow::Result<String>;
	async fn can_send_image(&self) -> anyhow::Result<bool>;
	async fn can_send_record(&self) -> anyhow::Result<bool>;
	async fn get_status(&self) -> anyhow::Result<GetStatusResponse>;
	async fn get_version_info(&self) -> anyhow::Result<GetVersionInfoResponse>;
	async fn set_restart(&self, delay: Option<i32>) -> anyhow::Result<()>;
	async fn clean_cache(&self) -> anyhow::Result<()>;
}
