use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;
use strum::{Display, EnumIs};
#[cfg(feature = "selector")]
use tynavi::Selector;

#[derive(Deserialize, Debug, Copy, Clone, Display, EnumIs, Ord, PartialOrd, Eq, PartialEq)]
pub enum LifecycleSubType {
	#[serde(rename = "enable")]
	Enable,
	#[serde(rename = "disable")]
	Disable,
	#[serde(rename = "connect")]
	Connect,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct MetaEventLifecycle {
	#[cfg_attr(feature = "selector", selector(variants(enable, disable, connect)))]
	pub sub_type: LifecycleSubType,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MetaEventHeartbeat {
	pub status: HashMap<String, Value>,
	pub interval: i64,
}

#[cfg_attr(feature = "selector", derive(Selector))]
#[cfg_attr(not(feature = "selector"), derive(EnumIs))]
#[derive(Deserialize, Debug, Clone, Display, Eq, PartialEq)]
#[serde(tag = "meta_event_type")]
pub enum MetaEvent {
	#[serde(rename = "lifecycle")]
	Lifecycle(MetaEventLifecycle),

	#[serde(rename = "heartbeat")]
	Heartbeat(MetaEventHeartbeat),
}
