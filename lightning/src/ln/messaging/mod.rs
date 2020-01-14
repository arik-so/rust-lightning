use ln::messaging::types::LightningMessageType;

mod errors;
mod messages;
mod types;

pub trait Serialize {
	fn placeholder_field_array() -> Vec<LightningMessageType>;
	fn to_field_array(&self) -> Vec<LightningMessageType>;
	fn from_field_array(fields: &[LightningMessageType]) -> Box<Self>;

	fn serialize(&self) -> Vec<u8> {
		let fields = self.to_field_array();
		let mut buffer = Vec::new();

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
		let mut index = 0;
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
		Self::from_field_array(&placeholder_fields)
	}
}