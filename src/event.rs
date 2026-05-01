use crate::event::{meta::MetaEvent, notice::NoticeEvent, request::RequestEvent};

#[cfg(feature = "selector")]
use crate::selector::Selector;

use message::MessageEvent;
use serde::Deserialize;
use strum::{Display, EnumIs};

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
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, EventMessage> {
	pub fn message_event_selector(&self) -> Selector<'a, MessageEvent> {
		Selector {
			data: self.data.map(|d| &*d.data),
		}
	}

	pub fn filter(&mut self, f: impl FnOnce(&'a EventMessage) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&'a EventMessage) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&'a EventMessage) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&'a EventMessage) -> bool) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_time(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.time)
		{
			self.data = None
		}
	}

	pub fn and_filter_time(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_time(f);
		self
	}

	pub async fn filter_time_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.time).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_time_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_time_async(f).await;
		self
	}

	pub fn filter_self_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.self_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_self_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_self_id(f);
		self
	}

	pub async fn filter_self_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.self_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_self_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_self_id_async(f).await;
		self
	}

	pub fn filter_message_event(&mut self, f: impl FnOnce(&MessageEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data)
		{
			self.data = None
		}
	}

	pub fn and_filter_message_event(mut self, f: impl FnOnce(&MessageEvent) -> bool) -> Self {
		self.filter_message_event(f);
		self
	}

	pub async fn filter_message_event_async(&mut self, f: impl AsyncFnOnce(&MessageEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_message_event_async(
		mut self,
		f: impl AsyncFnOnce(&MessageEvent) -> bool,
	) -> Self {
		self.filter_message_event_async(f).await;
		self
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
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, EventNotice> {
	pub fn notice_event_selector(&self) -> Selector<'a, NoticeEvent> {
		Selector {
			data: self.data.map(|d| &d.data),
		}
	}

	pub fn filter(&mut self, f: impl FnOnce(&'a EventNotice) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&'a EventNotice) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&'a EventNotice) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&'a EventNotice) -> bool) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_time(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.time)
		{
			self.data = None
		}
	}

	pub fn and_filter_time(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_time(f);
		self
	}

	pub async fn filter_time_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.time).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_time_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_time_async(f).await;
		self
	}

	pub fn filter_self_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.self_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_self_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_self_id(f);
		self
	}

	pub async fn filter_self_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.self_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_self_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_self_id_async(f).await;
		self
	}

	pub fn filter_notice_event(&mut self, f: impl FnOnce(&NoticeEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data)
		{
			self.data = None
		}
	}

	pub fn and_filter_notice_event(mut self, f: impl FnOnce(&NoticeEvent) -> bool) -> Self {
		self.filter_notice_event(f);
		self
	}

	pub async fn filter_notice_event_async(&mut self, f: impl AsyncFnOnce(&NoticeEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_notice_event_async(
		mut self,
		f: impl AsyncFnOnce(&NoticeEvent) -> bool,
	) -> Self {
		self.filter_notice_event_async(f).await;
		self
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
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, EventRequest> {
	pub fn request_event_selector(&self) -> Selector<'a, RequestEvent> {
		Selector {
			data: self.data.map(|d| &d.data),
		}
	}

	pub fn filter(&mut self, f: impl FnOnce(&'a EventRequest) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&'a EventRequest) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&'a EventRequest) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&'a EventRequest) -> bool) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_time(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.time)
		{
			self.data = None
		}
	}

	pub fn and_filter_time(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_time(f);
		self
	}

	pub async fn filter_time_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.time).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_time_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_time_async(f).await;
		self
	}

	pub fn filter_self_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.self_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_self_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_self_id(f);
		self
	}

	pub async fn filter_self_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.self_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_self_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_self_id_async(f).await;
		self
	}

	pub fn filter_request_event(&mut self, f: impl FnOnce(&RequestEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data)
		{
			self.data = None
		}
	}

	pub fn and_filter_request_event(mut self, f: impl FnOnce(&RequestEvent) -> bool) -> Self {
		self.filter_request_event(f);
		self
	}

	pub async fn filter_request_event_async(&mut self, f: impl AsyncFnOnce(&RequestEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_request_event_async(
		mut self,
		f: impl AsyncFnOnce(&RequestEvent) -> bool,
	) -> Self {
		self.filter_request_event_async(f).await;
		self
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
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, EventMetaEvent> {
	pub fn meta_event_selector(&self) -> Selector<'a, MetaEvent> {
		Selector {
			data: self.data.map(|d| &d.data),
		}
	}

	pub fn filter(&mut self, f: impl FnOnce(&'a EventMetaEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&'a EventMetaEvent) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&'a EventMetaEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&'a EventMetaEvent) -> bool) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_time(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.time)
		{
			self.data = None
		}
	}

	pub fn and_filter_time(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_time(f);
		self
	}

	pub async fn filter_time_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.time).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_time_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_time_async(f).await;
		self
	}

	pub fn filter_self_id(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.self_id)
		{
			self.data = None
		}
	}

	pub fn and_filter_self_id(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_self_id(f);
		self
	}

	pub async fn filter_self_id_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.self_id).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_self_id_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_self_id_async(f).await;
		self
	}

	pub fn filter_meta_event(&mut self, f: impl FnOnce(&MetaEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data)
		{
			self.data = None
		}
	}

	pub fn and_filter_meta_event(mut self, f: impl FnOnce(&MetaEvent) -> bool) -> Self {
		self.filter_meta_event(f);
		self
	}

	pub async fn filter_meta_event_async(&mut self, f: impl AsyncFnOnce(&MetaEvent) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_meta_event_async(
		mut self,
		f: impl AsyncFnOnce(&MetaEvent) -> bool,
	) -> Self {
		self.filter_meta_event_async(f).await;
		self
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
	#[cfg(feature = "selector")]
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

#[cfg(feature = "selector")]
impl<'a> Selector<'a, Event> {
	pub fn message(&self) -> Selector<'a, EventMessage> {
		Selector {
			data: self.data.and_then(|d| d.match_message()),
		}
	}

	pub fn notice(&self) -> Selector<'a, EventNotice> {
		Selector {
			data: self.data.and_then(|d| d.match_notice()),
		}
	}

	pub fn request(&self) -> Selector<'a, EventRequest> {
		Selector {
			data: self.data.and_then(|d| d.match_request()),
		}
	}

	pub fn meta_event(&self) -> Selector<'a, EventMetaEvent> {
		Selector {
			data: self.data.and_then(|d| d.match_meta_event()),
		}
	}
}

pub trait EventTrait {}

impl EventTrait for Event {}
