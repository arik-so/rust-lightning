use std::slice;
use std::sync::Arc;

use bitcoin::Network;
use secp256k1::SecretKey;

use lightning::ln::channelmanager::ChannelManager;
use lightning::ln::peer_handler::{MessageHandler, PeerManager};
use lightning::util::config::UserConfig;

use crate::buffer::{BufferArgument, BufferResponse};
use crate::chain::broadcaster::Broadcaster;
use crate::chain::fee_estimator::FeeEstimator;
use crate::channels::channel_keys::ChannelKeys;
use crate::channels::channel_message_handler::ChannelMessageHandler;
use crate::channels::channel_monitor::ChannelMonitor;
use crate::channels::routing_message_handler::RoutingMessageHandler;
use crate::error::Error;
use crate::util::logger::Logger;
use std::ffi::c_void;

// use lightning::chain::keysinterface::KeysManager;

pub struct Node {
	host_instance_pointer: Option<*const c_void>,
	// peer_manager: PeerManager<SocketDescriptor, Arc<ChannelMessageHandler>>,
	// channel_manager: ChannelManager<ChannelKeys, Arc<ChannelMonitor>, Arc<Broadcaster>, Arc<ChannelKeys>, Arc<FeeEstimator>>,
}

#[no_mangle]
pub extern "C" fn node_create(node_secret_key: *const u8, ephemeral_private_key: *const u8, current_blockchain_height: usize) -> *mut Node {

	/*

	// create channel handler
	let channel_handler = Arc::new(ChannelMessageHandler {});
	// create route handler
	let route_handler = Arc::new(RoutingMessageHandler {});
	// create message handler with channel handler and route handler
	let message_handler = MessageHandler {
		chan_handler: channel_handler,
		route_handler: route_handler,
	};

	let private_key_slice = unsafe {
		assert!(!node_secret_key.is_null());
		slice::from_raw_parts(node_secret_key, 32)
	};
	let ephemeral_private_key_slice = unsafe {
		assert!(!ephemeral_private_key.is_null());
		slice::from_raw_parts(ephemeral_private_key, 32)
	};

	println!("private key slice: {:?}", private_key_slice);
	println!("ephemeral_private_key_slice: {:?}", ephemeral_private_key_slice);

	let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();

	let mut ephemeral_private_key = [0u8; 32];
	ephemeral_private_key.copy_from_slice(ephemeral_private_key_slice);

	let logger = Arc::new(Logger {});

	// create peer handler with message handler
	let peer_manager = PeerManager::new(message_handler, private_key_object, &ephemeral_private_key, logger.clone());

	// create channel manager
	let fee_estimator = Arc::new(FeeEstimator {});
	let channel_monitor = Arc::new(ChannelMonitor {});
	let broadcaster = Arc::new(Broadcaster {});
	let key_manager = Arc::new(ChannelKeys {});
	// let key_manager = Arc::new(KeysManager::new(&ephemeral_private_key, Network::Testnet, logger.clone(), 51, 756));
	let user_config = UserConfig::default();
	let channel_manager = ChannelManager::new(Network::Testnet, fee_estimator, channel_monitor, broadcaster, logger.clone(), key_manager, user_config, current_blockchain_height).unwrap();

	*/

	let host_instance_pointer = None;

	// add peer handler to node
	let node = Node {
		host_instance_pointer,
		// peer_manager,
		// channel_manager,
	};
	Box::into_raw(Box::new(node))
}

#[no_mangle]
pub extern "C" fn trigger_callback(node: &mut Node, network_callback: fn(*const c_void, BufferResponse) -> BufferArgument) {

	let mut host_pointer = std::ptr::null();
	if let Some(host) = node.host_instance_pointer{
		host_pointer = host;
	}

	// call network argument
	let callback_argument: BufferResponse = vec![5, 4, 3, 2, 1].into();
	let callback_response = network_callback(host_pointer, callback_argument);
	let response = unsafe { callback_response.to_vec() };
	println!("callback response: {:?}", response);
}

#[no_mangle]
pub extern "C" fn set_host_instance_pointer(node: &mut Node, pointer: *const c_void) {
	node.host_instance_pointer = Some(pointer);
}

#[no_mangle]
pub extern "C" fn node_connect_outbound(node: &mut Node, remote_public_key: *const u8, error: *mut Error) -> *mut BufferResponse {
	unimplemented!()
}

#[no_mangle]
pub extern "C" fn node_create_channel(node: &mut Node, remote_public_key: *const u8) {}