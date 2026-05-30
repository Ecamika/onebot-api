use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[cfg(feature = "selector")]
use crate::Selector;

use crate::event::{meta::MetaEvent, notice::NoticeEvent, request::RequestEvent};
use message::MessageEvent;
use serde::Deserialize;
use strum::{Display, EnumIs};

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone)]
pub struct EventMessage {
	pub time: i64,
	pub self_id: i64,
	#[cfg_attr(feature = "selector", selector(through = "message_event_selector"))]
	#[serde(flatten)]
	pub data: Box<MessageEvent>,
	#[serde(flatten)]
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone)]
pub struct EventNotice {
	pub time: i64,
	pub self_id: i64,
	#[cfg_attr(feature = "selector", selector(through = "notice_event_selector"))]
	#[serde(flatten)]
	pub data: NoticeEvent,
	#[serde(flatten)]
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone)]
pub struct EventRequest {
	pub time: i64,
	pub self_id: i64,
	#[cfg_attr(feature = "selector", selector(through = "request_event_selector"))]
	#[serde(flatten)]
	pub data: RequestEvent,
	#[serde(flatten)]
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone)]
pub struct EventMetaEvent {
	pub time: i64,
	pub self_id: i64,
	#[cfg_attr(feature = "selector", selector(through = "meta_event_selector"))]
	#[serde(flatten)]
	pub data: MetaEvent,
	#[serde(flatten)]
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Display, EnumIs)]
#[serde(tag = "post_type")]
pub enum Event {
	#[serde(rename = "message")]
	Message(EventMessage),

	#[serde(rename = "notice")]
	Notice(EventNotice),

	#[serde(rename = "request")]
	Request(EventRequest),

	#[serde(rename = "meta_event")]
	MetaEvent(EventMetaEvent),
}

pub trait EventTrait {}

impl EventTrait for Event {}
