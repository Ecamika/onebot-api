use onebot_api_macros::api_sender;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Mutex;

type ApiResult<T> = Result<T, anyhow::Error>;

#[derive(Deserialize, Debug, PartialEq)]
pub struct SendMsgResponse {
	pub message_id: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetCookiesResponse {
	pub cookies: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GetLoginInfoResponse {
	pub user_id: i32,
	pub nickname: String,
}

#[async_trait::async_trait]
pub trait TestApi {
	async fn send_private_msg(
		&self,
		user_id: i64,
		message: String,
		auto_escape: Option<bool>,
	) -> ApiResult<i32>;

	async fn delete_msg(&self, message_id: i32) -> ApiResult<()>;

	async fn get_login_info(&self) -> ApiResult<GetLoginInfoResponse>;

	async fn get_cookies(&self, domain: Option<String>) -> ApiResult<String>;

	async fn get_group_honor_info(
		&self,
		group_id: i64,
		honor_type: String,
	) -> ApiResult<GetLoginInfoResponse>;

	async fn no_params_no_extract(&self) -> ApiResult<()>;
}

struct TestClient {
	last_action: Mutex<Option<String>>,
	last_params: Mutex<Option<Value>>,
}

impl TestClient {
	fn new() -> Self {
		Self {
			last_action: Mutex::new(None),
			last_params: Mutex::new(None),
		}
	}

	fn take_last_action(&self) -> Option<String> {
		self.last_action.lock().unwrap().take()
	}

	fn take_last_params(&self) -> Option<Value> {
		self.last_params.lock().unwrap().take()
	}

	async fn send_and_parse<T: serde::de::DeserializeOwned>(
		&self,
		action: impl ToString,
		params: Value,
	) -> ApiResult<T> {
		let action_str = action.to_string();
		*self.last_action.lock().unwrap() = Some(action_str.clone());
		*self.last_params.lock().unwrap() = Some(params.clone());

		let response = match action_str.as_str() {
			"send_private_msg" => json!({"message_id": 42}),
			"get_cookies" => json!({"cookies": "test_cookie_value"}),
			"get_login_info" => json!({"user_id": 1, "nickname": "test_user"}),
			"get_group_honor_info" => json!({"user_id": 2, "nickname": "group_user"}),
			"delete_msg" | "no_params_no_extract" => json!(null),
			_ => json!({}),
		};
		Ok(serde_json::from_value(response)?)
	}
}

#[api_sender]
#[async_trait::async_trait]
impl TestApi for TestClient {
	#[api(extract = "message_id", response = SendMsgResponse)]
	async fn send_private_msg(
		&self,
		user_id: i64,
		message: String,
		auto_escape: Option<bool>,
	) -> ApiResult<i32> {
	}

	async fn delete_msg(&self, message_id: i32) -> ApiResult<()> {}

	async fn get_login_info(&self) -> ApiResult<GetLoginInfoResponse> {}

	#[api(extract = "cookies", response = GetCookiesResponse)]
	async fn get_cookies(&self, domain: Option<String>) -> ApiResult<String> {}

	#[api(map(honor_type = "type"))]
	async fn get_group_honor_info(
		&self,
		group_id: i64,
		honor_type: String,
	) -> ApiResult<GetLoginInfoResponse> {
	}

	async fn no_params_no_extract(&self) -> ApiResult<()> {}
}

#[tokio::test]
async fn test_extract_field() {
	let client = TestClient::new();
	let result = client
		.send_private_msg(123456, "hello".to_string(), Some(false))
		.await
		.unwrap();
	assert_eq!(result, 42);
	assert_eq!(
		client.take_last_action().as_deref(),
		Some("send_private_msg")
	);

	let params = client.take_last_params().unwrap();
	assert_eq!(params["user_id"], json!(123456));
	assert_eq!(params["message"], json!("hello"));
	assert_eq!(params["auto_escape"], json!(false));
}

#[tokio::test]
async fn test_direct_pass_through() {
	let client = TestClient::new();
	client.delete_msg(999).await.unwrap();
	assert_eq!(client.take_last_action().as_deref(), Some("delete_msg"));

	let params = client.take_last_params().unwrap();
	assert_eq!(params["message_id"], json!(999));
}

#[tokio::test]
async fn test_no_params() {
	let client = TestClient::new();
	let result = client.get_login_info().await.unwrap();
	assert_eq!(
		result,
		GetLoginInfoResponse {
			user_id: 1,
			nickname: "test_user".to_string(),
		}
	);
	assert_eq!(client.take_last_action().as_deref(), Some("get_login_info"));

	let params = client.take_last_params().unwrap();
	assert_eq!(params, json!({}));
}

#[tokio::test]
async fn test_extract_cookies() {
	let client = TestClient::new();
	let cookies = client
		.get_cookies(Some("example.com".to_string()))
		.await
		.unwrap();
	assert_eq!(cookies, "test_cookie_value");
	assert_eq!(client.take_last_action().as_deref(), Some("get_cookies"));

	let params = client.take_last_params().unwrap();
	assert_eq!(params["domain"], json!("example.com"));
}

#[tokio::test]
async fn test_rename_param() {
	let client = TestClient::new();
	let result = client
		.get_group_honor_info(789, "talkative".to_string())
		.await
		.unwrap();
	assert_eq!(result.user_id, 2);
	assert_eq!(
		client.take_last_action().as_deref(),
		Some("get_group_honor_info")
	);

	let params = client.take_last_params().unwrap();
	assert_eq!(params["group_id"], json!(789));
	assert_eq!(params["type"], json!("talkative"));
	assert!(params.get("honor_type").is_none());
}

#[tokio::test]
async fn test_no_params_no_extract_empty_json() {
	let client = TestClient::new();
	client.no_params_no_extract().await.unwrap();
	assert_eq!(
		client.take_last_action().as_deref(),
		Some("no_params_no_extract")
	);

	let params = client.take_last_params().unwrap();
	assert_eq!(params, json!({}));
}

#[tokio::test]
async fn test_option_none_becomes_null() {
	let client = TestClient::new();
	client
		.send_private_msg(123, "msg".to_string(), None)
		.await
		.unwrap();

	let params = client.take_last_params().unwrap();
	assert_eq!(params["user_id"], json!(123));
	assert_eq!(params["message"], json!("msg"));
	assert_eq!(params["auto_escape"], json!(null));
}
