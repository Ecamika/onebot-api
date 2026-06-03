use serde::{Deserialize, Serialize};
use strum::EnumIs;

#[derive(Deserialize, Serialize, Debug, Clone, EnumIs)]
pub enum ImageType {
	#[serde(rename = "flash")]
	Flash,
}

#[cfg_attr(feature = "selector", derive(onebot_api_macros::Selector))]
#[derive(Deserialize, Serialize, Debug, Clone, EnumIs)]
#[serde(untagged)]
pub enum AtType {
	#[serde(rename = "all")]
	All,
	Id(String),
}

#[derive(Deserialize, Serialize, Debug, Clone, EnumIs)]
pub enum ContactType {
	#[serde(rename = "qq")]
	QQ,
	#[serde(rename = "group")]
	Group,
}

#[derive(Deserialize, Serialize, Debug, Clone, EnumIs)]
pub enum MusicType {
	#[serde(rename = "qq")]
	QQ,
	#[serde(rename = "163")]
	NetEaseCloudMusic,
	#[serde(rename = "xm")]
	Xm,
	#[serde(rename = "custom")]
	Custom,
}
