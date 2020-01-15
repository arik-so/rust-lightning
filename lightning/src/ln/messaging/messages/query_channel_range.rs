use ln::messaging::messages::LightningMessageId;
use ln::messaging::serde::Serde;
use ln::messaging::types::LightningMessageType;

#[derive(Debug)]
pub struct QueryChannelRangeMessage {
	pub chain_hash: [u8; 32],
	pub first_blocknum: u32,
	pub number_of_blocks: u32,
	pub query_channel_range_tlvs: Vec<u8>,
}

impl Serde for QueryChannelRangeMessage {
	fn id() -> LightningMessageId {
		LightningMessageId::QueryChannelRange
	}

	fn placeholder_field_array() -> Vec<LightningMessageType> {
		vec![
			LightningMessageType::Hash([0; 32]),
			LightningMessageType::Int32(0),
			LightningMessageType::Int32(0),
			LightningMessageType::TrailingBuffer(Vec::new())
		]
	}

	fn fill_field_array(&self, placeholders: &mut [LightningMessageType]) {
		unimplemented!()
	}

	fn from_field_array(fields: &[LightningMessageType]) -> Box<Self> {
		let num_pong_bytes = fields[0].int_16_value().unwrap();
		let ignored = fields[1].length_annotated_buffer_value().unwrap();

//		Box::new(QueryChannelRangeMessage {
//			num_pong_bytes,
//			ignored,
//		})
		unimplemented!();
	}

	fn to_field_array(&self) -> Vec<LightningMessageType> {
		let mut fields = Vec::new();
		fields.push(LightningMessageType::Hash(self.chain_hash));
		fields.push(LightningMessageType::Int32(self.first_blocknum));
		fields.push(LightningMessageType::Int32(self.number_of_blocks));
		fields.push(LightningMessageType::TrailingBuffer(self.query_channel_range_tlvs.clone()));
		fields
	}
}
