use lightning::ln::chan_utils::TxCreationKeys as RawTxCreationKeys;
use crate::buffer::BufferResponse;
use bitcoin::secp256k1::PublicKey;
use std::slice;

#[no_mangle]
pub struct TxCreationKeySetResponse {
	pub per_commitment_point: BufferResponse,
	pub revocation_key: BufferResponse,
	pub a_htlc_key: BufferResponse,
	pub b_htlc_key: BufferResponse,
	pub a_delayed_payment_key: BufferResponse,
}

#[no_mangle]
pub struct TxCreationKeySetArgument {
	pub per_commitment_point: *const u8,
	pub revocation_key: *const u8,
	pub a_htlc_key: *const u8,
	pub b_htlc_key: *const u8,
	pub a_delayed_payment_key: *const u8,
}

impl From<RawTxCreationKeys> for TxCreationKeySetResponse {
	fn from(key_set: RawTxCreationKeys) -> Self {
		let RawTxCreationKeys {
			per_commitment_point,
			revocation_key,
			a_htlc_key,
			b_htlc_key,
			a_delayed_payment_key,
		} = key_set;
		let per_commitment_point: BufferResponse = per_commitment_point.serialize().to_vec().into();
		let revocation_key: BufferResponse = revocation_key.serialize().to_vec().into();
		let a_htlc_key: BufferResponse = a_htlc_key.serialize().to_vec().into();
		let b_htlc_key: BufferResponse = b_htlc_key.serialize().to_vec().into();
		let a_delayed_payment_key: BufferResponse = a_delayed_payment_key.serialize().to_vec().into();
		TxCreationKeySetResponse {
			per_commitment_point,
			revocation_key,
			a_htlc_key,
			b_htlc_key,
			a_delayed_payment_key,
		}
	}
}

impl From<&RawTxCreationKeys> for TxCreationKeySetResponse {
	fn from(key_set: &RawTxCreationKeys) -> Self {
		let RawTxCreationKeys {
			per_commitment_point,
			revocation_key,
			a_htlc_key,
			b_htlc_key,
			a_delayed_payment_key,
		} = key_set;
		let per_commitment_point: BufferResponse = per_commitment_point.serialize().to_vec().into();
		let revocation_key: BufferResponse = revocation_key.serialize().to_vec().into();
		let a_htlc_key: BufferResponse = a_htlc_key.serialize().to_vec().into();
		let b_htlc_key: BufferResponse = b_htlc_key.serialize().to_vec().into();
		let a_delayed_payment_key: BufferResponse = a_delayed_payment_key.serialize().to_vec().into();
		TxCreationKeySetResponse {
			per_commitment_point,
			revocation_key,
			a_htlc_key,
			b_htlc_key,
			a_delayed_payment_key,
		}
	}
}

impl From<TxCreationKeySetArgument> for RawTxCreationKeys {
	fn from(key_set: TxCreationKeySetArgument) -> Self {
		let TxCreationKeySetArgument {
			per_commitment_point,
			revocation_key,
			a_htlc_key,
			b_htlc_key,
			a_delayed_payment_key,
		} = key_set;

		let per_commitment_point_slice = unsafe {
			assert!(!per_commitment_point.is_null());
			slice::from_raw_parts(per_commitment_point, 33)
		};
		let per_commitment_point = PublicKey::from_slice(per_commitment_point_slice).unwrap();

		let revocation_key_slice = unsafe {
			assert!(!revocation_key.is_null());
			slice::from_raw_parts(revocation_key, 33)
		};
		let revocation_key = PublicKey::from_slice(revocation_key_slice).unwrap();

		let a_htlc_key_slice = unsafe {
			assert!(!a_htlc_key.is_null());
			slice::from_raw_parts(a_htlc_key, 33)
		};
		let a_htlc_key = PublicKey::from_slice(a_htlc_key_slice).unwrap();

		let b_htlc_key_slice = unsafe {
			assert!(!b_htlc_key.is_null());
			slice::from_raw_parts(b_htlc_key, 33)
		};
		let b_htlc_key = PublicKey::from_slice(b_htlc_key_slice).unwrap();

		let a_delayed_payment_key_slice = unsafe {
			assert!(!a_delayed_payment_key.is_null());
			slice::from_raw_parts(a_delayed_payment_key, 33)
		};
		let a_delayed_payment_key = PublicKey::from_slice(a_delayed_payment_key_slice).unwrap();

		RawTxCreationKeys {
			per_commitment_point,
			revocation_key,
			a_htlc_key,
			b_htlc_key,
			a_delayed_payment_key,
		}
	}
}
