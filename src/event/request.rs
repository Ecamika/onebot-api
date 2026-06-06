#[cfg(feature = "quick_operation")]
use crate::api::APISender;
#[cfg(feature = "quick_operation")]
use crate::quick_operation::{QuickHandleFriendRequest, QuickHandleGroupRequest};
#[cfg(feature = "quick_operation")]
use async_trait::async_trait;
use serde::Deserialize;
use strum::{Display, EnumIs};
#[cfg(feature = "selector")]
use tynavi::Selector;

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum GroupType {
	#[serde(rename = "add")]
	Add,
	#[serde(rename = "invite")]
	Invite,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RequestEventFriend {
	pub user_id: i64,
	pub comment: String,
	pub flag: String,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickHandleFriendRequest<T> for RequestEventFriend {
	async fn approve(&self, api: &T, remark: Option<String>) -> crate::error::APIResult<()> {
		api
			.set_friend_add_request(self.flag.clone(), Some(true), remark)
			.await
	}

	async fn reject(&self, api: &T) -> crate::error::APIResult<()> {
		api
			.set_friend_add_request(self.flag.clone(), Some(false), None)
			.await
	}
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct RequestEventGroup {
	#[cfg_attr(feature = "selector", selector(variants(add, invite)))]
	pub sub_type: GroupType,
	pub group_id: i64,
	pub user_id: i64,
	pub comment: String,
	pub flag: String,
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickHandleGroupRequest<T> for RequestEventGroup {
	async fn approve(&self, api: &T) -> crate::error::APIResult<()> {
		api
			.set_group_add_request(
				self.flag.clone(),
				self.sub_type.to_string(),
				Some(true),
				None,
			)
			.await
	}

	async fn reject(&self, api: &T, reason: Option<String>) -> crate::error::APIResult<()> {
		api
			.set_group_add_request(
				self.flag.clone(),
				self.sub_type.to_string(),
				Some(false),
				reason,
			)
			.await
	}
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[cfg_attr(not(feature = "selector"), derive(EnumIs))]
#[derive(Deserialize, Debug, Clone, Display, Ord, PartialOrd, Eq, PartialEq)]
#[serde(tag = "request_type")]
pub enum RequestEvent {
	#[serde(rename = "friend")]
	Friend(RequestEventFriend),

	#[serde(rename = "group")]
	Group(RequestEventGroup),
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickHandleFriendRequest<T> for RequestEvent {
	async fn approve(&self, api: &T, remark: Option<String>) -> crate::error::APIResult<()> {
		match self {
			Self::Friend(data) => data.approve(api, remark).await,
			Self::Group(_) => Ok(()),
		}
	}

	async fn reject(&self, api: &T) -> crate::error::APIResult<()> {
		match self {
			Self::Friend(data) => data.reject(api).await,
			Self::Group(_) => Ok(()),
		}
	}
}

#[cfg(feature = "quick_operation")]
#[async_trait]
impl<T: APISender + Send + Sync> QuickHandleGroupRequest<T> for RequestEvent {
	async fn approve(&self, api: &T) -> crate::error::APIResult<()> {
		match self {
			Self::Group(data) => data.approve(api).await,
			Self::Friend(_) => Ok(()),
		}
	}

	async fn reject(&self, api: &T, reason: Option<String>) -> crate::error::APIResult<()> {
		match self {
			Self::Group(data) => data.reject(api, reason).await,
			Self::Friend(_) => Ok(()),
		}
	}
}
