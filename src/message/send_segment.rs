use super::utils::*;
use serde::Serialize;

pub trait SendSegmentData: Clone + Serialize {
	fn into_send_segment(self) -> SendSegment;
}

impl<T: SendSegmentData> From<T> for SendSegment {
	fn from(value: T) -> Self {
		value.into_send_segment()
	}
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SendSegment {
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

#[derive(Serialize, Debug, Clone)]
pub struct TextData {
	/// # 说明
	/// 纯文本内容
	pub text: String,
}

impl SendSegmentData for TextData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Text { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct FaceData {
	/// # 说明
	/// QQ 表情 ID
	/// # 可能的值
	/// 见 [QQ 表情 ID 表](https://github.com/richardchien/coolq-http-api/wiki/%E8%A1%A8%E6%83%85-CQ-%E7%A0%81-ID-%E8%A1%A8)
	pub id: String,
}

impl SendSegmentData for FaceData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Face { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct ImageData {
	/// # 说明
	/// 图片文件名
	/// # TIPS
	/// 发送时，`file` 参数除了支持使用收到的图片文件名直接发送外，还支持：
	/// - 绝对路径，例如 `file:///C:\\Users\Richard\Pictures\1.png`，格式使用 [`file` URI](https://tools.ietf.org/html/rfc8089)
	/// - 网络 URL，例如 `http://i1.piimg.com/567571/fdd6e7b6d93f1ef0.jpg`
	/// - Base64 编码，例如 `base64://iVBORw0KGgoAAAANSUhEUgAAABQAAAAVCAIAAADJt1n/AAAAKElEQVQ4EWPk5+RmIBcwkasRpG9UM4mhNxpgowFGMARGEwnBIEJVAAAdBgBNAZf+QAAAAABJRU5ErkJggg==`
	pub file: String,
	#[serde(rename = "type")]
	/// # 说明
	/// 图片类型，`flash` 表示闪照，无此参数表示普通图片
	/// # 可能的值
	/// `flash`
	pub image_type: Option<ImageType>,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，表示是否使用已缓存的文件，默认 `1`
	/// # 可能的值
	/// `0` `1`
	pub cache: Option<bool>,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，表示是否通过代理下载文件（需通过环境变量或配置文件配置代理），默认 `1`
	/// # 可能的值
	/// `0` `1`
	pub proxy: Option<bool>,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，单位秒，表示下载网络文件的超时时间，默认不超时
	pub timeout: Option<i64>,
}

impl SendSegmentData for ImageData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Image { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct RecordData {
	/// # 说明
	/// 语音文件名
	/// # TIPS
	/// 发送时，`file` 参数除了支持使用收到的语音文件名直接发送外，还支持其它形式，参考 [`ImageData::file`]。
	pub file: String,
	/// # 说明
	/// 发送时可选，默认 `0`，设置为 `1` 表示变声
	/// # 可能的值
	/// `0` `1`
	pub magic: Option<String>,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，表示是否使用已缓存的文件，默认 `1`
	/// # 可能的值
	/// `0` `1`
	pub cache: Option<bool>,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，表示是否通过代理下载文件（需通过环境变量或配置文件配置代理），默认 `1`
	/// # 可能的值
	/// `0` `1`
	pub proxy: Option<bool>,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，单位秒，表示下载网络文件的超时时间，默认不超时
	pub timeout: Option<i64>,
}

impl SendSegmentData for RecordData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Record { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct VideoData {
	/// # 说明
	/// 视频文件名
	/// # TIPS
	/// 发送时，`file` 参数除了支持使用收到的语音文件名直接发送外，还支持其它形式，参考 [`ImageData::file`]。
	pub file: String,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，表示是否使用已缓存的文件，默认 `1`
	/// # 可能的值
	/// `0` `1`
	pub cache: Option<bool>,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，表示是否通过代理下载文件（需通过环境变量或配置文件配置代理），默认 `1`
	/// # 可能的值
	/// `0` `1`
	pub proxy: Option<bool>,
	/// # 说明
	/// 只在通过网络 URL 发送时有效，单位秒，表示下载网络文件的超时时间，默认不超时
	pub timeout: Option<i64>,
}

impl SendSegmentData for VideoData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Video { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct AtData {
	/// # 说明
	/// @的 QQ 号，`all` 表示全体成员
	/// # 可能的值
	/// QQ 号、`all`
	pub qq: AtType,
}

impl SendSegmentData for AtData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::At { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct RpsData {}

impl SendSegmentData for RpsData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Rps { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct DiceData {}

impl SendSegmentData for DiceData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Dice { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct ShakeData {}

impl SendSegmentData for ShakeData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Shake { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
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
}

impl SendSegmentData for PokeData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Poke { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct AnonymousData {
	/// # 说明
	/// 可选，表示无法匿名时是否继续发送
	/// # 可能的值
	/// `0` `1`
	pub ignore: Option<bool>,
}

impl SendSegmentData for AnonymousData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Anonymous { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
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

impl SendSegmentData for ShareData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Share { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct ContactData {
	#[serde(rename = "type")]
	/// # 说明
	/// 推荐好友/群
	pub contact_type: ContactType,
	/// # 说明
	/// 被推荐人的 QQ 号/被推荐群的群号
	pub id: String,
}

impl SendSegmentData for ContactData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Contact { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct LocationData {
	/// # 说明
	/// 纬度
	pub lat: String,
	/// # 说明
	/// 经度
	pub lon: String,
	/// # 说明
	/// 发送时可选，标题
	pub title: Option<String>,
	/// # 说明
	/// 发送时可选，内容描述
	pub content: Option<String>,
}

impl SendSegmentData for LocationData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Location { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct MusicData {
	#[serde(rename = "type")]
	/// # 说明
	/// 分别表示使用 QQ 音乐、网易云音乐、虾米音乐 / 表示音乐自定义分享
	/// # 可能的值
	/// `qq` `163` `xm` `custom`
	pub music_type: MusicType,
	/// # 说明
	/// 歌曲 ID
	pub id: Option<String>,
	/// # 说明
	/// 点击后跳转目标 URL
	pub url: Option<String>,
	/// # 说明
	/// 音乐 URL
	pub audio: Option<String>,
	/// # 说明
	/// 标题
	pub title: Option<String>,
	/// # 说明
	/// 发送时可选，内容描述
	pub content: Option<String>,
	/// # 说明
	/// 发送时可选，图片 URL
	pub image: Option<String>,
}

impl SendSegmentData for MusicData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Music { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct ReplyData {
	/// # 说明
	/// 回复时引用的消息 ID
	pub id: String,
}

impl SendSegmentData for ReplyData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Reply { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct ForwardData {}

impl SendSegmentData for ForwardData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Forward { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct NodeData {
	/// # 说明
	/// 转发的消息 ID
	pub id: Option<String>,
	/// # 说明
	/// 发送者 QQ 号
	pub user_id: Option<String>,
	/// # 说明
	/// 发送者昵称
	pub nickname: Option<String>,
	/// # 说明
	/// 消息内容，支持发送消息时的 `message` 数据类型，见 [API 的参数](https://github.com/botuniverse/onebot-11/blob/master/api/#%E5%8F%82%E6%95%B0)
	pub content: Option<Vec<SendSegment>>,
}

impl SendSegmentData for NodeData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Node { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct XmlData {
	/// # 说明
	/// XML 内容
	pub data: String,
}

impl SendSegmentData for XmlData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Xml { data: self }
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct JsonData {
	/// 说明
	/// JSON 内容
	pub data: String,
}

impl SendSegmentData for JsonData {
	fn into_send_segment(self) -> SendSegment {
		SendSegment::Json { data: self }
	}
}
