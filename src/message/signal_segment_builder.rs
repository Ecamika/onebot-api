use super::segment_builder::SegmentBuilder;
use super::send_segment::*;
use super::utils::*;

pub struct ImageDataBuilder {
	raw: SegmentBuilder,
	data: ImageData,
}

impl ImageDataBuilder {
	pub fn new(raw: SegmentBuilder, file: impl ToString) -> Self {
		Self {
			raw,
			data: ImageData {
				file: file.to_string(),
				image_type: None,
				cache: None,
				proxy: None,
				timeout: None,
			},
		}
	}

	pub fn build(self) -> SegmentBuilder {
		self.raw.and_push(self.data)
	}

	pub fn image_type(mut self, image_type: ImageType) -> Self {
		self.data.image_type = Some(image_type);
		self
	}

	pub fn cache(mut self, cache: bool) -> Self {
		self.data.cache = Some(cache);
		self
	}

	pub fn proxy(mut self, proxy: bool) -> Self {
		self.data.proxy = Some(proxy);
		self
	}

	pub fn timeout(mut self, timeout: i64) -> Self {
		self.data.timeout = Some(timeout);
		self
	}
}
