use crate::communication::ws_utils::WebSocketService;
use async_trait::async_trait;
use flume::{Receiver, Sender};

pub struct WsReverseService {}

#[async_trait]
impl WebSocketService for WsReverseService {
	fn register_api_receiver(&mut self, api_receiver: Receiver<String>) {
		todo!()
	}

	fn register_msg_sender(&mut self, msg_sender: Sender<String>) {
		todo!()
	}

	async fn start(&self) -> anyhow::Result<()> {
		todo!()
	}
}
