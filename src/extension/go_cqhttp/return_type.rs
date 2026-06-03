use crate::message::receive_segment::ReceiveSegment;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct QidianGetAccountInfoResponse {
	#[serde(flatten)]
	pub data: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetModelShowResponse {
	pub variants: Vec<GetModelShowVariantResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetModelShowVariantResponse {
	pub model_show: String,
	pub need_pay: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetOnlineClientsResponse {
	pub clients: Vec<OnlineClientResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OnlineClientResponse {
	pub app_id: i64,
	pub device_name: String,
	pub device_kind: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetUnidirectionalFriendListResponse {
	pub user_id: i64,
	pub nickname: String,
	pub source: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SendForwardMsgResponse {
	pub message_id: i64,
	pub forward_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetGroupMsgHistoryResponse {
	pub messages: Vec<GoCqHttpMessageResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoCqHttpMessageResponse {
	pub message_id: i64,
	pub real_id: i64,
	pub group: bool,
	pub message_type: String,
	pub sender: GoCqHttpMessageSender,
	pub time: i64,
	pub message: Vec<ReceiveSegment>,
	pub raw_message: String,
	#[serde(default)]
	pub group_id: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GoCqHttpMessageSender {
	pub user_id: i64,
	pub nickname: String,
	#[serde(flatten)]
	pub data: HashMap<String, Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetGroupSystemMsgResponse {
	pub invited_requests: Option<Vec<GroupInvitedRequest>>,
	pub join_requests: Option<Vec<GroupJoinRequest>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupInvitedRequest {
	pub request_id: i64,
	pub invitor_uin: i64,
	pub invitor_nick: String,
	pub group_id: i64,
	pub group_name: String,
	pub checked: bool,
	pub actor: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupJoinRequest {
	pub request_id: i64,
	pub requester_uin: i64,
	pub requester_nick: String,
	pub message: String,
	pub group_id: i64,
	pub group_name: String,
	pub checked: bool,
	pub actor: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetEssenceMsgListResponse {
	pub sender_id: i64,
	pub sender_nick: String,
	pub sender_time: i64,
	pub operator_id: i64,
	pub operator_nick: String,
	pub operator_time: i64,
	pub message_id: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetGroupAtAllRemainResponse {
	pub can_at_all: bool,
	pub remain_at_all_count_for_group: i64,
	pub remain_at_all_count_for_uin: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupNoticeResponse {
	pub sender_id: i64,
	pub publish_time: i64,
	pub message: GroupNoticeMessage,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupNoticeMessage {
	pub text: String,
	#[serde(default)]
	pub images: Vec<GroupNoticeImage>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupNoticeImage {
	pub height: String,
	pub width: String,
	pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetGroupFileSystemInfoResponse {
	pub file_count: i64,
	pub limit_count: i64,
	pub used_space: i64,
	pub total_space: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetGroupFilesResponse {
	#[serde(default)]
	pub files: Vec<GroupFileResponse>,
	#[serde(default)]
	pub folders: Vec<GroupFolderResponse>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupFileResponse {
	pub group_id: i64,
	pub file_id: String,
	pub file_name: String,
	pub busid: i64,
	pub file_size: i64,
	pub upload_time: i64,
	pub dead_time: i64,
	pub modify_time: i64,
	pub download_times: i64,
	pub uploader: i64,
	pub uploader_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupFolderResponse {
	pub group_id: i64,
	pub folder_id: String,
	pub folder_name: String,
	pub create_time: i64,
	pub creator: i64,
	pub creator_name: String,
	pub total_file_count: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetGroupFileUrlResponse {
	pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OcrImageResponse {
	pub texts: Vec<OcrTextDetection>,
	pub language: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OcrTextDetection {
	pub text: String,
	pub confidence: i64,
	pub coordinates: Vec<OcrCoordinate>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OcrCoordinate {
	Pair([i64; 2]),
	Point { x: i64, y: i64 },
}

#[derive(Deserialize, Debug, Clone)]
pub struct DownloadFileResponse {
	pub file: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CheckUrlSafelyResponse {
	pub level: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetWordSlicesResponse {
	pub slices: Vec<String>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[test]
	fn deserialize_group_system_msg_with_null_lists() {
		let response: GetGroupSystemMsgResponse = serde_json::from_value(json!({
			"invited_requests": null,
			"join_requests": null
		}))
		.unwrap();

		assert!(response.invited_requests.is_none());
		assert!(response.join_requests.is_none());
	}

	#[test]
	fn deserialize_group_files_with_files_and_folders() {
		let response: GetGroupFilesResponse = serde_json::from_value(json!({
			"files": [
				{
					"group_id": 123,
					"file_id": "file-a",
					"file_name": "hello.txt",
					"busid": 1,
					"file_size": 64,
					"upload_time": 1,
					"dead_time": 0,
					"modify_time": 2,
					"download_times": 3,
					"uploader": 10001,
					"uploader_name": "alice"
				}
			],
			"folders": [
				{
					"group_id": 123,
					"folder_id": "/docs",
					"folder_name": "docs",
					"create_time": 4,
					"creator": 10002,
					"creator_name": "bob",
					"total_file_count": 5
				}
			]
		}))
		.unwrap();

		assert_eq!(response.files.len(), 1);
		assert_eq!(response.folders.len(), 1);
		assert_eq!(response.files[0].file_name, "hello.txt");
		assert_eq!(response.folders[0].folder_name, "docs");
	}

	#[test]
	fn deserialize_ocr_image_with_mixed_coordinate_shapes() {
		let response: OcrImageResponse = serde_json::from_value(json!({
			"language": "zh",
			"texts": [
				{
					"text": "hello",
					"confidence": 98,
					"coordinates": [
						[1, 2],
						{"x": 3, "y": 4}
					]
				}
			]
		}))
		.unwrap();

		assert_eq!(response.texts.len(), 1);
		assert_eq!(response.texts[0].coordinates.len(), 2);
	}

	#[test]
	fn deserialize_group_at_all_remain() {
		let response: GetGroupAtAllRemainResponse = serde_json::from_value(json!({
			"can_at_all": true,
			"remain_at_all_count_for_group": 2,
			"remain_at_all_count_for_uin": 1
		}))
		.unwrap();

		assert!(response.can_at_all);
		assert_eq!(response.remain_at_all_count_for_group, 2);
		assert_eq!(response.remain_at_all_count_for_uin, 1);
	}

	#[test]
	fn deserialize_download_file_and_group_file_url_wrappers() {
		let download: DownloadFileResponse = serde_json::from_value(json!({
			"file": "cache/test.png"
		}))
		.unwrap();
		let file_url: GetGroupFileUrlResponse = serde_json::from_value(json!({
			"url": "https://example.com/file"
		}))
		.unwrap();

		assert_eq!(download.file, "cache/test.png");
		assert_eq!(file_url.url, "https://example.com/file");
	}
}
