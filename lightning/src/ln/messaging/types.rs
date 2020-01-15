use ln::messaging::errors::UnexpectedTypeError;

pub enum LightningMessageType {
	Int16(u16),
	Int32(u32),
	Int64(u64),
	Hash([u8; 32]),
	Point([u8; 33]),
	Signature([u8; 64]),
	Byte(u8),
	Color([u8; 3]),
	ShortChannelId([u8; 8]),
	LengthAnnotatedBuffer(Vec<u8>),
	TrailingBuffer(Vec<u8>),
}

impl LightningMessageType {
	pub fn int_16_value(&self) -> Result<u16, Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Int16(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn int_32_value(&self) -> Result<u32, Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Int32(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn int_64_value(&self) -> Result<u64, Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Int64(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn hash_value(&self) -> Result<[u8; 32], Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Hash(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn point_value(&self) -> Result<[u8; 33], Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Point(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn signature_value(&self) -> Result<[u8; 64], Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Signature(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn byte_value(&self) -> Result<u8, Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Byte(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn color_value(&self) -> Result<[u8; 3], Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Color(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn short_channel_id_value(&self) -> Result<[u8; 8], Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::ShortChannelId(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn length_annotated_buffer_value(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::LengthAnnotatedBuffer(value) => {
				Ok(value.to_owned())
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn trailing_buffer_value(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::TrailingBuffer(value) => {
				Ok(value.to_owned())
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}
}