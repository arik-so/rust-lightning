use std::slice;

use secp256k1::{PublicKey, SecretKey};

use lightning::ln::peers::handshake::PeerHandshake as RawHandshake;

use crate::buffer::{LDKBufferArgument, LDKBufferResponse};
use crate::error::LDKError;
use crate::peers::conduit::LDKConduit;

pub struct LDKPeerHandshake(RawHandshake);

#[repr(C)]
pub struct LDKHandshakeResult {
	pub next_act: *mut LDKBufferResponse,
	pub conduit: *mut LDKConduit,
}

#[no_mangle]
pub extern "C" fn peer_handshake_new_outbound(private_key: *const u8, ephemeral_private_key: *const u8, remote_public_key: *const u8) -> *mut LDKPeerHandshake {
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
	let peer_handshake = LDKPeerHandshake(handshake);

	Box::into_raw(Box::new(peer_handshake))
}

#[no_mangle]
pub extern "C" fn peer_handshake_new_inbound(private_key: *const u8, ephemeral_private_key: *const u8) -> *mut LDKPeerHandshake {
	let private_key_slice = unsafe {
		assert!(!private_key.is_null());
		slice::from_raw_parts(private_key, 32)
	};
	let ephemeral_private_key_slice = unsafe {
		assert!(!ephemeral_private_key.is_null());
		slice::from_raw_parts(ephemeral_private_key, 32)
	};

	println!("private key slice: {:?}", private_key_slice);
	println!("ephemeral_private_key_slice: {:?}", ephemeral_private_key_slice);

	let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();
	let ephemeral_private_key_object = SecretKey::from_slice(ephemeral_private_key_slice).unwrap();

	let handshake = RawHandshake::new_inbound(&private_key_object, &ephemeral_private_key_object);
	let peer_handshake = LDKPeerHandshake(handshake);

	Box::into_raw(Box::new(peer_handshake))
}

#[no_mangle]
pub extern "C" fn peer_handshake_process_act(peer: &mut LDKPeerHandshake, act_data: &LDKBufferArgument, error: *mut LDKError) -> *mut LDKHandshakeResult {
	/*
	let ffi_error: Error = String::from("there is a massive error going on!").into();
	println!("about to write to the null pointer");
	unsafe { std::ptr::write(error, ffi_error); }
	println!("returning a null pointer");
	return std::ptr::null_mut();
	*/

	/* let input_data = unsafe {
		LDKBuffer::from_ptr(act_data)
	}; */
	let input_data = unsafe { act_data.to_vec() };

	let response = peer.0.process_act(&input_data);
	if response.is_err() {
		let ffi_error: LDKError = response.err().unwrap().into();
		unsafe { std::ptr::write(error, ffi_error); }
		return std::ptr::null_mut();
	}

	let act_response = response.unwrap();
	let mut result = LDKHandshakeResult {
		next_act: std::ptr::null_mut(),
		conduit: std::ptr::null_mut(),
	};

	let next_act_option = act_response.0;
	if let Some(next_act) = next_act_option {
		let next_act_vector = next_act.serialize();
		println!("next_act_vector: {:?}", next_act_vector);
		let buffer: LDKBufferResponse = next_act_vector.into();
		result.next_act = buffer.into_mut_ptr();
	}

	let conduit_option = act_response.1;
	if let Some(conduit) = conduit_option {
		let wrapped_conduit = LDKConduit(conduit);
		result.conduit = Box::into_raw(Box::new(wrapped_conduit));
	}

	Box::into_raw(Box::new(result))
}