//! Top level peer message handling and socket handling logic lives here.
//!
//! Instead of actually servicing sockets ourselves we require that you implement the
//! SocketDescriptor interface and use that to receive actions which you should perform on the
//! socket, and call into PeerManager with bytes read from the socket. The PeerManager will then
//! call into the provided message handlers (probably a ChannelManager and Router) with messages
//! they should handle, and encoding/sending response messages.

use std::{cmp, error, fmt, hash};
use std::collections::{hash_map, HashMap, HashSet, LinkedList};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use bitcoin_hashes::{Hash, HashEngine};
use bitcoin_hashes::sha256::Hash as Sha256;
use bitcoin_hashes::sha256::HashEngine as Sha256Engine;
use secp256k1::key::{PublicKey, SecretKey};

use ln::features::InitFeatures;
use ln::msgs;
use ln::peer_channel_encryptor::{NextNoiseStep, PeerChannelEncryptor};
use ln::peers::handshake::PeerHandshake;
pub use ln::peers::coordinator::PeerCoordinator as PeerManager;
use util::byte_utils;
use util::events::MessageSendEvent;
use util::logger::Logger;
use util::ser::{Readable, Writeable, Writer};
use ln::peers::peer::ConnectedPeer;

/// Provides references to trait impls which handle different types of messages.
pub struct MessageHandler {
	/// A message handler which handles messages specific to channels. Usually this is just a
	/// ChannelManager object.
	pub chan_handler: Arc<msgs::ChannelMessageHandler>,
	/// A message handler which handles messages updating our knowledge of the network channel
	/// graph. Usually this is just a Router object.
	pub route_handler: Arc<msgs::RoutingMessageHandler>,
}

/// Provides an object which can be used to send data to and which uniquely identifies a connection
/// to a remote host. You will need to be able to generate multiple of these which meet Eq and
/// implement Hash to meet the PeerManager API.
///
/// For efficiency, Clone should be relatively cheap for this type.
///
/// You probably want to just extend an int and put a file descriptor in a struct and implement
/// send_data. Note that if you are using a higher-level net library that may close() itself, be
/// careful to ensure you don't have races whereby you might register a new connection with an fd
/// the same as a yet-to-be-disconnect_event()-ed.
pub trait SocketDescriptor: cmp::Eq + hash::Hash + Clone {
	/// Attempts to send some data from the given slice to the peer.
	///
	/// Returns the amount of data which was sent, possibly 0 if the socket has since disconnected.
	/// Note that in the disconnected case, a disconnect_event must still fire and further write
	/// attempts may occur until that time.
	///
	/// If the returned size is smaller than data.len(), a write_available event must
	/// trigger the next time more data can be written. Additionally, until the a send_data event
	/// completes fully, no further read_events should trigger on the same peer!
	///
	/// If a read_event on this descriptor had previously returned true (indicating that read
	/// events should be paused to prevent DoS in the send buffer), resume_read may be set
	/// indicating that read events on this descriptor should resume. A resume_read of false does
	/// *not* imply that further read events should be paused.
	fn send_data(&mut self, data: &[u8], resume_read: bool) -> usize;
	/// Disconnect the socket pointed to by this SocketDescriptor. Once this function returns, no
	/// more calls to write_event, read_event or disconnect_event may be made with this descriptor.
	/// No disconnect_event should be generated as a result of this call, though obviously races
	/// may occur whereby disconnect_socket is called after a call to disconnect_event but prior to
	/// that event completing.
	fn disconnect_socket(&mut self);
}

/// Error for PeerManager errors. If you get one of these, you must disconnect the socket and
/// generate no further read/write_events for the descriptor, only triggering a single
/// disconnect_event (unless it was provided in response to a new_*_connection event, in which case
/// no such disconnect_event must be generated and the socket be silently disconencted).
pub struct PeerHandleError {
	/// Used to indicate that we probably can't make any future connections to this peer, implying
	/// we should go ahead and force-close any channels we have with it.
	pub(crate) no_connection_possible: bool,
}

impl fmt::Debug for PeerHandleError {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		formatter.write_str("Peer Sent Invalid Data")
	}
}

impl fmt::Display for PeerHandleError {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		formatter.write_str("Peer Sent Invalid Data")
	}
}

impl error::Error for PeerHandleError {
	fn description(&self) -> &str {
		"Peer Sent Invalid Data"
	}
}

pub(crate) enum InitSyncTracker {
	NoSyncRequested,
	ChannelsSyncing(u64),
	NodesSyncing(PublicKey),
}

pub(crate) struct Peer {
	pub(crate) channel_encryptor: PeerChannelEncryptor,
	pub(crate) handshake: PeerHandshake,
	pub(crate) conduit: Option<ConnectedPeer>,
	pub(crate) outbound: bool,
	pub(crate) their_node_id: Option<PublicKey>,
	pub(crate) their_features: Option<InitFeatures>,

	pub(crate) pending_outbound_buffer: LinkedList<Vec<u8>>,
	pub(crate) pending_outbound_buffer_first_msg_offset: usize,
	pub(crate) awaiting_write_event: bool,

	pub(crate) pending_read_buffer: Vec<u8>,

	pub(crate) sync_status: InitSyncTracker,

	pub(crate) awaiting_pong: bool,
}

impl Peer {
	/// Returns true if the channel announcements/updates for the given channel should be
	/// forwarded to this peer.
	/// If we are sending our routing table to this peer and we have not yet sent channel
	/// announcements/updates for the given channel_id then we will send it when we get to that
	/// point and we shouldn't send it yet to avoid sending duplicate updates. If we've already
	/// sent the old versions, we should send the update, and so return true here.
	pub(crate) fn should_forward_channel(&self, channel_id: u64) -> bool {
		match self.sync_status {
			InitSyncTracker::NoSyncRequested => true,
			InitSyncTracker::ChannelsSyncing(i) => i < channel_id,
			InitSyncTracker::NodesSyncing(_) => true,
		}
	}
}

#[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
fn _check_usize_is_32_or_64() {
	// See below, less than 32 bit pointers may be unsafe here!
	unsafe { mem::transmute::<*const usize, [u8; 4]>(panic!()); }
}

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use rand::{Rng, thread_rng};
	use secp256k1::key::{PublicKey, SecretKey};
	use secp256k1::Secp256k1;

	use ln::msgs;
	use ln::peer_handler::{MessageHandler, PeerManager, SocketDescriptor};
	use util::events;
	use util::logger::Logger;
	use util::test_utils;

	#[derive(PartialEq, Eq, Clone, Hash)]
	struct FileDescriptor {
		fd: u16,
	}

	impl SocketDescriptor for FileDescriptor {
		fn send_data(&mut self, data: &[u8], _resume_read: bool) -> usize {
			data.len()
		}

		fn disconnect_socket(&mut self) {}
	}

	fn create_network(peer_count: usize) -> Vec<PeerManager<FileDescriptor>> {
		let mut peers = Vec::new();
		let mut rng = thread_rng();
		let logger: Arc<Logger> = Arc::new(test_utils::TestLogger::new());
		let mut ephemeral_bytes = [0; 32];
		rng.fill_bytes(&mut ephemeral_bytes);

		for _ in 0..peer_count {
			let chan_handler = test_utils::TestChannelMessageHandler::new();
			let router = test_utils::TestRoutingMessageHandler::new();
			let node_id = {
				let mut key_slice = [0; 32];
				rng.fill_bytes(&mut key_slice);
				SecretKey::from_slice(&key_slice).unwrap()
			};
			let msg_handler = MessageHandler { chan_handler: Arc::new(chan_handler), route_handler: Arc::new(router) };
			let peer = PeerManager::new(msg_handler, node_id, &ephemeral_bytes, Arc::clone(&logger));
			peers.push(peer);
		}

		peers
	}

	fn establish_connection(peer_a: &PeerManager<FileDescriptor>, peer_b: &PeerManager<FileDescriptor>) {
		let secp_ctx = Secp256k1::new();
		let their_id = PublicKey::from_secret_key(&secp_ctx, &peer_b.our_node_secret);
		let fd = FileDescriptor { fd: 1 };
		peer_a.new_inbound_connection(fd.clone()).unwrap();
		peer_a.peers.lock().unwrap().node_id_to_descriptor.insert(their_id, fd.clone());
	}

	#[test]
	fn test_disconnect_peer() {
		// Simple test which builds a network of PeerManager, connects and brings them to NoiseState::Finished and
		// push a DisconnectPeer event to remove the node flagged by id
		let mut peers = create_network(2);
		establish_connection(&peers[0], &peers[1]);
		assert_eq!(peers[0].peers.lock().unwrap().peers.len(), 1);

		let secp_ctx = Secp256k1::new();
		let their_id = PublicKey::from_secret_key(&secp_ctx, &peers[1].our_node_secret);

		let chan_handler = test_utils::TestChannelMessageHandler::new();
		chan_handler.pending_events.lock().unwrap().push(events::MessageSendEvent::HandleError {
			node_id: their_id,
			action: msgs::ErrorAction::DisconnectPeer { msg: None },
		});
		assert_eq!(chan_handler.pending_events.lock().unwrap().len(), 1);
		peers[0].message_handler.chan_handler = Arc::new(chan_handler);

		peers[0].process_events();
		assert_eq!(peers[0].peers.lock().unwrap().peers.len(), 0);
	}

	#[test]
	fn test_timer_tick_occured() {
		// Create peers, a vector of two peer managers, perform initial set up and check that peers[0] has one Peer.
		let peers = create_network(2);
		establish_connection(&peers[0], &peers[1]);
		assert_eq!(peers[0].peers.lock().unwrap().peers.len(), 1);

		// peers[0] awaiting_pong is set to true, but the Peer is still connected
		peers[0].timer_tick_occured();
		assert_eq!(peers[0].peers.lock().unwrap().peers.len(), 1);

		// Since timer_tick_occured() is called again when awaiting_pong is true, all Peers are disconnected
		peers[0].timer_tick_occured();
		assert_eq!(peers[0].peers.lock().unwrap().peers.len(), 0);
	}
}
