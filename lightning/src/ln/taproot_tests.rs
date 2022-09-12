use bitcoin::secp256k1::ecdsa::Signature;
use bitcoin::{PackedLockTime, Transaction, TxOut};
use chain::transaction::OutPoint;

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

	let known_features = InitFeatures::known();
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

	// assert!(node_cfgs[0].features.supports_simple_taproot(), "Taproot must be supported!");
	// assert!(node_cfgs[0].features.requires_simple_taproot(), "Taproot must be required!");

	let channel_value_satoshis = 1000000;
	let push_msat = 10001;
	nodes[0].node.create_channel(nodes[1].node.get_our_node_id(), channel_value_satoshis, push_msat, 42, None).unwrap();
	let mut node0_to_1_send_open_channel = get_event_msg!(nodes[0], MessageSendEvent::SendOpenChannel, nodes[1].node.get_our_node_id());

	assert!(node0_to_1_send_open_channel.musig2_partial_nonces.is_some());

	nodes[1].node.handle_open_channel(&nodes[0].node.get_our_node_id(), InitFeatures::known(), &node0_to_1_send_open_channel);
	let bob_message_events = nodes[1].node.get_and_clear_pending_msg_events();
	let accept_channel = match bob_message_events[0] {
		MessageSendEvent::SendAcceptChannel { ref msg, node_id } => {
			msg.clone()
		}
		_ => panic!("Unexpected event"),
	};

	assert!(accept_channel.musig2_partial_nonces.is_some());
	nodes[0].node.handle_accept_channel(&nodes[1].node.get_our_node_id(), InitFeatures::known(), &accept_channel);

	let alice_events = nodes[0].node.get_and_clear_pending_events();
	let funding_generation_output_script = match alice_events[0] {
		Event::FundingGenerationReady { ref output_script, .. } => {
			output_script.clone()
		}
		_ => { panic!("Unexpected event") }
	};
	println!("output script: {:?}", funding_generation_output_script);

	// create_funding_transaction(&nodes[0], &nodes[1].node.get_our_node_id(), 100000, 42); // Get and check the FundingGenerationReady event


	let chan_id = nodes[0].network_chan_count.borrow().clone();
	let tx = Transaction { version: chan_id as i32, lock_time: PackedLockTime::ZERO, input: Vec::new(), output: vec![TxOut {
		value: channel_value_satoshis, script_pubkey: funding_generation_output_script.clone(),
	}]};
	let funding_outpoint = OutPoint { txid: tx.txid(), index: 0 };

	let funding_created = {
		let mut a_channel_lock = nodes[0].node.channel_state.lock().unwrap();
		// Once we call `get_outbound_funding_created` the channel has a duplicate channel_id as
		// another channel in the ChannelManager - an invalid state. Thus, we'd panic later when we
		// try to create another channel. Instead, we drop the channel entirely here (leaving the
		// channelmanager in a possibly nonsense state instead).
		let mut as_chan = a_channel_lock.by_id.remove(&node0_to_1_send_open_channel.temporary_channel_id).unwrap();
		let logger = test_utils::TestLogger::new();
		as_chan.get_outbound_funding_created(tx.clone(), funding_outpoint, &&logger).unwrap()
	};
	check_added_monitors!(nodes[0], 0);
	nodes[1].node.handle_funding_created(&nodes[0].node.get_our_node_id(), &funding_created);

	let bob_message_events = nodes[1].node.get_and_clear_pending_msg_events();
	let bob_events = nodes[1].node.get_and_clear_pending_events();
	println!("bob message events: {}", bob_message_events.len());
	println!("bob events: {}", bob_events.len());
}
