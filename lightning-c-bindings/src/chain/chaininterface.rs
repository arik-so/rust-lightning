//! " Traits and utility impls which allow other parts of rust-lightning to interact with the"
//! " blockchain."
//! ""
//! " Includes traits for monitoring and receiving notifications of new blocks and block"
//! " disconnections, transaction broadcasting, and feerate information requests."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

use bitcoin::blockdata::block::Block as lnBlock;
use bitcoin::blockdata::block::BlockHeader as lnBlockHeader;
use bitcoin::blockdata::transaction::Transaction as lnTransaction;
use bitcoin::blockdata::script::Script as lnScript;
use bitcoin::blockdata::constants::genesis_block as lngenesis_block;
use bitcoin::util::hash::BitcoinHash as lnBitcoinHash;
use bitcoin::network::constants::Network as lnNetwork;
use bitcoin::hash_types::Txid as lnTxid;
use bitcoin::hash_types::BlockHash as lnBlockHash;
/// " Used to give chain error details upstream"
#[repr(C)]
pub enum ChainError {
	/// " Client doesn't support UTXO lookup (but the chain hash matches our genesis block hash)"
	NotSupported,
	/// " Chain isn't the one watched"
	NotWatched,
	/// " Tx doesn't exist or is unconfirmed"
	UnknownTx,
}
use lightning::chain::chaininterface::ChainError as lnChainError;
impl ChainError {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnChainError {
		match self {
			ChainError::NotSupported => lnChainError::NotSupported,
			ChainError::NotWatched => lnChainError::NotWatched,
			ChainError::UnknownTx => lnChainError::UnknownTx,
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: lnChainError) -> Self {
		match lnt {
			lnChainError::NotSupported => ChainError::NotSupported,
			lnChainError::NotWatched => ChainError::NotWatched,
			lnChainError::UnknownTx => ChainError::UnknownTx,
		}
	}
}
/// " An interface to request notification of certain scripts as they appear the"
/// " chain."
/// ""
/// " Note that all of the functions implemented here *must* be reentrant-safe (obviously - they're"
/// " called from inside the library in response to ChainListener events, P2P events, or timer"
/// " events)."
#[repr(C)]
pub struct ChainWatchInterface {
	pub this_arg: *mut c_void,
	/// " Provides a txid/random-scriptPubKey-in-the-tx which much be watched for."
	pub install_watch_tx: extern "C" fn (this_arg: *const c_void, txid: *const [u8; 32], script_pub_key: crate::c_types::Script),
	//XXX: Need to export install_watch_outpoint
	/// " Indicates that a listener needs to see all transactions."
	pub watch_all_txn: extern "C" fn (this_arg: *const c_void),
	//XXX: Need to export get_chain_utxo
	//XXX: Need to export filter_block
	/// " Returns a usize that changes when the ChainWatchInterface's watched data is modified."
	/// " Users of `filter_block` should pre-save a copy of `reentered`'s return value and use it to"
	/// " determine whether they need to re-filter a given block."
	pub reentered: extern "C" fn (this_arg: *const c_void) -> usize,
}
unsafe impl Sync for ChainWatchInterface {}
unsafe impl Send for ChainWatchInterface {}

use lightning::chain::chaininterface::ChainWatchInterface as lnChainWatchInterface;
impl lnChainWatchInterface for ChainWatchInterface {
	fn install_watch_tx(&self, txid: &bitcoin::hash_types::Txid, script_pub_key: &bitcoin::blockdata::script::Script) {
		(self.install_watch_tx)(self.this_arg, txid.as_inner(), crate::c_types::Script::from_bitcoin(&script_pub_key))
	}
	fn install_watch_outpoint(&self, outpoint: (bitcoin::hash_types::Txid, u32), out_script: &bitcoin::blockdata::script::Script) {
		unimplemented!();
	}
	fn watch_all_txn(&self) {
		(self.watch_all_txn)(self.this_arg)
	}
	fn get_chain_utxo(&self, genesis_hash: bitcoin::hash_types::BlockHash, unspent_tx_output_identifier: u64) -> Result<(bitcoin::blockdata::script::Script, u64), lightning::chain::chaininterface::ChainError> {
		unimplemented!();
	}
	fn filter_block<'a>(&self, block: &'a bitcoin::blockdata::block::Block) -> (Vec<&'a bitcoin::blockdata::transaction::Transaction>, Vec<u32>) {
		unimplemented!();
	}
	fn reentered(&self) -> usize {
		let mut ret = (self.reentered)(self.this_arg);
		ret
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for ChainWatchInterface {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// " An interface to send a transaction to the Bitcoin network."
#[repr(C)]
pub struct BroadcasterInterface {
	pub this_arg: *mut c_void,
	/// " Sends a transaction out to (hopefully) be mined."
	pub broadcast_transaction: extern "C" fn (this_arg: *const c_void, tx: crate::c_types::Transaction),
}
unsafe impl Sync for BroadcasterInterface {}
unsafe impl Send for BroadcasterInterface {}

use lightning::chain::chaininterface::BroadcasterInterface as lnBroadcasterInterface;
impl lnBroadcasterInterface for BroadcasterInterface {
	fn broadcast_transaction(&self, tx: &bitcoin::blockdata::transaction::Transaction) {
		let local_tx = ::bitcoin::consensus::encode::serialize(tx);
		(self.broadcast_transaction)(self.this_arg, crate::c_types::Transaction::from_slice(&local_tx))
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for BroadcasterInterface {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// " An enum that represents the speed at which we want a transaction to confirm used for feerate"
/// " estimation."
#[repr(C)]
pub enum ConfirmationTarget {
	/// " We are happy with this transaction confirming slowly when feerate drops some."
	Background,
	/// " We'd like this transaction to confirm without major delay, but 12-18 blocks is fine."
	Normal,
	/// " We'd like this transaction to confirm in the next few blocks."
	HighPriority,
}
use lightning::chain::chaininterface::ConfirmationTarget as lnConfirmationTarget;
impl ConfirmationTarget {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnConfirmationTarget {
		match self {
			ConfirmationTarget::Background => lnConfirmationTarget::Background,
			ConfirmationTarget::Normal => lnConfirmationTarget::Normal,
			ConfirmationTarget::HighPriority => lnConfirmationTarget::HighPriority,
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: lnConfirmationTarget) -> Self {
		match lnt {
			lnConfirmationTarget::Background => ConfirmationTarget::Background,
			lnConfirmationTarget::Normal => ConfirmationTarget::Normal,
			lnConfirmationTarget::HighPriority => ConfirmationTarget::HighPriority,
		}
	}
}
/// " A trait which should be implemented to provide feerate information on a number of time"
/// " horizons."
/// ""
/// " Note that all of the functions implemented here *must* be reentrant-safe (obviously - they're"
/// " called from inside the library in response to ChainListener events, P2P events, or timer"
/// " events)."
#[repr(C)]
pub struct FeeEstimator {
	pub this_arg: *mut c_void,
	/// " Gets estimated satoshis of fee required per 1000 Weight-Units."
	/// ""
	/// " Must be no smaller than 253 (ie 1 satoshi-per-byte rounded up to ensure later round-downs"
	/// " don't put us below 1 satoshi-per-byte)."
	/// ""
	/// " This translates to:"
	/// "  * satoshis-per-byte * 250"
	/// "  * ceil(satoshis-per-kbyte / 4)"
	pub get_est_sat_per_1000_weight: extern "C" fn (this_arg: *const c_void, confirmation_target: ConfirmationTarget) -> u64,
}
unsafe impl Sync for FeeEstimator {}
unsafe impl Send for FeeEstimator {}

use lightning::chain::chaininterface::FeeEstimator as lnFeeEstimator;
impl lnFeeEstimator for FeeEstimator {
	fn get_est_sat_per_1000_weight(&self, confirmation_target: lightning::chain::chaininterface::ConfirmationTarget) -> u64 {
		let mut ret = (self.get_est_sat_per_1000_weight)(self.this_arg, crate::chain::chaininterface::ConfirmationTarget::from_ln(confirmation_target));
		ret
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for FeeEstimator {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}

use lightning::chain::chaininterface::ChainWatchInterfaceUtil as lnChainWatchInterfaceUtilImport;
type lnChainWatchInterfaceUtil = lnChainWatchInterfaceUtilImport;

/// " Utility to capture some common parts of ChainWatchInterface implementors."
/// ""
/// " Keeping a local copy of this in a ChainWatchInterface implementor is likely useful."
#[repr(C)]
pub struct ChainWatchInterfaceUtil {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChainWatchInterfaceUtil,
}

#[no_mangle]
pub extern "C" fn ChainWatchInterfaceUtil_free(this_ptr: ChainWatchInterfaceUtil) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChainWatchInterfaceUtil) };
}
#[no_mangle]
pub extern "C" fn ChainWatchInterfaceUtil_as_ChainWatchInterface(this_arg: *const ChainWatchInterfaceUtil) -> crate::chain::chaininterface::ChainWatchInterface {
	crate::chain::chaininterface::ChainWatchInterface {
		this_arg: unsafe { (*this_arg).inner as *mut c_void },
		install_watch_tx: ChainWatchInterfaceUtil_ChainWatchInterface_install_watch_tx,
		//XXX: Need to export install_watch_outpoint
		watch_all_txn: ChainWatchInterfaceUtil_ChainWatchInterface_watch_all_txn,
		//XXX: Need to export get_chain_utxo
		//XXX: Need to export filter_block
		reentered: ChainWatchInterfaceUtil_ChainWatchInterface_reentered,
	}
}
use lightning::chain::chaininterface::ChainWatchInterface as ChainWatchInterfaceTraitImport;
extern "C" fn ChainWatchInterfaceUtil_ChainWatchInterface_install_watch_tx(this_arg: *const c_void, txid: *const [u8; 32], script_pub_key: crate::c_types::Script) {
	unsafe { &*(*(this_arg as *const ChainWatchInterfaceUtil)).inner }.install_watch_tx(&::bitcoin::hash_types::Txid::from_slice(&unsafe { &*txid }[..]).unwrap(), &script_pub_key.into_bitcoin())
}
extern "C" fn ChainWatchInterfaceUtil_ChainWatchInterface_watch_all_txn(this_arg: *const c_void) {
	unsafe { &*(*(this_arg as *const ChainWatchInterfaceUtil)).inner }.watch_all_txn()
}
extern "C" fn ChainWatchInterfaceUtil_ChainWatchInterface_reentered(this_arg: *const c_void) -> usize {
	let mut ret = unsafe { &*(*(this_arg as *const ChainWatchInterfaceUtil)).inner }.reentered();
	ret
}

/// " Creates a new ChainWatchInterfaceUtil for the given network"
#[no_mangle]
pub extern "C" fn ChainWatchInterfaceUtil_new(network: crate::bitcoin::network::Network) -> ChainWatchInterfaceUtil {
	let mut ret = lightning::chain::chaininterface::ChainWatchInterfaceUtil::new(network.into_bitcoin());
	crate::chain::chaininterface::ChainWatchInterfaceUtil { inner: Box::into_raw(Box::new(ret)) }
}

/// " Checks if a given transaction matches the current filter."
#[no_mangle]
pub extern "C" fn ChainWatchInterfaceUtil_does_match_tx(this_arg: &ChainWatchInterfaceUtil, tx: crate::c_types::Transaction) -> bool {
	let mut ret = unsafe { &*this_arg.inner }.does_match_tx(&tx.into_bitcoin());
	ret
}

