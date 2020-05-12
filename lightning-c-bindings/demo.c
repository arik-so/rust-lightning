#include "include/rust_types.h"
#include "include/lightning.h"

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
	//TODO: need to export monitor update functions

	LDKBroadcasterInterface broadcast;
	broadcast.this_arg = NULL;
	broadcast.broadcast_transaction = broadcast_tx;

	LDKKeysManager keys = KeysManager_new(&node_seed, net, 0, 0);
	LDKKeysInterface keys_source = KeysManager_as_KeysInterface(&keys);

	LDKUserConfig config = UserConfig_default();

	LDKChannelManager cm = ChannelManager_new(net, fee_est, mon, broadcast, logger, keys_source, config, 0);

	LDKCVecChannelDetails channels = ChannelManager_list_channels(&cm);
	CVecChannelDetails_free(channels);

	ChannelManager_free(cm);
	KeysManager_free(keys);
}
