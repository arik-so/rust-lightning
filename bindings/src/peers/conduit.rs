use wasm_bindgen::prelude::*;

use lightning::ln::peers::conduit::Conduit as RawConduit;

use crate::buffer::{BufferArgument, BufferResponse};
use crate::error::Error;

#[wasm_bindgen]
pub struct Conduit(pub(super) RawConduit);

#[wasm_bindgen]
impl Conduit {

	pub fn peer_conduit_decrypt(&mut self, message: &[u8]) -> Vec<u8> {
		let input_data = message.to_vec();

		let response = self.0.decrypt_single_message(Some(&input_data));

		/*
		if response.is_none() {
			// figure out error handling
			let ffi_error: Error = "Nothing to decrypt".into();
			unsafe { std::ptr::write(error, ffi_error); }
			return std::ptr::null_mut();
		}
		*/

		response.unwrap()
	}
}

#[no_mangle]
pub extern "C" fn peer_conduit_decrypt(conduit: &mut Conduit, message: &BufferArgument, error: *mut Error) -> *mut BufferResponse {
	let input_data = unsafe { message.to_vec() };

	let response = conduit.0.decrypt_single_message(Some(&input_data));
	if response.is_none() {
		let ffi_error: Error = "Nothing to decrypt".into();
		unsafe { std::ptr::write(error, ffi_error); }
		return std::ptr::null_mut();
	}

	let buffer: BufferResponse = response.unwrap().into();
	buffer.into_mut_ptr()
}