use std::collections::HashMap;

#[cfg(feature = "selector")]
use crate::selector::Selector;
use serde::Deserialize;
use serde_json::Value;
use strum::{Display, EnumIs};

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum LifecycleSubType {
	#[serde(rename = "enable")]
	Enable,
	#[serde(rename = "disable")]
	Disable,
	#[serde(rename = "connect")]
	Connect,
}

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct MetaEventLifecycle {
	pub sub_type: LifecycleSubType,
}

impl MetaEventLifecycle {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, MetaEventLifecycle> {
	pub fn filter(&mut self, f: impl FnOnce(MetaEventLifecycle) -> bool) {
		if let Some(data) = self.data
			&& !f(*data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(MetaEventLifecycle) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(MetaEventLifecycle) -> bool) {
		if let Some(data) = self.data
			&& !f(*data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(MetaEventLifecycle) -> bool) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_sub_type(&mut self, f: impl FnOnce(LifecycleSubType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type)
		{
			self.data = None
		}
	}

	pub fn and_filter_sub_type(mut self, f: impl FnOnce(LifecycleSubType) -> bool) -> Self {
		self.filter_sub_type(f);
		self
	}

	pub async fn filter_sub_type_async(&mut self, f: impl AsyncFnOnce(LifecycleSubType) -> bool) {
		if let Some(data) = self.data
			&& !f(data.sub_type).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_sub_type_async(
		mut self,
		f: impl AsyncFnOnce(LifecycleSubType) -> bool,
	) -> Self {
		self.filter_sub_type_async(f).await;
		self
	}

	pub fn enable(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_enable()
		{
			self.data = None
		}
	}

	pub fn and_enable(mut self) -> Self {
		self.enable();
		self
	}

	pub fn not_enable(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_enable()
		{
			self.data = None
		}
	}

	pub fn and_not_enable(mut self) -> Self {
		self.not_enable();
		self
	}

	pub fn disable(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_disable()
		{
			self.data = None
		}
	}

	pub fn and_disable(mut self) -> Self {
		self.disable();
		self
	}

	pub fn not_disable(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_disable()
		{
			self.data = None
		}
	}

	pub fn and_not_disable(mut self) -> Self {
		self.not_disable();
		self
	}

	pub fn connect(&mut self) {
		if let Some(data) = self.data
			&& !data.sub_type.is_connect()
		{
			self.data = None
		}
	}

	pub fn and_connect(mut self) -> Self {
		self.connect();
		self
	}

	pub fn not_connect(&mut self) {
		if let Some(data) = self.data
			&& data.sub_type.is_connect()
		{
			self.data = None
		}
	}

	pub fn and_not_connect(mut self) -> Self {
		self.not_connect();
		self
	}
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MetaEventHeartbeat {
	pub status: HashMap<String, Value>,
	pub interval: i64,
}

impl MetaEventHeartbeat {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, MetaEventHeartbeat> {
	pub fn filter(&mut self, f: impl FnOnce(&MetaEventHeartbeat) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&MetaEventHeartbeat) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&MetaEventHeartbeat) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(
		mut self,
		f: impl AsyncFnOnce(&MetaEventHeartbeat) -> bool,
	) -> Self {
		self.filter_async(f).await;
		self
	}

	pub fn filter_status(&mut self, f: impl FnOnce(&HashMap<String, Value>) -> bool) {
		if let Some(data) = self.data
			&& !f(&data.status)
		{
			self.data = None
		}
	}

	pub fn and_filter_status(mut self, f: impl FnOnce(&HashMap<String, Value>) -> bool) -> Self {
		self.filter_status(f);
		self
	}

	pub async fn filter_status_async(
		&mut self,
		f: impl AsyncFnOnce(&HashMap<String, Value>) -> bool,
	) {
		if let Some(data) = self.data
			&& !f(&data.status).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_status_async(
		mut self,
		f: impl AsyncFnOnce(&HashMap<String, Value>) -> bool,
	) -> Self {
		self.filter_status_async(f).await;
		self
	}

	pub fn filter_interval(&mut self, f: impl FnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.interval)
		{
			self.data = None
		}
	}

	pub fn and_filter_interval(mut self, f: impl FnOnce(i64) -> bool) -> Self {
		self.filter_interval(f);
		self
	}

	pub async fn filter_interval_async(&mut self, f: impl AsyncFnOnce(i64) -> bool) {
		if let Some(data) = self.data
			&& !f(data.interval).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_interval_async(mut self, f: impl AsyncFnOnce(i64) -> bool) -> Self {
		self.filter_interval_async(f).await;
		self
	}
}

#[derive(Deserialize, Debug, Clone, Display, EnumIs, Eq, PartialEq)]
#[serde(tag = "meta_event_type")]
pub enum MetaEvent {
	#[serde(rename = "lifecycle")]
	Lifecycle(MetaEventLifecycle),

	#[serde(rename = "heartbeat")]
	Heartbeat(MetaEventHeartbeat),
}

impl MetaEvent {
	#[cfg(feature = "selector")]
	pub fn selector(&'_ self) -> Selector<'_, Self> {
		Selector { data: Some(self) }
	}

	pub fn match_lifecycle(&self) -> Option<&MetaEventLifecycle> {
		if let Self::Lifecycle(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_lifecycle<T>(&self, handler: impl FnOnce(&MetaEventLifecycle) -> T) -> Option<T> {
		if let Self::Lifecycle(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_lifecycle_async<T>(
		&self,
		handler: impl AsyncFnOnce(&MetaEventLifecycle) -> T,
	) -> Option<T> {
		if let Self::Lifecycle(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn match_heartbeat(&self) -> Option<&MetaEventHeartbeat> {
		if let Self::Heartbeat(data) = self {
			Some(data)
		} else {
			None
		}
	}

	pub fn on_heartbeat<T>(&self, handler: impl FnOnce(&MetaEventHeartbeat) -> T) -> Option<T> {
		if let Self::Heartbeat(data) = self {
			Some(handler(data))
		} else {
			None
		}
	}

	pub async fn on_heartbeat_async<T>(
		&self,
		handler: impl AsyncFnOnce(&MetaEventHeartbeat) -> T,
	) -> Option<T> {
		if let Self::Heartbeat(data) = self {
			Some(handler(data).await)
		} else {
			None
		}
	}
}

#[cfg(feature = "selector")]
impl<'a> Selector<'a, MetaEvent> {
	pub fn lifecycle(&self) -> Selector<'a, MetaEventLifecycle> {
		Selector {
			data: self.data.and_then(|d| d.match_lifecycle()),
		}
	}

	pub fn heartbeat(&self) -> Selector<'a, MetaEventHeartbeat> {
		Selector {
			data: self.data.and_then(|d| d.match_heartbeat()),
		}
	}
}
