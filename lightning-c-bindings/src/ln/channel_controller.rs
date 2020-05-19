use crate::ln::channelmanager::ChannelManager;
use std::slice;
use bitcoin::secp256k1::{PublicKey, SecretKey, Signature, Secp256k1};
use crate::error::Error;

use lightning::chain::keysinterface::{ChannelKeys as RawChannelKeys, InMemoryChannelKeys as RawInMemoryChannelKeys};
use lightning::ln::msgs::UnsignedChannelAnnouncement;
use bitcoin::{Transaction as RawTransaction, Transaction};
use bitcoin::secp256k1 as secp256k1;
use lightning::ln::chan_utils::{LocalCommitmentTransaction, TxCreationKeys, ChannelPublicKeys, HTLCOutputInCommitment};
use crate::buffer::{BufferArgument, BufferResponse, BufferResponseArray, BufferArgumentArray};
use bitcoin::consensus::{deserialize, serialize};
use std::ffi::c_void;
use crate::c_types::tx_creation_keys::TxCreationKeySetResponse;
use crate::c_types::htlc_commitment_output::HTLCOutputInCommitmentResponseArray;
use crate::c_types::local_commitment_transaction::LocalCommitmentTransactionResponse;

#[derive(Clone)]
#[no_mangle]
pub struct ChannelKeys {
	pub host_instance_pointer: *mut c_void,
	pub funding_key: *const u8,
	funding_key_cache: Option<SecretKey>,
	pub revocation_base_key: *const u8,
	pub payment_key: *const u8,
	pub delayed_payment_base_key: *const u8,
	pub htlc_base_key: *const u8,
	pub commitment_seed: *const u8,
	pub pubkeys: extern "C" fn (this_arg: *const c_void) -> BufferArgumentArray,
	pub sign_remote_commitment: extern "C" fn (this_arg: *const c_void, feerate_per_kw: u64, commitment_tx: &BufferResponse, keys: &TxCreationKeySetResponse, htlcs: HTLCOutputInCommitmentResponseArray, to_self_delay: u16, error: *mut Error) -> (BufferArgument, BufferArgumentArray),
	pub sign_local_commitment: extern "C" fn (this_arg: *const c_void, local_commitment_tx: LocalCommitmentTransactionResponse, error: *mut Error) -> BufferArgument,
	pub sign_local_commitment_htlc_transactions: extern "C" fn (this_arg: *const c_void, local_commitment_tx: LocalCommitmentTransactionResponse, error: *mut Error) -> BufferArgument,
	pub sign_closing_transaction: extern "C" fn (this_arg: *const c_void, closing_tx: BufferResponse, error: *mut Error) -> BufferArgument,
}

unsafe impl Send for ChannelKeys {}

pub struct InMemoryChannelKeys(pub(crate) RawInMemoryChannelKeys);

#[no_mangle]
pub extern "C" fn in_memory_channel_keys_create(
	funding_key: *const u8, 
	revocation_base_key: *const u8, 
	payment_key: *const u8, 
	delayed_payment_base_key: *const u8,
	htlc_base_key: *const u8, 
	commitment_seed: *const u8, 
	channel_value_satoshis: u64 
) -> *mut InMemoryChannelKeys{
	let curve = secp256k1::Secp256k1::new();

	let funding_key_slice = unsafe {
		assert!(!funding_key.is_null());
		slice::from_raw_parts(funding_key, 32)
	};
	let funding_key_object = SecretKey::from_slice(funding_key_slice).unwrap();

	let revocation_base_key_slice = unsafe {
		assert!(!revocation_base_key.is_null());
		slice::from_raw_parts(revocation_base_key, 32)
	};
	let revocation_base_key_object = SecretKey::from_slice(revocation_base_key_slice).unwrap();

	let payment_key_slice = unsafe {
		assert!(!payment_key.is_null());
		slice::from_raw_parts(payment_key, 32)
	};
	let payment_key_object = SecretKey::from_slice(payment_key_slice).unwrap();

	let delayed_payment_key_slice = unsafe {
		assert!(!delayed_payment_base_key.is_null());
		slice::from_raw_parts(delayed_payment_base_key, 32)
	};
	let delayed_payment_key_object = SecretKey::from_slice(delayed_payment_key_slice).unwrap();

	let htlc_key_slice = unsafe {
		assert!(!htlc_base_key.is_null());
		slice::from_raw_parts(htlc_base_key, 32)
	};
	let htlc_key_object = SecretKey::from_slice(htlc_key_slice).unwrap();

	let commitment_seed_slice = unsafe {
		assert!(!commitment_seed.is_null());
		slice::from_raw_parts(commitment_seed, 32)
	};
	let mut commitment_seed = [0u8; 32];
	commitment_seed.copy_from_slice(commitment_seed_slice);
	
	let channel_keys = RawInMemoryChannelKeys::new(
		&curve, 
		funding_key_object,
		revocation_base_key_object,
		payment_key_object,
		delayed_payment_key_object,
		htlc_key_object,
		commitment_seed,
		channel_value_satoshis
	);
	
	Box::into_raw(Box::new(InMemoryChannelKeys(channel_keys)))
}

impl RawChannelKeys for ChannelKeys{
	fn funding_key<'a>(&'a self) -> &'a SecretKey {
		let private_key_slice = unsafe {
			assert!(!self.funding_key.is_null());
			slice::from_raw_parts(self.funding_key, 32)
		};
		let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();
		&private_key_object;
		unimplemented!()
	}

	fn revocation_base_key<'a>(&'a self) -> &'a SecretKey {
		let private_key_slice = unsafe {
			assert!(!self.revocation_base_key.is_null());
			slice::from_raw_parts(self.revocation_base_key, 32)
		};
		let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();
		&private_key_object;
		unimplemented!()
	}

	fn payment_key<'a>(&'a self) -> &'a SecretKey {
		let private_key_slice = unsafe {
			assert!(!self.payment_key.is_null());
			slice::from_raw_parts(self.payment_key, 32)
		};
		let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();
		&private_key_object;
		unimplemented!()
	}

	fn delayed_payment_base_key<'a>(&'a self) -> &'a SecretKey {
		let private_key_slice = unsafe {
			assert!(!self.delayed_payment_base_key.is_null());
			slice::from_raw_parts(self.delayed_payment_base_key, 32)
		};
		let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();
		&private_key_object;
		unimplemented!()
	}

	fn htlc_base_key<'a>(&'a self) -> &'a SecretKey {
		let private_key_slice = unsafe {
			assert!(!self.htlc_base_key.is_null());
			slice::from_raw_parts(self.htlc_base_key, 32)
		};
		let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();
		&private_key_object;
		unimplemented!()
	}

	fn commitment_seed<'a>(&'a self) -> &'a [u8; 32] {
		let commitment_seed_slice = unsafe {
			assert!(!self.commitment_seed.is_null());
			slice::from_raw_parts(self.commitment_seed, 32)
		};
		let mut commitment_seed = [0u8; 32];
		commitment_seed.copy_from_slice(commitment_seed_slice);
		&commitment_seed;
		unimplemented!()
	}

	fn pubkeys<'a>(&'a self) -> &'a ChannelPublicKeys {
		unimplemented!()
	}

	fn sign_remote_commitment<T: secp256k1::Signing + secp256k1::Verification>(&self, feerate_per_kw: u64, commitment_tx: &RawTransaction, keys: &TxCreationKeys, htlcs: &[&HTLCOutputInCommitment], to_self_delay: u16, secp_ctx: &Secp256k1<T>) -> Result<(Signature, Vec<Signature>), ()> {
		let callback = self.sign_remote_commitment;
		let commitment_tx_buffer : BufferResponse = commitment_tx.into();
		let keys: TxCreationKeySetResponse = keys.into();
		let htlcs : HTLCOutputInCommitmentResponseArray = htlcs.into();
		let error: *mut Error = std::ptr::null_mut();
		let result = callback(self.host_instance_pointer, feerate_per_kw, &commitment_tx_buffer, &keys, htlcs, to_self_delay, error);
		if(!error.is_null()){
			let error = unsafe { Box::from_raw(error) };
			return Err(());
		}
		Ok((result.0.into(), result.1.into()))

	}

	fn sign_local_commitment<T: secp256k1::Signing + secp256k1::Verification>(&self, local_commitment_tx: &LocalCommitmentTransaction, secp_ctx: &Secp256k1<T>) -> Result<Signature, ()> {
		let callback = self.sign_local_commitment;
		let local_commitment_tx: LocalCommitmentTransactionResponse = (local_commitment_tx.clone()).into();
		let error: *mut Error = std::ptr::null_mut();
		let result = callback(self.host_instance_pointer, local_commitment_tx, error);
		if(!error.is_null()){
			let error = unsafe { Box::from_raw(error) };
			return Err(());
		};
		Ok(result.into())
	}

	#[cfg(test)]
	fn unsafe_sign_local_commitment<T: secp256k1::Signing + secp256k1::Verification>(&self, local_commitment_tx: &LocalCommitmentTransaction, secp_ctx: &Secp256k1<T>) -> Result<Signature, ()> {
		unimplemented!()
	}

	fn sign_local_commitment_htlc_transactions<T: secp256k1::Signing + secp256k1::Verification>(&self, local_commitment_tx: &LocalCommitmentTransaction, local_csv: u16, secp_ctx: &Secp256k1<T>) -> Result<Vec<Option<Signature>>, ()> {
		unimplemented!()
	}

	fn sign_closing_transaction<T: secp256k1::Signing>(&self, closing_tx: &Transaction, secp_ctx: &Secp256k1<T>) -> Result<Signature, ()> {
		unimplemented!()
	}

	fn sign_channel_announcement<T: secp256k1::Signing>(&self, msg: &UnsignedChannelAnnouncement, secp_ctx: &Secp256k1<T>) -> Result<Signature, ()> {
		unimplemented!()
	}

	fn set_remote_channel_pubkeys(&mut self, channel_points: &ChannelPublicKeys) {
		unimplemented!()
	}
}

#[no_mangle]
pub extern "C" fn channel_manager_open_channel(this_arg: &ChannelManager, their_network_key: *const u8, channel_value_satoshis: u64, push_msat: u64, user_id: u64, error: *mut Error) {
	let public_key_slice = unsafe {
		assert!(!their_network_key.is_null());
		slice::from_raw_parts(their_network_key, 33)
	};
	let public_key_object = PublicKey::from_slice(public_key_slice).unwrap();
	let channel = unsafe { &*this_arg.inner }.create_channel(public_key_object, channel_value_satoshis, push_msat, user_id, None);
	if channel.is_err() {
		let ffi_error: Error = channel.err().unwrap().into();
		unsafe { std::ptr::write(error, ffi_error); }
	}
}

#[no_mangle]
pub extern "C" fn get_tx_creation_keys() {

}
