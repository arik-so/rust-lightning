//! " Error types live here."

use std::ffi::c_void;
use bitcoin::hashes::Hash;


use lightning::util::errors::APIError as lnAPIErrorImport;
type lnAPIError = lnAPIErrorImport;

/// " Indicates an error on the client's part (usually some variant of attempting to use too-low or"
/// " too-high values)"
#[repr(C)]
pub struct APIError {
	pub(crate) inner: *const lnAPIError,
}

#[no_mangle]
pub extern "C" fn APIError_free(this_ptr: APIError) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnAPIError) };
}
