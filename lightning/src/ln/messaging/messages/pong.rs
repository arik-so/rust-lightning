use ln::messaging::Serialize;
use ln::messaging::types::LightningMessageType;

#[derive(Debug)]
pub struct PongMessage {
	pub byteslen: Option<u16>,
	pub ignored: Vec<u8>,
}

impl Serialize for PongMessage {
	fn placeholder_field_array() -> Vec<LightningMessageType> {
		vec![
			LightningMessageType::LengthAnnotatedBuffer(Vec::new())
		]
	}

	fn to_field_array(&self) -> Vec<LightningMessageType> {
		let mut fields = Vec::new();
		fields.push(LightningMessageType::LengthAnnotatedBuffer(self.ignored.clone()));
		fields
	}

	fn from_field_array(fields: &[LightningMessageType]) -> Box<Self> {
		let ignored = fields[1].length_annotated_buffer_value().unwrap();

		Box::new(PongMessage {
			byteslen: Some(ignored.len() as u16),
			ignored,
		})
	}
}