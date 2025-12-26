use super::return_type::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub enum MessageType {
	#[serde(rename = "private")]
	Private,
	#[serde(rename = "group")]
	Group,
}
