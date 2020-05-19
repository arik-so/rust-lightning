use crate::buffer::{BufferArgument, BufferResponse, BufferArgumentArray};

use bitcoin::consensus::{deserialize, serialize};
use bitcoin::Transaction as RawTransaction;
use bitcoin::secp256k1::Signature as RawSignature;

impl From<BufferArgument> for RawTransaction {
	fn from(tx_buffer: BufferArgument) -> Self {
		let tx_vec = unsafe { tx_buffer.to_vec() };
		let tx: RawTransaction = deserialize(&tx_vec).unwrap();
		tx
	}
}

impl From<&RawTransaction> for BufferResponse {
	fn from(tx: &RawTransaction) -> Self {
		let serialization = serialize(tx);
		let buffer: BufferResponse = serialization.into();
		buffer
	}
}

impl From<RawTransaction> for BufferResponse {
	fn from(tx: RawTransaction) -> Self {
		let serialization = serialize(&tx);
		let buffer: BufferResponse = serialization.into();
		buffer
	}
}

impl From<RawSignature> for BufferResponse {
	fn from(signature: RawSignature) -> Self {
		let signature = signature.serialize_der().to_vec();
		let buffer: BufferResponse = signature.into();
		buffer
	}
}

impl From<BufferArgument> for RawSignature {
	fn from(signature_buffer: BufferArgument) -> Self {
		let signature_vec = unsafe { signature_buffer.to_vec() };
		let signature = RawSignature::from_der(&signature_vec);
		signature.unwrap()
	}
}

impl From<BufferArgumentArray> for Vec<RawSignature>{
	fn from(signature_buffers: BufferArgumentArray) -> Self {
		let signature_byte_array = unsafe { signature_buffers.to_vec() };
		let mut signatures = Vec::new();
		for current_buffer in signature_byte_array.iter() {
			let current_signature = RawSignature::from_der(&current_buffer);
			signatures.push(current_signature.unwrap());
		};
		signatures
	}
}
