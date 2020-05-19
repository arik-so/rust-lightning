use lightning::ln::chan_utils::{HTLCOutputInCommitment as RawHTLCOutputInCommitment, HTLCOutputInCommitment};
use crate::buffer::BufferResponse;
use std::slice;
use lightning::ln::channelmanager::PaymentHash;

#[no_mangle]
pub struct HTLCOutputInCommitmentResponse {
	pub offered: bool,
	pub amount_msat: u64,
	pub cltv_expiry: u32,
	pub payment_hash: BufferResponse,
	pub transaction_output_index: i64, // signed i64, -1 means not set
}

#[no_mangle]
pub struct HTLCOutputInCommitmentArgument {
	pub offered: bool,
	pub amount_msat: u64,
	pub cltv_expiry: u32,
	pub payment_hash: *const u8,
	pub transaction_output_index: i64, // signed i64, -1 means not set
}

impl From<RawHTLCOutputInCommitment> for HTLCOutputInCommitmentResponse {
	fn from(commitment: RawHTLCOutputInCommitment) -> Self {
		let RawHTLCOutputInCommitment {
			offered,
			amount_msat,
			cltv_expiry,
			payment_hash,
			transaction_output_index
		} = commitment;
		let payment_hash: BufferResponse = payment_hash.0.to_vec().into();
		let transaction_output_index: i64 = match transaction_output_index {
			None => { -1 }
			Some(index) => { index as i64 }
		};
		HTLCOutputInCommitmentResponse {
			offered,
			amount_msat,
			cltv_expiry,
			payment_hash,
			transaction_output_index,
		}
	}
}

impl From<&&RawHTLCOutputInCommitment> for HTLCOutputInCommitmentResponse {
	fn from(commitment: &&RawHTLCOutputInCommitment) -> Self {
		let RawHTLCOutputInCommitment {
			offered,
			amount_msat,
			cltv_expiry,
			payment_hash,
			transaction_output_index
		} = commitment;
		let payment_hash: BufferResponse = payment_hash.0.to_vec().into();
		let transaction_output_index: i64 = match transaction_output_index {
			None => { -1 }
			Some(index) => { *index as i64 }
		};
		HTLCOutputInCommitmentResponse {
			offered: *offered,
			amount_msat: *amount_msat,
			cltv_expiry: *cltv_expiry,
			payment_hash,
			transaction_output_index,
		}
	}
}

impl From<HTLCOutputInCommitmentArgument> for RawHTLCOutputInCommitment {
	fn from(commitment: HTLCOutputInCommitmentArgument) -> Self {
		let HTLCOutputInCommitmentArgument {
			offered,
			amount_msat,
			cltv_expiry,
			payment_hash,
			transaction_output_index
		} = commitment;

		let payment_hash_slice = unsafe {
			assert!(!payment_hash.is_null());
			slice::from_raw_parts(payment_hash, 32)
		};
		let mut payment_hash = [0u8; 32];
		payment_hash.copy_from_slice(payment_hash_slice);

		let transaction_output_index = match transaction_output_index {
			d if d < 0 => {
				None
			}
			index => {
				Some(index as u32)
			}
		};

		RawHTLCOutputInCommitment {
			offered,
			amount_msat,
			cltv_expiry,
			payment_hash: PaymentHash(payment_hash),
			transaction_output_index,
		}
	}
}

#[no_mangle]
pub struct HTLCOutputInCommitmentResponseArray {
	htlc_outputs_in_commitments: *const HTLCOutputInCommitmentResponse,
	length: usize,
}

impl From<&[&RawHTLCOutputInCommitment]> for HTLCOutputInCommitmentResponseArray {
	fn from(commitment_outputs: &[&HTLCOutputInCommitment]) -> Self {
		let mut outputs_in_commitments = Vec::new();
		for current_output in commitment_outputs.iter() {
			let current_commitment: HTLCOutputInCommitmentResponse = current_output.into();
			outputs_in_commitments.push(current_commitment);
		}

		let htlc_outputs_in_commitments = outputs_in_commitments.as_ptr();
		let length = outputs_in_commitments.len();
		Self { htlc_outputs_in_commitments, length }
	}
}
