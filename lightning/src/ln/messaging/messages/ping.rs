use ln::messaging::messages::LightningMessageId;
use ln::messaging::serde::Serde;
use ln::messaging::types::LightningMessageType;

#[derive(Debug)]
pub struct PingMessage {
	pub num_pong_bytes: u16,
	pub ignored: Vec<u8>,
}

impl Serde for PingMessage {
	fn id() -> LightningMessageId {
		LightningMessageId::Ping
	}

	fn placeholder_field_array() -> Vec<LightningMessageType> {
		vec![
			LightningMessageType::Int16(0),
			LightningMessageType::LengthAnnotatedBuffer(Vec::new())
		]
	}

	fn fill_field_array(&self, placeholders: &mut [LightningMessageType]) {
		if let LightningMessageType::Int16(ref mut value) = placeholders[0] {
			*value = self.num_pong_bytes;
		}
		if let LightningMessageType::LengthAnnotatedBuffer(ref mut value) = placeholders[1] {
			*value = self.ignored.to_vec();
		}
	}

	fn from_field_array(fields: &mut Vec<LightningMessageType>) -> Box<Self> {
		let num_pong_bytes = fields.remove(0).into_int16().unwrap();
		let ignored = fields.remove(0).into_length_annotated_buffer().unwrap();

		Box::new(Self {
			num_pong_bytes,
			ignored,
		})
	}
}