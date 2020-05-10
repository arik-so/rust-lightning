//! " Top level peer message handling and socket handling logic lives here."
//! ""
//! " Instead of actually servicing sockets ourselves we require that you implement the"
//! " SocketDescriptor interface and use that to receive actions which you should perform on the"
//! " socket, and call into PeerManager with bytes read from the socket. The PeerManager will then"
//! " call into the provided message handlers (probably a ChannelManager and Router) with messages"
//! " they should handle, and encoding/sending response messages."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

use bitcoin::secp256k1::key::SecretKey as lnSecretKey;
use bitcoin::secp256k1::key::PublicKey as lnPublicKey;
use bitcoin::hashes::HashEngine as lnHashEngine;
use bitcoin::hashes::Hash as lnHash;

use lightning::ln::peer_handler::MessageHandler as lnMessageHandlerImport;
type lnMessageHandler = lnMessageHandlerImport<crate::ln::msgs::ChannelMessageHandler>;

/// " Provides references to trait impls which handle different types of messages."
#[repr(C)]
pub struct MessageHandler {
	pub(crate) inner: *const lnMessageHandler,
}

#[no_mangle]
pub extern "C" fn MessageHandler_get_chan_handler(this_ptr: *const MessageHandler) -> *const crate::ln::msgs::ChannelMessageHandler {
	&unsafe { &*(*this_ptr).inner }.chan_handler
}
/// " Provides an object which can be used to send data to and which uniquely identifies a connection"
/// " to a remote host. You will need to be able to generate multiple of these which meet Eq and"
/// " implement Hash to meet the PeerManager API."
/// ""
/// " For efficiency, Clone should be relatively cheap for this type."
/// ""
/// " You probably want to just extend an int and put a file descriptor in a struct and implement"
/// " send_data. Note that if you are using a higher-level net library that may call close() itself,"
/// " be careful to ensure you don't have races whereby you might register a new connection with an"
/// " fd which is the same as a previous one which has yet to be removed via"
/// " PeerManager::socket_disconnected()."
#[derive(Clone)]
#[repr(C)]
pub struct SocketDescriptor {
	pub this_arg: *mut c_void,
	/// " Attempts to send some data from the given slice to the peer."
	/// ""
	/// " Returns the amount of data which was sent, possibly 0 if the socket has since disconnected."
	/// " Note that in the disconnected case, socket_disconnected must still fire and further write"
	/// " attempts may occur until that time."
	/// ""
	/// " If the returned size is smaller than data.len(), a write_available event must"
	/// " trigger the next time more data can be written. Additionally, until the a send_data event"
	/// " completes fully, no further read_events should trigger on the same peer!"
	/// ""
	/// " If a read_event on this descriptor had previously returned true (indicating that read"
	/// " events should be paused to prevent DoS in the send buffer), resume_read may be set"
	/// " indicating that read events on this descriptor should resume. A resume_read of false does"
	/// " *not* imply that further read events should be paused."
	pub send_data: extern "C" fn (this_arg: *mut c_void, data: crate::c_types::u8slice, resume_read: bool) -> usize,
	/// " Disconnect the socket pointed to by this SocketDescriptor. Once this function returns, no"
	/// " more calls to write_buffer_space_avail, read_event or socket_disconnected may be made with"
	/// " this descriptor. No socket_disconnected call should be generated as a result of this call,"
	/// " though races may occur whereby disconnect_socket is called after a call to"
	/// " socket_disconnected but prior to socket_disconnected returning."
	pub disconnect_socket: extern "C" fn (this_arg: *mut c_void),
	pub eq: extern "C" fn (this_arg: *const c_void, other_arg: *const c_void) -> bool,
	pub hash: extern "C" fn (this_arg: *const c_void) -> u64,
}
impl std::cmp::Eq for SocketDescriptor {}
impl std::cmp::PartialEq for SocketDescriptor {
	fn eq(&self, o: &Self) -> bool { (self.eq)(self.this_arg, o.this_arg) }
}
impl std::hash::Hash for SocketDescriptor {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) { hasher.write_u64((self.hash)(self.this_arg)) }
}

use lightning::ln::peer_handler::SocketDescriptor as lnSocketDescriptor;
impl lnSocketDescriptor for SocketDescriptor {
	fn send_data(&mut self, data: &[u8], resume_read: bool) -> usize {
		let c_data = crate::c_types::u8slice::from_slice(data);
		(self.send_data)(self.this_arg, c_data, resume_read)
	}
	fn disconnect_socket(&mut self) {
		(self.disconnect_socket)(self.this_arg)
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for SocketDescriptor {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}

use lightning::ln::peer_handler::PeerHandleError as lnPeerHandleErrorImport;
type lnPeerHandleError = lnPeerHandleErrorImport;

/// " Error for PeerManager errors. If you get one of these, you must disconnect the socket and"
/// " generate no further read_event/write_buffer_space_avail calls for the descriptor, only"
/// " triggering a single socket_disconnected call (unless it was provided in response to a"
/// " new_*_connection event, in which case no such socket_disconnected() must be called and the"
/// " socket silently disconencted)."
#[repr(C)]
pub struct PeerHandleError {
	pub(crate) inner: *const lnPeerHandleError,
}


use lightning::ln::peer_handler::PeerManager as lnPeerManagerImport;
type lnPeerManager = lnPeerManagerImport<crate::ln::peer_handler::SocketDescriptor, crate::ln::msgs::ChannelMessageHandler>;

/// " A PeerManager manages a set of peers, described by their SocketDescriptor and marshalls socket"
/// " events into messages which it passes on to its MessageHandlers."
/// ""
/// " Rather than using a plain PeerManager, it is preferable to use either a SimpleArcPeerManager"
/// " a SimpleRefPeerManager, for conciseness. See their documentation for more details, but"
/// " essentially you should default to using a SimpleRefPeerManager, and use a"
/// " SimpleArcPeerManager when you require a PeerManager with a static lifetime, such as when"
/// " you're using lightning-net-tokio."
#[repr(C)]
pub struct PeerManager {
	pub(crate) inner: *const lnPeerManager,
}

/// " Constructs a new PeerManager with the given message handlers and node_id secret key"
/// " ephemeral_random_data is used to derive per-connection ephemeral keys and must be"
/// " cryptographically secure random bytes."
#[no_mangle]
pub extern "C" fn PeerManager_new(message_handler: MessageHandler, our_node_secret: crate::c_types::SecretKey, ephemeral_random_data: *const [u8; 32], logger: crate::util::logger::Logger) -> PeerManager {
	let rust_logger = std::sync::Arc::new(logger);
	PeerManager { inner: Box::into_raw(Box::new(lightning::ln::peer_handler::PeerManager::new(*unsafe { Box::from_raw(message_handler.inner as *mut _) }, our_node_secret.into_rust(), unsafe { &*ephemeral_random_data}, rust_logger))) }
}

/// " Checks for any events generated by our handlers and processes them. Includes sending most"
/// " response messages as well as messages generated by calls to handler functions directly (eg"
/// " functions like ChannelManager::process_pending_htlc_forward or send_payment)."
#[no_mangle]
pub extern "C" fn PeerManager_process_events(this_arg: *const PeerManager) {
	unsafe { &*(*this_arg).inner }.process_events()
}

/// " Indicates that the given socket descriptor's connection is now closed."
/// ""
/// " This must only be called if the socket has been disconnected by the peer or your own"
/// " decision to disconnect it and must NOT be called in any case where other parts of this"
/// " library (eg PeerHandleError, explicit disconnect_socket calls) instruct you to disconnect"
/// " the peer."
/// ""
/// " Panics if the descriptor was not previously registered in a successful new_*_connection event."
#[no_mangle]
pub extern "C" fn PeerManager_socket_disconnected(this_arg: *const PeerManager, descriptor: *const SocketDescriptor) {
	unsafe { &*(*this_arg).inner }.socket_disconnected(unsafe { &*descriptor })
}

/// " This function should be called roughly once every 30 seconds."
/// " It will send pings to each peer and disconnect those which did not respond to the last round of pings."
/// " Will most likely call send_data on all of the registered descriptors, thus, be very careful with reentrancy issues!"
#[no_mangle]
pub extern "C" fn PeerManager_timer_tick_occured(this_arg: *const PeerManager) {
	unsafe { &*(*this_arg).inner }.timer_tick_occured()
}

