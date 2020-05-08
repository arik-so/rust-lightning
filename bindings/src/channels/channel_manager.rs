use lightning::ln::channelmanager::ChannelManager as lnChannelManager;

use std::sync::Arc;

use crate::bitcoin::network::Network;
use crate::chain::chaininterface::{BroadcasterInterface, FeeEstimator};
use crate::chain::keysinterface::{ChannelKeys, KeysInterface};
use crate::channels::ManyChannelMonitor;
use crate::util::logger::Logger;

pub struct ChannelManager {
	mgr: lnChannelManager<ChannelKeys, ManyChannelMonitor, BroadcasterInterface, KeysInterface, FeeEstimator>,
}

#[no_mangle]
pub extern "C" fn ChannelManager_new(
	network: Network,
	fee_est: FeeEstimator,
	monitor: ManyChannelMonitor,
	tx_broadcaster: BroadcasterInterface,
	logger: Logger,
	keys_manager: KeysInterface,
	//TODO: config
	current_blockchain_height: u32,
) -> *const ChannelManager {
	let config = Default::default();
	if let Ok(mgr) = lnChannelManager::new(network.to_bitcoin(), fee_est, monitor, tx_broadcaster, Arc::new(logger), keys_manager, config, current_blockchain_height as usize) {
		Box::leak(Box::new(ChannelManager { mgr }))
	} else {
		std::ptr::null()
	}
}