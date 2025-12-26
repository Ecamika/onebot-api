use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Segment {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextData {
	pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FaceData {
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageData {
	pub file: String,
	#[serde(rename = "type")]
	pub image_type: Option<String>,
	pub url: Option<String>,
	pub cache: Option<String>,
	pub proxy: Option<String>,
	pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecordData {
	pub file: String,
	pub magic: String,
	pub url: Option<String>,
	pub cache: Option<String>,
	pub proxy: Option<String>,
	pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoData {
	pub file: String,
	pub url: Option<String>,
	pub cache: Option<String>,
	pub proxy: Option<String>,
	pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AtData {
	pub qq: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RpsData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiceData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShakeData {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PokeData {
	#[serde(rename = "type")]
	pub poke_type: String,
	pub id: String,
	pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnonymousData {
	pub ignore: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShareData {
	pub url: String,
	pub title: String,
	pub content: String,
	pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactData {
	#[serde(rename = "type")]
	pub contact_type: String,
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationData {
	pub lat: String,
	pub lon: String,
	pub title: String,
	pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MusicData {
	#[serde(rename = "type")]
	pub music_type: Option<String>,
	pub id: Option<String>,
	pub url: Option<String>,
	pub audio: Option<String>,
	pub title: Option<String>,
	pub content: Option<String>,
	pub image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReplyData {
	pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForwardData {
	pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeData {
	pub id: Option<String>,
	pub user_id: Option<String>,
	pub nickname: Option<String>,
	pub content: Option<Vec<Segment>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlData {
	pub data: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonData {
	pub data: String,
}
