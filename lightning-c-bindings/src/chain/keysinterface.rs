//! " keysinterface provides keys into rust-lightning and defines some useful enums which describe"
//! " spendable on-chain outputs which the user owns and is responsible for using just as any other"
//! " on-chain output which is theirs."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::*;


use lightning::chain::keysinterface::SpendableOutputDescriptor as lnSpendableOutputDescriptorImport;
type lnSpendableOutputDescriptor = lnSpendableOutputDescriptorImport;

/// " When on-chain outputs are created by rust-lightning (which our counterparty is not able to"
/// " claim at any point in the future) an event is generated which you must track and be able to"
/// " spend on-chain. The information needed to do this is provided in this enum, including the"
/// " outpoint describing which txid and output index is available, the full output which exists at"
/// " that txid/index, and any keys or other information required to sign."
#[must_use]
#[repr(C)]
pub struct SpendableOutputDescriptor {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnSpendableOutputDescriptor,
	pub _underlying_ref: bool,
}

impl Drop for SpendableOutputDescriptor {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnSpendableOutputDescriptor) };
		}
	}
}
#[no_mangle]
pub extern "C" fn SpendableOutputDescriptor_free(this_ptr: SpendableOutputDescriptor) { }
impl Clone for SpendableOutputDescriptor {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
#[no_mangle]
pub extern "C" fn SpendableOutputDescriptor_write(obj: *const SpendableOutputDescriptor) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn SpendableOutputDescriptor_read(ser: crate::c_types::u8slice) -> SpendableOutputDescriptor {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		SpendableOutputDescriptor { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		SpendableOutputDescriptor { inner: std::ptr::null(), _underlying_ref: false }
	}
}
/// " Set of lightning keys needed to operate a channel as described in BOLT 3."
/// ""
/// " Signing services could be implemented on a hardware wallet. In this case,"
/// " the current ChannelKeys would be a front-end on top of a communication"
/// " channel connected to your secure device and lightning key material wouldn't"
/// " reside on a hot server. Nevertheless, a this deployment would still need"
/// " to trust the ChannelManager to avoid loss of funds as this latest component"
/// " could ask to sign commitment transaction with HTLCs paying to attacker pubkeys."
/// ""
/// " A more secure iteration would be to use hashlock (or payment points) to pair"
/// " invoice/incoming HTLCs with outgoing HTLCs to implement a no-trust-ChannelManager"
/// " at the price of more state and computation on the hardware wallet side. In the future,"
/// " we are looking forward to design such interface."
/// ""
/// " In any case, ChannelMonitor or fallback watchtowers are always going to be trusted"
/// " to act, as liveness and breach reply correctness are always going to be hard requirements"
/// " of LN security model, orthogonal of key management issues."
/// ""
/// " If you're implementing a custom signer, you almost certainly want to implement"
/// " Readable/Writable to serialize out a unique reference to this set of keys so"
/// " that you can serialize the full ChannelManager object."
/// ""
#[derive(Clone)]
#[repr(C)]
pub struct ChannelKeys {
	pub this_arg: *mut c_void,
	/// " Gets the commitment seed"
	pub commitment_seed: crate::c_types::ThirtyTwoBytes,
	/// Fill in the commitment_seed field as a reference to it will be given to Rust after this returns
	/// Note that this takes a pointer to this object, not the this_ptr like other methods do
	pub set_commitment_seed: Option<extern "C" fn(&ChannelKeys)>,
	/// " Gets the local channel public keys and basepoints"
	pub pubkeys: crate::ln::chan_utils::ChannelPublicKeys,
	/// Fill in the pubkeys field as a reference to it will be given to Rust after this returns
	/// Note that this takes a pointer to this object, not the this_ptr like other methods do
	pub set_pubkeys: Option<extern "C" fn(&ChannelKeys)>,
	/// " Gets arbitrary identifiers describing the set of keys which are provided back to you in"
	/// " some SpendableOutputDescriptor types. These should be sufficient to identify this"
	/// " ChannelKeys object uniquely and lookup or re-derive its keys."
	#[must_use]
	pub key_derivation_params: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::C2Tuple_u64u64Z,
	/// " Create a signature for a remote commitment transaction and associated HTLC transactions."
	/// ""
	/// " Note that if signing fails or is rejected, the channel will be force-closed."
	#[must_use]
	pub sign_remote_commitment: extern "C" fn (this_arg: *const c_void, feerate_per_kw: u64, commitment_tx: crate::c_types::Transaction, keys: &crate::ln::chan_utils::TxCreationKeys, htlcs: crate::c_types::derived::CHTLCOutputInCommitmentSlice, to_self_delay: u16) -> crate::c_types::derived::CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ,
	/// " Create a signature for a local commitment transaction. This will only ever be called with"
	/// " the same local_commitment_tx (or a copy thereof), though there are currently no guarantees"
	/// " that it will not be called multiple times."
	#[must_use]
	pub sign_local_commitment: extern "C" fn (this_arg: *const c_void, local_commitment_tx: &crate::ln::chan_utils::LocalCommitmentTransaction) -> crate::c_types::derived::CResult_SignatureNoneZ,
	//XXX: Need to export sign_local_commitment_htlc_transactions
	/// " Create a signature for the given input in a transaction spending an HTLC or commitment"
	/// " transaction output when our counterparty broadcasts an old state."
	/// ""
	/// " A justice transaction may claim multiples outputs at the same time if timelocks are"
	/// " similar, but only a signature for the input at index `input` should be signed for here."
	/// " It may be called multiples time for same output(s) if a fee-bump is needed with regards"
	/// " to an upcoming timelock expiration."
	/// ""
	/// " Amount is value of the output spent by this input, committed to in the BIP 143 signature."
	/// ""
	/// " per_commitment_key is revocation secret which was provided by our counterparty when they"
	/// " revoked the state which they eventually broadcast. It's not a _local_ secret key and does"
	/// " not allow the spending of any funds by itself (you need our local revocation_secret to do"
	/// " so)."
	/// ""
	/// " htlc holds HTLC elements (hash, timelock) if the output being spent is a HTLC output, thus"
	/// " changing the format of the witness script (which is committed to in the BIP 143"
	/// " signatures)."
	/// ""
	/// " on_remote_tx_csv is the relative lock-time that that our counterparty would have to set on"
	/// " their transaction were they to spend the same output. It is included in the witness script"
	/// " and thus committed to in the BIP 143 signature."
	#[must_use]
	pub sign_justice_transaction: extern "C" fn (this_arg: *const c_void, justice_tx: crate::c_types::Transaction, input: usize, amount: u64, per_commitment_key: *const [u8; 32], htlc: &crate::ln::chan_utils::HTLCOutputInCommitment, on_remote_tx_csv: u16) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// " Create a signature for a claiming transaction for a HTLC output on a remote commitment"
	/// " transaction, either offered or received."
	/// ""
	/// " Such a transaction may claim multiples offered outputs at same time if we know the"
	/// " preimage for each when we create it, but only the input at index `input` should be"
	/// " signed for here. It may be called multiple times for same output(s) if a fee-bump is"
	/// " needed with regards to an upcoming timelock expiration."
	/// ""
	/// " Witness_script is either a offered or received script as defined in BOLT3 for HTLC"
	/// " outputs."
	/// ""
	/// " Amount is value of the output spent by this input, committed to in the BIP 143 signature."
	/// ""
	/// " Per_commitment_point is the dynamic point corresponding to the channel state"
	/// " detected onchain. It has been generated by our counterparty and is used to derive"
	/// " channel state keys, which are then included in the witness script and committed to in the"
	/// " BIP 143 signature."
	#[must_use]
	pub sign_remote_htlc_transaction: extern "C" fn (this_arg: *const c_void, htlc_tx: crate::c_types::Transaction, input: usize, amount: u64, per_commitment_point: crate::c_types::PublicKey, htlc: &crate::ln::chan_utils::HTLCOutputInCommitment) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// " Create a signature for a (proposed) closing transaction."
	/// ""
	/// " Note that, due to rounding, there may be one \"missing\" satoshi, and either party may have"
	/// " chosen to forgo their output as dust."
	#[must_use]
	pub sign_closing_transaction: extern "C" fn (this_arg: *const c_void, closing_tx: crate::c_types::Transaction) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// " Signs a channel announcement message with our funding key, proving it comes from one"
	/// " of the channel participants."
	/// ""
	/// " Note that if this fails or is rejected, the channel will not be publicly announced and"
	/// " our counterparty may (though likely will not) close the channel on us for violating the"
	/// " protocol."
	#[must_use]
	pub sign_channel_announcement: extern "C" fn (this_arg: *const c_void, msg: &crate::ln::msgs::UnsignedChannelAnnouncement) -> crate::c_types::derived::CResult_SignatureNoneZ,
	/// " Set the remote channel basepoints.  This is done immediately on incoming channels"
	/// " and as soon as the channel is accepted on outgoing channels."
	/// ""
	/// " Will be called before any signatures are applied."
	pub set_remote_channel_pubkeys: extern "C" fn (this_arg: *mut c_void, channel_points: &crate::ln::chan_utils::ChannelPublicKeys),
}
unsafe impl Send for ChannelKeys {}

use lightning::chain::keysinterface::ChannelKeys as lnChannelKeys;
impl lnChannelKeys for ChannelKeys {
	fn commitment_seed(&self) -> &[u8; 32] {
		if let Some(f) = self.set_commitment_seed {
			(f)(self);
		}
		&self.commitment_seed.data
	}
	fn pubkeys(&self) -> &lightning::ln::chan_utils::ChannelPublicKeys {
		if let Some(f) = self.set_pubkeys {
			(f)(self);
		}
		unsafe { &*self.pubkeys.inner }
	}
	fn key_derivation_params(&self) -> (u64, u64) {
		let mut ret = (self.key_derivation_params)(self.this_arg);
		let (mut orig_ret_0, mut orig_ret_1) = ret.to_rust(); let local_ret = (orig_ret_0, orig_ret_1);
		local_ret
	}
	fn sign_remote_commitment<T:bitcoin::secp256k1::Signing + bitcoin::secp256k1::Verification>(&self, feerate_per_kw: u64, commitment_tx: &bitcoin::blockdata::transaction::Transaction, keys: &lightning::ln::chan_utils::TxCreationKeys, htlcs: &[&lightning::ln::chan_utils::HTLCOutputInCommitment], to_self_delay: u16, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<(bitcoin::secp256k1::Signature, Vec<bitcoin::secp256k1::Signature>), ()> {
		let local_commitment_tx = ::bitcoin::consensus::encode::serialize(commitment_tx);
		let mut ret = (self.sign_remote_commitment)(self.this_arg, feerate_per_kw, crate::c_types::Transaction::from_slice(&local_commitment_tx), &crate::ln::chan_utils::TxCreationKeys { inner: keys, _underlying_ref: true }, htlcs.into(), to_self_delay);
		let mut local_ret = match ret.result_good { true => Ok( { let (mut orig_ret_0_0, mut orig_ret_0_1) = (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }).to_rust(); let mut local_orig_ret_0_1 = Vec::new(); for mut item in orig_ret_0_1.into_rust().drain(..) { local_orig_ret_0_1.push( { item.into_rust() }); }; let local_ret_0 = (orig_ret_0_0.into_rust(), local_orig_ret_0_1); local_ret_0 }), false => Err( { () /*(*unsafe { Box::from_raw(ret.contents.err.take_ptr()) })*/ })};
		local_ret
	}
	fn sign_local_commitment<T:bitcoin::secp256k1::Signing + bitcoin::secp256k1::Verification>(&self, local_commitment_tx: &lightning::ln::chan_utils::LocalCommitmentTransaction, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.sign_local_commitment)(self.this_arg, &crate::ln::chan_utils::LocalCommitmentTransaction { inner: local_commitment_tx, _underlying_ref: true });
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(ret.contents.err.take_ptr()) })*/ })};
		local_ret
	}
	fn sign_local_commitment_htlc_transactions<T:bitcoin::secp256k1::Signing + bitcoin::secp256k1::Verification>(&self, local_commitment_tx: &lightning::ln::chan_utils::LocalCommitmentTransaction, local_csv: u16, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<Vec<Option<bitcoin::secp256k1::Signature>>, ()> {
		unimplemented!();
	}
	fn sign_justice_transaction<T:bitcoin::secp256k1::Signing + bitcoin::secp256k1::Verification>(&self, justice_tx: &bitcoin::blockdata::transaction::Transaction, input: usize, amount: u64, per_commitment_key: &bitcoin::secp256k1::key::SecretKey, htlc: &Option<lightning::ln::chan_utils::HTLCOutputInCommitment>, on_remote_tx_csv: u16, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let local_justice_tx = ::bitcoin::consensus::encode::serialize(justice_tx);
		let mut local_htlc = &crate::ln::chan_utils::HTLCOutputInCommitment { inner: if htlc.is_none() { std::ptr::null() } else {  { (htlc.as_ref().unwrap()) } }, _underlying_ref: true };
		let mut ret = (self.sign_justice_transaction)(self.this_arg, crate::c_types::Transaction::from_slice(&local_justice_tx), input, amount, per_commitment_key.as_ref(), local_htlc, on_remote_tx_csv);
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(ret.contents.err.take_ptr()) })*/ })};
		local_ret
	}
	fn sign_remote_htlc_transaction<T:bitcoin::secp256k1::Signing + bitcoin::secp256k1::Verification>(&self, htlc_tx: &bitcoin::blockdata::transaction::Transaction, input: usize, amount: u64, per_commitment_point: &bitcoin::secp256k1::key::PublicKey, htlc: &lightning::ln::chan_utils::HTLCOutputInCommitment, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let local_htlc_tx = ::bitcoin::consensus::encode::serialize(htlc_tx);
		let mut ret = (self.sign_remote_htlc_transaction)(self.this_arg, crate::c_types::Transaction::from_slice(&local_htlc_tx), input, amount, crate::c_types::PublicKey::from_rust(&per_commitment_point), &crate::ln::chan_utils::HTLCOutputInCommitment { inner: htlc, _underlying_ref: true });
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(ret.contents.err.take_ptr()) })*/ })};
		local_ret
	}
	fn sign_closing_transaction<T:bitcoin::secp256k1::Signing>(&self, closing_tx: &bitcoin::blockdata::transaction::Transaction, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let local_closing_tx = ::bitcoin::consensus::encode::serialize(closing_tx);
		let mut ret = (self.sign_closing_transaction)(self.this_arg, crate::c_types::Transaction::from_slice(&local_closing_tx));
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(ret.contents.err.take_ptr()) })*/ })};
		local_ret
	}
	fn sign_channel_announcement<T:bitcoin::secp256k1::Signing>(&self, msg: &lightning::ln::msgs::UnsignedChannelAnnouncement, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let mut ret = (self.sign_channel_announcement)(self.this_arg, &crate::ln::msgs::UnsignedChannelAnnouncement { inner: msg, _underlying_ref: true });
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(ret.contents.err.take_ptr()) })*/ })};
		local_ret
	}
	fn set_remote_channel_pubkeys(&mut self, channel_points: &lightning::ln::chan_utils::ChannelPublicKeys) {
		(self.set_remote_channel_pubkeys)(self.this_arg, &crate::ln::chan_utils::ChannelPublicKeys { inner: channel_points, _underlying_ref: true })
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for ChannelKeys {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// " A trait to describe an object which can get user secrets and key material."
#[repr(C)]
pub struct KeysInterface {
	pub this_arg: *mut c_void,
	/// " Get node secret key (aka node_id or network_key)"
	#[must_use]
	pub get_node_secret: extern "C" fn (this_arg: *const c_void) -> crate::c_types::SecretKey,
	/// " Get destination redeemScript to encumber static protocol exit points."
	#[must_use]
	pub get_destination_script: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::CVec_u8Z,
	/// " Get shutdown_pubkey to use as PublicKey at channel closure"
	#[must_use]
	pub get_shutdown_pubkey: extern "C" fn (this_arg: *const c_void) -> crate::c_types::PublicKey,
	/// " Get a new set of ChannelKeys for per-channel secrets. These MUST be unique even if you"
	/// " restarted with some stale data!"
	#[must_use]
	pub get_channel_keys: extern "C" fn (this_arg: *const c_void, inbound: bool, channel_value_satoshis: u64) -> crate::chain::keysinterface::ChannelKeys,
	/// " Get a secret and PRNG seed for constructing an onion packet"
	#[must_use]
	pub get_onion_rand: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::C2Tuple_SecretKey_u832Z,
	/// " Get a unique temporary channel id. Channels will be referred to by this until the funding"
	/// " transaction is created, at which point they will use the outpoint in the funding"
	/// " transaction."
	#[must_use]
	pub get_channel_id: extern "C" fn (this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes,
}
unsafe impl Send for KeysInterface {}
unsafe impl Sync for KeysInterface {}

use lightning::chain::keysinterface::KeysInterface as lnKeysInterface;
impl lnKeysInterface for KeysInterface {
	type ChanKeySigner = crate::chain::keysinterface::ChannelKeys;
	fn get_node_secret(&self) -> bitcoin::secp256k1::key::SecretKey {
		let mut ret = (self.get_node_secret)(self.this_arg);
		ret.into_rust()
	}
	fn get_destination_script(&self) -> bitcoin::blockdata::script::Script {
		let mut ret = (self.get_destination_script)(self.this_arg);
		::bitcoin::blockdata::script::Script::from(ret.into_rust())
	}
	fn get_shutdown_pubkey(&self) -> bitcoin::secp256k1::key::PublicKey {
		let mut ret = (self.get_shutdown_pubkey)(self.this_arg);
		ret.into_rust()
	}
	fn get_channel_keys(&self, inbound: bool, channel_value_satoshis: u64) -> Self::ChanKeySigner {
		let mut ret = (self.get_channel_keys)(self.this_arg, inbound, channel_value_satoshis);
		ret
	}
	fn get_onion_rand(&self) -> (bitcoin::secp256k1::key::SecretKey, [u8; 32]) {
		let mut ret = (self.get_onion_rand)(self.this_arg);
		let (mut orig_ret_0, mut orig_ret_1) = ret.to_rust(); let local_ret = (orig_ret_0.into_rust(), orig_ret_1.data);
		local_ret
	}
	fn get_channel_id(&self) -> [u8; 32] {
		let mut ret = (self.get_channel_id)(self.this_arg);
		ret.data
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for KeysInterface {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}

use lightning::chain::keysinterface::InMemoryChannelKeys as lnInMemoryChannelKeysImport;
type lnInMemoryChannelKeys = lnInMemoryChannelKeysImport;

/// " A simple implementation of ChannelKeys that just keeps the private keys in memory."
#[must_use]
#[repr(C)]
pub struct InMemoryChannelKeys {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnInMemoryChannelKeys,
	pub _underlying_ref: bool,
}

impl Drop for InMemoryChannelKeys {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnInMemoryChannelKeys) };
		}
	}
}
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_free(this_ptr: InMemoryChannelKeys) { }
impl Clone for InMemoryChannelKeys {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
/// " Private key of anchor tx"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_get_funding_key(this_ptr: &InMemoryChannelKeys) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.funding_key;
	(*inner_val).as_ref()
}
/// " Private key of anchor tx"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_set_funding_key(this_ptr: &mut InMemoryChannelKeys, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnInMemoryChannelKeys) }.funding_key = val.into_rust();
}
/// " Local secret key for blinded revocation pubkey"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_get_revocation_base_key(this_ptr: &InMemoryChannelKeys) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.revocation_base_key;
	(*inner_val).as_ref()
}
/// " Local secret key for blinded revocation pubkey"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_set_revocation_base_key(this_ptr: &mut InMemoryChannelKeys, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnInMemoryChannelKeys) }.revocation_base_key = val.into_rust();
}
/// " Local secret key used for our balance in remote-broadcasted commitment transactions"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_get_payment_key(this_ptr: &InMemoryChannelKeys) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.payment_key;
	(*inner_val).as_ref()
}
/// " Local secret key used for our balance in remote-broadcasted commitment transactions"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_set_payment_key(this_ptr: &mut InMemoryChannelKeys, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnInMemoryChannelKeys) }.payment_key = val.into_rust();
}
/// " Local secret key used in HTLC tx"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_get_delayed_payment_base_key(this_ptr: &InMemoryChannelKeys) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.delayed_payment_base_key;
	(*inner_val).as_ref()
}
/// " Local secret key used in HTLC tx"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_set_delayed_payment_base_key(this_ptr: &mut InMemoryChannelKeys, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnInMemoryChannelKeys) }.delayed_payment_base_key = val.into_rust();
}
/// " Local htlc secret key used in commitment tx htlc outputs"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_get_htlc_base_key(this_ptr: &InMemoryChannelKeys) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.htlc_base_key;
	(*inner_val).as_ref()
}
/// " Local htlc secret key used in commitment tx htlc outputs"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_set_htlc_base_key(this_ptr: &mut InMemoryChannelKeys, mut val: crate::c_types::SecretKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnInMemoryChannelKeys) }.htlc_base_key = val.into_rust();
}
/// " Commitment seed"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_get_commitment_seed(this_ptr: &InMemoryChannelKeys) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.commitment_seed;
	&(*inner_val)
}
/// " Commitment seed"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_set_commitment_seed(this_ptr: &mut InMemoryChannelKeys, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *(this_ptr.inner as *mut lnInMemoryChannelKeys) }.commitment_seed = val.data;
}
/// " Create a new InMemoryChannelKeys"
#[must_use]
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_new(mut funding_key: crate::c_types::SecretKey, mut revocation_base_key: crate::c_types::SecretKey, mut payment_key: crate::c_types::SecretKey, mut delayed_payment_base_key: crate::c_types::SecretKey, mut htlc_base_key: crate::c_types::SecretKey, mut commitment_seed: crate::c_types::ThirtyTwoBytes, mut channel_value_satoshis: u64, mut key_derivation_params: crate::c_types::derived::C2Tuple_u64u64Z) -> crate::chain::keysinterface::InMemoryChannelKeys {
	let (mut orig_key_derivation_params_0, mut orig_key_derivation_params_1) = key_derivation_params.to_rust(); let local_key_derivation_params = (orig_key_derivation_params_0, orig_key_derivation_params_1);
	let mut ret = lightning::chain::keysinterface::InMemoryChannelKeys::new(&bitcoin::secp256k1::Secp256k1::new(), funding_key.into_rust(), revocation_base_key.into_rust(), payment_key.into_rust(), delayed_payment_base_key.into_rust(), htlc_base_key.into_rust(), commitment_seed.data, channel_value_satoshis, local_key_derivation_params);
	crate::chain::keysinterface::InMemoryChannelKeys { inner: Box::into_raw(Box::new(ret)), _underlying_ref: false }
}

#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_as_ChannelKeys(this_arg: *const InMemoryChannelKeys) -> crate::chain::keysinterface::ChannelKeys {
	crate::chain::keysinterface::ChannelKeys {
		this_arg: unsafe { (*this_arg).inner as *mut c_void },

		commitment_seed: crate::c_types::ThirtyTwoBytes { data: [0; 32] },
		set_commitment_seed: Some(InMemoryChannelKeys_ChannelKeys_set_commitment_seed),

		pubkeys: crate::ln::chan_utils::ChannelPublicKeys { inner: std::ptr::null(), _underlying_ref: false },
		set_pubkeys: Some(InMemoryChannelKeys_ChannelKeys_set_pubkeys),
		key_derivation_params: InMemoryChannelKeys_ChannelKeys_key_derivation_params,
		sign_remote_commitment: InMemoryChannelKeys_ChannelKeys_sign_remote_commitment,
		sign_local_commitment: InMemoryChannelKeys_ChannelKeys_sign_local_commitment,
		//XXX: Need to export sign_local_commitment_htlc_transactions
		sign_justice_transaction: InMemoryChannelKeys_ChannelKeys_sign_justice_transaction,
		sign_remote_htlc_transaction: InMemoryChannelKeys_ChannelKeys_sign_remote_htlc_transaction,
		sign_closing_transaction: InMemoryChannelKeys_ChannelKeys_sign_closing_transaction,
		sign_channel_announcement: InMemoryChannelKeys_ChannelKeys_sign_channel_announcement,
		set_remote_channel_pubkeys: InMemoryChannelKeys_ChannelKeys_set_remote_channel_pubkeys,
	}
}
use lightning::chain::keysinterface::ChannelKeys as ChannelKeysTraitImport;
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_commitment_seed(this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes {
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.commitment_seed();
	let mut ret = unsafe { (*ret).clone() };
	crate::c_types::ThirtyTwoBytes { data: ret }
}
extern "C" fn InMemoryChannelKeys_ChannelKeys_set_commitment_seed(trait_self_arg: &ChannelKeys) {
	// This is a bit race-y in the general case, but for our specific use-cases today, we're safe
	// Specifically, we must ensure that the first time we're called it can never be in parallel
	if trait_self_arg.commitment_seed.data == [0; 32] {
		unsafe { &mut *(trait_self_arg as *const ChannelKeys  as *mut ChannelKeys) }.commitment_seed = InMemoryChannelKeys_ChannelKeys_commitment_seed(trait_self_arg.this_arg);
	}
}
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_pubkeys(this_arg: *const c_void) -> crate::ln::chan_utils::ChannelPublicKeys {
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.pubkeys();
	let mut ret = unsafe { (*ret).clone() };
	crate::ln::chan_utils::ChannelPublicKeys { inner: Box::into_raw(Box::new(ret)), _underlying_ref: false }
}
extern "C" fn InMemoryChannelKeys_ChannelKeys_set_pubkeys(trait_self_arg: &ChannelKeys) {
	// This is a bit race-y in the general case, but for our specific use-cases today, we're safe
	// Specifically, we must ensure that the first time we're called it can never be in parallel
	if trait_self_arg.pubkeys.inner.is_null() {
		unsafe { &mut *(trait_self_arg as *const ChannelKeys  as *mut ChannelKeys) }.pubkeys = InMemoryChannelKeys_ChannelKeys_pubkeys(trait_self_arg.this_arg);
	}
}
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_key_derivation_params(this_arg: *const c_void) -> crate::c_types::derived::C2Tuple_u64u64Z {
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.key_derivation_params();
	let (mut orig_ret_0, mut orig_ret_1) = ret; let local_ret = (orig_ret_0, orig_ret_1).into();
	local_ret
}
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_sign_remote_commitment(this_arg: *const c_void, mut feerate_per_kw: u64, commitment_tx: crate::c_types::Transaction, keys: &crate::ln::chan_utils::TxCreationKeys, htlcs: crate::c_types::derived::CHTLCOutputInCommitmentSlice, mut to_self_delay: u16) -> crate::c_types::derived::CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ {
	let local_htlcs = htlcs.into_vec();
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.sign_remote_commitment(feerate_per_kw, &commitment_tx.into_bitcoin(), unsafe { &*keys.inner }, &local_htlcs[..], to_self_delay, &bitcoin::secp256k1::Secp256k1::new());
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { let (mut orig_ret_0_0, mut orig_ret_0_1) = o; let mut local_orig_ret_0_1 = Vec::new(); for item in orig_ret_0_1.drain(..) { local_orig_ret_0_1.push( { crate::c_types::Signature::from_rust(&item) }); }; let local_ret_0 = (crate::c_types::Signature::from_rust(&orig_ret_0_0), local_orig_ret_0_1.into()).into(); local_ret_0 }), Err(mut e) => crate::c_types::CResultTempl::err( { 0u8 /*e*/ }) };
	local_ret
}
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_sign_local_commitment(this_arg: *const c_void, local_commitment_tx: &crate::ln::chan_utils::LocalCommitmentTransaction) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.sign_local_commitment(unsafe { &*local_commitment_tx.inner }, &bitcoin::secp256k1::Secp256k1::new());
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { crate::c_types::Signature::from_rust(&o) }), Err(mut e) => crate::c_types::CResultTempl::err( { 0u8 /*e*/ }) };
	local_ret
}
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_sign_justice_transaction(this_arg: *const c_void, justice_tx: crate::c_types::Transaction, mut input: usize, mut amount: u64, per_commitment_key: *const [u8; 32], htlc: &crate::ln::chan_utils::HTLCOutputInCommitment, mut on_remote_tx_csv: u16) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut local_htlc = if htlc.inner.is_null() { None } else { Some((* { unsafe { &*(*htlc).inner } }).clone()) };
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.sign_justice_transaction(&justice_tx.into_bitcoin(), input, amount, &::bitcoin::secp256k1::key::SecretKey::from_slice(&unsafe { *per_commitment_key}[..]).unwrap(), &local_htlc, on_remote_tx_csv, &bitcoin::secp256k1::Secp256k1::new());
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { crate::c_types::Signature::from_rust(&o) }), Err(mut e) => crate::c_types::CResultTempl::err( { 0u8 /*e*/ }) };
	local_ret
}
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_sign_remote_htlc_transaction(this_arg: *const c_void, htlc_tx: crate::c_types::Transaction, mut input: usize, mut amount: u64, per_commitment_point: crate::c_types::PublicKey, htlc: &crate::ln::chan_utils::HTLCOutputInCommitment) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.sign_remote_htlc_transaction(&htlc_tx.into_bitcoin(), input, amount, &per_commitment_point.into_rust(), unsafe { &*htlc.inner }, &bitcoin::secp256k1::Secp256k1::new());
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { crate::c_types::Signature::from_rust(&o) }), Err(mut e) => crate::c_types::CResultTempl::err( { 0u8 /*e*/ }) };
	local_ret
}
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_sign_closing_transaction(this_arg: *const c_void, closing_tx: crate::c_types::Transaction) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.sign_closing_transaction(&closing_tx.into_bitcoin(), &bitcoin::secp256k1::Secp256k1::new());
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { crate::c_types::Signature::from_rust(&o) }), Err(mut e) => crate::c_types::CResultTempl::err( { 0u8 /*e*/ }) };
	local_ret
}
#[must_use]
extern "C" fn InMemoryChannelKeys_ChannelKeys_sign_channel_announcement(this_arg: *const c_void, msg: &crate::ln::msgs::UnsignedChannelAnnouncement) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.sign_channel_announcement(unsafe { &*msg.inner }, &bitcoin::secp256k1::Secp256k1::new());
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { crate::c_types::Signature::from_rust(&o) }), Err(mut e) => crate::c_types::CResultTempl::err( { 0u8 /*e*/ }) };
	local_ret
}
extern "C" fn InMemoryChannelKeys_ChannelKeys_set_remote_channel_pubkeys(this_arg: *mut c_void, channel_pubkeys: &crate::ln::chan_utils::ChannelPublicKeys) {
	unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.set_remote_channel_pubkeys(unsafe { &*channel_pubkeys.inner })
}

#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_write(obj: *const InMemoryChannelKeys) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_read(ser: crate::c_types::u8slice) -> InMemoryChannelKeys {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		InMemoryChannelKeys { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		InMemoryChannelKeys { inner: std::ptr::null(), _underlying_ref: false }
	}
}

use lightning::chain::keysinterface::KeysManager as lnKeysManagerImport;
type lnKeysManager = lnKeysManagerImport;

/// " Simple KeysInterface implementor that takes a 32-byte seed for use as a BIP 32 extended key"
/// " and derives keys from that."
/// ""
/// " Your node_id is seed/0'"
/// " ChannelMonitor closes may use seed/1'"
/// " Cooperative closes may use seed/2'"
/// " The two close keys may be needed to claim on-chain funds!"
#[must_use]
#[repr(C)]
pub struct KeysManager {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnKeysManager,
	pub _underlying_ref: bool,
}

impl Drop for KeysManager {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnKeysManager) };
		}
	}
}
#[no_mangle]
pub extern "C" fn KeysManager_free(this_ptr: KeysManager) { }
/// " Constructs a KeysManager from a 32-byte seed. If the seed is in some way biased (eg your"
/// " RNG is busted) this may panic (but more importantly, you will possibly lose funds)."
/// " starting_time isn't strictly required to actually be a time, but it must absolutely,"
/// " without a doubt, be unique to this instance. ie if you start multiple times with the same"
/// " seed, starting_time must be unique to each run. Thus, the easiest way to achieve this is to"
/// " simply use the current time (with very high precision)."
/// ""
/// " The seed MUST be backed up safely prior to use so that the keys can be re-created, however,"
/// " obviously, starting_time should be unique every time you reload the library - it is only"
/// " used to generate new ephemeral key data (which will be stored by the individual channel if"
/// " necessary)."
/// ""
/// " Note that the seed is required to recover certain on-chain funds independent of"
/// " ChannelMonitor data, though a current copy of ChannelMonitor data is also required for any"
/// " channel, and some on-chain during-closing funds."
/// ""
/// " Note that until the 0.1 release there is no guarantee of backward compatibility between"
/// " versions. Once the library is more fully supported, the docs will be updated to include a"
/// " detailed description of the guarantee."
#[must_use]
#[no_mangle]
pub extern "C" fn KeysManager_new(seed: *const [u8; 32], mut network: crate::bitcoin::network::Network, mut starting_time_secs: u64, mut starting_time_nanos: u32) -> KeysManager {
	let mut ret = lightning::chain::keysinterface::KeysManager::new(unsafe { &*seed}, network.into_bitcoin(), starting_time_secs, starting_time_nanos);
	KeysManager { inner: Box::into_raw(Box::new(ret)), _underlying_ref: true }
}

/// " Derive an old set of ChannelKeys for per-channel secrets based on a key derivation"
/// " parameters."
/// " Key derivation parameters are accessible through a per-channel secrets"
/// " ChannelKeys::key_derivation_params and is provided inside DynamicOuputP2WSH in case of"
/// " onchain output detection for which a corresponding delayed_payment_key must be derived."
#[must_use]
#[no_mangle]
pub extern "C" fn KeysManager_derive_channel_keys(this_arg: &KeysManager, mut channel_value_satoshis: u64, mut params_1: u64, mut params_2: u64) -> crate::chain::keysinterface::InMemoryChannelKeys {
	let mut ret = unsafe { &*this_arg.inner }.derive_channel_keys(channel_value_satoshis, params_1, params_2);
	crate::chain::keysinterface::InMemoryChannelKeys { inner: Box::into_raw(Box::new(ret)), _underlying_ref: false }
}

#[no_mangle]
pub extern "C" fn KeysManager_as_KeysInterface(arg_a: u8, mut arg_b: u8, this_arg: *const KeysManager) -> crate::chain::keysinterface::KeysInterface {
	println!("kmaki arg_a: {}", arg_a);
	println!("kmaki arg_b: {}", arg_b);
	unsafe { println!("kmaki: has underlying ref? {}", (*this_arg)._underlying_ref); }
	crate::chain::keysinterface::KeysInterface {
		this_arg: unsafe { (*this_arg).inner as *mut c_void },
		get_node_secret: KeysManager_KeysInterface_get_node_secret,
		get_destination_script: KeysManager_KeysInterface_get_destination_script,
		get_shutdown_pubkey: KeysManager_KeysInterface_get_shutdown_pubkey,
		get_channel_keys: KeysManager_KeysInterface_get_channel_keys,
		get_onion_rand: KeysManager_KeysInterface_get_onion_rand,
		get_channel_id: KeysManager_KeysInterface_get_channel_id,
	}
}

#[repr(C)]
pub union ArikUnionType {
	pub eight: u8,
	pub sixteen: u16
}

#[repr(C)]
pub struct ArikType {
	pub first: *const ArikUnionType,
	pub message: *const crate::c_types::derived::CResult_CVec_u8ZPeerHandleErrorZ,
	// pub union_max: u16,
	pub second: u8,
	pub third: u8,
	pub fourth: u8,
	pub fifth: u8,
	pub sixth: u8,
	pub seventh: u8,
	pub eigth: u8,
	pub ninth: u8,
	pub tenth: u8,
	pub eleventh: u8,
	pub twelvth: u8,
	pub thirteenth: u8,
	pub fourteenth: u8,
	// pub fifteenth: u8
}

#[repr(C)]
pub struct ArikSubType {
	pub fifth: u8,
	pub sixth: u8,
	pub seventh: u8,
	// eigth: u8,
}

#[no_mangle]
pub extern "C" fn arik_test_km(arg_a: u8, mut arg_b: u8, this_arg: *const KeysManager) -> ArikType {
	println!("arik_test_km");

	println!("size of ariktype: {}", std::mem::size_of::<ArikType>());

	println!("arg_a: {}", arg_a);
	println!("arg_b: {}", arg_b);


	let union = ArikUnionType{
		eight: 2
	};
	let ptr = Box::into_raw(Box::new(union));

	ArikType{
		// union_max: 3,
		first: ptr,
		message: std::ptr::null(),
		second: 0,
		third: 0,
		fourth: 0,
		fifth: 0,
		sixth: 0,
		seventh: 0,
		eigth: 0,
		ninth: 0,
		tenth: 0,
		eleventh: 0,
		twelvth: 0,
		thirteenth: 0,
		fourteenth: 0,
		// fifteenth: 0
	}

	// ArikType {
	// 	first: 15,
	// 	second: 16,
	// 	third: 17,
	// 	// fourth: 18,
	// 	sub: ArikSubType{
	// 		fifth: 20,
	// 		sixth: 21,
	// 		seventh: 22,
	// 		// eigth: 23
	// 	}
	// }
}


use lightning::chain::keysinterface::KeysInterface as KeysInterfaceTraitImport;
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_node_secret(this_arg: *const c_void) -> crate::c_types::SecretKey {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_node_secret();
	crate::c_types::SecretKey::from_rust(ret)
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_destination_script(this_arg: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_destination_script();
	ret.into_bytes().into()
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_shutdown_pubkey(this_arg: *const c_void) -> crate::c_types::PublicKey {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_shutdown_pubkey();
	crate::c_types::PublicKey::from_rust(&ret)
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_channel_keys(this_arg: *const c_void, mut _inbound: bool, mut channel_value_satoshis: u64) -> crate::chain::keysinterface::ChannelKeys {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_channel_keys(_inbound, channel_value_satoshis);
	ChannelKeys {
		this_arg: Box::into_raw(Box::new(ret)) as *mut c_void,
		commitment_seed: crate::c_types::ThirtyTwoBytes { data: [0; 32] },
		set_commitment_seed: Some(InMemoryChannelKeys_ChannelKeys_set_commitment_seed),
		pubkeys: crate::ln::chan_utils::ChannelPublicKeys { inner: std::ptr::null(), _underlying_ref: false },
		set_pubkeys: Some(InMemoryChannelKeys_ChannelKeys_set_pubkeys),
		key_derivation_params: InMemoryChannelKeys_ChannelKeys_key_derivation_params,
		sign_remote_commitment: InMemoryChannelKeys_ChannelKeys_sign_remote_commitment,
		sign_local_commitment: InMemoryChannelKeys_ChannelKeys_sign_local_commitment,
		sign_justice_transaction: InMemoryChannelKeys_ChannelKeys_sign_justice_transaction,
		sign_remote_htlc_transaction: InMemoryChannelKeys_ChannelKeys_sign_remote_htlc_transaction,
		sign_closing_transaction: InMemoryChannelKeys_ChannelKeys_sign_closing_transaction,
		sign_channel_announcement: InMemoryChannelKeys_ChannelKeys_sign_channel_announcement,
		set_remote_channel_pubkeys: InMemoryChannelKeys_ChannelKeys_set_remote_channel_pubkeys,
	}
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_onion_rand(this_arg: *const c_void) -> crate::c_types::derived::C2Tuple_SecretKey_u832Z {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_onion_rand();
	let (mut orig_ret_0, mut orig_ret_1) = ret; let local_ret = (crate::c_types::SecretKey::from_rust(orig_ret_0), crate::c_types::ThirtyTwoBytes { data: orig_ret_1 }).into();
	local_ret
}
#[must_use]
extern "C" fn KeysManager_KeysInterface_get_channel_id(this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_channel_id();
	crate::c_types::ThirtyTwoBytes { data: ret }
}

