//! " keysinterface provides keys into rust-lightning and defines some useful enums which describe"
//! " spendable on-chain outputs which the user owns and is responsible for using just as any other"
//! " on-chain output which is theirs."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

use bitcoin::blockdata::transaction::Transaction as lnTransaction;
use bitcoin::blockdata::transaction::OutPoint as lnOutPoint;
use bitcoin::blockdata::transaction::TxOut as lnTxOut;
use bitcoin::blockdata::script::Script as lnScript;
use bitcoin::blockdata::script::Builder as lnBuilder;
use bitcoin::blockdata::opcodes as lnopcodes;
use bitcoin::network::constants::Network as lnNetwork;
use bitcoin::util::bip32::ExtendedPrivKey as lnExtendedPrivKey;
use bitcoin::util::bip32::ExtendedPubKey as lnExtendedPubKey;
use bitcoin::util::bip32::ChildNumber as lnChildNumber;
use bitcoin::util::bip143 as lnbip143;
use bitcoin::hashes::Hash as lnHash;
use bitcoin::hashes::HashEngine as lnHashEngine;
use bitcoin::hash_types::WPubkeyHash as lnWPubkeyHash;
use bitcoin::secp256k1::key::SecretKey as lnSecretKey;
use bitcoin::secp256k1::key::PublicKey as lnPublicKey;
use bitcoin::secp256k1::Secp256k1 as lnSecp256k1;
use bitcoin::secp256k1::Signature as lnSignature;
use bitcoin::secp256k1::Signing as lnSigning;
use bitcoin::secp256k1 as lnsecp256k1;

use lightning::chain::keysinterface::SpendableOutputDescriptor as lnSpendableOutputDescriptorImport;
type lnSpendableOutputDescriptor = lnSpendableOutputDescriptorImport;

/// " When on-chain outputs are created by rust-lightning (which our counterparty is not able to"
/// " claim at any point in the future) an event is generated which you must track and be able to"
/// " spend on-chain. The information needed to do this is provided in this enum, including the"
/// " outpoint describing which txid and output index is available, the full output which exists at"
/// " that txid/index, and any keys or other information required to sign."
#[repr(C)]
pub struct SpendableOutputDescriptor {
	pub(crate) inner: *const lnSpendableOutputDescriptor,
}

#[no_mangle]
pub extern "C" fn SpendableOutputDescriptor_free(this_ptr: SpendableOutputDescriptor) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnSpendableOutputDescriptor) };
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
	//XXX: Need to export sign_closing_transaction
	//XXX: Need to export sign_channel_announcement
	//XXX: Need to export set_remote_channel_pubkeys
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
		unimplemented!();
	}
	fn sign_channel_announcement<T:bitcoin::secp256k1::Signing>(&self, msg: &lightning::ln::msgs::UnsignedChannelAnnouncement, _secp_ctx: &bitcoin::secp256k1::Secp256k1<T>) -> Result<bitcoin::secp256k1::Signature, ()> {
		unimplemented!();
	}
	fn set_remote_channel_pubkeys(&mut self, channel_points: &lightning::ln::chan_utils::ChannelPublicKeys) {
		unimplemented!();
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
	pub get_node_secret: extern "C" fn (this_arg: *const  c_void) -> crate::c_types::SecretKey,
	/// " Get destination redeemScript to encumber static protocol exit points."
	pub get_destination_script: extern "C" fn (this_arg: *const  c_void) -> crate::c_types::Script,
	/// " Get shutdown_pubkey to use as PublicKey at channel closure"
	pub get_shutdown_pubkey: extern "C" fn (this_arg: *const  c_void) -> crate::c_types::PublicKey,
	//XXX: Need to export get_channel_keys
	//XXX: Need to export get_onion_rand
	/// " Get a unique temporary channel id. Channels will be referred to by this until the funding"
	/// " transaction is created, at which point they will use the outpoint in the funding"
	/// " transaction."
	pub get_channel_id: extern "C" fn (this_arg: *const  c_void) -> crate::c_types::ThirtyTwoBytes,
}
unsafe impl Send for KeysInterface {}
unsafe impl Sync for KeysInterface {}

use lightning::chain::keysinterface::KeysInterface as lnKeysInterface;
impl lnKeysInterface for KeysInterface {
	type ChanKeySigner = crate::chain::keysinterface::ChannelKeys;
	fn get_node_secret(&self) -> bitcoin::secp256k1::key::SecretKey {
		(self.get_node_secret)(self.this_arg).into_rust()
	}
	fn get_destination_script(&self) -> bitcoin::blockdata::script::Script {
		(self.get_destination_script)(self.this_arg).into_bitcoin()
	}
	fn get_shutdown_pubkey(&self) -> bitcoin::secp256k1::key::PublicKey {
		(self.get_shutdown_pubkey)(self.this_arg).into_rust()
	}
	fn get_channel_keys(&self, inbound: bool, channel_value_satoshis: u64) -> Self::ChanKeySigner {
		unimplemented!();
	}
	fn get_onion_rand(&self) -> (bitcoin::secp256k1::key::SecretKey, [u8; 32]) {
		unimplemented!();
	}
	fn get_channel_id(&self) -> [u8; 32] {
		(self.get_channel_id)(self.this_arg).data
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
	pub(crate) inner: *const lnInMemoryChannelKeys,
}

#[no_mangle]
pub extern "C" fn InMemoryChannelKeys_free(this_ptr: InMemoryChannelKeys) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnInMemoryChannelKeys) };
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
	pub(crate) inner: *const lnKeysManager,
}

#[no_mangle]
pub extern "C" fn KeysManager_free(this_ptr: KeysManager) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnKeysManager) };
}
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
pub extern "C" fn KeysManager_new(seed: *const [u8; 32], network: crate::bitcoin::network::Network, logger: crate::util::logger::Logger, starting_time_secs: u64, starting_time_nanos: u32) -> KeysManager {
	let rust_logger = std::sync::Arc::new(logger);
	let ret = lightning::chain::keysinterface::KeysManager::new(unsafe { &*seed}, network.into_bitcoin(), rust_logger, starting_time_secs, starting_time_nanos);
	KeysManager { inner: Box::into_raw(Box::new(ret)) }
}

