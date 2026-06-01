#![cfg(feature = "http-post")]

use std::time::Duration;

use hmac::KeyInit;
use onebot_api::communication::http_post::{HttpPostService, get_sig};
use onebot_api::communication::utils::{CommunicationService, InternalEventSender};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::sleep;

fn reserve_addr() -> String {
	let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
	let addr = listener.local_addr().unwrap();
	drop(listener);
	addr.to_string()
}

async fn start_service(
	addr: String,
	secret: Option<&str>,
) -> (
	HttpPostService<String>,
	flume::Receiver<onebot_api::communication::utils::DeserializedEvent>,
) {
	let (_api_sender, api_receiver) = flume::bounded(1);
	let (event_sender, event_receiver): (InternalEventSender, flume::Receiver<_>) = flume::bounded(8);
	let mut service =
		HttpPostService::new(addr, Some("/".to_string()), secret.map(str::to_string)).unwrap();
	service.install(api_receiver, event_sender);
	service.start().await.unwrap();
	(service, event_receiver)
}

#[tokio::test]
async fn http_post_returns_expected_status_codes_for_invalid_requests() {
	let addr = reserve_addr();
	let base_url = format!("http://{addr}/");
	let (mut service, _event_receiver) = start_service(addr.clone(), Some("secret")).await;
	let client = reqwest::Client::new();

	sleep(Duration::from_millis(50)).await;

	let response = client.post(&base_url).body("{}").send().await.unwrap();
	assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);

	let response = client
		.post(&base_url)
		.header("X-Signature", "sha1=wrong")
		.body("{}")
		.send()
		.await
		.unwrap();
	assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);

	let signature = format!(
		"sha1={}",
		get_sig(
			hmac::Hmac::<sha1::Sha1>::new_from_slice(b"secret").unwrap(),
			b"not-json",
		)
	);
	let mut stream = tokio::net::TcpStream::connect(&addr).await.unwrap();
	let request = format!(
		"POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: 8\r\nX-Signature: {signature}\r\n\r\nnot-json"
	);
	stream.write_all(request.as_bytes()).await.unwrap();
	let mut response = vec![0; 512];
	let n = stream.read(&mut response).await.unwrap();
	let response = String::from_utf8_lossy(&response[..n]);
	assert!(response.starts_with("HTTP/1.1 400"));
	assert!(service.is_running());

	service.stop();
}

#[tokio::test]
async fn http_post_accepts_valid_requests_and_rejects_invalid_signature_headers() {
	let addr = reserve_addr();
	let base_url = format!("http://{addr}/");
	let (mut service, event_receiver) = start_service(addr.clone(), Some("secret")).await;
	let body = r#"{"status":"ok","retcode":0,"data":{},"echo":"demo"}"#;

	sleep(Duration::from_millis(50)).await;

	let mut stream = tokio::net::TcpStream::connect(&addr).await.unwrap();
	stream
		.write_all(
			b"POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: 2\r\nX-Signature: \xFF\r\n\r\n{}",
		)
		.await
		.unwrap();
	let mut response = vec![0; 512];
	let n = stream.read(&mut response).await.unwrap();
	let response = String::from_utf8_lossy(&response[..n]);
	assert!(response.starts_with("HTTP/1.1 400"));

	let client = reqwest::Client::new();
	let signature = format!(
		"sha1={}",
		get_sig(
			hmac::Hmac::<sha1::Sha1>::new_from_slice(b"secret").unwrap(),
			body.as_bytes(),
		)
	);
	let response = client
		.post(&base_url)
		.header("X-Signature", signature)
		.body(body)
		.send()
		.await
		.unwrap();
	assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);
	assert!(event_receiver.recv_timeout(Duration::from_secs(1)).is_ok());
	assert!(service.is_running());

	service.stop();
}
