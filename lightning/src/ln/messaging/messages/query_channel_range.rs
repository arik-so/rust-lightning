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
		if let LightningMessageType::Hash(ref mut value) = placeholders[0] {
			*value = self.chain_hash
		}
		if let LightningMessageType::Int32(ref mut value) = placeholders[1] {
			*value = self.first_blocknum
		}
		if let LightningMessageType::Int32(ref mut value) = placeholders[2] {
			*value = self.number_of_blocks
		}
		if let LightningMessageType::TrailingBuffer(ref mut value) = placeholders[3] {
			*value = self.query_channel_range_tlvs.to_vec()
		}
	}

	fn from_field_array(fields: &[LightningMessageType]) -> Box<Self> {
		let chain_hash = fields[0].hash_value().unwrap();
		let first_blocknum = fields[1].int_32_value().unwrap();
		let number_of_blocks = fields[2].int_32_value().unwrap();
		let query_channel_range_tlvs = fields[3].trailing_buffer_value().unwrap();

		Box::new(QueryChannelRangeMessage {
			chain_hash,
			first_blocknum,
			number_of_blocks,
			query_channel_range_tlvs,
		})
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
