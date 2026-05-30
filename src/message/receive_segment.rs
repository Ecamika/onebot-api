use super::utils::*;
use serde::Deserialize;
use strum::EnumIs;

#[cfg_attr(feature = "selector", derive(crate::Selector))]
#[derive(Deserialize, Debug, Clone, EnumIs)]
#[serde(tag = "type")]
pub enum ReceiveSegment {
	#[serde(rename = "text")]
	Text { data: TextData },

	#[serde(rename = "face")]
	Face { data: FaceData },

	#[serde(rename = "image")]
	Image { data: ImageData },

	#[serde(rename = "record")]
	Record { data: RecordData },

	#[serde(rename = "video")]
	Video { data: VideoData },

	#[serde(rename = "at")]
	At { data: AtData },

	#[serde(rename = "rps")]
	Rps { data: RpsData },

	#[serde(rename = "dice")]
	Dice { data: DiceData },

	#[serde(rename = "shake")]
	Shake { data: ShakeData },

	#[serde(rename = "poke")]
	Poke { data: PokeData },

	#[serde(rename = "anonymous")]
	Anonymous { data: AnonymousData },

	#[serde(rename = "share")]
	Share { data: ShareData },

	#[serde(rename = "contact")]
	Contact { data: ContactData },

	#[serde(rename = "location")]
	Location { data: LocationData },

	#[serde(rename = "music")]
	Music { data: MusicData },

	#[serde(rename = "reply")]
	Reply { data: ReplyData },

	#[serde(rename = "forward")]
	Forward { data: ForwardData },

	#[serde(rename = "node")]
	Node { data: NodeData },

	#[serde(rename = "xml")]
	Xml { data: XmlData },

	#[serde(rename = "json")]
	Json { data: JsonData },
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
