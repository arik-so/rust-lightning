//! " Various utilities for building scripts and deriving keys related to channels. These are"
//! " largely of interest for those implementing chain::keysinterface::ChannelKeys message signing"
//! " by hand."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

use bitcoin::blockdata::script::Script as lnScript;
use bitcoin::blockdata::script::Builder as lnBuilder;
use bitcoin::blockdata::opcodes as lnopcodes;
use bitcoin::blockdata::transaction::TxIn as lnTxIn;
use bitcoin::blockdata::transaction::TxOut as lnTxOut;
use bitcoin::blockdata::transaction::OutPoint as lnOutPoint;
use bitcoin::blockdata::transaction::Transaction as lnTransaction;
use bitcoin::blockdata::transaction::SigHashType as lnSigHashType;
use bitcoin::consensus::encode::self as lnself;
use bitcoin::consensus::encode::Decodable as lnDecodable;
use bitcoin::consensus::encode::Encodable as lnEncodable;
use bitcoin::util::bip143 as lnbip143;
use bitcoin::hashes::Hash as lnHash;
use bitcoin::hashes::HashEngine as lnHashEngine;
use bitcoin::hash_types::Txid as lnTxid;
use bitcoin::hash_types::PubkeyHash as lnPubkeyHash;
use bitcoin::secp256k1::key::SecretKey as lnSecretKey;
use bitcoin::secp256k1::key::PublicKey as lnPublicKey;
use bitcoin::secp256k1::Secp256k1 as lnSecp256k1;
use bitcoin::secp256k1::Signature as lnSignature;
use bitcoin::secp256k1 as lnsecp256k1;

use lightning::ln::chan_utils::TxCreationKeys as lnTxCreationKeysImport;
type lnTxCreationKeys = lnTxCreationKeysImport;

/// " The set of public keys which are used in the creation of one commitment transaction."
/// " These are derived from the channel base keys and per-commitment data."
#[repr(C)]
pub struct TxCreationKeys {
	pub(crate) inner: *const lnTxCreationKeys,
}

#[no_mangle]
pub extern "C" fn TxCreationKeys_free(this_ptr: TxCreationKeys) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnTxCreationKeys) };
}
#[no_mangle]
pub extern "C" fn TxCreationKeys_get_per_commitment_point(this_ptr: *const TxCreationKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&&unsafe { &*(*this_ptr).inner }.per_commitment_point)
}
#[no_mangle]
pub extern "C" fn TxCreationKeys_set_per_commitment_point(this_ptr: *mut TxCreationKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *((*this_ptr).inner as *mut lnTxCreationKeys) }.per_commitment_point = val.into_rust();
}

use lightning::ln::chan_utils::ChannelPublicKeys as lnChannelPublicKeysImport;
type lnChannelPublicKeys = lnChannelPublicKeysImport;

/// " One counterparty's public keys which do not change over the life of a channel."
#[repr(C)]
pub struct ChannelPublicKeys {
	pub(crate) inner: *const lnChannelPublicKeys,
}

#[no_mangle]
pub extern "C" fn ChannelPublicKeys_free(this_ptr: ChannelPublicKeys) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelPublicKeys) };
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_funding_pubkey(this_ptr: *const ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&&unsafe { &*(*this_ptr).inner }.funding_pubkey)
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_funding_pubkey(this_ptr: *mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelPublicKeys) }.funding_pubkey = val.into_rust();
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_revocation_basepoint(this_ptr: *const ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&&unsafe { &*(*this_ptr).inner }.revocation_basepoint)
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_revocation_basepoint(this_ptr: *mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelPublicKeys) }.revocation_basepoint = val.into_rust();
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_payment_point(this_ptr: *const ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&&unsafe { &*(*this_ptr).inner }.payment_point)
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_payment_point(this_ptr: *mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelPublicKeys) }.payment_point = val.into_rust();
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_delayed_payment_basepoint(this_ptr: *const ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&&unsafe { &*(*this_ptr).inner }.delayed_payment_basepoint)
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_delayed_payment_basepoint(this_ptr: *mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelPublicKeys) }.delayed_payment_basepoint = val.into_rust();
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_htlc_basepoint(this_ptr: *const ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&&unsafe { &*(*this_ptr).inner }.htlc_basepoint)
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_htlc_basepoint(this_ptr: *mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelPublicKeys) }.htlc_basepoint = val.into_rust();
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_new(funding_pubkey_arg: crate::c_types::PublicKey, revocation_basepoint_arg: crate::c_types::PublicKey, payment_point_arg: crate::c_types::PublicKey, delayed_payment_basepoint_arg: crate::c_types::PublicKey, htlc_basepoint_arg: crate::c_types::PublicKey) -> ChannelPublicKeys {
	ChannelPublicKeys { inner: Box::into_raw(Box::new(lnChannelPublicKeys {
		funding_pubkey: funding_pubkey_arg.into_rust(),
		revocation_basepoint: revocation_basepoint_arg.into_rust(),
		payment_point: payment_point_arg.into_rust(),
		delayed_payment_basepoint: delayed_payment_basepoint_arg.into_rust(),
		htlc_basepoint: htlc_basepoint_arg.into_rust(),
	}))}
}

use lightning::ln::chan_utils::HTLCOutputInCommitment as lnHTLCOutputInCommitmentImport;
type lnHTLCOutputInCommitment = lnHTLCOutputInCommitmentImport;

/// " Information about an HTLC as it appears in a commitment transaction"
#[repr(C)]
pub struct HTLCOutputInCommitment {
	pub(crate) inner: *const lnHTLCOutputInCommitment,
}

#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_free(this_ptr: HTLCOutputInCommitment) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnHTLCOutputInCommitment) };
}
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_offered(this_ptr: *mut HTLCOutputInCommitment, val: bool) {
	unsafe { &mut *((*this_ptr).inner as *mut lnHTLCOutputInCommitment) }.offered = val;
}
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_amount_msat(this_ptr: *mut HTLCOutputInCommitment, val: u64) {
	unsafe { &mut *((*this_ptr).inner as *mut lnHTLCOutputInCommitment) }.amount_msat = val;
}
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_cltv_expiry(this_ptr: *mut HTLCOutputInCommitment, val: u32) {
	unsafe { &mut *((*this_ptr).inner as *mut lnHTLCOutputInCommitment) }.cltv_expiry = val;
}
