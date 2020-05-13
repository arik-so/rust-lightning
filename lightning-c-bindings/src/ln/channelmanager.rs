//! " The top-level channel management and payment tracking stuff lives here."
//! ""
//! " The ChannelManager is the main chunk of logic implementing the lightning protocol and is"
//! " responsible for tracking which channels are open, HTLCs are in flight and reestablishing those"
//! " upon reconnect to the relevant peer(s)."
//! ""
//! " It does not manage routing logic (see routing::router::get_route for that) nor does it manage constructing"
//! " on-chain transactions (it only monitors the chain to watch for any force-closes that might"
//! " imply it needs to fail HTLCs/payments/channels it manages)."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

use bitcoin::blockdata::block::BlockHeader as lnBlockHeader;
use bitcoin::blockdata::transaction::Transaction as lnTransaction;
use bitcoin::blockdata::constants::genesis_block as lngenesis_block;
use bitcoin::network::constants::Network as lnNetwork;
use bitcoin::util::hash::BitcoinHash as lnBitcoinHash;
use bitcoin::hashes::Hash as lnHash;
use bitcoin::hashes::HashEngine as lnHashEngine;
use bitcoin::hashes::hmac::Hmac as lnHmac;
use bitcoin::hashes::hmac::HmacEngine as lnHmacEngine;
use bitcoin::hashes::cmp::fixed_time_eq as lnfixed_time_eq;
use bitcoin::hash_types::BlockHash as lnBlockHash;
use bitcoin::secp256k1::key::SecretKey as lnSecretKey;
use bitcoin::secp256k1::key::PublicKey as lnPublicKey;
use bitcoin::secp256k1::Secp256k1 as lnSecp256k1;
use bitcoin::secp256k1::ecdh::SharedSecret as lnSharedSecret;
use bitcoin::secp256k1 as lnsecp256k1;

use lightning::ln::channelmanager::ChannelManager as lnChannelManagerImport;
type lnChannelManager = lnChannelManagerImport<crate::chain::keysinterface::ChannelKeys, crate::ln::channelmonitor::ManyChannelMonitor, crate::chain::chaininterface::BroadcasterInterface, crate::chain::keysinterface::KeysInterface, crate::chain::chaininterface::FeeEstimator>;

/// " Manager which keeps track of a number of channels and sends messages to the appropriate"
/// " channel, also tracking HTLC preimages and forwarding onion packets appropriately."
/// ""
/// " Implements ChannelMessageHandler, handling the multi-channel parts and passing things through"
/// " to individual Channels."
/// ""
/// " Implements Writeable to write out all channel state to disk. Implies peer_disconnected() for"
/// " all peers during write/read (though does not modify this instance, only the instance being"
/// " serialized). This will result in any channels which have not yet exchanged funding_created (ie"
/// " called funding_transaction_generated for outbound channels)."
/// ""
/// " Note that you can be a bit lazier about writing out ChannelManager than you can be with"
/// " ChannelMonitors. With ChannelMonitors you MUST write each monitor update out to disk before"
/// " returning from ManyChannelMonitor::add_/update_monitor, with ChannelManagers, writing updates"
/// " happens out-of-band (and will prevent any other ChannelManager operations from occurring during"
/// " the serialization process). If the deserialized version is out-of-date compared to the"
/// " ChannelMonitors passed by reference to read(), those channels will be force-closed based on the"
/// " ChannelMonitor state and no funds will be lost (mod on-chain transaction fees)."
/// ""
/// " Note that the deserializer is only implemented for (Sha256dHash, ChannelManager), which"
/// " tells you the last block hash which was block_connect()ed. You MUST rescan any blocks along"
/// " the \"reorg path\" (ie call block_disconnected() until you get to a common block and then call"
/// " block_connected() to step towards your best block) upon deserialization before using the"
/// " object!"
/// ""
/// " Note that ChannelManager is responsible for tracking liveness of its channels and generating"
/// " ChannelUpdate messages informing peers that the channel is temporarily disabled. To avoid"
/// " spam due to quick disconnection/reconnection, updates are not sent until the channel has been"
/// " offline for a full minute. In order to track this, you must call"
/// " timer_chan_freshness_every_min roughly once per minute, though it doesn't have to be perfect."
/// ""
/// " Rather than using a plain ChannelManager, it is preferable to use either a SimpleArcChannelManager"
/// " a SimpleRefChannelManager, for conciseness. See their documentation for more details, but"
/// " essentially you should default to using a SimpleRefChannelManager, and use a"
/// " SimpleArcChannelManager when you require a ChannelManager with a static lifetime, such as when"
/// " you're using lightning-net-tokio."
#[repr(C)]
pub struct ChannelManager {
	pub(crate) inner: *const lnChannelManager,
}

#[no_mangle]
pub extern "C" fn ChannelManager_free(this_ptr: ChannelManager) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelManager) };
}

use lightning::ln::channelmanager::ChannelDetails as lnChannelDetailsImport;
type lnChannelDetails = lnChannelDetailsImport;

/// " Details of a channel, as returned by ChannelManager::list_channels and ChannelManager::list_usable_channels"
#[repr(C)]
pub struct ChannelDetails {
	pub(crate) inner: *const lnChannelDetails,
}

#[no_mangle]
pub extern "C" fn ChannelDetails_free(this_ptr: ChannelDetails) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelDetails) };
}
/// " The channel's ID (prior to funding transaction generation, this is a random 32 bytes,"
/// " thereafter this is the txid of the funding transaction xor the funding transaction output)."
/// " Note that this means this value is *not* persistent - it can change once during the"
/// " lifetime of the channel."
#[no_mangle]
pub extern "C" fn ChannelDetails_get_channel_id(this_ptr: &ChannelDetails) -> *const [u8; 32] {
	&unsafe { &*this_ptr.inner }.channel_id
}
/// " The channel's ID (prior to funding transaction generation, this is a random 32 bytes,"
/// " thereafter this is the txid of the funding transaction xor the funding transaction output)."
/// " Note that this means this value is *not* persistent - it can change once during the"
/// " lifetime of the channel."
#[no_mangle]
pub extern "C" fn ChannelDetails_set_channel_id(this_ptr: &mut ChannelDetails, val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelDetails) }.channel_id = val.data;
}
/// " The node_id of our counterparty"
#[no_mangle]
pub extern "C" fn ChannelDetails_get_remote_network_id(this_ptr: &ChannelDetails) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.remote_network_id)
}
/// " The node_id of our counterparty"
#[no_mangle]
pub extern "C" fn ChannelDetails_set_remote_network_id(this_ptr: &mut ChannelDetails, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelDetails) }.remote_network_id = val.into_rust();
}
/// " The Features the channel counterparty provided upon last connection."
/// " Useful for routing as it is the most up-to-date copy of the counterparty's features and"
/// " many routing-relevant features are present in the init context."
#[no_mangle]
pub extern "C" fn ChannelDetails_get_counterparty_features(this_ptr: &ChannelDetails) -> *const crate::ln::features::InitFeatures {
	Box::into_raw(Box::new(crate::ln::features::InitFeatures { inner: &unsafe { &*this_ptr.inner }.counterparty_features }))
}
/// " The Features the channel counterparty provided upon last connection."
/// " Useful for routing as it is the most up-to-date copy of the counterparty's features and"
/// " many routing-relevant features are present in the init context."
#[no_mangle]
pub extern "C" fn ChannelDetails_set_counterparty_features(this_ptr: &mut ChannelDetails, val: crate::ln::features::InitFeatures) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelDetails) }.counterparty_features = *unsafe { Box::from_raw(val.inner as *mut _) };
}
/// " The value, in satoshis, of this channel as appears in the funding output"
#[no_mangle]
pub extern "C" fn ChannelDetails_set_channel_value_satoshis(this_ptr: &mut ChannelDetails, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelDetails) }.channel_value_satoshis = val;
}
/// " The user_id passed in to create_channel, or 0 if the channel was inbound."
#[no_mangle]
pub extern "C" fn ChannelDetails_set_user_id(this_ptr: &mut ChannelDetails, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelDetails) }.user_id = val;
}
/// " The available outbound capacity for sending HTLCs to the remote peer. This does not include"
/// " any pending HTLCs which are not yet fully resolved (and, thus, who's balance is not"
/// " available for inclusion in new outbound HTLCs). This further does not include any pending"
/// " outgoing HTLCs which are awaiting some other resolution to be sent."
#[no_mangle]
pub extern "C" fn ChannelDetails_set_outbound_capacity_msat(this_ptr: &mut ChannelDetails, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelDetails) }.outbound_capacity_msat = val;
}
/// " The available inbound capacity for the remote peer to send HTLCs to us. This does not"
/// " include any pending HTLCs which are not yet fully resolved (and, thus, who's balance is not"
/// " available for inclusion in new inbound HTLCs)."
/// " Note that there are some corner cases not fully handled here, so the actual available"
/// " inbound capacity may be slightly higher than this."
#[no_mangle]
pub extern "C" fn ChannelDetails_set_inbound_capacity_msat(this_ptr: &mut ChannelDetails, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelDetails) }.inbound_capacity_msat = val;
}
/// " True if the channel is (a) confirmed and funding_locked messages have been exchanged, (b)"
/// " the peer is connected, and (c) no monitor update failure is pending resolution."
#[no_mangle]
pub extern "C" fn ChannelDetails_set_is_live(this_ptr: &mut ChannelDetails, val: bool) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelDetails) }.is_live = val;
}

use lightning::ln::channelmanager::PaymentSendFailure as lnPaymentSendFailureImport;
type lnPaymentSendFailure = lnPaymentSendFailureImport;

/// " If a payment fails to send, it can be in one of several states. This enum is returned as the"
/// " Err() type describing which state the payment is in, see the description of individual enum"
/// " states for more."
#[repr(C)]
pub struct PaymentSendFailure {
	pub(crate) inner: *const lnPaymentSendFailure,
}

#[no_mangle]
pub extern "C" fn PaymentSendFailure_free(this_ptr: PaymentSendFailure) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnPaymentSendFailure) };
}
/// " Constructs a new ChannelManager to hold several channels and route between them."
/// ""
/// " This is the main \"logic hub\" for all channel-related actions, and implements"
/// " ChannelMessageHandler."
/// ""
/// " Non-proportional fees are fixed according to our risk using the provided fee estimator."
/// ""
/// " panics if channel_value_satoshis is >= `MAX_FUNDING_SATOSHIS`!"
/// ""
/// " Users must provide the current blockchain height from which to track onchain channel"
/// " funding outpoints and send payments with reliable timelocks."
/// ""
/// " Users need to notify the new ChannelManager when a new block is connected or"
/// " disconnected using its `block_connected` and `block_disconnected` methods."
/// " However, rather than calling these methods directly, the user should register"
/// " the ChannelManager as a listener to the BlockNotifier and call the BlockNotifier's"
/// " `block_(dis)connected` methods, which will notify all registered listeners in one"
/// " go."
#[no_mangle]
pub extern "C" fn ChannelManager_new(network: crate::bitcoin::network::Network, fee_est: crate::chain::chaininterface::FeeEstimator, monitor: crate::ln::channelmonitor::ManyChannelMonitor, tx_broadcaster: crate::chain::chaininterface::BroadcasterInterface, logger: crate::util::logger::Logger, keys_manager: crate::chain::keysinterface::KeysInterface, config: crate::util::config::UserConfig, current_blockchain_height: usize) -> ChannelManager {
	let rust_logger = std::sync::Arc::new(logger);
	let ret = lightning::ln::channelmanager::ChannelManager::new(network.into_bitcoin(), fee_est, monitor, tx_broadcaster, rust_logger, keys_manager, *unsafe { Box::from_raw(config.inner as *mut lightning::util::config::UserConfig) }, current_blockchain_height);
	ChannelManager { inner: Box::into_raw(Box::new(ret)) }
}

/// " Force closes a channel, immediately broadcasting the latest local commitment transaction to"
/// " the chain and rejecting new HTLCs on the given channel."
#[no_mangle]
pub extern "C" fn ChannelManager_force_close_channel(this_arg: &ChannelManager, channel_id: *const [u8; 32]) {
	unsafe { &*this_arg.inner }.force_close_channel(unsafe { &*channel_id})
}

/// " Force close all channels, immediately broadcasting the latest local commitment transaction"
/// " for each to the chain and rejecting new HTLCs on each."
#[no_mangle]
pub extern "C" fn ChannelManager_force_close_all_channels(this_arg: &ChannelManager) {
	unsafe { &*this_arg.inner }.force_close_all_channels()
}

/// " Processes HTLCs which are pending waiting on random forward delay."
/// ""
/// " Should only really ever be called in response to a PendingHTLCsForwardable event."
/// " Will likely generate further events."
#[no_mangle]
pub extern "C" fn ChannelManager_process_pending_htlc_forwards(this_arg: &ChannelManager) {
	unsafe { &*this_arg.inner }.process_pending_htlc_forwards()
}

/// " If a peer is disconnected we mark any channels with that peer as 'disabled'."
/// " After some time, if channels are still disabled we need to broadcast a ChannelUpdate"
/// " to inform the network about the uselessness of these channels."
/// ""
/// " This method handles all the details, and must be called roughly once per minute."
#[no_mangle]
pub extern "C" fn ChannelManager_timer_chan_freshness_every_min(this_arg: &ChannelManager) {
	unsafe { &*this_arg.inner }.timer_chan_freshness_every_min()
}

/// " Gets the node_id held by this ChannelManager"
#[no_mangle]
pub extern "C" fn ChannelManager_get_our_node_id(this_arg: &ChannelManager) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_arg.inner }.get_our_node_id())
}

