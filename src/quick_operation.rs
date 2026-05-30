use crate::api::APISender;
use crate::error::APIResult;
use crate::message::send_segment::SendSegment;
use async_trait::async_trait;

#[async_trait]
pub trait QuickSendMsg<T: APISender + Sync + Send> {
	async fn send_msg(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32>;
}

#[async_trait]
pub trait QuickReplyAt<T: APISender + Sync + Send> {
	async fn reply_at(
		&self,
		api: &T,
		msg: Vec<SendSegment>,
		auto_escape: Option<bool>,
	) -> APIResult<i32>;
}

#[async_trait]
pub trait QuickDeleteMsg<T: APISender + Sync + Send> {
	async fn delete_msg(&self, api: &T) -> APIResult<()>;
}

#[async_trait]
pub trait QuickKick<T: APISender + Sync + Send> {
	async fn kick(&self, api: &T, reject_add_request: Option<bool>) -> APIResult<()>;
}

#[async_trait]
pub trait QuickBan<T: APISender + Sync + Send> {
	async fn ban(&self, api: &T, duration: Option<i32>) -> APIResult<()>;
}

#[async_trait]
pub trait QuickHandleFriendRequest<T: APISender + Sync + Send> {
	async fn approve(&self, api: &T, remark: Option<String>) -> APIResult<()>;
	async fn reject(&self, api: &T) -> APIResult<()>;
}

#[async_trait]
pub trait QuickHandleGroupRequest<T: APISender + Sync + Send> {
	async fn approve(&self, api: &T) -> APIResult<()>;
	async fn reject(&self, api: &T, reason: Option<String>) -> APIResult<()>;
}
