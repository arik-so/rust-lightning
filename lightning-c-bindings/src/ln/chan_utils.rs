//! " Various utilities for building scripts and deriving keys related to channels. These are"
//! " largely of interest for those implementing chain::keysinterface::ChannelKeys message signing"
//! " by hand."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::TakePointer;


use lightning::ln::chan_utils::TxCreationKeys as lnTxCreationKeysImport;
type lnTxCreationKeys = lnTxCreationKeysImport;

/// " The set of public keys which are used in the creation of one commitment transaction."
/// " These are derived from the channel base keys and per-commitment data."
#[repr(C)]
pub struct TxCreationKeys {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnTxCreationKeys,
	pub _underlying_ref: bool,
}

impl Drop for TxCreationKeys {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnTxCreationKeys) };
		}
	}
}
#[no_mangle]
pub extern "C" fn TxCreationKeys_free(this_ptr: TxCreationKeys) { }
/// " The per-commitment public key which was used to derive the other keys."
#[no_mangle]
pub extern "C" fn TxCreationKeys_get_per_commitment_point(this_ptr: &TxCreationKeys) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.per_commitment_point;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The per-commitment public key which was used to derive the other keys."
#[no_mangle]
pub extern "C" fn TxCreationKeys_set_per_commitment_point(this_ptr: &mut TxCreationKeys, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnTxCreationKeys) }.per_commitment_point = val.into_rust();
}

use lightning::ln::chan_utils::ChannelPublicKeys as lnChannelPublicKeysImport;
type lnChannelPublicKeys = lnChannelPublicKeysImport;

/// " One counterparty's public keys which do not change over the life of a channel."
#[repr(C)]
pub struct ChannelPublicKeys {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelPublicKeys,
	pub _underlying_ref: bool,
}

impl Drop for ChannelPublicKeys {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelPublicKeys) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_free(this_ptr: ChannelPublicKeys) { }
/// " The public key which is used to sign all commitment transactions, as it appears in the"
/// " on-chain channel lock-in 2-of-2 multisig output."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_funding_pubkey(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.funding_pubkey;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The public key which is used to sign all commitment transactions, as it appears in the"
/// " on-chain channel lock-in 2-of-2 multisig output."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_funding_pubkey(this_ptr: &mut ChannelPublicKeys, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.funding_pubkey = val.into_rust();
}
/// " The base point which is used (with derive_public_revocation_key) to derive per-commitment"
/// " revocation keys. The per-commitment revocation private key is then revealed by the owner of"
/// " a commitment transaction so that their counterparty can claim all available funds if they"
/// " broadcast an old state."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_revocation_basepoint(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.revocation_basepoint;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The base point which is used (with derive_public_revocation_key) to derive per-commitment"
/// " revocation keys. The per-commitment revocation private key is then revealed by the owner of"
/// " a commitment transaction so that their counterparty can claim all available funds if they"
/// " broadcast an old state."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_revocation_basepoint(this_ptr: &mut ChannelPublicKeys, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.revocation_basepoint = val.into_rust();
}
/// " The public key which receives our immediately spendable primary channel balance in"
/// " remote-broadcasted commitment transactions. This key is static across every commitment"
/// " transaction."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_payment_point(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.payment_point;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The public key which receives our immediately spendable primary channel balance in"
/// " remote-broadcasted commitment transactions. This key is static across every commitment"
/// " transaction."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_payment_point(this_ptr: &mut ChannelPublicKeys, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.payment_point = val.into_rust();
}
/// " The base point which is used (with derive_public_key) to derive a per-commitment payment"
/// " public key which receives non-HTLC-encumbered funds which are only available for spending"
/// " after some delay (or can be claimed via the revocation path)."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_delayed_payment_basepoint(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.delayed_payment_basepoint;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The base point which is used (with derive_public_key) to derive a per-commitment payment"
/// " public key which receives non-HTLC-encumbered funds which are only available for spending"
/// " after some delay (or can be claimed via the revocation path)."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_delayed_payment_basepoint(this_ptr: &mut ChannelPublicKeys, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.delayed_payment_basepoint = val.into_rust();
}
/// " The base point which is used (with derive_public_key) to derive a per-commitment public key"
/// " which is used to encumber HTLC-in-flight outputs."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_get_htlc_basepoint(this_ptr: &ChannelPublicKeys) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.htlc_basepoint;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The base point which is used (with derive_public_key) to derive a per-commitment public key"
/// " which is used to encumber HTLC-in-flight outputs."
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_set_htlc_basepoint(this_ptr: &mut ChannelPublicKeys, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelPublicKeys) }.htlc_basepoint = val.into_rust();
}
#[no_mangle]
pub extern "C" fn ChannelPublicKeys_new(mut funding_pubkey_arg: crate::c_types::PublicKey, mut revocation_basepoint_arg: crate::c_types::PublicKey, mut payment_point_arg: crate::c_types::PublicKey, mut delayed_payment_basepoint_arg: crate::c_types::PublicKey, mut htlc_basepoint_arg: crate::c_types::PublicKey) -> ChannelPublicKeys {
	ChannelPublicKeys { inner: Box::into_raw(Box::new(lnChannelPublicKeys {
		funding_pubkey: funding_pubkey_arg.into_rust(),
		revocation_basepoint: revocation_basepoint_arg.into_rust(),
		payment_point: payment_point_arg.into_rust(),
		delayed_payment_basepoint: delayed_payment_basepoint_arg.into_rust(),
		htlc_basepoint: htlc_basepoint_arg.into_rust(),
	})), _underlying_ref: false }
}

use lightning::ln::chan_utils::HTLCOutputInCommitment as lnHTLCOutputInCommitmentImport;
type lnHTLCOutputInCommitment = lnHTLCOutputInCommitmentImport;

/// " Information about an HTLC as it appears in a commitment transaction"
#[repr(C)]
pub struct HTLCOutputInCommitment {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnHTLCOutputInCommitment,
	pub _underlying_ref: bool,
}

impl Drop for HTLCOutputInCommitment {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnHTLCOutputInCommitment) };
		}
	}
}
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_free(this_ptr: HTLCOutputInCommitment) { }
/// " Whether the HTLC was \"offered\" (ie outbound in relation to this commitment transaction)."
/// " Note that this is not the same as whether it is ountbound *from us*. To determine that you"
/// " need to compare this value to whether the commitment transaction in question is that of"
/// " the remote party or our own."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_get_offered(this_ptr: &HTLCOutputInCommitment) -> bool {
	let inner_val = &unsafe { &*this_ptr.inner }.offered;
	(*inner_val)
}
/// " Whether the HTLC was \"offered\" (ie outbound in relation to this commitment transaction)."
/// " Note that this is not the same as whether it is ountbound *from us*. To determine that you"
/// " need to compare this value to whether the commitment transaction in question is that of"
/// " the remote party or our own."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_offered(this_ptr: &mut HTLCOutputInCommitment, mut val: bool) {
	unsafe { &mut *(this_ptr.inner as *mut lnHTLCOutputInCommitment) }.offered = val;
}
/// " The value, in msat, of the HTLC. The value as it appears in the commitment transaction is"
/// " this divided by 1000."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_get_amount_msat(this_ptr: &HTLCOutputInCommitment) -> u64 {
	let inner_val = &unsafe { &*this_ptr.inner }.amount_msat;
	(*inner_val)
}
/// " The value, in msat, of the HTLC. The value as it appears in the commitment transaction is"
/// " this divided by 1000."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_amount_msat(this_ptr: &mut HTLCOutputInCommitment, mut val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnHTLCOutputInCommitment) }.amount_msat = val;
}
/// " The CLTV lock-time at which this HTLC expires."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_get_cltv_expiry(this_ptr: &HTLCOutputInCommitment) -> u32 {
	let inner_val = &unsafe { &*this_ptr.inner }.cltv_expiry;
	(*inner_val)
}
/// " The CLTV lock-time at which this HTLC expires."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_cltv_expiry(this_ptr: &mut HTLCOutputInCommitment, mut val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnHTLCOutputInCommitment) }.cltv_expiry = val;
}
/// " The hash of the preimage which unlocks this HTLC."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_get_payment_hash(this_ptr: &HTLCOutputInCommitment) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.payment_hash;
	&(*inner_val).0
}
/// " The hash of the preimage which unlocks this HTLC."
#[no_mangle]
pub extern "C" fn HTLCOutputInCommitment_set_payment_hash(this_ptr: &mut HTLCOutputInCommitment, mut val: [u8; 32]) {
	unsafe { &mut *(this_ptr.inner as *mut lnHTLCOutputInCommitment) }.payment_hash = ::lightning::ln::channelmanager::PaymentHash(val);
}

use lightning::ln::chan_utils::LocalCommitmentTransaction as lnLocalCommitmentTransactionImport;
type lnLocalCommitmentTransaction = lnLocalCommitmentTransactionImport;

/// " We use this to track local commitment transactions and put off signing them until we are ready"
/// " to broadcast. Eventually this will require a signer which is possibly external, but for now we"
/// " just pass in the SecretKeys required."
#[repr(C)]
pub struct LocalCommitmentTransaction {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnLocalCommitmentTransaction,
	pub _underlying_ref: bool,
}

impl Drop for LocalCommitmentTransaction {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnLocalCommitmentTransaction) };
		}
	}
}
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_free(this_ptr: LocalCommitmentTransaction) { }
/// " The commitment transaction itself, in unsigned form."
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_get_unsigned_tx(this_ptr: &LocalCommitmentTransaction) -> crate::c_types::Transaction {
	let inner_val = &unsafe { &*this_ptr.inner }.unsigned_tx;
	let local_inner_val = ::bitcoin::consensus::encode::serialize(inner_val);
	crate::c_types::Transaction::from_slice(&local_inner_val)
}
/// " The commitment transaction itself, in unsigned form."
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_set_unsigned_tx(this_ptr: &mut LocalCommitmentTransaction, mut val: crate::c_types::Transaction) {
	unsafe { &mut *(this_ptr.inner as *mut lnLocalCommitmentTransaction) }.unsigned_tx = val.into_bitcoin();
}
/// " Our counterparty's signature for the transaction, above."
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_get_their_sig(this_ptr: &LocalCommitmentTransaction) -> crate::c_types::Signature {
	let inner_val = &unsafe { &*this_ptr.inner }.their_sig;
	crate::c_types::Signature::from_rust(&(*inner_val))
}
/// " Our counterparty's signature for the transaction, above."
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_set_their_sig(this_ptr: &mut LocalCommitmentTransaction, mut val: crate::c_types::Signature) {
	unsafe { &mut *(this_ptr.inner as *mut lnLocalCommitmentTransaction) }.their_sig = val.into_rust();
}
/// " The key derivation parameters for this commitment transaction"
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_get_local_keys(this_ptr: &LocalCommitmentTransaction) -> *const TxCreationKeys {
	let inner_val = &unsafe { &*this_ptr.inner }.local_keys;
	Box::into_raw(Box::new(crate::ln::chan_utils::TxCreationKeys { inner: &(*inner_val), _underlying_ref: true } ))
}
/// " The key derivation parameters for this commitment transaction"
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_set_local_keys(this_ptr: &mut LocalCommitmentTransaction, mut val: TxCreationKeys) {
	unsafe { &mut *(this_ptr.inner as *mut lnLocalCommitmentTransaction) }.local_keys = *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) };
}
/// " The feerate paid per 1000-weight-unit in this commitment transaction. This value is"
/// " controlled by the channel initiator."
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_get_feerate_per_kw(this_ptr: &LocalCommitmentTransaction) -> u64 {
	let inner_val = &unsafe { &*this_ptr.inner }.feerate_per_kw;
	(*inner_val)
}
/// " The feerate paid per 1000-weight-unit in this commitment transaction. This value is"
/// " controlled by the channel initiator."
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_set_feerate_per_kw(this_ptr: &mut LocalCommitmentTransaction, mut val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnLocalCommitmentTransaction) }.feerate_per_kw = val;
}
/// " Gets our signature for the contained commitment transaction given our funding private key."
/// ""
/// " Funding key is your key included in the 2-2 funding_outpoint lock. Should be provided"
/// " by your ChannelKeys."
/// " Funding redeemscript is script locking funding_outpoint. This is the mutlsig script"
/// " between your own funding key and your counterparty's. Currently, this is provided in"
/// " ChannelKeys::sign_local_commitment() calls directly."
/// " Channel value is amount locked in funding_outpoint."
#[no_mangle]
pub extern "C" fn LocalCommitmentTransaction_get_local_sig(this_arg: &LocalCommitmentTransaction, funding_key: *const [u8; 32], funding_redeemscript: crate::c_types::u8slice, mut channel_value_satoshis: u64) -> crate::c_types::Signature {
	let mut ret = unsafe { &*this_arg.inner }.get_local_sig(&::bitcoin::secp256k1::key::SecretKey::from_slice(&unsafe { *funding_key}[..]).unwrap(), &::bitcoin::blockdata::script::Script::from(Vec::from(funding_redeemscript.to_slice())), channel_value_satoshis, &bitcoin::secp256k1::Secp256k1::new());
	crate::c_types::Signature::from_rust(&ret)
}

