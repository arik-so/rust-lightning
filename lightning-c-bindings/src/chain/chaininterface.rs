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
		let c_tx = ::bitcoin::consensus::encode::serialize(tx);
		(self.broadcast_transaction)(self.this_arg, crate::c_types::Transaction::from_slice(&c_tx))
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
		(self.get_est_sat_per_1000_weight)(self.this_arg, ConfirmationTarget::from_ln(confirmation_target))
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

use lightning::chain::chaininterface::BlockNotifierArc as lnBlockNotifierArcImport;
type lnBlockNotifierArc = lnBlockNotifierArcImport;

/// " BlockNotifierArc is useful when you need a BlockNotifier that points to ChainListeners with"
/// " static lifetimes, e.g. when you're using lightning-net-tokio (since tokio::spawn requires"
/// " parameters with static lifetimes). Other times you can afford a reference, which is more"
/// " efficient, in which case BlockNotifierRef is a more appropriate type. Defining these type"
/// " aliases prevents issues such as overly long function definitions."
#[repr(C)]
pub struct BlockNotifierArc {
	pub(crate) inner: *const lnBlockNotifierArc,
}

