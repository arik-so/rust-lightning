use lightning::ln::msgs::{HTLCFailChannelUpdate, ChannelAnnouncement, NodeAnnouncement, LightningError, ChannelUpdate};
use secp256k1::PublicKey;

pub struct RoutingMessageHandler {

}

impl lightning::ln::msgs::RoutingMessageHandler for RoutingMessageHandler {
	fn handle_node_announcement(&self, msg: &NodeAnnouncement) -> Result<bool, LightningError> {
		unimplemented!()
	}

	fn handle_channel_announcement(&self, msg: &ChannelAnnouncement) -> Result<bool, LightningError> {
		unimplemented!()
	}

	fn handle_channel_update(&self, msg: &ChannelUpdate) -> Result<bool, LightningError> {
		unimplemented!()
	}

	fn handle_htlc_fail_channel_update(&self, update: &HTLCFailChannelUpdate) {
		unimplemented!()
	}

	fn get_next_channel_announcements(&self, starting_point: u64, batch_amount: u8) -> Vec<(ChannelAnnouncement, Option<ChannelUpdate>, Option<ChannelUpdate>)> {
		unimplemented!()
	}

	fn get_next_node_announcements(&self, starting_point: Option<&PublicKey>, batch_amount: u8) -> Vec<NodeAnnouncement> {
		unimplemented!()
	}

	fn should_request_full_sync(&self, node_id: &PublicKey) -> bool {
		unimplemented!()
	}
}