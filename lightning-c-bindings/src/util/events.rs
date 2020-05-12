//! " Events are returned from various bits in the library which indicate some action must be taken"
//! " by the client."
//! ""
//! " Because we don't have a built-in runtime, it's up to the client to call events at a time in the"
//! " future, as well as generate and broadcast funding transactions handle payment preimages and a"
//! " few other things."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::*;


use lightning::util::events::Event as lnEventImport;
type lnEvent = lnEventImport;

/// " An Event which you should probably take some action in response to."
/// ""
/// " Note that while Writeable and Readable are implemented for Event, you probably shouldn't use"
/// " them directly as they don't round-trip exactly (for example FundingGenerationReady is never"
/// " written as it makes no sense to respond to it after reconnecting to peers)."
#[repr(C)]
pub struct Event {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnEvent,
	pub _underlying_ref: bool,
}

impl Drop for Event {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnEvent) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Event_free(this_ptr: Event) { }
#[no_mangle]
pub extern "C" fn Event_write(obj: *const Event) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}

use lightning::util::events::MessageSendEvent as lnMessageSendEventImport;
type lnMessageSendEvent = lnMessageSendEventImport;

/// " An event generated by ChannelManager which indicates a message should be sent to a peer (or"
/// " broadcast to most peers)."
/// " These events are handled by PeerManager::process_events if you are using a PeerManager."
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
/// " A trait indicating an object may generate message send events"
#[repr(C)]
pub struct MessageSendEventsProvider {
	pub this_arg: *mut c_void,
	/// " Gets the list of pending events which were generated by previous actions, clearing the list"
	/// " in the process."
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
	pub get_and_clear_pending_events: extern "C" fn (this_arg: *const c_void) -> crate::c_types::derived::CVec_EventZ,
}

use lightning::util::events::EventsProvider as lnEventsProvider;
impl lnEventsProvider for EventsProvider {
	fn get_and_clear_pending_events(&self) -> Vec<lightning::util::events::Event> {
		let mut ret = (self.get_and_clear_pending_events)(self.this_arg);
		let mut local_ret = Vec::new(); for mut item in ret.into_rust().drain(..) { local_ret.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
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
