//! " keysinterface provides keys into rust-lightning and defines some useful enums which describe"
//! " spendable on-chain outputs which the user owns and is responsible for using just as any other"
//! " on-chain output which is theirs."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::TakePointer;


use lightning::chain::keysinterface::SpendableOutputDescriptor as lnSpendableOutputDescriptorImport;
type lnSpendableOutputDescriptor = lnSpendableOutputDescriptorImport;

/// " When on-chain outputs are created by rust-lightning (which our counterparty is not able to"
/// " claim at any point in the future) an event is generated which you must track and be able to"
/// " spend on-chain. The information needed to do this is provided in this enum, including the"
/// " outpoint describing which txid and output index is available, the full output which exists at"
/// " that txid/index, and any keys or other information required to sign."
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
	//XXX: Need to export funding_key
	//XXX: Need to export revocation_base_key
	//XXX: Need to export payment_key
	//XXX: Need to export delayed_payment_base_key
	//XXX: Need to export htlc_base_key
	//XXX: Need to export commitment_seed
	//XXX: Need to export pubkeys
	//XXX: Need to export sign_remote_commitment
	//XXX: Need to export sign_local_commitment
	//XXX: Need to export sign_local_commitment_htlc_transactions
	/// " Create a signature for a (proposed) closing transaction."
	/// ""
	/// " Note that, due to rounding, there may be one \"missing\" satoshi, and either party may have"
	/// " chosen to forgo their output as dust."
	pub sign_closing_transaction: extern "C" fn (this_arg: *const c_void, closing_tx: crate::c_types::Transaction) -> crate::c_types::derived::CResult_SignatureNoneZ,
	//XXX: Need to export sign_channel_announcement
	/// " Set the remote channel basepoints.  This is done immediately on incoming channels"
	/// " and as soon as the channel is accepted on outgoing channels."
	/// ""
	/// " Will be called before any signatures are applied."
	pub set_remote_channel_pubkeys: extern "C" fn (this_arg: *mut c_void, channel_points: &crate::ln::chan_utils::ChannelPublicKeys),
}
unsafe impl Send for ChannelKeys {}

use lightning::chain::keysinterface::ChannelKeys as lnChannelKeys;
impl lnChannelKeys for ChannelKeys {
	fn funding_key<'a>(&'a self) -> &'a bitcoin::secp256k1::key::SecretKey {
		unimplemented!();
	}
	fn revocation_base_key<'a>(&'a self) -> &'a bitcoin::secp256k1::key::SecretKey {
		unimplemented!();
	}
	fn payment_key<'a>(&'a self) -> &'a bitcoin::secp256k1::key::SecretKey {
		unimplemented!();
	}
	fn delayed_payment_base_key<'a>(&'a self) -> &'a bitcoin::secp256k1::key::SecretKey {
		unimplemented!();
	}
	fn htlc_base_key<'a>(&'a self) -> &'a bitcoin::secp256k1::key::SecretKey {
		unimplemented!();
	}
	fn commitment_seed<'a>(&'a self) -> &'a [u8; 32] {
		unimplemented!();
	}
	fn pubkeys<'a>(&'a self) -> &'a lightning::ln::chan_utils::ChannelPublicKeys {
		unimplemented!();
	}
	fn sign_remote_commitment<T:bitcoin::secp256k1::Signing + bitcoin::secp256k1::Verification>(&self, feerate_per_kw: u64, commitment_tx: &bitcoin::blockdata::transaction::Transaction, keys: &lightning::ln::chan_utils::TxCreationKeys, htlcs: &[&lightning::ln::chan_utils::HTLCOutputInCommitment], to_self_delay: u16, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<(bitcoin::secp256k1::Signature, Vec<bitcoin::secp256k1::Signature>), ()> {
		unimplemented!();
	}
	fn sign_local_commitment<T:bitcoin::secp256k1::Signing + bitcoin::secp256k1::Verification>(&self, local_commitment_tx: &lightning::ln::chan_utils::LocalCommitmentTransaction, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		unimplemented!();
	}
	fn sign_local_commitment_htlc_transactions<T:bitcoin::secp256k1::Signing + bitcoin::secp256k1::Verification>(&self, local_commitment_tx: &lightning::ln::chan_utils::LocalCommitmentTransaction, local_csv: u16, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<Vec<Option<bitcoin::secp256k1::Signature>>, ()> {
		unimplemented!();
	}
	fn sign_closing_transaction<T:bitcoin::secp256k1::Signing>(&self, closing_tx: &bitcoin::blockdata::transaction::Transaction, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		let local_closing_tx = ::bitcoin::consensus::encode::serialize(closing_tx);
		let mut ret = (self.sign_closing_transaction)(self.this_arg, crate::c_types::Transaction::from_slice(&local_closing_tx));
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result) }).into_rust() }), false => Err( { () /*(*unsafe { Box::from_raw(ret.contents.err) })*/ })};
		local_ret
	}
	fn sign_channel_announcement<T:bitcoin::secp256k1::Signing>(&self, msg: &lightning::ln::msgs::UnsignedChannelAnnouncement, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		unimplemented!();
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
	pub get_node_secret: extern "C" fn (this_arg: *const c_void) -> crate::c_types::SecretKey,
	/// " Get destination redeemScript to encumber static protocol exit points."
	pub get_destination_script: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::CVec_u8Z,
	/// " Get shutdown_pubkey to use as PublicKey at channel closure"
	pub get_shutdown_pubkey: extern "C" fn (this_arg: *const c_void) -> crate::c_types::PublicKey,
	/// " Get a new set of ChannelKeys for per-channel secrets. These MUST be unique even if you"
	/// " restarted with some stale data!"
	pub get_channel_keys: extern "C" fn (this_arg: *const c_void, inbound: bool, channel_value_satoshis: u64) -> ChannelKeys,
	/// " Get a secret and PRNG seed for constructing an onion packet"
	pub get_onion_rand: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::C2Tuple_SecretKey_u832Z,
	/// " Get a unique temporary channel id. Channels will be referred to by this until the funding"
	/// " transaction is created, at which point they will use the outpoint in the funding"
	/// " transaction."
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
		let orig_ret = ret.to_rust(); let local_ret = (orig_ret.0.into_rust(), orig_ret.1.data);
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
/// " Create a new InMemoryChannelKeys"
#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_new(mut funding_key: crate::c_types::SecretKey, mut revocation_base_key: crate::c_types::SecretKey, mut payment_key: crate::c_types::SecretKey, mut delayed_payment_base_key: crate::c_types::SecretKey, mut htlc_base_key: crate::c_types::SecretKey, mut commitment_seed: crate::c_types::ThirtyTwoBytes, mut channel_value_satoshis: u64) -> InMemoryChannelKeys {
	let mut ret = lightning::chain::keysinterface::InMemoryChannelKeys::new(&bitcoin::secp256k1::Secp256k1::new(), funding_key.into_rust(), revocation_base_key.into_rust(), payment_key.into_rust(), delayed_payment_base_key.into_rust(), htlc_base_key.into_rust(), commitment_seed.data, channel_value_satoshis);
	crate::chain::keysinterface::InMemoryChannelKeys { inner: Box::into_raw(Box::new(ret)), _underlying_ref: false }
}

#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_as_ChannelKeys(this_arg: *const InMemoryChannelKeys) -> crate::chain::keysinterface::ChannelKeys {
	crate::chain::keysinterface::ChannelKeys {
		this_arg: unsafe { (*this_arg).inner as *mut c_void },
		//XXX: Need to export funding_key
		//XXX: Need to export revocation_base_key
		//XXX: Need to export payment_key
		//XXX: Need to export delayed_payment_base_key
		//XXX: Need to export htlc_base_key
		//XXX: Need to export commitment_seed
		//XXX: Need to export pubkeys
		//XXX: Need to export sign_remote_commitment
		//XXX: Need to export sign_local_commitment
		//XXX: Need to export sign_local_commitment_htlc_transactions
		sign_closing_transaction: InMemoryChannelKeys_ChannelKeys_sign_closing_transaction,
		//XXX: Need to export sign_channel_announcement
		set_remote_channel_pubkeys: InMemoryChannelKeys_ChannelKeys_set_remote_channel_pubkeys,
	}
}
use lightning::chain::keysinterface::ChannelKeys as ChannelKeysTraitImport;
extern "C" fn InMemoryChannelKeys_ChannelKeys_sign_closing_transaction(this_arg: *const c_void, closing_tx: crate::c_types::Transaction) -> crate::c_types::derived::CResult_SignatureNoneZ {
	let mut ret = unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.sign_closing_transaction(&closing_tx.into_bitcoin(), &bitcoin::secp256k1::Secp256k1::new());
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { crate::c_types::Signature::from_rust(&o) }), Err(mut e) => crate::c_types::CResultTempl::err( { 0u8 /*e*/ }) };
	local_ret
}
extern "C" fn InMemoryChannelKeys_ChannelKeys_set_remote_channel_pubkeys(this_arg: *mut c_void, channel_pubkeys: &crate::ln::chan_utils::ChannelPublicKeys) {
	unsafe { &mut *(this_arg as *mut lnInMemoryChannelKeys) }.set_remote_channel_pubkeys(unsafe { &*channel_pubkeys.inner })
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
#[no_mangle]
pub extern "C" fn KeysManager_new(seed: *const [u8; 32], mut network: crate::bitcoin::network::Network, mut starting_time_secs: u64, mut starting_time_nanos: u32) -> KeysManager {
	let mut ret = lightning::chain::keysinterface::KeysManager::new(unsafe { &*seed}, network.into_bitcoin(), starting_time_secs, starting_time_nanos);
	KeysManager { inner: Box::into_raw(Box::new(ret)), _underlying_ref: false }
}

#[no_mangle]
pub extern "C" fn KeysManager_as_KeysInterface(this_arg: *const KeysManager) -> crate::chain::keysinterface::KeysInterface {
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
use lightning::chain::keysinterface::KeysInterface as KeysInterfaceTraitImport;
extern "C" fn KeysManager_KeysInterface_get_node_secret(this_arg: *const c_void) -> crate::c_types::SecretKey {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_node_secret();
	crate::c_types::SecretKey::from_rust(ret)
}
extern "C" fn KeysManager_KeysInterface_get_destination_script(this_arg: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_destination_script();
	ret.into_bytes().into()
}
extern "C" fn KeysManager_KeysInterface_get_shutdown_pubkey(this_arg: *const c_void) -> crate::c_types::PublicKey {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_shutdown_pubkey();
	crate::c_types::PublicKey::from_rust(&ret)
}
extern "C" fn KeysManager_KeysInterface_get_channel_keys(this_arg: *const c_void, mut _inbound: bool, mut channel_value_satoshis: u64) -> ChannelKeys {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_channel_keys(_inbound, channel_value_satoshis);
	ChannelKeys {
		this_arg: Box::into_raw(Box::new(ret)) as *mut c_void,
		sign_closing_transaction: InMemoryChannelKeys_ChannelKeys_sign_closing_transaction,
		set_remote_channel_pubkeys: InMemoryChannelKeys_ChannelKeys_set_remote_channel_pubkeys,
	}
}
extern "C" fn KeysManager_KeysInterface_get_onion_rand(this_arg: *const c_void) -> crate::c_types::derived::C2Tuple_SecretKey_u832Z {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_onion_rand();
	let orig_ret = ret; let local_ret = (crate::c_types::SecretKey::from_rust(orig_ret.0), crate::c_types::ThirtyTwoBytes { data: orig_ret.1 }).into();
	local_ret
}
extern "C" fn KeysManager_KeysInterface_get_channel_id(this_arg: *const c_void) -> crate::c_types::ThirtyTwoBytes {
	let mut ret = unsafe { &mut *(this_arg as *mut lnKeysManager) }.get_channel_id();
	crate::c_types::ThirtyTwoBytes { data: ret }
}

