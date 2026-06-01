#![cfg(all(feature = "websocket", feature = "websocket-reverse"))]

use std::time::Duration;

use futures::{SinkExt, StreamExt};
use onebot_api::communication::utils::{
	CommunicationService, InternalAPIReceiver, InternalEventSender,
};
use onebot_api::communication::ws_reverse::WsReverseService;
use tokio::net::ToSocketAddrs;
use tokio::time::{sleep, timeout};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

fn reserve_addr() -> String {
	let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
	let addr = listener.local_addr().unwrap();
	drop(listener);
	addr.to_string()
}

async fn connect_with_retry(
	url: &str,
) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
	for _ in 0..40 {
		if let Ok((ws, _)) = connect_async(url).await {
			return ws;
		}
		sleep(Duration::from_millis(25)).await;
	}
	panic!("failed to connect to {url}");
}

async fn start_service<T: ToSocketAddrs + Clone + Send + Sync>(
	addr: T,
) -> (
	WsReverseService<T>,
	flume::Sender<onebot_api::communication::utils::APIRequest>,
) {
	let (api_sender, api_receiver): (
		flume::Sender<onebot_api::communication::utils::APIRequest>,
		InternalAPIReceiver,
	) = flume::bounded(8);
	let (event_sender, _event_receiver): (InternalEventSender, flume::Receiver<_>) =
		flume::bounded(8);
	let mut service = WsReverseService::new(addr, None);
	service.install(api_receiver, event_sender);
	service.start().await.unwrap();
	(service, api_sender)
}

#[tokio::test]
async fn reverse_ws_rejects_second_connection_and_recovers_after_disconnect() {
	let addr = reserve_addr();
	let url = format!("ws://{addr}");
	let (mut service, _api_sender) = start_service(addr.clone()).await;

	let mut first = connect_with_retry(&url).await;
	sleep(Duration::from_millis(50)).await;
	assert!(connect_async(&url).await.is_err());

	first.close(None).await.unwrap();
	drop(first);

	let _second = connect_with_retry(&url).await;
	service.stop();
}

#[tokio::test]
async fn reverse_ws_can_restart_after_stop() {
	let addr = reserve_addr();
	let url = format!("ws://{addr}");
	let (mut service, _api_sender) = start_service(addr.clone()).await;

	let mut ws = connect_with_retry(&url).await;
	service.stop();

	let _ = timeout(Duration::from_secs(1), ws.next()).await;
	service.start().await.unwrap();

	let _reconnected = connect_with_retry(&url).await;
	service.stop();
}

#[tokio::test]
async fn reverse_ws_cleans_up_after_invalid_text_and_disconnect() {
	let addr = reserve_addr();
	let url = format!("ws://{addr}");
	let (mut service, _api_sender) = start_service(addr.clone()).await;

	let mut ws = connect_with_retry(&url).await;
	ws.send(Message::Text("not json".into())).await.unwrap();
	ws.close(None).await.unwrap();
	drop(ws);

	let _reconnected = connect_with_retry(&url).await;
	service.stop();
}

#[tokio::test]
async fn reverse_ws_closes_when_api_channel_is_dropped() {
	let addr = reserve_addr();
	let url = format!("ws://{addr}");
	let (mut service, api_sender) = start_service(addr.clone()).await;

	let mut ws = connect_with_retry(&url).await;
	drop(api_sender);

	let closed = timeout(Duration::from_secs(1), ws.next()).await;
	assert!(closed.is_ok());

	let _reconnected = connect_with_retry(&url).await;
	service.stop();
}
