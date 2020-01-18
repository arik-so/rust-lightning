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
	pub fn into_int16(self) -> Result<u16, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::Int16(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_int32(self) -> Result<u32, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::Int32(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_int64(self) -> Result<u64, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::Int64(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_hash(self) -> Result<[u8; 32], Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::Hash(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_point(self) -> Result<[u8; 33], Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::Point(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_signature(self) -> Result<[u8; 64], Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::Signature(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_byte(self) -> Result<u8, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::Byte(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_color(self) -> Result<[u8; 3], Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::Color(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_short_channel_id(self) -> Result<[u8; 8], Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::ShortChannelId(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_length_annotated_buffer(self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::LengthAnnotatedBuffer(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}

	pub fn into_trailing_buffer(self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::TrailingBuffer(value) => {
				Ok(value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}
}