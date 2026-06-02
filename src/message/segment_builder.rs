use super::send_segment::*;
use super::signal_segment_builder::*;
use super::utils::*;

#[derive(Default, Debug, Clone)]
pub struct SegmentBuilder {
	segments: Vec<SendSegment>,
}

impl From<Vec<SendSegment>> for SegmentBuilder {
	fn from(value: Vec<SendSegment>) -> Self {
		Self { segments: value }
	}
}

impl SegmentBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn build(self) -> Vec<SendSegment> {
		self.segments
	}

	pub fn push(&mut self, segment: impl SendSegmentData) {
		self.segments.push(segment.into_send_segment());
	}

	pub fn and_push(mut self, segment: impl SendSegmentData) -> Self {
		self.segments.push(segment.into_send_segment());
		self
	}

	pub fn text(self, text: impl ToString) -> Self {
		self.and_push(TextData {
			text: text.to_string(),
		})
	}

	pub fn face(self, id: impl ToString) -> Self {
		self.and_push(FaceData { id: id.to_string() })
	}

	pub fn image_with_options(
		self,
		file: impl ToString,
		image_type: Option<ImageType>,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<i64>,
	) -> Self {
		self.and_push(ImageData {
			file: file.to_string(),
			image_type,
			cache,
			proxy,
			timeout,
		})
	}

	pub fn image(self, file: impl ToString) -> Self {
		self.image_with_options(file, None, None, None, None)
	}

	pub fn image_builder(self, file: impl ToString) -> ImageDataBuilder {
		ImageDataBuilder::new(self, file)
	}

	pub fn record_with_options(
		self,
		file: impl ToString,
		magic: Option<String>,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<i64>,
	) -> Self {
		self.and_push(RecordData {
			file: file.to_string(),
			magic,
			cache,
			proxy,
			timeout,
		})
	}

	pub fn record(self, file: impl ToString) -> Self {
		self.record_with_options(file, None, None, None, None)
	}

	pub fn video_with_options(
		self,
		file: impl ToString,
		cache: Option<bool>,
		proxy: Option<bool>,
		timeout: Option<i64>,
	) -> Self {
		self.and_push(VideoData {
			file: file.to_string(),
			cache,
			proxy,
			timeout,
		})
	}

	pub fn video(self, file: impl ToString) -> Self {
		self.video_with_options(file, None, None, None)
	}

	pub fn at(self, qq: AtType) -> Self {
		self.and_push(AtData { qq })
	}

	pub fn at_all(self) -> Self {
		self.at(AtType::All)
	}

	pub fn at_id(self, id: String) -> Self {
		self.at(AtType::Id(id))
	}

	pub fn rps(self) -> Self {
		self.and_push(RpsData {})
	}

	pub fn dice(self) -> Self {
		self.and_push(DiceData {})
	}

	pub fn shake(self) -> Self {
		self.and_push(ShakeData {})
	}

	pub fn poke(self, poke_type: impl ToString, id: impl ToString) -> Self {
		self.and_push(PokeData {
			poke_type: poke_type.to_string(),
			id: id.to_string(),
		})
	}

	pub fn anonymous(self, ignore: Option<bool>) -> Self {
		self.and_push(AnonymousData { ignore })
	}

	pub fn share(
		self,
		url: impl ToString,
		title: impl ToString,
		content: impl ToString,
		image: impl ToString,
	) -> Self {
		self.and_push(ShareData {
			url: url.to_string(),
			title: title.to_string(),
			content: content.to_string(),
			image: image.to_string(),
		})
	}

	pub fn contact(self, contact_type: ContactType, id: impl ToString) -> Self {
		self.and_push(ContactData {
			contact_type,
			id: id.to_string(),
		})
	}

	pub fn contact_qq(self, id: impl ToString) -> Self {
		self.contact(ContactType::QQ, id)
	}

	pub fn contact_group(self, id: impl ToString) -> Self {
		self.contact(ContactType::Group, id)
	}

	pub fn location(
		self,
		lat: impl ToString,
		lon: impl ToString,
		title: Option<String>,
		content: Option<String>,
	) -> Self {
		self.and_push(LocationData {
			lat: lat.to_string(),
			lon: lon.to_string(),
			title,
			content,
		})
	}

	#[allow(clippy::too_many_arguments)]
	pub fn music(
		self,
		music_type: MusicType,
		id: Option<String>,
		url: Option<String>,
		audio: Option<String>,
		title: Option<String>,
		content: Option<String>,
		image: Option<String>,
	) -> Self {
		self.and_push(MusicData {
			music_type,
			id,
			url,
			audio,
			title,
			content,
			image,
		})
	}

	pub fn music_typed(self, music_type: MusicType, id: impl ToString) -> Self {
		self.music(
			music_type,
			Some(id.to_string()),
			None,
			None,
			None,
			None,
			None,
		)
	}

	pub fn music_qq(self, id: impl ToString) -> Self {
		self.music_typed(MusicType::QQ, id)
	}

	pub fn music_163(self, id: impl ToString) -> Self {
		self.music_typed(MusicType::NetEaseCloudMusic, id)
	}

	pub fn music_xm(self, id: impl ToString) -> Self {
		self.music_typed(MusicType::Xm, id)
	}

	pub fn music_custom(
		self,
		url: impl ToString,
		audio: impl ToString,
		title: impl ToString,
		content: impl ToString,
		image: impl ToString,
	) -> Self {
		self.music(
			MusicType::Custom,
			None,
			Some(url.to_string()),
			Some(audio.to_string()),
			Some(title.to_string()),
			Some(content.to_string()),
			Some(image.to_string()),
		)
	}

	pub fn reply(self, id: impl ToString) -> Self {
		self.and_push(ReplyData { id: id.to_string() })
	}

	pub fn forward(self) -> Self {
		self.and_push(ForwardData {})
	}

	pub fn node(
		self,
		id: Option<String>,
		user_id: Option<String>,
		nickname: Option<String>,
		content: Option<Vec<SendSegment>>,
	) -> Self {
		self.and_push(NodeData {
			id,
			user_id,
			nickname,
			content,
		})
	}

	pub fn node_forward(self, id: impl ToString) -> Self {
		self.node(Some(id.to_string()), None, None, None)
	}

	pub fn node_custom(
		self,
		user_id: impl ToString,
		nickname: impl ToString,
		content: Vec<SendSegment>,
	) -> Self {
		self.node(
			None,
			Some(user_id.to_string()),
			Some(nickname.to_string()),
			Some(content),
		)
	}

	pub fn xml(self, data: impl ToString) -> Self {
		self.and_push(XmlData {
			data: data.to_string(),
		})
	}

	pub fn json(self, data: impl ToString) -> Self {
		self.and_push(JsonData {
			data: data.to_string(),
		})
	}
}

#[macro_export]
macro_rules! text {
	( $( $arg: tt )* ) => {
		{
			$crate::message::segment_builder::SegmentBuilder::new().text( format!( $( $arg )* ) ).build()
		}
	};
}
