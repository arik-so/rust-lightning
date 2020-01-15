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

	/*pub fn int_16_value_mut(&self) -> Result<&mut u16, Box<dyn std::error::Error>> {
		match *self {
			LightningMessageType::Int16(mut value) => {
				Ok(&mut value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}*/


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

	/*pub fn length_annotated_buffer_value_mut(self) -> Result<&mut Vec<u8>, Box<dyn std::error::Error>> {
		match self {
			LightningMessageType::LengthAnnotatedBuffer(mut value) => {
				Ok(&mut value)
			}
			_ => {
				Err(Box::new(UnexpectedTypeError {}))
			}
		}
	}*/
}