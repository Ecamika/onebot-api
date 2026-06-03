use crate::communication::utils::Client;
use crate::error::APIResult;
use crate::message::send_segment::SendSegment;
use async_trait::async_trait;
use onebot_api_macros::api_sender;
use return_type::*;
use serde::Serialize;

#[path = "return_type.rs"]
pub mod return_type;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum DownloadFileHeaders {
	Raw(String),
	Lines(Vec<String>),
}

/// go-cqhttp 扩展 API 发送者 trait。
#[async_trait]
pub trait GoCqHttpAPISender {
	async fn set_qq_profile(
		&self,
		nickname: Option<String>,
		company: Option<String>,
		email: Option<String>,
		college: Option<String>,
		personal_note: Option<String>,
	) -> APIResult<()>;

	async fn qidian_get_account_info(&self) -> APIResult<QidianGetAccountInfoResponse>;

	async fn get_model_show(&self, model: String) -> APIResult<Vec<GetModelShowVariantResponse>>;

	async fn set_model_show(&self, model: String, model_show: String) -> APIResult<()>;

	async fn get_online_clients(
		&self,
		no_cache: Option<bool>,
	) -> APIResult<Vec<OnlineClientResponse>>;

	async fn get_unidirectional_friend_list(
		&self,
	) -> APIResult<Vec<GetUnidirectionalFriendListResponse>>;

	async fn delete_friend(&self, user_id: i64) -> APIResult<()>;

	async fn delete_unidirectional_friend(&self, user_id: i64) -> APIResult<()>;

	async fn mark_msg_as_read(&self, message_id: i64) -> APIResult<()>;

	async fn send_group_forward_msg(
		&self,
		group_id: i64,
		messages: Vec<SendSegment>,
	) -> APIResult<SendForwardMsgResponse>;

	async fn send_private_forward_msg(
		&self,
		user_id: i64,
		messages: Vec<SendSegment>,
	) -> APIResult<SendForwardMsgResponse>;

	async fn get_group_msg_history(
		&self,
		group_id: i64,
		message_seq: Option<i64>,
	) -> APIResult<GetGroupMsgHistoryResponse>;

	async fn get_group_system_msg(&self) -> APIResult<GetGroupSystemMsgResponse>;

	async fn get_essence_msg_list(&self, group_id: i64) -> APIResult<Vec<GetEssenceMsgListResponse>>;

	async fn get_group_at_all_remain(&self, group_id: i64) -> APIResult<GetGroupAtAllRemainResponse>;

	async fn set_group_portrait(
		&self,
		group_id: i64,
		file: String,
		cache: Option<i64>,
	) -> APIResult<()>;

	async fn set_essence_msg(&self, message_id: i64) -> APIResult<()>;

	async fn delete_essence_msg(&self, message_id: i64) -> APIResult<()>;

	async fn send_group_sign(&self, group_id: i64) -> APIResult<()>;

	async fn send_group_notice(
		&self,
		group_id: i64,
		content: String,
		image: Option<String>,
	) -> APIResult<()>;

	async fn get_group_notice(&self, group_id: i64) -> APIResult<Vec<GroupNoticeResponse>>;

	async fn upload_group_file(
		&self,
		group_id: i64,
		file: String,
		name: String,
		folder: Option<String>,
	) -> APIResult<()>;

	async fn delete_group_file(&self, group_id: i64, file_id: String, busid: i64) -> APIResult<()>;

	async fn create_group_file_folder(
		&self,
		group_id: i64,
		name: String,
		parent_id: Option<String>,
	) -> APIResult<()>;

	async fn delete_group_folder(&self, group_id: i64, folder_id: String) -> APIResult<()>;

	async fn get_group_file_system_info(
		&self,
		group_id: i64,
	) -> APIResult<GetGroupFileSystemInfoResponse>;

	async fn get_group_root_files(&self, group_id: i64) -> APIResult<GetGroupFilesResponse>;

	async fn get_group_files_by_folder(
		&self,
		group_id: i64,
		folder_id: String,
	) -> APIResult<GetGroupFilesResponse>;

	async fn get_group_file_url(
		&self,
		group_id: i64,
		file_id: String,
		busid: i64,
	) -> APIResult<String>;

	async fn upload_private_file(&self, user_id: i64, file: String, name: String) -> APIResult<()>;

	async fn ocr_image(&self, image: String) -> APIResult<OcrImageResponse>;

	async fn reload_event_filter(&self) -> APIResult<()>;

	async fn download_file(
		&self,
		url: String,
		thread_count: Option<i64>,
		headers: Option<DownloadFileHeaders>,
	) -> APIResult<String>;

	async fn check_url_safely(&self, url: String) -> APIResult<i64>;

	async fn get_word_slices(&self, content: String) -> APIResult<Vec<String>>;
}

#[api_sender]
#[async_trait]
impl GoCqHttpAPISender for Client {
	#[api(discard = true)]
	async fn set_qq_profile(
		&self,
		nickname: Option<String>,
		company: Option<String>,
		email: Option<String>,
		college: Option<String>,
		personal_note: Option<String>,
	) -> APIResult<()> {
	}

	async fn qidian_get_account_info(&self) -> APIResult<QidianGetAccountInfoResponse> {}

	#[api(action = "_get_model_show", extract = "variants", response = GetModelShowResponse)]
	async fn get_model_show(&self, model: String) -> APIResult<Vec<GetModelShowVariantResponse>> {}

	#[api(action = "_set_model_show", discard = true)]
	async fn set_model_show(&self, model: String, model_show: String) -> APIResult<()> {}

	#[api(extract = "clients", response = GetOnlineClientsResponse)]
	async fn get_online_clients(
		&self,
		no_cache: Option<bool>,
	) -> APIResult<Vec<OnlineClientResponse>> {
	}

	async fn get_unidirectional_friend_list(
		&self,
	) -> APIResult<Vec<GetUnidirectionalFriendListResponse>> {
	}

	#[api(discard = true)]
	async fn delete_friend(&self, user_id: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn delete_unidirectional_friend(&self, user_id: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn mark_msg_as_read(&self, message_id: i64) -> APIResult<()> {}

	async fn send_group_forward_msg(
		&self,
		group_id: i64,
		messages: Vec<SendSegment>,
	) -> APIResult<SendForwardMsgResponse> {
	}

	async fn send_private_forward_msg(
		&self,
		user_id: i64,
		messages: Vec<SendSegment>,
	) -> APIResult<SendForwardMsgResponse> {
	}

	async fn get_group_msg_history(
		&self,
		group_id: i64,
		message_seq: Option<i64>,
	) -> APIResult<GetGroupMsgHistoryResponse> {
	}

	async fn get_group_system_msg(&self) -> APIResult<GetGroupSystemMsgResponse> {}

	async fn get_essence_msg_list(&self, group_id: i64) -> APIResult<Vec<GetEssenceMsgListResponse>> {
	}

	async fn get_group_at_all_remain(&self, group_id: i64) -> APIResult<GetGroupAtAllRemainResponse> {
	}

	#[api(discard = true)]
	async fn set_group_portrait(
		&self,
		group_id: i64,
		file: String,
		cache: Option<i64>,
	) -> APIResult<()> {
	}

	#[api(discard = true)]
	async fn set_essence_msg(&self, message_id: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn delete_essence_msg(&self, message_id: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn send_group_sign(&self, group_id: i64) -> APIResult<()> {}

	#[api(action = "_send_group_notice", discard = true)]
	async fn send_group_notice(
		&self,
		group_id: i64,
		content: String,
		image: Option<String>,
	) -> APIResult<()> {
	}

	#[api(action = "_get_group_notice")]
	async fn get_group_notice(&self, group_id: i64) -> APIResult<Vec<GroupNoticeResponse>> {}

	#[api(discard = true)]
	async fn upload_group_file(
		&self,
		group_id: i64,
		file: String,
		name: String,
		folder: Option<String>,
	) -> APIResult<()> {
	}

	#[api(discard = true)]
	async fn delete_group_file(&self, group_id: i64, file_id: String, busid: i64) -> APIResult<()> {}

	#[api(discard = true)]
	async fn create_group_file_folder(
		&self,
		group_id: i64,
		name: String,
		parent_id: Option<String>,
	) -> APIResult<()> {
	}

	#[api(discard = true)]
	async fn delete_group_folder(&self, group_id: i64, folder_id: String) -> APIResult<()> {}

	async fn get_group_file_system_info(
		&self,
		group_id: i64,
	) -> APIResult<GetGroupFileSystemInfoResponse> {
	}

	async fn get_group_root_files(&self, group_id: i64) -> APIResult<GetGroupFilesResponse> {}

	async fn get_group_files_by_folder(
		&self,
		group_id: i64,
		folder_id: String,
	) -> APIResult<GetGroupFilesResponse> {
	}

	#[api(extract = "url", response = GetGroupFileUrlResponse)]
	async fn get_group_file_url(
		&self,
		group_id: i64,
		file_id: String,
		busid: i64,
	) -> APIResult<String> {
	}

	#[api(discard = true)]
	async fn upload_private_file(&self, user_id: i64, file: String, name: String) -> APIResult<()> {}

	#[api(action = ".ocr_image", map(image = "image"))]
	async fn ocr_image(&self, image: String) -> APIResult<OcrImageResponse> {}

	#[api(discard = true)]
	async fn reload_event_filter(&self) -> APIResult<()> {}

	#[api(extract = "file", response = DownloadFileResponse)]
	async fn download_file(
		&self,
		url: String,
		thread_count: Option<i64>,
		headers: Option<DownloadFileHeaders>,
	) -> APIResult<String> {
	}

	#[api(extract = "level", response = CheckUrlSafelyResponse)]
	async fn check_url_safely(&self, url: String) -> APIResult<i64> {}

	#[api(action = ".get_word_slices", extract = "slices", response = GetWordSlicesResponse)]
	async fn get_word_slices(&self, content: String) -> APIResult<Vec<String>> {}
}

#[cfg(test)]
mod tests {
	use super::GoCqHttpAPISender;
	use crate::communication::utils::Client;

	fn assert_impl<T: GoCqHttpAPISender>() {}

	#[test]
	fn client_implements_go_cqhttp_api_sender() {
		assert_impl::<Client>();
	}
}
