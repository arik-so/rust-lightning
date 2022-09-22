use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::{PackedLockTime, Transaction, TxOut};
use chain::transaction::OutPoint;
use ln::channelmanager;

use ln::features::InitFeatures;
use ln::functional_test_utils::*;
use ln::msgs::{ChannelMessageHandler, ErrorAction};
use util::config::UserConfig;
use util::events::{Event, MessageSendEvent, MessageSendEventsProvider};
use util::test_utils;

#[test]
fn test_all_zero_signature_deserialization() {
	let signature_input = [0u8; 64];
	let signature = Signature::from_compact(&signature_input);
	signature.expect("Failed to deserialize 0-byte-signature");
}

#[test]
fn test_accept_channel_fails_if_open_channel_lacks_nonces() {
	let chanmon_cfgs = create_chanmon_cfgs(2);
	let mut node_cfgs = create_node_cfgs(2, &chanmon_cfgs);

	let mut node_config = UserConfig::default();
	node_config.channel_handshake_config.negotiate_simple_taproot = true;

	let node_chanmgrs = create_node_chanmgrs(2, &node_cfgs, &[Some(node_config), Some(node_config)]);
	let nodes = create_network(2, &node_cfgs, &node_chanmgrs);

	let channel_value_satoshis = 1000000;
	let push_msat = 10001;
	nodes[0].node.create_channel(nodes[1].node.get_our_node_id(), channel_value_satoshis, push_msat, 42, None).unwrap();
	let mut node0_to_1_send_open_channel = get_event_msg!(nodes[0], MessageSendEvent::SendOpenChannel, nodes[1].node.get_our_node_id());
	node0_to_1_send_open_channel.musig2_partial_nonces = None;

	let known_features = channelmanager::provided_init_features();
	assert!(known_features.supports_simple_taproot());

	nodes[1].node.handle_open_channel(&nodes[0].node.get_our_node_id(), known_features, &node0_to_1_send_open_channel);
	let events = nodes[1].node.get_and_clear_pending_msg_events();
	let err_msg = match events[0] {
		MessageSendEvent::HandleError { action: ErrorAction::SendErrorMessage { ref msg }, node_id: _ } => {
			msg.clone()
		}
		_ => panic!("Unexpected event"),
	};
	assert_eq!(err_msg.data, "Peer is signaling simple_taproot support without providing values for local_musig2_pubnonce and remote_musig2_pubnonce");
}

#[test]
fn test_taproot_open_channel() {
	let chanmon_cfgs = create_chanmon_cfgs(2);
	let mut node_cfgs = create_node_cfgs(2, &chanmon_cfgs);

	let mut node_config = UserConfig::default();
	node_config.channel_handshake_config.negotiate_simple_taproot = true;

	let node_chanmgrs = create_node_chanmgrs(2, &node_cfgs, &[Some(node_config), Some(node_config)]);
	let nodes = create_network(2, &node_cfgs, &node_chanmgrs);

	let channel_value_satoshis = 1000000;
	let push_msat = 10001;

	let alice = nodes[0].node;
	let bob = nodes[1].node;
	let alice_node_id = alice.get_our_node_id();
	let bob_node_id = bob.get_our_node_id();

	alice.create_channel(bob_node_id, channel_value_satoshis, push_msat, 42, None).unwrap();
	let mut node0_to_1_send_open_channel = get_event_msg!(nodes[0], MessageSendEvent::SendOpenChannel, nodes[1].node.get_our_node_id());

	assert!(node0_to_1_send_open_channel.musig2_partial_nonces.is_some());


	bob.handle_open_channel(&alice_node_id, channelmanager::provided_init_features(), &node0_to_1_send_open_channel);
	let bob_message_events = bob.get_and_clear_pending_msg_events();
	let accept_channel = match bob_message_events[0] {
		MessageSendEvent::SendAcceptChannel { ref msg, node_id } => {
			msg.clone()
		}
		_ => panic!("Unexpected event"),
	};

	assert!(accept_channel.musig2_partial_nonces.is_some());
	alice.handle_accept_channel(&bob_node_id, channelmanager::provided_init_features(), &accept_channel);

	let (temporary_channel_id, tx, _) = create_funding_transaction(&nodes[0], &bob_node_id, channel_value_satoshis, 42); // Get and check the FundingGenerationReady event

	alice.funding_transaction_generated(&temporary_channel_id, &bob_node_id, tx);
	let alice_message_events = alice.get_and_clear_pending_msg_events();

	let funding_created = match &alice_message_events[0] {
		MessageSendEvent::SendFundingCreated { node_id, msg } => msg.clone(),
		_ => panic!("Unexpected event")
	};

	check_added_monitors!(nodes[0], 0);
	bob.handle_funding_created(&alice_node_id, &funding_created);

	let bob_message_events = bob.get_and_clear_pending_msg_events();

	let funding_signed = match &bob_message_events[0] {
		MessageSendEvent::SendFundingSigned { node_id, msg } => msg.clone(),
		_ => panic!("Unexpected event")
	};

	alice.handle_funding_signed(&bob_node_id, &funding_signed);
}
