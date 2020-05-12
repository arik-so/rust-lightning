extern "C" {
#include "include/rust_types.h"
#include "include/lightning.h"
}
#include "include/lightningpp.hpp"

#include <assert.h>
#include <stdio.h>
#include <sys/socket.h>
#include <unistd.h>

#include <functional>
#include <thread>

const uint8_t valid_node_announcement[] = {
	0x94, 0xe4, 0xf5, 0x61, 0x41, 0x24, 0x7d, 0x90, 0x23, 0xa0, 0xc8, 0x34, 0x8c, 0xc4, 0xca, 0x51,
	0xd8, 0x17, 0x59, 0xff, 0x7d, 0xac, 0x8c, 0x9b, 0x63, 0x29, 0x1c, 0xe6, 0x12, 0x12, 0x93, 0xbd,
	0x66, 0x4d, 0x6b, 0x9c, 0xfb, 0x35, 0xda, 0x16, 0x06, 0x3d, 0xf0, 0x8f, 0x8a, 0x39, 0x99, 0xa2,
	0xf2, 0x5d, 0x12, 0x0f, 0x2b, 0x42, 0x1b, 0x8b, 0x9a, 0xfe, 0x33, 0x0c, 0xeb, 0x33, 0x5e, 0x52,
	0xee, 0x99, 0xa1, 0x07, 0x06, 0xed, 0xf8, 0x48, 0x7a, 0xc6, 0xe5, 0xf5, 0x5e, 0x01, 0x3a, 0x41,
	0x2f, 0x18, 0x94, 0x8a, 0x3b, 0x0a, 0x52, 0x3f, 0xbf, 0x61, 0xa9, 0xc5, 0x4f, 0x70, 0xee, 0xb8,
	0x79, 0x23, 0xbb, 0x1a, 0x44, 0x7d, 0x91, 0xe6, 0x2a, 0xbc, 0xa1, 0x07, 0xbc, 0x65, 0x3b, 0x02,
	0xd9, 0x1d, 0xb2, 0xf2, 0x3a, 0xcb, 0x75, 0x79, 0xc6, 0x66, 0xd8, 0xc1, 0x71, 0x29, 0xdf, 0x04,
	0x60, 0xf4, 0xbf, 0x07, 0x7b, 0xb9, 0xc2, 0x11, 0x94, 0x6a, 0x28, 0xc2, 0xdd, 0xd8, 0x7b, 0x44,
	0x8f, 0x08, 0xe3, 0xc8, 0xd8, 0xf4, 0x81, 0xb0, 0x9f, 0x94, 0xcb, 0xc8, 0xc1, 0x3c, 0xc2, 0x6e,
	0x31, 0x26, 0xfc, 0x33, 0x16, 0x3b, 0xe0, 0xde, 0xa1, 0x16, 0x21, 0x9f, 0x89, 0xdd, 0x97, 0xa4,
	0x41, 0xf2, 0x9f, 0x19, 0xb1, 0xae, 0x82, 0xf7, 0x85, 0x9a, 0xb7, 0x8f, 0xb7, 0x52, 0x7a, 0x72,
	0xf1, 0x5e, 0x89, 0xe1, 0x8a, 0xcd, 0x40, 0xb5, 0x8e, 0xc3, 0xca, 0x42, 0x76, 0xa3, 0x6e, 0x1b,
	0xf4, 0x87, 0x35, 0x30, 0x58, 0x43, 0x04, 0xd9, 0x2c, 0x50, 0x54, 0x55, 0x47, 0x6f, 0x70, 0x9b,
	0x42, 0x1f, 0x91, 0xfc, 0xa1, 0xdb, 0x72, 0x53, 0x96, 0xc8, 0xe5, 0xcd, 0x0e, 0xcb, 0xa0, 0xfe,
	0x6b, 0x08, 0x77, 0x48, 0xb7, 0xad, 0x4a, 0x69, 0x7c, 0xdc, 0xd8, 0x04, 0x28, 0x35, 0x9b, 0x73,
	0x00, 0x00, 0x43, 0x49, 0x7f, 0xd7, 0xf8, 0x26, 0x95, 0x71, 0x08, 0xf4, 0xa3, 0x0f, 0xd9, 0xce,
	0xc3, 0xae, 0xba, 0x79, 0x97, 0x20, 0x84, 0xe9, 0x0e, 0xad, 0x01, 0xea, 0x33, 0x09, 0x00, 0x00,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x5b, 0xe5, 0xe9, 0x47, 0x82,
	0x09, 0x67, 0x4a, 0x96, 0xe6, 0x0f, 0x1f, 0x03, 0x7f, 0x61, 0x76, 0x54, 0x0f, 0xd0, 0x01, 0xfa,
	0x1d, 0x64, 0x69, 0x47, 0x70, 0xc5, 0x6a, 0x77, 0x09, 0xc4, 0x2c, 0x03, 0x5c, 0x4e, 0x0d, 0xec,
	0x72, 0x15, 0xe2, 0x68, 0x33, 0x93, 0x87, 0x30, 0xe5, 0xe5, 0x05, 0xaa, 0x62, 0x50, 0x4d, 0xa8,
	0x5b, 0xa5, 0x71, 0x06, 0xa4, 0x6b, 0x5a, 0x24, 0x04, 0xfc, 0x9d, 0x8e, 0x02, 0xba, 0x72, 0xa6,
	0xe8, 0xba, 0x53, 0xe8, 0xb9, 0x71, 0xad, 0x0c, 0x98, 0x23, 0x96, 0x8a, 0xef, 0x4d, 0x78, 0xce,
	0x8a, 0xf2, 0x55, 0xab, 0x43, 0xdf, 0xf8, 0x30, 0x03, 0xc9, 0x02, 0xfb, 0x8d, 0x02, 0x16, 0x34,
	0x5b, 0xf8, 0x31, 0x16, 0x4a, 0x03, 0x75, 0x8e, 0xae, 0xa5, 0xe8, 0xb6, 0x6f, 0xee, 0x2b, 0xe7,
	0x71, 0x0b, 0x8f, 0x19, 0x0e, 0xe8, 0x80, 0x24, 0x90, 0x32, 0xa2, 0x9e, 0xd6, 0x6e
};

void print_log(const void *this_arg, const char *record) {
	printf("%p - %s\n", this_arg, record);
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
	return CResult_C2Tuple_Scriptu64ZChainErrorZ_err(LDKChainError::NotSupported);
}


uintptr_t sock_send_data(void *this_arg, LDKu8slice data, bool resume_read) {
	return write((int)((long)this_arg), data.data, data.datalen);
}
void sock_disconnect_socket(void *this_arg) {
	close((int)((long)this_arg));
}
bool sock_eq(const void *this_arg, const void *other_arg) {
	return this_arg == other_arg;
}
uint64_t sock_hash(const void *this_arg) {
	return (uint64_t)this_arg;
}
void sock_read_data_thread(int rdfd, LDKSocketDescriptor *peer_descriptor, LDKPeerManager *pm) {
	unsigned char buf[1024];
	LDKu8slice data;
	data.data = buf;
	ssize_t readlen = 0;
	while ((readlen = read(rdfd, buf, 1024)) > 0) {
		data.datalen = readlen;
		LDK::CResult_boolPeerHandleErrorZ res = PeerManager_read_event(&*pm, peer_descriptor, data);
		if (!res.self.result_good) {
			peer_descriptor->disconnect_socket(peer_descriptor->this_arg);
			return;
		}
		PeerManager_process_events(pm);
	}
	PeerManager_socket_disconnected(&*pm, peer_descriptor);
}

int main() {
	uint8_t node_seed[32];
	memset(&node_seed, 0, 32);

	LDKPublicKey null_pk;
	memset(&null_pk, 0, sizeof(null_pk));

	LDKNetwork network = Bitcoin;

	// Trait implementations:
	LDK::Logger logger;
	logger.this_arg = (void*)1;
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

	// Instantiate classes for node 1:

	LDK::Logger logger1;
	logger1.this_arg = (void*)1;
	logger1.log = print_log;

	LDK::KeysManager keys1 = KeysManager_new(&node_seed, network, 0, 0);
	LDK::KeysInterface keys_source1 = KeysManager_as_KeysInterface(&keys1);
	LDKSecretKey node_secret1 = keys_source1.get_node_secret(keys_source1.this_arg);

	LDK::UserConfig config1 = UserConfig_default();

	LDK::ChannelManager cm1 = ChannelManager_new(network, fee_est, mon, broadcast, logger1, keys_source1, config1, 0);

	LDK::CVec_ChannelDetailsZ channels = ChannelManager_list_channels(&cm1);
	assert(channels.self.datalen == 0);

	LDK::NetGraphMsgHandler net_graph1 = NetGraphMsgHandler_new(chain, logger1);

	LDK::MessageHandler msg_handler1 = MessageHandler_new(ChannelManager_as_ChannelMessageHandler(&cm1), NetGraphMsgHandler_as_RoutingMessageHandler(&net_graph1));

	LDKThirtyTwoBytes random_bytes = keys_source1.get_channel_id(keys_source1.this_arg);
	LDK::PeerManager net1 = PeerManager_new(msg_handler1, node_secret1, &random_bytes.data, logger1);

	// Demo getting a channel key and check that its returning real pubkeys:
	LDK::ChannelKeys chan_keys1 = keys_source1.get_channel_keys(keys_source1.this_arg, false, 42);
	chan_keys1.set_pubkeys(&chan_keys1); // Make sure pubkeys is defined
	LDKPublicKey payment_point = ChannelPublicKeys_get_payment_point(&chan_keys1.pubkeys);
	assert(memcmp(&payment_point, &null_pk, sizeof(null_pk)));

	// Instantiate classes for node 2:

	LDK::Logger logger2;
	logger2.this_arg = (void*)2;
	logger2.log = print_log;

	memset(&node_seed, 1, 32);
	LDK::KeysManager keys2 = KeysManager_new(&node_seed, network, 0, 0);
	LDK::KeysInterface keys_source2 = KeysManager_as_KeysInterface(&keys2);
	LDKSecretKey node_secret2 = keys_source2.get_node_secret(keys_source2.this_arg);

	LDK::UserConfig config2 = UserConfig_default();

	LDK::ChannelManager cm2 = ChannelManager_new(network, fee_est, mon, broadcast, logger2, keys_source2, config2, 0);

	LDK::CVec_ChannelDetailsZ channels2 = ChannelManager_list_channels(&cm2);
	assert(channels2.self.datalen == 0);

	LDK::NetGraphMsgHandler net_graph2 = NetGraphMsgHandler_new(chain, logger2);
	LDK::RoutingMessageHandler net_msgs2 = NetGraphMsgHandler_as_RoutingMessageHandler(&net_graph2);
	LDK::ChannelAnnouncement chan_ann = ChannelAnnouncement_read(LDKu8slice { data: valid_node_announcement, datalen: sizeof(valid_node_announcement) });
	net_msgs2.handle_channel_announcement(net_msgs2.this_arg, &chan_ann);

	LDK::MessageHandler msg_handler2 = MessageHandler_new(ChannelManager_as_ChannelMessageHandler(&cm2), net_msgs2);

	LDKThirtyTwoBytes random_bytes2 = keys_source2.get_channel_id(keys_source2.this_arg);
	LDK::PeerManager net2 = PeerManager_new(msg_handler2, node_secret2, &random_bytes2.data, logger2);

	// Open a connection!
	int pipefds_1_to_2[2];
	int pipefds_2_to_1[2];
	assert(!pipe(pipefds_1_to_2));
	assert(!pipe(pipefds_2_to_1));

	LDKSocketDescriptor sock1;
	sock1.this_arg = (void*)(long)pipefds_1_to_2[1];
	sock1.send_data = sock_send_data;
	sock1.disconnect_socket = sock_disconnect_socket;
	sock1.eq = sock_eq;
	sock1.hash = sock_hash;

	LDKSocketDescriptor sock2;
	sock2.this_arg = (void*)(long)pipefds_2_to_1[1];
	sock2.send_data = sock_send_data;
	sock2.disconnect_socket = sock_disconnect_socket;
	sock2.eq = sock_eq;
	sock2.hash = sock_hash;

	std::thread t1(&sock_read_data_thread, pipefds_2_to_1[0], &sock1, &net1);
	std::thread t2(&sock_read_data_thread, pipefds_1_to_2[0], &sock2, &net2);

	// Note that we have to bind the result to a C++ class to make sure it gets free'd
	LDK::CResult_CVec_u8ZPeerHandleErrorZ con_res = PeerManager_new_outbound_connection(&net1, ChannelManager_get_our_node_id(&cm2), sock1);
	assert((&con_res)->result_good);
	LDK::CResult_NonePeerHandleErrorZ con_res2 = PeerManager_new_inbound_connection(&net2, sock2);
	assert((&con_res2)->result_good);

	auto writelen = write(pipefds_1_to_2[1], (&con_res)->contents.result->data, (&con_res)->contents.result->datalen);
	assert(writelen == (&con_res)->contents.result->datalen);

	while (true) {
		// Wait for the initial handshakes to complete...
		LDK::CVec_PublicKeyZ peers_1 = PeerManager_get_peer_node_ids(&net1);
		LDK::CVec_PublicKeyZ peers_2 = PeerManager_get_peer_node_ids(&net2);
		if (peers_1.self.datalen == 1 && peers_2.self.datalen ==1) { break; }
		std::this_thread::yield();
	}

	// Note that we have to bind the result to a C++ class to make sure it gets free'd
	LDK::CResult_NoneAPIErrorZ res = ChannelManager_create_channel(&cm1, ChannelManager_get_our_node_id(&cm2), 40000, 1000, 42, config1);
	assert((&res)->result_good);
	PeerManager_process_events(&net1);

	LDK::CVec_ChannelDetailsZ new_channels = ChannelManager_list_channels(&cm1);
	assert(new_channels.self.datalen == 1);
	LDKPublicKey chan_open_pk = ChannelDetails_get_remote_network_id(&new_channels.self.data[0]);
	assert(!memcmp(chan_open_pk.compressed_form, ChannelManager_get_our_node_id(&cm2).compressed_form, 33));

	while (true) {
		LDK::CVec_ChannelDetailsZ new_channels_2 = ChannelManager_list_channels(&cm2);
		if (new_channels_2.self.datalen == 1) { break; }
		std::this_thread::yield();
	}

	close(pipefds_1_to_2[0]);
	close(pipefds_2_to_1[0]);
	close(pipefds_1_to_2[1]);
	close(pipefds_2_to_1[1]);
	t1.join();
	t2.join();
}
