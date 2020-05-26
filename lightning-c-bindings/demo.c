#include "include/rust_types.h"
#include "include/lightning.h"

#include <assert.h>
#include <stdio.h>

void print_log(const void *this_arg, const char *record) {
	printf("%s", record);
}

uint64_t get_fee(const void *this_arg, LDKConfirmationTarget target) {
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

int main() {
	uint8_t node_seed[32];

	LDKNetwork net = Bitcoin;

	LDKLogger logger;
	logger.this_arg = NULL;
	logger.log = print_log;

	LDKFeeEstimator fee_est;
	fee_est.this_arg = NULL;
	fee_est.get_est_sat_per_1000_weight = get_fee;

	LDKManyChannelMonitor mon;
	mon.this_arg = NULL;
	mon.add_monitor = add_channel_monitor;
	mon.update_monitor = update_channel_monitor;
	mon.get_and_clear_pending_htlcs_updated = monitors_pending_htlcs_updated;

	LDKBroadcasterInterface broadcast;
	broadcast.this_arg = NULL;
	broadcast.broadcast_transaction = broadcast_tx;

	LDKKeysManager keys = KeysManager_new(&node_seed, net, 0, 0);
	LDKKeysInterface keys_source = KeysManager_as_KeysInterface(&keys);

	LDKUserConfig config = UserConfig_default();

	LDKChannelManager cm = ChannelManager_new(net, fee_est, mon, broadcast, logger, keys_source, config, 0);

	LDKCVec_ChannelDetailsZ channels = ChannelManager_list_channels(&cm);
	assert((unsigned long)channels.data < 4096); // There's an offset, but it should still be an offset against null in the 0 page
	assert(channels.datalen == 0);
	CVec_ChannelDetailsZ_free(channels);

	LDKEventsProvider prov = ChannelManager_as_EventsProvider(&cm);
	LDKCVec_EventZ events = (prov.get_and_clear_pending_events)(prov.this_arg);
	assert((unsigned long)events.data < 4096); // There's an offset, but it should still be an offset against null in the 0 page
	assert(events.datalen == 0);

	ChannelManager_free(cm);
	KeysManager_free(keys);

	// Check that passing empty vecs to rust doesn't blow it up:
	LDKCVec_HTLCUpdateZ empty_htlc_vec;
	empty_htlc_vec.data = NULL;
	empty_htlc_vec.datalen = 0;
	CVec_HTLCUpdateZ_free(empty_htlc_vec);
}
