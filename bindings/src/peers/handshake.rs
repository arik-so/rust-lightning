use std::slice;

use secp256k1::{PublicKey, SecretKey};

use lightning::ln::peers::handshake::PeerHandshake as RawHandshake;

use crate::buffer::Buffer;

pub struct PeerHandshake(RawHandshake);

#[no_mangle]
pub extern "C" fn peer_handshake_new_outbound(private_key: *const u8, ephemeral_private_key: *const u8, remote_public_key: *const u8) -> *mut PeerHandshake {
	let private_key_slice = unsafe {
		assert!(!private_key.is_null());
		slice::from_raw_parts(private_key, 32)
	};
	let ephemeral_private_key_slice = unsafe {
		assert!(!ephemeral_private_key.is_null());
		slice::from_raw_parts(ephemeral_private_key, 32)
	};
	let public_key_slice = unsafe {
		assert!(!remote_public_key.is_null());
		slice::from_raw_parts(remote_public_key, 33)
	};

	println!("private key slice: {:?}", private_key_slice);
	println!("ephemeral_private_key_slice: {:?}", ephemeral_private_key_slice);
	println!("public_key_slice: {:?}", public_key_slice);

	let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();
	let ephemeral_private_key_object = SecretKey::from_slice(ephemeral_private_key_slice).unwrap();
	let public_key_object = PublicKey::from_slice(public_key_slice).unwrap();

	let handshake = RawHandshake::new_outbound(&private_key_object, &public_key_object, &ephemeral_private_key_object);
	let peer_handshake = PeerHandshake(handshake);

	Box::into_raw(Box::new(peer_handshake))
}

#[no_mangle]
pub extern "C" fn peer_handshake_process_act(peer: &mut PeerHandshake) -> *mut Buffer {
	let act_response = peer.0.process_act(&vec![]).unwrap().0.unwrap().serialize();
	println!("act_response: {:?}", act_response);

	let buffer: Buffer = act_response.into();
	buffer.as_mut_ptr()
}