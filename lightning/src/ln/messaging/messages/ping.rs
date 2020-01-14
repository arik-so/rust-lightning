use ln::messaging::messages::LightningMessageId;
use ln::messaging::Serialize;
use ln::messaging::types::LightningMessageType;

#[derive(Debug)]
pub struct PingMessage {
	pub num_pong_bytes: u16,
	pub ignored: Vec<u8>,
}

impl Serialize for PingMessage {
	fn id() -> LightningMessageId {
		LightningMessageId::Ping
	}

	fn placeholder_field_array() -> Vec<LightningMessageType> {
		vec![
			LightningMessageType::Int16(0),
			LightningMessageType::LengthAnnotatedBuffer(Vec::new())
		]
	}

	fn to_field_array(&self) -> Vec<LightningMessageType> {
		let mut fields = Vec::new();
		fields.push(LightningMessageType::Int16(self.num_pong_bytes));
		fields.push(LightningMessageType::LengthAnnotatedBuffer(self.ignored.clone()));
		fields
	}

	fn from_field_array(fields: &[LightningMessageType]) -> Box<Self> {
		let num_pong_bytes = fields[0].int_16_value().unwrap();
		let ignored = fields[1].length_annotated_buffer_value().unwrap();

		Box::new(PingMessage {
			num_pong_bytes,
			ignored,
		})
	}
}