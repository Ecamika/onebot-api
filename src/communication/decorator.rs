use crate::communication::utils::{Client, PublicEventReceiver};
use crate::event::Event;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

pub struct EventBroadcastDecorator {
	client: Client,
	broadcast_sender: broadcast::Sender<Arc<Event>>,
	processor_handle: JoinHandle<()>,
}

impl Deref for EventBroadcastDecorator {
	type Target = Client;

	fn deref(&self) -> &Self::Target {
		&self.client
	}
}

impl DerefMut for EventBroadcastDecorator {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.client
	}
}

impl Drop for EventBroadcastDecorator {
	fn drop(&mut self) {
		self.processor_handle.abort()
	}
}

impl EventBroadcastDecorator {
	async fn processor(
		event_receiver: PublicEventReceiver,
		broadcast_sender: broadcast::Sender<Arc<Event>>,
	) {
		loop {
			if let Ok(e) = event_receiver.recv_async().await {
				let _ = broadcast_sender.send(Arc::new(e));
			}
		}
	}

	pub fn new(client: Client, broadcast_cap: usize) -> Self {
		let event_receiver = client.get_normal_event_receiver();
		let (broadcast_sender, _) = broadcast::channel(broadcast_cap);

		let processor_handle = tokio::spawn(Self::processor(event_receiver, broadcast_sender.clone()));

		Self {
			client,
			broadcast_sender,
			processor_handle,
		}
	}

	pub fn subscribe(&self) -> broadcast::Receiver<Arc<Event>> {
		self.broadcast_sender.subscribe()
	}
}
