use crate::message::Segment;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Clone)]
pub enum Sex {
	#[serde(rename = "male")]
	Male,
	#[serde(rename = "female")]
	Female,
	#[serde(rename = "unknown")]
	Unknown,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "message_type")]
pub enum MessageEvent {
	#[serde(rename = "private")]
	Private {
		sub_type: PrivateMessageSubType,
		message_id: i32,
		user_id: i64,
		message: Vec<Segment>,
		raw_message: String,
		font: i32,
		sender: PrivateMessageSender,
	},

	#[serde(rename = "group")]
	Group {
		sub_type: GroupMessageSubType,
		message_id: i32,
		group_id: i64,
		user_id: i64,
		anonymous: Option<GroupMessageAnonymous>,
		message: Vec<Segment>,
		raw_message: String,
		font: i32,
		sender: GroupMessageSender,
	},
}

#[derive(Deserialize, Debug, Clone)]
pub struct PrivateMessageSender {
	user_id: Option<i64>,
	nickname: Option<String>,
	sex: Option<Sex>,
	age: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum PrivateMessageSubType {
	#[serde(rename = "friend")]
	Friend,
	#[serde(rename = "group")]
	Group,
	#[serde(rename = "other")]
	Other,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GroupMessageAnonymous {
	id: i64,
	name: String,
	flag: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupMessageSender {
	user_id: Option<i64>,
	nickname: Option<String>,
	card: Option<String>,
	sex: Option<Sex>,
	age: Option<i32>,
	area: Option<String>,
	level: Option<String>,
	role: Option<GroupMessageSenderRole>,
	title: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum GroupMessageSenderRole {
	#[serde(rename = "owner")]
	Owner,
	#[serde(rename = "admin")]
	Admin,
	#[serde(rename = "member")]
	Member,
}

#[derive(Deserialize, Debug, Clone)]
pub enum GroupMessageSubType {
	#[serde(rename = "normal")]
	Normal,
	#[serde(rename = "anonymous")]
	Anonymous,
	#[serde(rename = "notice")]
	Notice,
}
