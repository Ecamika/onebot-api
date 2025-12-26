use crate::event::{Event, EventReceiver};
use axum::body::Bytes;
use axum::handler::HandlerWithoutStateExt;
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::sync::mpsc;

pub struct HttpPostServer {}

impl HttpPostServer {
	// pub async fn new(addr: impl ToSocketAddrs) -> anyhow::Result<Self> {
	// 	let tcp_listener = TcpListener::bind(addr).await?;
	// 	let (tx, mut rx) = mpsc::unbounded_channel();
	// 	let router = Router::new().route("/", post(
	// 		async move |headers: HeaderMap, body: Bytes| {
	// 			let body_str = String::from_utf8(body.to_vec()).unwrap();
	// 			let event: Event = serde_json::from_str(&body_str).unwrap();
	// 			tx.send(event).unwrap();
	// 		}
	// 	));
	// 	tokio::spawn(async move {
	// 		let data = rx.recv().await.unwrap();
	// 		println!("receive data: {data:?}");
	// 	});
	// 	axum::serve(tcp_listener, router.into_make_service()).await?;
	// 	Ok(Self {})
	// }
}

// impl EventReceiver for HttpPostServer {}
