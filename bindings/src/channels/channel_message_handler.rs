use lightning::ln::msgs::{OpenChannel, UpdateAddHTLC, ClosingSigned, AcceptChannel, CommitmentSigned, FundingLocked, FundingSigned, Init, FundingCreated, UpdateFulfillHTLC, UpdateFailHTLC, Shutdown, UpdateFailMalformedHTLC, RevokeAndACK, AnnouncementSignatures, ChannelReestablish, ErrorMessage, UpdateFee};
use secp256k1::PublicKey;
use lightning::ln::features::InitFeatures;
use lightning::util::events::MessageSendEvent;

pub struct ChannelMessageHandler {

}

impl lightning::ln::msgs::ChannelMessageHandler for ChannelMessageHandler{
	fn handle_open_channel(&self, their_node_id: &PublicKey, their_features: InitFeatures, msg: &OpenChannel) {
		unimplemented!()
	}

	fn handle_accept_channel(&self, their_node_id: &PublicKey, their_features: InitFeatures, msg: &AcceptChannel) {
		unimplemented!()
	}

	fn handle_funding_created(&self, their_node_id: &PublicKey, msg: &FundingCreated) {
		unimplemented!()
	}

	fn handle_funding_signed(&self, their_node_id: &PublicKey, msg: &FundingSigned) {
		unimplemented!()
	}

	fn handle_funding_locked(&self, their_node_id: &PublicKey, msg: &FundingLocked) {
		unimplemented!()
	}

	fn handle_shutdown(&self, their_node_id: &PublicKey, msg: &Shutdown) {
		unimplemented!()
	}

	fn handle_closing_signed(&self, their_node_id: &PublicKey, msg: &ClosingSigned) {
		unimplemented!()
	}

	fn handle_update_add_htlc(&self, their_node_id: &PublicKey, msg: &UpdateAddHTLC) {
		unimplemented!()
	}

	fn handle_update_fulfill_htlc(&self, their_node_id: &PublicKey, msg: &UpdateFulfillHTLC) {
		unimplemented!()
	}

	fn handle_update_fail_htlc(&self, their_node_id: &PublicKey, msg: &UpdateFailHTLC) {
		unimplemented!()
	}

	fn handle_update_fail_malformed_htlc(&self, their_node_id: &PublicKey, msg: &UpdateFailMalformedHTLC) {
		unimplemented!()
	}

	fn handle_commitment_signed(&self, their_node_id: &PublicKey, msg: &CommitmentSigned) {
		unimplemented!()
	}

	fn handle_revoke_and_ack(&self, their_node_id: &PublicKey, msg: &RevokeAndACK) {
		unimplemented!()
	}

	fn handle_update_fee(&self, their_node_id: &PublicKey, msg: &UpdateFee) {
		unimplemented!()
	}

	fn handle_announcement_signatures(&self, their_node_id: &PublicKey, msg: &AnnouncementSignatures) {
		unimplemented!()
	}

	fn peer_disconnected(&self, their_node_id: &PublicKey, no_connection_possible: bool) {
		unimplemented!()
	}

	fn peer_connected(&self, their_node_id: &PublicKey, msg: &Init) {
		unimplemented!()
	}

	fn handle_channel_reestablish(&self, their_node_id: &PublicKey, msg: &ChannelReestablish) {
		unimplemented!()
	}

	fn handle_error(&self, their_node_id: &PublicKey, msg: &ErrorMessage) {
		unimplemented!()
	}
}

impl lightning::util::events::MessageSendEventsProvider for ChannelMessageHandler{
	fn get_and_clear_pending_msg_events(&self) -> Vec<MessageSendEvent> {
		unimplemented!()
	}
}