use ln::messaging::messages::LightningMessageId;
use ln::messaging::serde::Serde;
use ln::messaging::types::LightningMessageType;

#[derive(Debug)]
pub struct PongMessage {
	pub byteslen: Option<u16>,
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
		unimplemented!()
	}

	fn from_field_array(fields: &[LightningMessageType]) -> Box<Self> {
		let ignored = fields[1].length_annotated_buffer_value().unwrap();

		Box::new(PongMessage {
			byteslen: Some(ignored.len() as u16),
			ignored,
		})
	}

	fn to_field_array(&self) -> Vec<LightningMessageType> {
		let mut fields = Vec::new();
		fields.push(LightningMessageType::LengthAnnotatedBuffer(self.ignored.clone()));
		fields
	}
}