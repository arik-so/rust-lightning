use ln::features::InitFeatures;
use ln::functional_test_utils::*;
use ln::msgs::{ChannelMessageHandler, ErrorAction};
use util::events::{MessageSendEvent, MessageSendEventsProvider};

#[test]
fn test_accept_channel_fails_if_open_channel_lacks_nonces() {
	let chanmon_cfgs = create_chanmon_cfgs(2);
	let mut node_cfgs = create_node_cfgs(2, &chanmon_cfgs);
	{
		// setup Taproot configuration
		node_cfgs[0].features.set_simple_taproot_required();
	}
	let node_chanmgrs = create_node_chanmgrs(2, &node_cfgs, &[None, None]);
	let nodes = create_network(2, &node_cfgs, &node_chanmgrs);

	assert!(node_cfgs[0].features.supports_simple_taproot(), "Taproot must be supported!");
	assert!(node_cfgs[0].features.requires_simple_taproot(), "Taproot must be required!");

	let channel_value_satoshis = 1000000;
	let push_msat = 10001;
	nodes[0].node.create_channel(nodes[1].node.get_our_node_id(), channel_value_satoshis, push_msat, 42, None).unwrap();
	let mut node0_to_1_send_open_channel = get_event_msg!(nodes[0], MessageSendEvent::SendOpenChannel, nodes[1].node.get_our_node_id());
	node0_to_1_send_open_channel.musig2_nonces = None;

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
	{
		// setup Taproot configuration
		node_cfgs[0].features.set_simple_taproot_required();
	}
	let node_chanmgrs = create_node_chanmgrs(2, &node_cfgs, &[None, None]);
	let nodes = create_network(2, &node_cfgs, &node_chanmgrs);

	assert!(node_cfgs[0].features.supports_simple_taproot(), "Taproot must be supported!");
	assert!(node_cfgs[0].features.requires_simple_taproot(), "Taproot must be required!");

	let channel_value_satoshis = 1000000;
	let push_msat = 10001;
	nodes[0].node.create_channel(nodes[1].node.get_our_node_id(), channel_value_satoshis, push_msat, 42, None).unwrap();
	let mut node0_to_1_send_open_channel = get_event_msg!(nodes[0], MessageSendEvent::SendOpenChannel, nodes[1].node.get_our_node_id());

	assert!(node0_to_1_send_open_channel.musig2_nonces.is_some());

	nodes[1].node.handle_open_channel(&nodes[0].node.get_our_node_id(), InitFeatures::known(), &node0_to_1_send_open_channel);
	let events = nodes[1].node.get_and_clear_pending_msg_events();
	let accept_channel = match events[0] {
		MessageSendEvent::SendAcceptChannel{ ref msg, node_id } => {
			msg.clone()
		}
		_ => panic!("Unexpected event"),
	};

	assert!(accept_channel.musig2_nonces.is_some())
}
