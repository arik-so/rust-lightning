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
use bitcoin::consensus::encode::Decodable as lnDecodable;
use bitcoin::consensus::encode::Encodable as lnEncodable;
use bitcoin::consensus::encode as lnencode;
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
/// " The per-commitment public key which was used to derive the other keys."
#[no_mangle]
pub extern "C" fn TxCreationKeys_get_per_commitment_point(this_ptr: &TxCreationKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.per_commitment_point)
}
/// " The per-commitment public key which was used to derive the other keys."
#[no_mangle]
pub extern "C" fn TxCreationKeys_set_per_commitment_point(this_ptr: &mut TxCreationKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnTxCreationKeys) }.per_commitment_point = val.into_rust();
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
/// " The public key which is used to sign all commitment transactions, as it appears in the"
/// " on-chain channel lock-in 2-of-2 multisig output."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_funding_pubkey(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.funding_pubkey)
}
/// " The public key which is used to sign all commitment transactions, as it appears in the"
/// " on-chain channel lock-in 2-of-2 multisig output."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_funding_pubkey(this_ptr: &mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.funding_pubkey = val.into_rust();
}
/// " The base point which is used (with derive_public_revocation_key) to derive per-commitment"
/// " revocation keys. The per-commitment revocation private key is then revealed by the owner of"
/// " a commitment transaction so that their counterparty can claim all available funds if they"
/// " broadcast an old state."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_revocation_basepoint(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.revocation_basepoint)
}
/// " The base point which is used (with derive_public_revocation_key) to derive per-commitment"
/// " revocation keys. The per-commitment revocation private key is then revealed by the owner of"
/// " a commitment transaction so that their counterparty can claim all available funds if they"
/// " broadcast an old state."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_revocation_basepoint(this_ptr: &mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.revocation_basepoint = val.into_rust();
}
/// " The public key which receives our immediately spendable primary channel balance in"
/// " remote-broadcasted commitment transactions. This key is static across every commitment"
/// " transaction."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_payment_point(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.payment_point)
}
/// " The public key which receives our immediately spendable primary channel balance in"
/// " remote-broadcasted commitment transactions. This key is static across every commitment"
/// " transaction."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_payment_point(this_ptr: &mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.payment_point = val.into_rust();
}
/// " The base point which is used (with derive_public_key) to derive a per-commitment payment"
/// " public key which receives non-HTLC-encumbered funds which are only available for spending"
/// " after some delay (or can be claimed via the revocation path)."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_delayed_payment_basepoint(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.delayed_payment_basepoint)
}
/// " The base point which is used (with derive_public_key) to derive a per-commitment payment"
/// " public key which receives non-HTLC-encumbered funds which are only available for spending"
/// " after some delay (or can be claimed via the revocation path)."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_delayed_payment_basepoint(this_ptr: &mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.delayed_payment_basepoint = val.into_rust();
}
/// " The base point which is used (with derive_public_key) to derive a per-commitment public key"
/// " which is used to encumber HTLC-in-flight outputs."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_htlc_basepoint(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.htlc_basepoint)
}
/// " The base point which is used (with derive_public_key) to derive a per-commitment public key"
/// " which is used to encumber HTLC-in-flight outputs."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_htlc_basepoint(this_ptr: &mut ChannelPublicKeys, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.htlc_basepoint = val.into_rust();
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
/// " Whether the HTLC was \"offered\" (ie outbound in relation to this commitment transaction)."
/// " Note that this is not the same as whether it is ountbound *from us*. To determine that you"
/// " need to compare this value to whether the commitment transaction in question is that of"
/// " the remote party or our own."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_offered(this_ptr: &mut HTLCOutputInCommitment, val: bool) {
	unsafe { &mut *(this_ptr.inner as *mut lnHTLCOutputInCommitment) }.offered = val;
}
/// " The value, in msat, of the HTLC. The value as it appears in the commitment transaction is"
/// " this divided by 1000."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_amount_msat(this_ptr: &mut HTLCOutputInCommitment, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnHTLCOutputInCommitment) }.amount_msat = val;
}
/// " The CLTV lock-time at which this HTLC expires."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_cltv_expiry(this_ptr: &mut HTLCOutputInCommitment, val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnHTLCOutputInCommitment) }.cltv_expiry = val;
}
