use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[cfg(feature = "selector")]
use tynavi::Selector;

use crate::event::{meta::MetaEvent, notice::NoticeEvent, request::RequestEvent};
use message::MessageEvent;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use strum::Display;
#[cfg(not(feature = "selector"))]
use strum::EnumIs;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Debug, Clone)]
pub struct EventMessage {
	pub time: i64,
	pub self_id: i64,
	pub data: Box<MessageEvent>,
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Debug, Clone)]
pub struct EventNotice {
	pub time: i64,
	pub self_id: i64,
	pub data: NoticeEvent,
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Debug, Clone)]
pub struct EventRequest {
	pub time: i64,
	pub self_id: i64,
	pub data: RequestEvent,
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Debug, Clone)]
pub struct EventMetaEvent {
	pub time: i64,
	pub self_id: i64,
	pub data: MetaEvent,
	pub extra_body: HashMap<String, JsonValue>,
}

#[derive(Deserialize)]
struct RawEvent<T> {
	time: i64,
	self_id: i64,
	#[serde(flatten)]
	data: T,
}

fn deserialize_event<'de, D, T>(
	deserializer: D,
	extra_body_filter: impl Fn(&T, &str) -> bool,
) -> Result<(i64, i64, T, HashMap<String, JsonValue>), D::Error>
where
	D: Deserializer<'de>,
	T: DeserializeOwned,
{
	let raw_body = HashMap::<String, JsonValue>::deserialize(deserializer)?;
	let raw_event: RawEvent<T> =
		serde_json::from_value(JsonValue::Object(raw_body.clone().into_iter().collect()))
			.map_err(serde::de::Error::custom)?;

	let extra_body = raw_body
		.into_iter()
		.filter(|(key, _)| key != "time" && key != "self_id")
		.filter(|(key, _)| extra_body_filter(&raw_event.data, key))
		.collect();

	Ok((
		raw_event.time,
		raw_event.self_id,
		raw_event.data,
		extra_body,
	))
}

impl<'de> Deserialize<'de> for EventMessage {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let (time, self_id, data, extra_body) =
			deserialize_event(deserializer, |data: &MessageEvent, key| match data {
				MessageEvent::Private(_) => !matches!(
					key,
					"message_type"
						| "sub_type"
						| "message_id"
						| "user_id"
						| "message"
						| "raw_message"
						| "font"
						| "sender"
				),
				MessageEvent::Group(_) => !matches!(
					key,
					"message_type"
						| "sub_type"
						| "message_id"
						| "group_id"
						| "user_id"
						| "anonymous"
						| "message"
						| "raw_message"
						| "font"
						| "sender"
				),
			})?;

		Ok(Self {
			time,
			self_id,
			data: Box::new(data),
			extra_body,
		})
	}
}

impl<'de> Deserialize<'de> for EventNotice {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let (time, self_id, data, extra_body) =
			deserialize_event(deserializer, |data: &NoticeEvent, key| match data {
				NoticeEvent::GroupUpload(_) => {
					!matches!(key, "notice_type" | "group_id" | "user_id" | "file")
				}
				NoticeEvent::GroupAdmin(_) => {
					!matches!(key, "notice_type" | "sub_type" | "group_id" | "user_id")
				}
				NoticeEvent::GroupDecrease(_) => !matches!(
					key,
					"notice_type" | "sub_type" | "group_id" | "operator_id" | "user_id"
				),
				NoticeEvent::GroupIncrease(_) => !matches!(
					key,
					"notice_type" | "sub_type" | "group_id" | "operator_id" | "user_id"
				),
				NoticeEvent::GroupBan(_) => !matches!(
					key,
					"notice_type" | "sub_type" | "group_id" | "operator_id" | "user_id" | "duration"
				),
				NoticeEvent::FriendAdd(_) => !matches!(key, "notice_type" | "user_id"),
				NoticeEvent::GroupRecall(_) => !matches!(
					key,
					"notice_type" | "group_id" | "user_id" | "operator_id" | "message_id"
				),
				NoticeEvent::FriendRecall(_) => !matches!(key, "notice_type" | "user_id" | "message_id"),
				NoticeEvent::Notify(data) => {
					let notify_known = match &data.data {
						notice::NotifyType::Poke { .. } | notice::NotifyType::LuckyKing { .. } => {
							matches!(key, "sub_type" | "target_id")
						}
						notice::NotifyType::Honor { .. } => matches!(key, "sub_type" | "honor_type"),
					};

					!(matches!(key, "notice_type" | "group_id" | "user_id") || notify_known)
				}
			})?;

		Ok(Self {
			time,
			self_id,
			data,
			extra_body,
		})
	}
}

impl<'de> Deserialize<'de> for EventRequest {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let (time, self_id, data, extra_body) =
			deserialize_event(deserializer, |data: &RequestEvent, key| match data {
				RequestEvent::Friend(_) => !matches!(key, "request_type" | "user_id" | "comment" | "flag"),
				RequestEvent::Group(_) => !matches!(
					key,
					"request_type" | "sub_type" | "group_id" | "user_id" | "comment" | "flag"
				),
			})?;

		Ok(Self {
			time,
			self_id,
			data,
			extra_body,
		})
	}
}

impl<'de> Deserialize<'de> for EventMetaEvent {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let (time, self_id, data, extra_body) =
			deserialize_event(deserializer, |data: &MetaEvent, key| match data {
				MetaEvent::Lifecycle(_) => !matches!(key, "meta_event_type" | "sub_type"),
				MetaEvent::Heartbeat(_) => !matches!(key, "meta_event_type" | "status" | "interval"),
			})?;

		Ok(Self {
			time,
			self_id,
			data,
			extra_body,
		})
	}
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[cfg_attr(not(feature = "selector"), derive(EnumIs))]
#[derive(Deserialize, Debug, Clone, Display)]
#[serde(tag = "post_type")]
pub enum KnownEvent {
	#[serde(rename = "message")]
	Message(EventMessage),

	#[serde(rename = "notice")]
	Notice(EventNotice),

	#[serde(rename = "request")]
	Request(EventRequest),

	#[serde(rename = "meta_event")]
	MetaEvent(EventMetaEvent),
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[cfg_attr(not(feature = "selector"), derive(EnumIs))]
#[derive(Deserialize, Debug, Clone, Display)]
#[serde(untagged)]
pub enum Event {
	Known(KnownEvent),
	Unknown(JsonValue),
}

pub trait EventTrait {}

impl EventTrait for Event {}
