use serde::{Deserialize, Serialize};

/// 消息类型，用于 [`send_msg`](super::APISender::send_msg) 等接口指定消息的目标类型。
#[derive(Serialize, Debug, Deserialize)]
pub enum MessageType {
	/// 私聊消息。
	#[serde(rename = "private")]
	Private,
	/// 群消息。
	#[serde(rename = "group")]
	Group,
}
