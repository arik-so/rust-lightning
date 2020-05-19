use lightning::ln::chan_utils::HTLCOutputInCommitment as RawHTLCOutputInCommitment;
use crate::buffer::BufferResponse;
use std::slice;
use lightning::ln::channelmanager::PaymentHash;

#[no_mangle]
pub struct HTLCOutputInCommitmentResponse {
	/// Whether the HTLC was "offered" (ie outbound in relation to this commitment transaction).
	/// Note that this is not the same as whether it is ountbound *from us*. To determine that you
	/// need to compare this value to whether the commitment transaction in question is that of
	/// the remote party or our own.
	pub offered: bool,
	/// The value, in msat, of the HTLC. The value as it appears in the commitment transaction is
	/// this divided by 1000.
	pub amount_msat: u64,
	/// The CLTV lock-time at which this HTLC expires.
	pub cltv_expiry: u32,
	/// The hash of the preimage which unlocks this HTLC.
	pub payment_hash: BufferResponse,
	// 32 bytes
	/// The position within the commitment transactions' outputs. This may be None if the value is
	/// below the dust limit (in which case no output appears in the commitment transaction and the
	/// value is spent to additional transaction fees).
	pub transaction_output_index: i64, // signed i64, -1 means not set
}

#[no_mangle]
pub struct HTLCOutputInCommitmentArgument {
	/// Whether the HTLC was "offered" (ie outbound in relation to this commitment transaction).
	/// Note that this is not the same as whether it is ountbound *from us*. To determine that you
	/// need to compare this value to whether the commitment transaction in question is that of
	/// the remote party or our own.
	pub offered: bool,
	/// The value, in msat, of the HTLC. The value as it appears in the commitment transaction is
	/// this divided by 1000.
	pub amount_msat: u64,
	/// The CLTV lock-time at which this HTLC expires.
	pub cltv_expiry: u32,
	/// The hash of the preimage which unlocks this HTLC.
	pub payment_hash: *const u8,
	// 32 bytes
	/// The position within the commitment transactions' outputs. This may be None if the value is
	/// below the dust limit (in which case no output appears in the commitment transaction and the
	/// value is spent to additional transaction fees).
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
