use crate::event::{Event, EventMessage, EventMetaEvent, EventNotice, EventRequest};

pub struct Selector<'a, T> {
	pub data: Option<&'a T>,
}

impl<'a, T> Selector<'a, T> {
	pub fn select<R>(&self, handler: impl FnOnce(&'a T) -> R) -> Option<R> {
		self.data.map(handler)
	}

	pub async fn select_async<R>(&self, handler: impl AsyncFnOnce(&'a T) -> R) -> Option<R> {
		if let Some(data) = self.data {
			Some(handler(data).await)
		} else {
			None
		}
	}
}

impl<'a> Selector<'a, Event> {
	pub fn message(&self) -> Selector<'a, EventMessage> {
		Selector {
			data: self.data.and_then(|d| d.match_message()),
		}
	}

	pub fn notice(&self) -> Selector<'a, EventNotice> {
		Selector {
			data: self.data.and_then(|d| d.match_notice()),
		}
	}

	pub fn request(&self) -> Selector<'a, EventRequest> {
		Selector {
			data: self.data.and_then(|d| d.match_request()),
		}
	}

	pub fn meta_event(&self) -> Selector<'a, EventMetaEvent> {
		Selector {
			data: self.data.and_then(|d| d.match_meta_event()),
		}
	}
}
