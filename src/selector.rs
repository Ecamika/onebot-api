pub trait AsSelector<'a, T> {
	fn as_selector(&'a self) -> Selector<'a, T>;
}

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

	pub const fn select(&self) -> Option<&'a T> {
		self.data
	}

	pub const fn is_matched(&self) -> bool {
		self.data.is_some()
	}

	pub fn filter(&mut self, f: impl FnOnce(&T) -> bool) {
		if let Some(data) = self.data
			&& !f(data)
		{
			self.data = None
		}
	}

	pub fn and_filter(mut self, f: impl FnOnce(&T) -> bool) -> Self {
		self.filter(f);
		self
	}

	pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&T) -> bool) {
		if let Some(data) = self.data
			&& !f(data).await
		{
			self.data = None
		}
	}

	pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&T) -> bool) -> Self {
		self.filter_async(f).await;
		self
	}
}
