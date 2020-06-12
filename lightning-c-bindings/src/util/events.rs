//! " Events are returned from various bits in the library which indicate some action must be taken"
//! " by the client."
//! ""
//! " Because we don't have a built-in runtime, it's up to the client to call events at a time in the"
//! " future, as well as generate and broadcast funding transactions handle payment preimages and a"
//! " few other things."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::*;

/// " An Event which you should probably take some action in response to."
/// ""
/// " Note that while Writeable and Readable are implemented for Event, you probably shouldn't use"
/// " them directly as they don't round-trip exactly (for example FundingGenerationReady is never"
/// " written as it makes no sense to respond to it after reconnecting to peers)."
#[must_use]
#[derive(Clone)]
#[repr(C)]
pub enum Event {
	/// " Used to indicate that the client should generate a funding transaction with the given"
	/// " parameters and then call ChannelManager::funding_transaction_generated."
	/// " Generated in ChannelManager message handling."
	/// " Note that *all inputs* in the funding transaction must spend SegWit outputs or your"
	/// " counterparty can steal your funds!"
	FundingGenerationReady {
		temporary_channel_id: crate::c_types::ThirtyTwoBytes,
		channel_value_satoshis: u64,
		output_script: crate::c_types::derived::CVec_u8Z,
		user_channel_id: u64,
	},
	/// " Used to indicate that the client may now broadcast the funding transaction it created for a"
	/// " channel. Broadcasting such a transaction prior to this event may lead to our counterparty"
	/// " trivially stealing all funds in the funding transaction!"
	FundingBroadcastSafe {
		funding_txo: crate::chain::transaction::OutPoint,
		user_channel_id: u64,
	},
	/// " Indicates we've received money! Just gotta dig out that payment preimage and feed it to"
	/// " ChannelManager::claim_funds to get it...."
	/// " Note that if the preimage is not known or the amount paid is incorrect, you should call"
	/// " ChannelManager::fail_htlc_backwards to free up resources for this HTLC and avoid"
	/// " network congestion."
	/// " The amount paid should be considered 'incorrect' when it is less than or more than twice"
	/// " the amount expected."
	/// " If you fail to call either ChannelManager::claim_funds or"
	/// " ChannelManager::fail_htlc_backwards within the HTLC's timeout, the HTLC will be"
	/// " automatically failed."
	PaymentReceived {
		payment_hash: [u8; 32],
		payment_secret: *const crate::c_types::ThirtyTwoBytes,
		amt: u64,
	},
	/// " Indicates an outbound payment we made succeeded (ie it made it all the way to its target"
	/// " and we got back the payment preimage for it)."
	/// " Note that duplicative PaymentSent Events may be generated - it is your responsibility to"
	/// " deduplicate them by payment_preimage (which MUST be unique)!"
	PaymentSent {
		payment_preimage: [u8; 32],
	},
	/// " Indicates an outbound payment we made failed. Probably some intermediary node dropped"
	/// " something. You may wish to retry with a different route."
	/// " Note that duplicative PaymentFailed Events may be generated - it is your responsibility to"
	/// " deduplicate them by payment_hash (which MUST be unique)!"
	PaymentFailed {
		payment_hash: [u8; 32],
		rejected_by_dest: bool,
	},
	/// " Used to indicate that ChannelManager::process_pending_htlc_forwards should be called at a"
	/// " time in the future."
	PendingHTLCsForwardable {
		time_forwardable: u64,
	},
	/// " Used to indicate that an output was generated on-chain which you should know how to spend."
	/// " Such an output will *not* ever be spent by rust-lightning, and are not at risk of your"
	/// " counterparty spending them due to some kind of timeout. Thus, you need to store them"
	/// " somewhere and spend them when you create on-chain transactions."
	SpendableOutputs {
		outputs: crate::c_types::derived::CVec_SpendableOutputDescriptorZ,
	},
}
use lightning::util::events::Event as lnEvent;
impl Event {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnEvent {
		match self {
			Event::FundingGenerationReady {ref temporary_channel_id, ref channel_value_satoshis, ref output_script, ref user_channel_id, } => {
				let mut temporary_channel_id_nonref = (*temporary_channel_id).clone();
				let mut channel_value_satoshis_nonref = (*channel_value_satoshis).clone();
				let mut output_script_nonref = (*output_script).clone();
				let mut user_channel_id_nonref = (*user_channel_id).clone();
				lnEvent::FundingGenerationReady {
					temporary_channel_id: temporary_channel_id_nonref.data,
					channel_value_satoshis: channel_value_satoshis_nonref,
					output_script: ::bitcoin::blockdata::script::Script::from(output_script_nonref.into_rust()),
					user_channel_id: user_channel_id_nonref,
				}
			},
			Event::FundingBroadcastSafe {ref funding_txo, ref user_channel_id, } => {
				let mut funding_txo_nonref = (*funding_txo).clone();
				let mut user_channel_id_nonref = (*user_channel_id).clone();
				lnEvent::FundingBroadcastSafe {
					funding_txo: *unsafe { Box::from_raw(funding_txo_nonref.inner.take_ptr() as *mut _) },
					user_channel_id: user_channel_id_nonref,
				}
			},
			Event::PaymentReceived {ref payment_hash, ref payment_secret, ref amt, } => {
				let mut payment_hash_nonref = (*payment_hash).clone();
				let mut payment_secret_nonref = (*payment_secret).clone();let mut local_payment_secret_nonref = if payment_secret_nonref.is_null() { None } else { Some( { ::lightning::ln::channelmanager::PaymentSecret(unsafe { *payment_secret_nonref }.data) }) };
				let mut amt_nonref = (*amt).clone();
				lnEvent::PaymentReceived {
					payment_hash: ::lightning::ln::channelmanager::PaymentHash(payment_hash_nonref),
					payment_secret: local_payment_secret_nonref,
					amt: amt_nonref,
				}
			},
			Event::PaymentSent {ref payment_preimage, } => {
				let mut payment_preimage_nonref = (*payment_preimage).clone();
				lnEvent::PaymentSent {
					payment_preimage: ::lightning::ln::channelmanager::PaymentPreimage(payment_preimage_nonref),
				}
			},
			Event::PaymentFailed {ref payment_hash, ref rejected_by_dest, } => {
				let mut payment_hash_nonref = (*payment_hash).clone();
				let mut rejected_by_dest_nonref = (*rejected_by_dest).clone();
				lnEvent::PaymentFailed {
					payment_hash: ::lightning::ln::channelmanager::PaymentHash(payment_hash_nonref),
					rejected_by_dest: rejected_by_dest_nonref,
				}
			},
			Event::PendingHTLCsForwardable {ref time_forwardable, } => {
				let mut time_forwardable_nonref = (*time_forwardable).clone();
				lnEvent::PendingHTLCsForwardable {
					time_forwardable: std::time::Duration::from_secs(time_forwardable_nonref),
				}
			},
			Event::SpendableOutputs {ref outputs, } => {
				let mut outputs_nonref = (*outputs).clone();let mut local_outputs_nonref = Vec::new(); for mut item in outputs_nonref.into_rust().drain(..) { local_outputs_nonref.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
				lnEvent::SpendableOutputs {
					outputs: local_outputs_nonref,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_ln(self) -> lnEvent {
		match self {
			Event::FundingGenerationReady {mut temporary_channel_id, mut channel_value_satoshis, mut output_script, mut user_channel_id, } => {
				lnEvent::FundingGenerationReady {
					temporary_channel_id: temporary_channel_id.data,
					channel_value_satoshis: channel_value_satoshis,
					output_script: ::bitcoin::blockdata::script::Script::from(output_script.into_rust()),
					user_channel_id: user_channel_id,
				}
			},
			Event::FundingBroadcastSafe {mut funding_txo, mut user_channel_id, } => {
				lnEvent::FundingBroadcastSafe {
					funding_txo: *unsafe { Box::from_raw(funding_txo.inner.take_ptr() as *mut _) },
					user_channel_id: user_channel_id,
				}
			},
			Event::PaymentReceived {mut payment_hash, mut payment_secret, mut amt, } => {
				let mut local_payment_secret = if payment_secret.is_null() { None } else { Some( { ::lightning::ln::channelmanager::PaymentSecret(unsafe { *payment_secret }.data) }) };
				lnEvent::PaymentReceived {
					payment_hash: ::lightning::ln::channelmanager::PaymentHash(payment_hash),
					payment_secret: local_payment_secret,
					amt: amt,
				}
			},
			Event::PaymentSent {mut payment_preimage, } => {
				lnEvent::PaymentSent {
					payment_preimage: ::lightning::ln::channelmanager::PaymentPreimage(payment_preimage),
				}
			},
			Event::PaymentFailed {mut payment_hash, mut rejected_by_dest, } => {
				lnEvent::PaymentFailed {
					payment_hash: ::lightning::ln::channelmanager::PaymentHash(payment_hash),
					rejected_by_dest: rejected_by_dest,
				}
			},
			Event::PendingHTLCsForwardable {mut time_forwardable, } => {
				lnEvent::PendingHTLCsForwardable {
					time_forwardable: std::time::Duration::from_secs(time_forwardable),
				}
			},
			Event::SpendableOutputs {mut outputs, } => {
				let mut local_outputs = Vec::new(); for mut item in outputs.into_rust().drain(..) { local_outputs.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
				lnEvent::SpendableOutputs {
					outputs: local_outputs,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: &lnEvent) -> Self {
		match lnt {
			lnEvent::FundingGenerationReady {ref temporary_channel_id, ref channel_value_satoshis, ref output_script, ref user_channel_id, } => {
				let mut temporary_channel_id_nonref = (*temporary_channel_id).clone();
				let mut channel_value_satoshis_nonref = (*channel_value_satoshis).clone();
				let mut output_script_nonref = (*output_script).clone();
				let mut user_channel_id_nonref = (*user_channel_id).clone();
				Event::FundingGenerationReady {
					temporary_channel_id: crate::c_types::ThirtyTwoBytes { data: temporary_channel_id_nonref },
					channel_value_satoshis: channel_value_satoshis_nonref,
					output_script: output_script_nonref.into_bytes().into(),
					user_channel_id: user_channel_id_nonref,
				}
			},
			lnEvent::FundingBroadcastSafe {ref funding_txo, ref user_channel_id, } => {
				let mut funding_txo_nonref = (*funding_txo).clone();
				let mut user_channel_id_nonref = (*user_channel_id).clone();
				Event::FundingBroadcastSafe {
					funding_txo: crate::chain::transaction::OutPoint { inner: Box::into_raw(Box::new(funding_txo_nonref)), _underlying_ref: false },
					user_channel_id: user_channel_id_nonref,
				}
			},
			lnEvent::PaymentReceived {ref payment_hash, ref payment_secret, ref amt, } => {
				let mut payment_hash_nonref = (*payment_hash).clone();
				let mut payment_secret_nonref = (*payment_secret).clone();let mut local_payment_secret_nonref = if payment_secret_nonref.is_none() { std::ptr::null_mut() } else { Box::into_raw(Box::new( { crate::c_types::ThirtyTwoBytes { data: (payment_secret_nonref.unwrap()).0 } })) };
				let mut amt_nonref = (*amt).clone();
				Event::PaymentReceived {
					payment_hash: payment_hash_nonref.0,
					payment_secret: local_payment_secret_nonref,
					amt: amt_nonref,
				}
			},
			lnEvent::PaymentSent {ref payment_preimage, } => {
				let mut payment_preimage_nonref = (*payment_preimage).clone();
				Event::PaymentSent {
					payment_preimage: payment_preimage_nonref.0,
				}
			},
			lnEvent::PaymentFailed {ref payment_hash, ref rejected_by_dest, } => {
				let mut payment_hash_nonref = (*payment_hash).clone();
				let mut rejected_by_dest_nonref = (*rejected_by_dest).clone();
				Event::PaymentFailed {
					payment_hash: payment_hash_nonref.0,
					rejected_by_dest: rejected_by_dest_nonref,
				}
			},
			lnEvent::PendingHTLCsForwardable {ref time_forwardable, } => {
				let mut time_forwardable_nonref = (*time_forwardable).clone();
				Event::PendingHTLCsForwardable {
					time_forwardable: time_forwardable_nonref.as_secs(),
				}
			},
			lnEvent::SpendableOutputs {ref outputs, } => {
				let mut outputs_nonref = (*outputs).clone();let mut local_outputs_nonref = Vec::new(); for item in outputs_nonref.drain(..) { local_outputs_nonref.push( { crate::chain::keysinterface::SpendableOutputDescriptor { inner: Box::into_raw(Box::new(item)), _underlying_ref: false } }); };
				Event::SpendableOutputs {
					outputs: local_outputs_nonref.into(),
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn ln_into(lnt: lnEvent) -> Self {
		match lnt {
			lnEvent::FundingGenerationReady {mut temporary_channel_id, mut channel_value_satoshis, mut output_script, mut user_channel_id, } => {
				Event::FundingGenerationReady {
					temporary_channel_id: crate::c_types::ThirtyTwoBytes { data: temporary_channel_id },
					channel_value_satoshis: channel_value_satoshis,
					output_script: output_script.into_bytes().into(),
					user_channel_id: user_channel_id,
				}
			},
			lnEvent::FundingBroadcastSafe {mut funding_txo, mut user_channel_id, } => {
				Event::FundingBroadcastSafe {
					funding_txo: crate::chain::transaction::OutPoint { inner: Box::into_raw(Box::new(funding_txo)), _underlying_ref: false },
					user_channel_id: user_channel_id,
				}
			},
			lnEvent::PaymentReceived {mut payment_hash, mut payment_secret, mut amt, } => {
				let mut local_payment_secret = if payment_secret.is_none() { std::ptr::null_mut() } else { Box::into_raw(Box::new( { crate::c_types::ThirtyTwoBytes { data: (payment_secret.unwrap()).0 } })) };
				Event::PaymentReceived {
					payment_hash: payment_hash.0,
					payment_secret: local_payment_secret,
					amt: amt,
				}
			},
			lnEvent::PaymentSent {mut payment_preimage, } => {
				Event::PaymentSent {
					payment_preimage: payment_preimage.0,
				}
			},
			lnEvent::PaymentFailed {mut payment_hash, mut rejected_by_dest, } => {
				Event::PaymentFailed {
					payment_hash: payment_hash.0,
					rejected_by_dest: rejected_by_dest,
				}
			},
			lnEvent::PendingHTLCsForwardable {mut time_forwardable, } => {
				Event::PendingHTLCsForwardable {
					time_forwardable: time_forwardable.as_secs(),
				}
			},
			lnEvent::SpendableOutputs {mut outputs, } => {
				let mut local_outputs = Vec::new(); for item in outputs.drain(..) { local_outputs.push( { crate::chain::keysinterface::SpendableOutputDescriptor { inner: Box::into_raw(Box::new(item)), _underlying_ref: false } }); };
				Event::SpendableOutputs {
					outputs: local_outputs.into(),
				}
			},
		}
	}
}
#[no_mangle]
pub extern "C" fn Event_free(this_ptr: Event) { }
/// " An event generated by ChannelManager which indicates a message should be sent to a peer (or"
/// " broadcast to most peers)."
/// " These events are handled by PeerManager::process_events if you are using a PeerManager."
#[must_use]
#[derive(Clone)]
#[repr(C)]
pub enum MessageSendEvent {
	/// " Used to indicate that we've accepted a channel open and should send the accept_channel"
	/// " message provided to the given peer."
	SendAcceptChannel {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::AcceptChannel,
	},
	/// " Used to indicate that we've initiated a channel open and should send the open_channel"
	/// " message provided to the given peer."
	SendOpenChannel {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::OpenChannel,
	},
	/// " Used to indicate that a funding_created message should be sent to the peer with the given node_id."
	SendFundingCreated {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::FundingCreated,
	},
	/// " Used to indicate that a funding_signed message should be sent to the peer with the given node_id."
	SendFundingSigned {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::FundingSigned,
	},
	/// " Used to indicate that a funding_locked message should be sent to the peer with the given node_id."
	SendFundingLocked {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::FundingLocked,
	},
	/// " Used to indicate that an announcement_signatures message should be sent to the peer with the given node_id."
	SendAnnouncementSignatures {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::AnnouncementSignatures,
	},
	/// " Used to indicate that a series of HTLC update messages, as well as a commitment_signed"
	/// " message should be sent to the peer with the given node_id."
	UpdateHTLCs {
		node_id: crate::c_types::PublicKey,
		updates: crate::ln::msgs::CommitmentUpdate,
	},
	/// " Used to indicate that a revoke_and_ack message should be sent to the peer with the given node_id."
	SendRevokeAndACK {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::RevokeAndACK,
	},
	/// " Used to indicate that a closing_signed message should be sent to the peer with the given node_id."
	SendClosingSigned {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::ClosingSigned,
	},
	/// " Used to indicate that a shutdown message should be sent to the peer with the given node_id."
	SendShutdown {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::Shutdown,
	},
	/// " Used to indicate that a channel_reestablish message should be sent to the peer with the given node_id."
	SendChannelReestablish {
		node_id: crate::c_types::PublicKey,
		msg: crate::ln::msgs::ChannelReestablish,
	},
	/// " Used to indicate that a channel_announcement and channel_update should be broadcast to all"
	/// " peers (except the peer with node_id either msg.contents.node_id_1 or msg.contents.node_id_2)."
	/// ""
	/// " Note that after doing so, you very likely (unless you did so very recently) want to call"
	/// " ChannelManager::broadcast_node_announcement to trigger a BroadcastNodeAnnouncement event."
	/// " This ensures that any nodes which see our channel_announcement also have a relevant"
	/// " node_announcement, including relevant feature flags which may be important for routing"
	/// " through or to us."
	BroadcastChannelAnnouncement {
		msg: crate::ln::msgs::ChannelAnnouncement,
		update_msg: crate::ln::msgs::ChannelUpdate,
	},
	/// " Used to indicate that a node_announcement should be broadcast to all peers."
	BroadcastNodeAnnouncement {
		msg: crate::ln::msgs::NodeAnnouncement,
	},
	/// " Used to indicate that a channel_update should be broadcast to all peers."
	BroadcastChannelUpdate {
		msg: crate::ln::msgs::ChannelUpdate,
	},
	/// " Broadcast an error downstream to be handled"
	HandleError {
		node_id: crate::c_types::PublicKey,
		action: crate::ln::msgs::ErrorAction,
	},
	/// " When a payment fails we may receive updates back from the hop where it failed. In such"
	/// " cases this event is generated so that we can inform the network graph of this information."
	PaymentFailureNetworkUpdate {
		update: crate::ln::msgs::HTLCFailChannelUpdate,
	},
}
use lightning::util::events::MessageSendEvent as lnMessageSendEvent;
impl MessageSendEvent {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnMessageSendEvent {
		match self {
			MessageSendEvent::SendAcceptChannel {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendAcceptChannel {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendOpenChannel {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendOpenChannel {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendFundingCreated {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendFundingCreated {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendFundingSigned {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendFundingSigned {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendFundingLocked {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendFundingLocked {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendAnnouncementSignatures {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendAnnouncementSignatures {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::UpdateHTLCs {ref node_id, ref updates, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut updates_nonref = (*updates).clone();
				lnMessageSendEvent::UpdateHTLCs {
					node_id: node_id_nonref.into_rust(),
					updates: *unsafe { Box::from_raw(updates_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendRevokeAndACK {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendRevokeAndACK {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendClosingSigned {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendClosingSigned {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendShutdown {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendShutdown {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendChannelReestablish {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::SendChannelReestablish {
					node_id: node_id_nonref.into_rust(),
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::BroadcastChannelAnnouncement {ref msg, ref update_msg, } => {
				let mut msg_nonref = (*msg).clone();
				let mut update_msg_nonref = (*update_msg).clone();
				lnMessageSendEvent::BroadcastChannelAnnouncement {
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
					update_msg: *unsafe { Box::from_raw(update_msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::BroadcastNodeAnnouncement {ref msg, } => {
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::BroadcastNodeAnnouncement {
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::BroadcastChannelUpdate {ref msg, } => {
				let mut msg_nonref = (*msg).clone();
				lnMessageSendEvent::BroadcastChannelUpdate {
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::HandleError {ref node_id, ref action, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut action_nonref = (*action).clone();
				lnMessageSendEvent::HandleError {
					node_id: node_id_nonref.into_rust(),
					action: action_nonref.into_ln(),
				}
			},
			MessageSendEvent::PaymentFailureNetworkUpdate {ref update, } => {
				let mut update_nonref = (*update).clone();
				lnMessageSendEvent::PaymentFailureNetworkUpdate {
					update: update_nonref.into_ln(),
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_ln(self) -> lnMessageSendEvent {
		match self {
			MessageSendEvent::SendAcceptChannel {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendAcceptChannel {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendOpenChannel {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendOpenChannel {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendFundingCreated {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendFundingCreated {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendFundingSigned {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendFundingSigned {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendFundingLocked {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendFundingLocked {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendAnnouncementSignatures {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendAnnouncementSignatures {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::UpdateHTLCs {mut node_id, mut updates, } => {
				lnMessageSendEvent::UpdateHTLCs {
					node_id: node_id.into_rust(),
					updates: *unsafe { Box::from_raw(updates.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendRevokeAndACK {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendRevokeAndACK {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendClosingSigned {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendClosingSigned {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendShutdown {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendShutdown {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::SendChannelReestablish {mut node_id, mut msg, } => {
				lnMessageSendEvent::SendChannelReestablish {
					node_id: node_id.into_rust(),
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::BroadcastChannelAnnouncement {mut msg, mut update_msg, } => {
				lnMessageSendEvent::BroadcastChannelAnnouncement {
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
					update_msg: *unsafe { Box::from_raw(update_msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::BroadcastNodeAnnouncement {mut msg, } => {
				lnMessageSendEvent::BroadcastNodeAnnouncement {
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::BroadcastChannelUpdate {mut msg, } => {
				lnMessageSendEvent::BroadcastChannelUpdate {
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			MessageSendEvent::HandleError {mut node_id, mut action, } => {
				lnMessageSendEvent::HandleError {
					node_id: node_id.into_rust(),
					action: action.into_ln(),
				}
			},
			MessageSendEvent::PaymentFailureNetworkUpdate {mut update, } => {
				lnMessageSendEvent::PaymentFailureNetworkUpdate {
					update: update.into_ln(),
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: &lnMessageSendEvent) -> Self {
		match lnt {
			lnMessageSendEvent::SendAcceptChannel {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendAcceptChannel {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::AcceptChannel { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendOpenChannel {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendOpenChannel {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::OpenChannel { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendFundingCreated {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendFundingCreated {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::FundingCreated { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendFundingSigned {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendFundingSigned {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::FundingSigned { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendFundingLocked {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendFundingLocked {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::FundingLocked { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendAnnouncementSignatures {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendAnnouncementSignatures {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::AnnouncementSignatures { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::UpdateHTLCs {ref node_id, ref updates, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut updates_nonref = (*updates).clone();
				MessageSendEvent::UpdateHTLCs {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					updates: crate::ln::msgs::CommitmentUpdate { inner: Box::into_raw(Box::new(updates_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendRevokeAndACK {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendRevokeAndACK {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::RevokeAndACK { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendClosingSigned {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendClosingSigned {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::ClosingSigned { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendShutdown {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendShutdown {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::Shutdown { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendChannelReestablish {ref node_id, ref msg, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::SendChannelReestablish {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					msg: crate::ln::msgs::ChannelReestablish { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::BroadcastChannelAnnouncement {ref msg, ref update_msg, } => {
				let mut msg_nonref = (*msg).clone();
				let mut update_msg_nonref = (*update_msg).clone();
				MessageSendEvent::BroadcastChannelAnnouncement {
					msg: crate::ln::msgs::ChannelAnnouncement { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
					update_msg: crate::ln::msgs::ChannelUpdate { inner: Box::into_raw(Box::new(update_msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::BroadcastNodeAnnouncement {ref msg, } => {
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::BroadcastNodeAnnouncement {
					msg: crate::ln::msgs::NodeAnnouncement { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::BroadcastChannelUpdate {ref msg, } => {
				let mut msg_nonref = (*msg).clone();
				MessageSendEvent::BroadcastChannelUpdate {
					msg: crate::ln::msgs::ChannelUpdate { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::HandleError {ref node_id, ref action, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut action_nonref = (*action).clone();
				MessageSendEvent::HandleError {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					action: crate::ln::msgs::ErrorAction::ln_into(action_nonref),
				}
			},
			lnMessageSendEvent::PaymentFailureNetworkUpdate {ref update, } => {
				let mut update_nonref = (*update).clone();
				MessageSendEvent::PaymentFailureNetworkUpdate {
					update: crate::ln::msgs::HTLCFailChannelUpdate::ln_into(update_nonref),
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn ln_into(lnt: lnMessageSendEvent) -> Self {
		match lnt {
			lnMessageSendEvent::SendAcceptChannel {mut node_id, mut msg, } => {
				MessageSendEvent::SendAcceptChannel {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::AcceptChannel { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendOpenChannel {mut node_id, mut msg, } => {
				MessageSendEvent::SendOpenChannel {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::OpenChannel { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendFundingCreated {mut node_id, mut msg, } => {
				MessageSendEvent::SendFundingCreated {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::FundingCreated { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendFundingSigned {mut node_id, mut msg, } => {
				MessageSendEvent::SendFundingSigned {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::FundingSigned { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendFundingLocked {mut node_id, mut msg, } => {
				MessageSendEvent::SendFundingLocked {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::FundingLocked { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendAnnouncementSignatures {mut node_id, mut msg, } => {
				MessageSendEvent::SendAnnouncementSignatures {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::AnnouncementSignatures { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::UpdateHTLCs {mut node_id, mut updates, } => {
				MessageSendEvent::UpdateHTLCs {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					updates: crate::ln::msgs::CommitmentUpdate { inner: Box::into_raw(Box::new(updates)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendRevokeAndACK {mut node_id, mut msg, } => {
				MessageSendEvent::SendRevokeAndACK {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::RevokeAndACK { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendClosingSigned {mut node_id, mut msg, } => {
				MessageSendEvent::SendClosingSigned {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::ClosingSigned { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendShutdown {mut node_id, mut msg, } => {
				MessageSendEvent::SendShutdown {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::Shutdown { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::SendChannelReestablish {mut node_id, mut msg, } => {
				MessageSendEvent::SendChannelReestablish {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					msg: crate::ln::msgs::ChannelReestablish { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::BroadcastChannelAnnouncement {mut msg, mut update_msg, } => {
				MessageSendEvent::BroadcastChannelAnnouncement {
					msg: crate::ln::msgs::ChannelAnnouncement { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
					update_msg: crate::ln::msgs::ChannelUpdate { inner: Box::into_raw(Box::new(update_msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::BroadcastNodeAnnouncement {mut msg, } => {
				MessageSendEvent::BroadcastNodeAnnouncement {
					msg: crate::ln::msgs::NodeAnnouncement { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::BroadcastChannelUpdate {mut msg, } => {
				MessageSendEvent::BroadcastChannelUpdate {
					msg: crate::ln::msgs::ChannelUpdate { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnMessageSendEvent::HandleError {mut node_id, mut action, } => {
				MessageSendEvent::HandleError {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					action: crate::ln::msgs::ErrorAction::ln_into(action),
				}
			},
			lnMessageSendEvent::PaymentFailureNetworkUpdate {mut update, } => {
				MessageSendEvent::PaymentFailureNetworkUpdate {
					update: crate::ln::msgs::HTLCFailChannelUpdate::ln_into(update),
				}
			},
		}
	}
}
#[no_mangle]
pub extern "C" fn MessageSendEvent_free(this_ptr: MessageSendEvent) { }
/// " A trait indicating an object may generate message send events"
#[repr(C)]
pub struct MessageSendEventsProvider {
	pub this_arg: *mut c_void,
	/// " Gets the list of pending events which were generated by previous actions, clearing the list"
	/// " in the process."
	#[must_use]
	pub get_and_clear_pending_msg_events: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::CVec_MessageSendEventZ,
}

use lightning::util::events::MessageSendEventsProvider as lnMessageSendEventsProvider;
impl lnMessageSendEventsProvider for MessageSendEventsProvider {
	fn get_and_clear_pending_msg_events(&self) -> Vec<lightning::util::events::MessageSendEvent> {
		let mut ret = (self.get_and_clear_pending_msg_events)(self.this_arg);
		let mut local_ret = Vec::new(); for mut item in ret.into_rust().drain(..) { local_ret.push( { item.into_ln() }); };
		local_ret
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for MessageSendEventsProvider {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// " A trait indicating an object may generate events"
#[repr(C)]
pub struct EventsProvider {
	pub this_arg: *mut c_void,
	/// " Gets the list of pending events which were generated by previous actions, clearing the list"
	/// " in the process."
	#[must_use]
	pub get_and_clear_pending_events: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::CVec_EventZ,
}

use lightning::util::events::EventsProvider as lnEventsProvider;
impl lnEventsProvider for EventsProvider {
	fn get_and_clear_pending_events(&self) -> Vec<lightning::util::events::Event> {
		let mut ret = (self.get_and_clear_pending_events)(self.this_arg);
		let mut local_ret = Vec::new(); for mut item in ret.into_rust().drain(..) { local_ret.push( { item.into_ln() }); };
		local_ret
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for EventsProvider {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
