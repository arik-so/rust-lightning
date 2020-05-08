use secp256k1::{SecretKey, PublicKey, Secp256k1};
use std::slice;
use crate::buffer::BufferResponse;

#[no_mangle]
pub extern "C" fn private_key_to_public_key(private_key: *const u8) -> *mut BufferResponse {
	let private_key_slice = unsafe {
		assert!(!private_key.is_null());
		slice::from_raw_parts(private_key, 32)
	};
	let private_key_object = SecretKey::from_slice(private_key_slice).unwrap();

	let curve = Secp256k1::new();
	let public_key_object = PublicKey::from_secret_key(&curve,&private_key_object);
	let buffer = public_key_object.serialize().to_vec().into();
	Box::into_raw(Box::new(buffer))
}

#[no_mangle]
pub extern "C" fn is_public_key(public_key: *const u8) -> bool {
	let public_key_slice = unsafe {
		assert!(!public_key.is_null());
		slice::from_raw_parts(public_key, 33)
	};
	let public_key_object = PublicKey::from_slice(public_key_slice);
	public_key_object.is_ok()
}