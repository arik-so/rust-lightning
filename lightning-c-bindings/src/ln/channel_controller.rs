use crate::ln::channelmanager::ChannelManager;
use std::slice;
use bitcoin::secp256k1::{PublicKey, SecretKey, Signature, Secp256k1};
use crate::error::Error;

use lightning::chain::keysinterface::ChannelKeys as RawChannelKeys;
use lightning::ln::msgs::UnsignedChannelAnnouncement;
use bitcoin::{Transaction as RawTransaction, Transaction};
use bitcoin::secp256k1 as secp256k1;
use lightning::ln::chan_utils::{LocalCommitmentTransaction, TxCreationKeys, ChannelPublicKeys, HTLCOutputInCommitment};
use crate::buffer::{BufferArgument, BufferResponse, BufferResponseArray};
use bitcoin::consensus::{deserialize, serialize};
use std::ffi::c_void;

impl From<BufferArgument> for RawTransaction {
	fn from(txBinary: BufferArgument) -> Self {
		let tx_vec = unsafe { txBinary.to_vec() };
		let tx: RawTransaction = deserialize(&tx_vec).unwrap();
		tx
	}
}

impl From<&RawTransaction> for BufferResponse {
	fn from(tx: &Transaction) -> Self {
		let serialization = serialize(tx);
		let buffer: BufferResponse = serialization.into();
		buffer
	}
}

impl From<LocalCommitmentTransaction> for BufferResponse {
	fn from(_: LocalCommitmentTransaction) -> Self {
		unimplemented!()
	}
}

impl From<&[&lightning::ln::chan_utils::HTLCOutputInCommitment]> for BufferResponseArray {
	fn from(_: &[&HTLCOutputInCommitment]) -> Self {
		unimplemented!()
	}
}

impl From<&lightning::ln::chan_utils::TxCreationKeys> for BufferResponse {
	fn from(_: &TxCreationKeys) -> Self {
		unimplemented!()
	}
}

#[derive(Clone)]
struct ChannelKeys {
	pub host_instance_pointer: *mut c_void,
	pub sign_remote_commitment: extern "C" fn (this_arg: *const c_void, feerate_per_kw: u64, commitment_tx: &BufferResponse, keys: &BufferResponse, htlcs: &BufferResponseArray, to_self_delay: u16, error: *mut Error) -> (Signature, Vec<Signature>),
}

unsafe impl Send for ChannelKeys {}

impl RawChannelKeys for ChannelKeys{
	fn funding_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn revocation_base_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn payment_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn delayed_payment_base_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn htlc_base_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn commitment_seed<'a>(&'a self) -> &'a [u8; 32] {
		unimplemented!()
	}

	fn pubkeys<'a>(&'a self) -> &'a ChannelPublicKeys {
		unimplemented!()
	}

	fn sign_remote_commitment<T: secp256k1::Signing + secp256k1::Verification>(&self, feerate_per_kw: u64, commitment_tx: &RawTransaction, keys: &TxCreationKeys, htlcs: &[&HTLCOutputInCommitment], to_self_delay: u16, secp_ctx: &Secp256k1<T>) -> Result<(Signature, Vec<Signature>), ()> {
		let callback = self.sign_remote_commitment;
		let commitment_tx_buffer : BufferResponse = commitment_tx.into();
		let keys_buffer : BufferResponse = keys.into();
		let htlcs : BufferResponseArray = htlcs.into();
		let error: *mut Error = std::ptr::null_mut();
		let result = callback(self.host_instance_pointer, feerate_per_kw, &commitment_tx_buffer, &keys_buffer, &htlcs, to_self_delay, error);
		if(!error.is_null()){
			let error = unsafe { Box::from_raw(error) };
			return Err(());
		}
		// result
		unimplemented!()
	}

	fn sign_local_commitment<T: secp256k1::Signing + secp256k1::Verification>(&self, local_commitment_tx: &LocalCommitmentTransaction, secp_ctx: &Secp256k1<T>) -> Result<Signature, ()> {
		unimplemented!()
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
