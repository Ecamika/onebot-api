use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "meta_event_type")]
pub enum MetaEvent {
	#[serde(rename = "lifecycle")]
	Lifecycle { sub_type: LifecycleSubType },

	#[serde(rename = "heartbeat")]
	Heartbeat {
		status: HashMap<String, Value>,
		interval: i64,
	},
}

#[derive(Deserialize, Debug, Clone)]
pub enum LifecycleSubType {
	#[serde(rename = "enable")]
	Enable,
	#[serde(rename = "disable")]
	Disable,
	#[serde(rename = "connect")]
	Connect,
}
