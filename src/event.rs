use crate::event::{meta::MetaEvent, notice::NoticeEvent, request::RequestEvent};
// use flume::Receiver;
use crate::selector::Selector;
use async_trait::async_trait;
use message::MessageEvent;
use serde::Deserialize;
use strum::{Display, EnumIs};
use tokio::sync::broadcast;

pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(Deserialize, Debug, Clone)]
pub struct EventMessage {
	pub time: i64,
	pub self_id: i64,
	#[serde(flatten)]
	pub data: Box<MessageEvent>,
}

impl EventMessage {
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventNotice {
	pub time: i64,
	pub self_id: i64,
	#[serde(flatten)]
	pub data: NoticeEvent,
}

impl EventNotice {
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventRequest {
	pub time: i64,
	pub self_id: i64,
	#[serde(flatten)]
	pub data: RequestEvent,
}

impl EventRequest {
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct EventMetaEvent {
	pub time: i64,
	pub self_id: i64,
	#[serde(flatten)]
	pub data: MetaEvent,
}

impl EventMetaEvent {
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

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

impl Event {
	pub fn selector(&'_ self) -> Selector<'_, Event> {
		Selector { data: Some(self) }
	}

	pub fn match_message(&self) -> Option<&EventMessage> {
		if let Self::Message(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_message<T>(&self, handler: impl FnOnce(&EventMessage) -> T) -> Option<T> {
		if let Self::Message(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_message_async<T>(
		&self,
		handler: impl AsyncFnOnce(&EventMessage) -> T,
	) -> Option<T> {
		if let Self::Message(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_notice(&self) -> Option<&EventNotice> {
		if let Self::Notice(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_notice<T>(&self, handler: impl FnOnce(&EventNotice) -> T) -> Option<T> {
		if let Self::Notice(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_notice_async<T>(
		&self,
		handler: impl AsyncFnOnce(&EventNotice) -> T,
	) -> Option<T> {
		if let Self::Notice(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_request(&self) -> Option<&EventRequest> {
		if let Self::Request(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_request<T>(&self, handler: impl FnOnce(&EventRequest) -> T) -> Option<T> {
		if let Self::Request(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_request_async<T>(
		&self,
		handler: impl AsyncFnOnce(&EventRequest) -> T,
	) -> Option<T> {
		if let Self::Request(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_meta_event(&self) -> Option<&EventMetaEvent> {
		if let Self::MetaEvent(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_meta_event<T>(&self, handler: impl FnOnce(&EventMetaEvent) -> T) -> Option<T> {
		if let Self::MetaEvent(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_meta_event_async<T>(
		&self,
		handler: impl AsyncFnOnce(&EventMetaEvent) -> T,
	) -> Option<T> {
		if let Self::MetaEvent(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}

pub trait EventTrait {}

impl EventTrait for Event {}

#[async_trait]
pub trait EventReceiver<T: EventTrait> {
	fn subscribe(&self) -> broadcast::Receiver<T>;
}
