use super::utils::*;
use serde::Deserialize;
#[cfg(not(feature = "selector"))]
use strum::EnumIs;

#[cfg_attr(feature = "selector", derive(tynavi::Selector))]
#[cfg_attr(not(feature = "selector"), derive(EnumIs))]
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ReceiveSegment {
	#[serde(rename = "text")]
	Text(TextData),

	#[serde(rename = "face")]
	Face(FaceData),

	#[serde(rename = "image")]
	Image(ImageData),

	#[serde(rename = "record")]
	Record(RecordData),

	#[serde(rename = "video")]
	Video(VideoData),

	#[serde(rename = "at")]
	At(AtData),

	#[serde(rename = "rps")]
	Rps(RpsData),

	#[serde(rename = "dice")]
	Dice(DiceData),

	#[serde(rename = "shake")]
	Shake(ShakeData),

	#[serde(rename = "poke")]
	Poke(PokeData),

	#[serde(rename = "anonymous")]
	Anonymous(AnonymousData),

	#[serde(rename = "share")]
	Share(ShareData),

	#[serde(rename = "contact")]
	Contact(ContactData),

	#[serde(rename = "location")]
	Location(LocationData),

	#[serde(rename = "music")]
	Music(MusicData),

	#[serde(rename = "reply")]
	Reply(ReplyData),

	#[serde(rename = "forward")]
	Forward(ForwardData),

	#[serde(rename = "node")]
	Node(NodeData),

	#[serde(rename = "xml")]
	Xml(XmlData),

	#[serde(rename = "json")]
	Json(JsonData),
}

#[derive(Deserialize, Debug, Clone)]
pub struct TextData {
	/// # 说明
	/// 纯文本内容
	pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FaceData {
	/// # 说明
	/// QQ 表情 ID
	/// # 可能的值
	/// 见 [QQ 表情 ID 表](https://github.com/richardchien/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8)
	pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ImageData {
	/// # 说明
	/// 图片文件名
	pub file: String,
	#[serde(rename = "type")]
	/// # 说明
	/// 图片类型，`flash` 表示闪照，无此参数表示普通图片
	/// # 可能的值
	/// `flash`
	pub image_type: Option<ImageType>,
	/// # 说明
	/// 图片 URL
	pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RecordData {
	/// # 说明
	/// 语音文件名
	pub file: String,
	/// # 说明
	/// 发送时可选，默认 `0`，设置为 `1` 表示变声
	/// # 可能的值
	/// `0` `1`
	pub magic: String,
	/// # 说明
	/// 语音 URL
	pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct VideoData {
	/// # 说明
	/// 视频文件名
	pub file: String,
	/// # 说明
	/// 视频 URL
	pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AtData {
	/// # 说明
	/// @的 QQ 号，`all` 表示全体成员
	/// # 可能的值
	/// QQ 号、`all`
	pub qq: AtType,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RpsData {}

#[derive(Deserialize, Debug, Clone)]
pub struct DiceData {}

#[derive(Deserialize, Debug, Clone)]
pub struct ShakeData {}

#[derive(Deserialize, Debug, Clone)]
pub struct PokeData {
	#[serde(rename = "type")]
	/// # 说明
	/// 类型
	/// # 可能的值
	/// 见 [Mirai 的 PokeMessage 类](https://github.com/mamoe/mirai/blob/f5eefae7ecee84d18a66afce3f89b89fe1584b78/mirai-core/src/commonMain/kotlin/net.mamoe.mirai/message/data/HummerMessage.kt#L49)
	pub poke_type: String,
	/// # 说明
	/// ID
	/// # 可能的值
	/// 同上
	pub id: String,
	/// # 说明
	/// 表情名
	/// # 可能的值
	/// 同上
	pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AnonymousData {}

#[derive(Deserialize, Debug, Clone)]
pub struct ShareData {
	/// # 说明
	/// URL
	pub url: String,
	/// # 说明
	/// 标题
	pub title: String,
	/// # 说明
	/// 发送时可选，内容描述
	pub content: String,
	/// # 说明
	/// 发送时可选，图片 URL
	pub image: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactData {
	#[serde(rename = "type")]
	/// # 说明
	/// 推荐好友/群
	pub contact_type: ContactType,
	/// # 说明
	/// 被推荐人的 QQ 号/被推荐群的群号
	pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LocationData {
	/// # 说明
	/// 纬度
	pub lat: String,
	/// # 说明
	/// 经度
	pub lon: String,
	/// # 说明
	/// 发送时可选，标题
	pub title: String,
	/// # 说明
	/// 发送时可选，内容描述
	pub content: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MusicData {}

#[derive(Deserialize, Debug, Clone)]
pub struct ReplyData {
	/// # 说明
	/// 回复时引用的消息 ID
	pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ForwardData {
	/// # 说明
	/// 合并转发 ID，需通过 [`get_forward_msg` API](https://github.com/botuniverse/onebot-11/blob/master/api/public.md#get_forward_msg-%E8%8E%B7%E5%8F%96%E5%90%88%E5%B9%B6%E8%BD%AC%E5%8F%91%E6%B6%88%E6%81%AF) 获取具体内容
	pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NodeData {
	/// # 说明
	/// 发送者 QQ 号
	pub user_id: String,
	/// # 说明
	/// 发送者昵称
	pub nickname: String,
	/// # 说明
	/// 消息内容，支持发送消息时的 `message` 数据类型，见 [API 的参数](https://github.com/botuniverse/onebot-11/blob/master/api/#%E5%8F%82%E6%95%B0)
	pub content: Vec<ReceiveSegment>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct XmlData {
	/// # 说明
	/// XML 内容
	pub data: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonData {
	/// 说明
	/// JSON 内容
	pub data: String,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn deserialize_text_segment_with_adjacently_tagged_shape() {
		let segment: ReceiveSegment =
			serde_json::from_str(r#"{"type":"text","data":{"text":"hello"}}"#)
				.expect("text segment should deserialize");

		match segment {
			ReceiveSegment::Text(data) => assert_eq!(data.text, "hello"),
			other => panic!("unexpected segment: {other:?}"),
		}
	}

	#[test]
	fn deserialize_nested_node_segment() {
		let segment: ReceiveSegment = serde_json::from_str(
			r#"{"type":"node","data":{"user_id":"1","nickname":"bot","content":[{"type":"text","data":{"text":"nested"}}]}}"#,
		)
		.expect("node segment should deserialize");

		match segment {
			ReceiveSegment::Node(data) => match data.content.as_slice() {
				[ReceiveSegment::Text(text)] => assert_eq!(text.text, "nested"),
				other => panic!("unexpected nested content: {other:?}"),
			},
			other => panic!("unexpected segment: {other:?}"),
		}
	}
}
