use ln::messaging::messages::LightningMessageId;
use ln::messaging::types::LightningMessageType;

pub trait Serde {
	fn id() -> LightningMessageId;
	fn placeholder_field_array() -> Vec<LightningMessageType>;
	fn fill_field_array(&self, placeholders: &mut [LightningMessageType]);
	fn from_field_array(fields: &mut Vec<LightningMessageType>) -> Box<Self>;

	fn to_field_array(&self) -> Vec<LightningMessageType> {
		let mut fields = Self::placeholder_field_array();
		self.fill_field_array(&mut fields[..]);
		fields
	}

	fn serialize(&self) -> Vec<u8> {
		let fields = self.to_field_array();
		let mut buffer = Vec::new();

		let id = Self::id() as u16;
		let id_bytes: [u8; 2] = id.to_be_bytes();
		buffer.extend(&id_bytes);

		// add data
		for field in fields {
			let extension = match field {
				LightningMessageType::TrailingBuffer(data) => {
					data
				}
				LightningMessageType::Byte(value) => {
					vec![value]
				}
				LightningMessageType::Int16(integer) => {
					let bytes: [u8; 2] = integer.to_be_bytes();
					bytes.to_vec()
				}
				LightningMessageType::Int32(integer) => {
					let bytes: [u8; 4] = integer.to_be_bytes();
					bytes.to_vec()
				}
				LightningMessageType::Int64(integer) => {
					let bytes: [u8; 8] = integer.to_be_bytes();
					bytes.to_vec()
				}
				LightningMessageType::Color(color) => {
					color.to_vec()
				}
				LightningMessageType::ShortChannelId(data) => {
					data.to_vec()
				}
				LightningMessageType::Hash(data) => {
					data.to_vec()
				}
				LightningMessageType::Point(data) => {
					data.to_vec()
				}
				LightningMessageType::Signature(data) => {
					data.to_vec()
				}
				LightningMessageType::LengthAnnotatedBuffer(data) => {
					let length_bytes: [u8; 2] = (data.len() as u16).to_be_bytes();
					let mut extension = length_bytes.to_vec();
					extension.extend(data);
					extension
				}
			};
			buffer.extend(extension);
		}
		buffer
	}

	fn parse(buffer: &[u8]) -> Box<Self> {
		// read the elements
		let mut placeholder_fields = Self::placeholder_field_array();
		let mut index = 2;
		for field in placeholder_fields.iter_mut() {
			match field {
				LightningMessageType::Int16(integer) => {
					let current_bytes = &buffer[index..index + 2];
					index += 2;

					let mut be_bytes = [0; 2];
					be_bytes.copy_from_slice(current_bytes);

					*integer = u16::from_be_bytes(be_bytes);
				}
				LightningMessageType::Int32(integer) => {
					let current_bytes = &buffer[index..index + 4];
					index += 4;

					let mut be_bytes = [0; 4];
					be_bytes.copy_from_slice(current_bytes);

					*integer = u32::from_be_bytes(be_bytes);
				}
				LightningMessageType::Int64(integer) => {
					let current_bytes = &buffer[index..index + 8];
					index += 8;

					let mut be_bytes = [0; 8];
					be_bytes.copy_from_slice(current_bytes);

					*integer = u64::from_be_bytes(be_bytes);
				}
				LightningMessageType::Byte(integer) => {
					let current_byte = buffer[index];
					index += 1;

					*integer = current_byte;
				}
				LightningMessageType::Color(color) => {
					let current_bytes = &buffer[index..index + 3];
					index += 3;

					let mut bytes = [0; 3];
					bytes.copy_from_slice(current_bytes);

					*color = bytes;
				}
				LightningMessageType::ShortChannelId(channel) => {
					let current_bytes = &buffer[index..index + 8];
					index += 8;

					let mut bytes = [0; 8];
					bytes.copy_from_slice(current_bytes);

					*channel = bytes;
				}
				LightningMessageType::Hash(hash) => {
					let current_bytes = &buffer[index..index + 32];
					index += 32;

					let mut bytes = [0; 32];
					bytes.copy_from_slice(current_bytes);

					*hash = bytes;
				}
				LightningMessageType::Point(point) => {
					let current_bytes = &buffer[index..index + 33];
					index += 33;

					let mut bytes = [0; 33];
					bytes.copy_from_slice(current_bytes);

					*point = bytes;
				}
				LightningMessageType::Signature(signature) => {
					let current_bytes = &buffer[index..index + 64];
					index += 64;

					let mut bytes = [0; 64];
					bytes.copy_from_slice(current_bytes);

					*signature = bytes;
				}
				LightningMessageType::LengthAnnotatedBuffer(data) => {
					let current_bytes = &buffer[index..index + 2];
					index += 2;

					let mut be_bytes = [0; 2];
					be_bytes.copy_from_slice(current_bytes);

					let length = u16::from_be_bytes(be_bytes);
					*data = (&buffer[index..index + length as usize]).to_vec();
					index += length as usize;
				}
				LightningMessageType::TrailingBuffer(data) => {
					let bytes = &buffer[index..];
					index += bytes.len();
					*data = bytes.to_vec();
				}
			};
		};
		let instance = Self::from_field_array(&mut placeholder_fields);
		assert_eq!(placeholder_fields.len(), 0);
		instance
	}
}

#[cfg(test)]
mod tests {
	use ln::messaging::messages::{LightningMessage, Ping};
	use ln::messaging::serde::Serde;

	#[test]
	fn codec_works() {
		let message = Ping {
			num_pong_bytes: 290,
			ignored: vec![0, 0, 0, 0],
		};
		let serialization = message.serialize();
		println!("extension: {:?}", serialization);
		let deserialization = Ping::parse(&serialization);
		println!("recovery: {:#?}", deserialization);
		assert_eq!(deserialization.num_pong_bytes, 290);
		assert_eq!(deserialization.ignored.len(), 4);
		receive_lightning_message(LightningMessage::Ping(*deserialization));
		let smart_parse = LightningMessage::parse(&serialization);
		println!("recovery: {:#?}", smart_parse);
	}

	#[test]
	fn test_send_arbitrary_message() {
		let message = Ping {
			num_pong_bytes: 290,
			ignored: vec![0, 0, 0, 0],
		};
		let bytes = serialize_message_send(&message);
		assert_eq!(bytes.len(), 10);
	}

	fn serialize_message_send(message: &impl Serde) -> Vec<u8>{
		message.serialize()
	}

	pub fn receive_lightning_message(message: LightningMessage) {
		match message {
			LightningMessage::Ping(ping) => {
				println!("It's a ping message! {:?}", ping);
			}
			LightningMessage::Pong(pong) => {
				println!("It's a pong message! {:?}", pong);
			}
			_ => {}
		};
	}
}