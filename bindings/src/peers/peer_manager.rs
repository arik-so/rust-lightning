use lightning::ln::peer_handler::{PeerManager as RawPeerManager, MessageHandler};
use crate::channels::channel_message_handler::ChannelMessageHandler;
use std::slice;
use secp256k1::{SecretKey, PublicKey};
use crate::util::logger::Logger;
use wasm_bindgen::__rt::std::sync::Arc;
use crate::channels::routing_message_handler::RoutingMessageHandler;
use crate::error::Error;
use crate::buffer::BufferResponse;
use crate::peers::socket_descriptor::SocketDescriptor;
use std::ffi::c_void;
use lightning::ln::peer_handler::SocketDescriptor as RawSocketDescriptor;

pub struct PeerManager(RawPeerManager<SocketDescriptor, Arc<ChannelMessageHandler>>, u8);



#[no_mangle]
pub extern "C" fn peer_manager_create(node_private_key: *const u8, ephemeral_seed: *const u8) -> *mut PeerManager {
	let private_key_slice = unsafe {
		assert!(!node_private_key.is_null());
		slice::from_raw_parts(node_private_key, 32)
	};
	let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();

	let ephemeral_seed_slice = unsafe {
		assert!(!ephemeral_seed.is_null());
		slice::from_raw_parts(ephemeral_seed, 32)
	};
	let mut ephemeral_seed = [0u8; 32];
	ephemeral_seed.copy_from_slice(ephemeral_seed_slice);

	// create channel handler
	let channel_handler = Arc::new(ChannelMessageHandler {});
	// create route handler
	let route_handler = Arc::new(RoutingMessageHandler {});
	// create message handler with channel handler and route handler
	let message_handler = MessageHandler {
		chan_handler: channel_handler,
		route_handler: route_handler,
	};

	let logger = Arc::new(Logger{});

	// should be refactored to not require ownership of MessageHandler
	let peer_manager = RawPeerManager::new(message_handler, private_key_object, &ephemeral_seed, logger);
	let wrapped_peer_manager = PeerManager(peer_manager, 0);

	Box::into_raw(Box::new(wrapped_peer_manager))
}

#[no_mangle]
pub extern "C" fn peer_manager_new_outbound(peer_manager: &mut PeerManager, remote_public_key: *const u8, peer_instance_pointer: *const c_void, socket_callback: fn(*const c_void, *mut BufferResponse) -> usize, error: *mut Error) -> *mut SocketDescriptor{
	let public_key_slice = unsafe {
		assert!(!remote_public_key.is_null());
		slice::from_raw_parts(remote_public_key, 33)
	};
	let public_key_object = PublicKey::from_slice(public_key_slice).unwrap();

	let socket_id = peer_manager.1;
	peer_manager.1 += 1;

	let mut socket_descriptor = SocketDescriptor {
		socket_id,
		host_instance_pointer: peer_instance_pointer,
		send_data_callback: socket_callback
	}; // determined by dice-roll to be random

	// TODO: the socket descriptor should not require cloning
	let connection = peer_manager.0.new_outbound_connection(public_key_object, socket_descriptor.clone());
	if connection.is_err() {
		let ffi_error: Error = connection.err().unwrap().into();
		unsafe { std::ptr::write(error, ffi_error); }
		return std::ptr::null_mut();
	}

	let first_message = connection.unwrap();
	socket_descriptor.send_data(&first_message, true);
	Box::into_raw(Box::new(socket_descriptor))
}

#[no_mangle]
pub unsafe extern "C" fn peer_manager_free(raw_peer_manager: *mut PeerManager){
	if raw_peer_manager.is_null() { return; }
	let _ = Box::from_raw(raw_peer_manager);
}