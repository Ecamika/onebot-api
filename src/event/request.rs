use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "request_type")]
pub enum RequestEvent {
	#[serde(rename = "friend")]
	Friend {
		user_id: i64,
		comment: String,
		flag: String,
	},

	#[serde(rename = "group")]
	Group {
		sub_type: GroupType,
		group_id: i64,
		user_id: i64,
		comment: String,
		flag: String,
	},
}

#[derive(Deserialize, Debug, Clone)]
pub enum GroupType {
	#[serde(rename = "add")]
	Add,
	#[serde(rename = "invite")]
	Invite,
}
