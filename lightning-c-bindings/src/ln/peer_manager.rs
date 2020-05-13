use lightning::ln::peer_handler::{PeerManager as RawPeerManager, MessageHandler as lnMessageHandlerImport};
use std::slice;
use bitcoin::secp256k1::{SecretKey, PublicKey};
use crate::error::Error;
use crate::buffer::{BufferResponse, BufferArgument};
use std::ffi::c_void;
use lightning::ln::peer_handler::SocketDescriptor as RawSocketDescriptor;



use bitcoin::secp256k1::key::SecretKey as lnSecretKey;
use bitcoin::secp256k1::key::PublicKey as lnPublicKey;
use bitcoin::hashes::HashEngine as lnHashEngine;
use bitcoin::hashes::Hash as lnHash;
use crate::ln::socket_descriptor::SocketDescriptor;
use crate::ln::msgs::{ChannelMessageHandler, RoutingMessageHandler};
use std::sync::Arc;

type lnMessageHandler = lnMessageHandlerImport<crate::ln::msgs::ChannelMessageHandler, crate::ln::msgs::RoutingMessageHandler>;

/// " Provides references to trait impls which handle different types of messages."
#[repr(C)]
pub struct MessageHandler {
	pub(crate) inner: *const lnMessageHandler,
}

#[no_mangle]
pub extern "C" fn MessageHandler_free(this_ptr: MessageHandler) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnMessageHandler) };
}
#[no_mangle]
pub extern "C" fn MessageHandler_get_chan_handler(this_ptr: *const MessageHandler) -> *const crate::ln::msgs::ChannelMessageHandler {
	&unsafe { &*(*this_ptr).inner }.chan_handler
}
#[no_mangle]
pub extern "C" fn MessageHandler_set_chan_handler(this_ptr: *mut MessageHandler, val: crate::ln::msgs::ChannelMessageHandler) {
	unsafe { &mut *((*this_ptr).inner as *mut lnMessageHandler) }.chan_handler = val;
}
#[no_mangle]
pub extern "C" fn MessageHandler_get_route_handler(this_ptr: *const MessageHandler) -> *const crate::ln::msgs::RoutingMessageHandler {
	&unsafe { &*(*this_ptr).inner }.route_handler
}
#[no_mangle]
pub extern "C" fn MessageHandler_set_route_handler(this_ptr: *mut MessageHandler, val: crate::ln::msgs::RoutingMessageHandler) {
	unsafe { &mut *((*this_ptr).inner as *mut lnMessageHandler) }.route_handler = val;
}
#[no_mangle]
pub extern "C" fn MessageHandler_new(chan_handler_arg: crate::ln::msgs::ChannelMessageHandler, route_handler_arg: crate::ln::msgs::RoutingMessageHandler) -> MessageHandler {
	MessageHandler { inner: Box::into_raw(Box::new(lnMessageHandler {
		chan_handler: chan_handler_arg,
		route_handler: route_handler_arg,
	}))}
}




pub struct PeerManager(RawPeerManager<SocketDescriptor, ChannelMessageHandler, RoutingMessageHandler>, u8);



#[no_mangle]
pub extern "C" fn peer_manager_create(node_private_key: *const u8, ephemeral_seed: *const u8, message_handler: MessageHandler, logger: crate::util::logger::Logger) -> *mut PeerManager {
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


	let logger = Arc::new(logger);

	// should be refactored to not require ownership of MessageHandler
	let peer_manager = RawPeerManager::new(*unsafe { Box::from_raw(message_handler.inner as *mut _) }, private_key_object, &ephemeral_seed, logger);
	let wrapped_peer_manager = PeerManager(peer_manager, 0);

	Box::into_raw(Box::new(wrapped_peer_manager))
}

#[no_mangle]
pub extern "C" fn peer_manager_new_outbound(peer_manager: &mut PeerManager, remote_public_key: *const u8, peer_instance_pointer: *const c_void, socket_callback: fn(*const c_void, *mut BufferResponse) -> usize, disconnect_callback: fn(*const c_void) -> c_void, error: *mut Error) -> *mut SocketDescriptor{
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
		send_data_callback: socket_callback,
		disconnect_callback
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
pub extern "C" fn peer_read(peer_manager: &mut PeerManager, socket_descriptor: &SocketDescriptor, data: &BufferArgument) {
	let mut descriptor_clone = socket_descriptor.clone();
	let data = unsafe { data.to_vec() };
	peer_manager.0.read_event(&mut descriptor_clone, &data);
}

#[no_mangle]
pub extern "C" fn peer_force_tick(peer_manager: &mut PeerManager){
	peer_manager.0.timer_tick_occured();
}

#[no_mangle]
pub extern "C" fn peer_manager_try_direct(peer_manager: &mut PeerManager) -> BufferResponse {
	let vector = vec![0u8, 2, 3, 10];
	vector.into()
}

#[no_mangle]
pub unsafe extern "C" fn peer_manager_free(raw_peer_manager: *mut PeerManager){
	if raw_peer_manager.is_null() { return; }
	let _ = Box::from_raw(raw_peer_manager);
}
