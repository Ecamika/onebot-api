pub struct Selector<'a, T> {
	pub(crate) data: Option<&'a T>,
}

impl<'a, T> Selector<'a, T> {
	pub fn map<R>(&self, handler: impl FnOnce(&'a T) -> R) -> Option<R> {
		self.data.map(handler)
	}

	pub async fn map_async<R>(&self, handler: impl AsyncFnOnce(&'a T) -> R) -> Option<R> {
		if let Some(data) = self.data {
			Some(handler(data).await)
		} else {
			None
		}
	}

	pub fn select(&self) -> Option<&'a T> {
		self.data
	}

	pub fn is_matched(&self) -> bool {
		self.data.is_some()
	}
}
