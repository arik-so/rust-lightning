extern "C" {
#include "include/rust_types.h"
#include "include/lightning.h"
}
#include "include/lightningpp.hpp"

#include <assert.h>
#include <stdio.h>

void print_log(const void *this_arg, const char *record) {
	printf("%s", record);
}

uint64_t get_fee(const void *this_arg, LDK::ConfirmationTarget target) {
	if (target == Background) {
		return 253;
	} else {
		return 507;
	}
}

void broadcast_tx(const void *this_arg, LDKTransaction tx) {
	//TODO
}

LDKCResult_NoneChannelMonitorUpdateErrZ add_channel_monitor(const void *this_arg, LDKOutPoint funding_txo, LDKChannelMonitor monitor) {
	return CResult_NoneChannelMonitorUpdateErrZ_good();
}
LDKCResult_NoneChannelMonitorUpdateErrZ update_channel_monitor(const void *this_arg, LDKOutPoint funding_txo, LDKChannelMonitorUpdate monitor) {
	return CResult_NoneChannelMonitorUpdateErrZ_good();
}
LDKCVec_HTLCUpdateZ monitors_pending_htlcs_updated(const void *this_arg) {
	LDKCVec_HTLCUpdateZ empty_htlc_vec;
	empty_htlc_vec.data = NULL;
	empty_htlc_vec.datalen = 0;
	return empty_htlc_vec;
}

void chain_install_watch_tx(const void *this_arg, const uint8_t (*txid)[32], LDKu8slice script_pub_key) {}
void chain_watch_all_txn(const void *this_arg) {}
LDKCResult_C2Tuple_Scriptu64ZChainErrorZ get_chain_utxo(const void *this_arg, uint8_t genesis_hash[32], uint64_t unspent_tx_output_identifier) {
	// TODO: Create constructors for Tuples
}
/**
* " Returns a usize that changes when the ChainWatchInterface's watched data is modified."
* " Users of `filter_block` should pre-save a copy of `reentered`'s return value and use it to"
* " determine whether they need to re-filter a given block."
*/
uintptr_t (*reentered)(const void *this_arg);

int main() {
	uint8_t node_seed[32];

	LDKNetwork network = Bitcoin;

	// Trait implementations:
	LDK::Logger logger;
	logger.this_arg = NULL;
	logger.log = print_log;

	LDK::FeeEstimator fee_est;
	fee_est.this_arg = NULL;
	fee_est.get_est_sat_per_1000_weight = get_fee;

	LDK::ManyChannelMonitor mon;
	mon.this_arg = NULL;
	mon.add_monitor = add_channel_monitor;
	mon.update_monitor = update_channel_monitor;
	mon.get_and_clear_pending_htlcs_updated = monitors_pending_htlcs_updated;

	LDK::BroadcasterInterface broadcast;
	broadcast.this_arg = NULL;
	broadcast.broadcast_transaction = broadcast_tx;

	LDK::ChainWatchInterface chain;
	chain.this_arg = NULL;
	chain.install_watch_tx = chain_install_watch_tx;
	chain.watch_all_txn = chain_watch_all_txn;
	chain.get_chain_utxo = get_chain_utxo;

	// Instantiate classes:

	LDK::KeysManager keys = KeysManager_new(&node_seed, network, 0, 0);
	LDK::KeysInterface keys_source = KeysManager_as_KeysInterface(&keys);
	LDKSecretKey node_secret = keys_source.get_node_secret(keys_source.this_arg);

	LDK::UserConfig config = UserConfig_default();

	LDK::ChannelManager cm = ChannelManager_new(network, fee_est, mon, broadcast, logger, keys_source, config, 0);

	LDK::CVec_ChannelDetailsZ channels = ChannelManager_list_channels(&cm);
	assert(channels.self.datalen == 0);

	LDK::NetGraphMsgHandler net_graph = NetGraphMsgHandler_new(chain, logger);

	LDK::MessageHandler msg_handler = MessageHandler_new(ChannelManager_as_ChannelMessageHandler(&cm), NetGraphMsgHandler_as_RoutingMessageHandler(&net_graph));

	LDKThirtyTwoBytes random_bytes = keys_source.get_channel_id(keys_source.this_arg);
	LDK::PeerManager net = PeerManager_new(msg_handler, node_secret, &random_bytes.data, logger);
}
