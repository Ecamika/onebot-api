use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[cfg(feature = "selector")]
use onebot_api_macros::Selector;

use crate::event::{meta::MetaEvent, notice::NoticeEvent, request::RequestEvent};
use message::MessageEvent;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};
use strum::{Display, EnumIs};

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Debug, Clone)]
pub struct EventMessage {
	pub time: i64,
	pub self_id: i64,
	#[cfg_attr(feature = "selector", selector(through = "message_event_selector"))]
	pub data: Box<MessageEvent>,
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Debug, Clone)]
pub struct EventNotice {
	pub time: i64,
	pub self_id: i64,
	#[cfg_attr(feature = "selector", selector(through = "notice_event_selector"))]
	pub data: NoticeEvent,
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Debug, Clone)]
pub struct EventRequest {
	pub time: i64,
	pub self_id: i64,
	#[cfg_attr(feature = "selector", selector(through = "request_event_selector"))]
	pub data: RequestEvent,
	pub extra_body: HashMap<String, JsonValue>,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Debug, Clone)]
pub struct EventMetaEvent {
	pub time: i64,
	pub self_id: i64,
	#[cfg_attr(feature = "selector", selector(through = "meta_event_selector"))]
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

#[cfg(test)]
mod tests {
	use super::Event;

	#[test]
	fn message_event_extra_body_only_keeps_unknown_fields() {
		let raw = r#"{
			"time": 1,
			"self_id": 2,
			"post_type": "message",
			"message_type": "private",
			"sub_type": "friend",
			"message_id": 3,
			"user_id": 4,
			"message": [],
			"raw_message": "",
			"font": 5,
			"sender": {},
			"message_seq": 6,
			"custom_flag": true
		}"#;

		let event: Event = serde_json::from_str(raw).expect("message event should deserialize");
		let Event::Message(event) = event else {
			panic!("expected message event");
		};

		assert_eq!(event.extra_body.len(), 2);
		assert_eq!(event.extra_body["message_seq"], serde_json::json!(6));
		assert_eq!(event.extra_body["custom_flag"], serde_json::json!(true));
		assert!(!event.extra_body.contains_key("message_id"));
		assert!(!event.extra_body.contains_key("message_type"));
	}

	#[test]
	fn notice_event_extra_body_excludes_notify_discriminant_fields() {
		let raw = r#"{
			"time": 1,
			"self_id": 2,
			"post_type": "notice",
			"notice_type": "notify",
			"sub_type": "honor",
			"group_id": 3,
			"user_id": 4,
			"honor_type": "talkative",
			"rank": 1
		}"#;

		let event: Event = serde_json::from_str(raw).expect("notice event should deserialize");
		let Event::Notice(event) = event else {
			panic!("expected notice event");
		};

		assert_eq!(event.extra_body.len(), 1);
		assert_eq!(event.extra_body["rank"], serde_json::json!(1));
		assert!(!event.extra_body.contains_key("notice_type"));
		assert!(!event.extra_body.contains_key("sub_type"));
		assert!(!event.extra_body.contains_key("honor_type"));
	}
}
