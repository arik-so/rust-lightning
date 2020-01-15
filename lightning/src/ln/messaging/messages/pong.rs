use ln::messaging::messages::LightningMessageId;
use ln::messaging::serde::Serde;
use ln::messaging::types::LightningMessageType;

#[derive(Debug)]
pub struct PongMessage {
	pub ignored: Vec<u8>,
}

impl Serde for PongMessage {
	fn id() -> LightningMessageId {
		LightningMessageId::Pong
	}

	fn placeholder_field_array() -> Vec<LightningMessageType> {
		vec![
			LightningMessageType::LengthAnnotatedBuffer(Vec::new())
		]
	}

	fn fill_field_array(&self, placeholders: &mut [LightningMessageType]) {
		if let LightningMessageType::LengthAnnotatedBuffer(ref mut value) = placeholders[0] {
			*value = self.ignored.to_vec();
		}
	}

	fn from_field_array(fields: &[LightningMessageType]) -> Box<Self> {
		let ignored = fields[1].length_annotated_buffer_value().unwrap();

		Box::new(PongMessage {
			ignored
		})
	}
}