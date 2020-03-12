use lightning::ln::peers::conduit::Conduit as RawConduit;
use crate::buffer::{LDKBufferArgument, LDKBufferResponse};
use crate::error::LDKError;

pub struct LDKConduit(pub(super) RawConduit);

#[no_mangle]
pub extern "C" fn peer_conduit_decrypt(conduit: &mut LDKConduit, message: &LDKBufferArgument, error: *mut LDKError) -> *mut LDKBufferResponse {
	let input_data = unsafe { message.to_vec() };

	let response = conduit.0.decrypt_single_message(Some(&input_data));
	if response.is_none() {
		let ffi_error: LDKError = "Nothing to decrypt".into();
		unsafe { std::ptr::write(error, ffi_error); }
		return std::ptr::null_mut();
	}

	let buffer: LDKBufferResponse = response.unwrap().into();
	buffer.into_mut_ptr()
}