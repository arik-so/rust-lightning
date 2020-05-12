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

use lightning::util::events::MessageSendEvent as lnMessageSendEventImport;
type lnMessageSendEvent = lnMessageSendEventImport;

/// " An event generated by ChannelManager which indicates a message should be sent to a peer (or"
/// " broadcast to most peers)."
/// " These events are handled by PeerManager::process_events if you are using a PeerManager."
#[must_use]
#[repr(C)]
pub struct MessageSendEvent {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnMessageSendEvent,
	pub _underlying_ref: bool,
}

impl Drop for MessageSendEvent {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnMessageSendEvent) };
		}
	}
}
#[no_mangle]
pub extern "C" fn MessageSendEvent_free(this_ptr: MessageSendEvent) { }
impl Clone for MessageSendEvent {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
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
		let mut local_ret = Vec::new(); for mut item in ret.into_rust().drain(..) { local_ret.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
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
