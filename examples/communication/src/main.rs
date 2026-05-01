use onebot_api::communication::http::HttpService;
use onebot_api::communication::http_post::HttpPostService;
use onebot_api::communication::sse::SseService;
use onebot_api::communication::utils::Client;
use onebot_api::communication::ws::WsService;
use onebot_api::communication::ws_reverse::WsReverseService;

#[tokio::main]
async fn main() {
	let _http_client = http();
	let _http_post_client = http_post();
	let _sse_client = sse();
	let _websocket_client = websocket();
	let _websocket_reverse_client = websocket_reverse();
}

fn http() -> Client {
	let http_service =
		HttpService::new("http://localhost:5000", Some("example_token".to_string())).unwrap();
	Client::new(http_service)
}

fn http_post() -> Client {
	let http_post_service = HttpPostService::new(
		"127.0.0.1:6000",
		Some("/prefix".to_string()),
		Some("example_secret".to_string()),
	)
	.unwrap();
	Client::new(http_post_service)
}

fn sse() -> Client {
	let sse_service =
		SseService::new("http://localhost:7000", Some("example_token".to_string())).unwrap();
	Client::new(sse_service)
}

fn websocket() -> Client {
	let ws_service = WsService::new(
		"ws://localhost:8000".parse().unwrap(),
		Some("example_token".to_string()),
	);
	Client::new(ws_service)
}

fn websocket_reverse() -> Client {
	let ws_reverse_service =
		WsReverseService::new("127.0.0.1:9000", Some("example_token".to_string()));
	Client::new(ws_reverse_service)
}
