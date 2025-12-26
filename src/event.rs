use crate::event::{meta::MetaEvent, notice::NoticeEvent, request::RequestEvent};
use async_trait::async_trait;
use flume::Receiver;
use message::MessageEvent;
use serde::Deserialize;
use tokio::sync::broadcast;
pub mod message;
pub mod meta;
pub mod notice;
pub mod request;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "post_type")]
pub enum Event {
	#[serde(rename = "message")]
	Message {
		time: i64,
		self_id: i64,
		#[serde(flatten)]
		data: Box<MessageEvent>,
	},

	#[serde(rename = "notice")]
	Notice {
		time: i64,
		self_id: i64,
		#[serde(flatten)]
		data: NoticeEvent,
	},

	#[serde(rename = "request")]
	Request {
		time: i64,
		self_id: i64,
		#[serde(flatten)]
		data: RequestEvent,
	},

	#[serde(rename = "meta_event")]
	MetaEvent {
		time: i64,
		self_id: i64,
		#[serde(flatten)]
		data: MetaEvent,
	},
}

// pub struct EventStream {}
//
// impl Stream for EventStream {
// 	type Item = ();
//
// 	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
// 		todo!()
// 	}
// }

pub trait EventTrait {}

impl EventTrait for Event {}

#[async_trait]
pub trait EventReceiver<T: EventTrait> {
	// async fn wait_event(&self) -> anyhow::Result<EventStream>; TODO
	// async fn listen(&mut self, listener: T) -> anyhow::Result<()>; TODO

	fn get_receiver(&self) -> broadcast::Receiver<T>;
}
