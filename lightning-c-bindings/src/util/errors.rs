//! " Error types live here."

use std::ffi::c_void;
use bitcoin::hashes::Hash;


use lightning::util::errors::APIError as lnAPIErrorImport;
type lnAPIError = lnAPIErrorImport;

/// " Indicates an error on the client's part (usually some variant of attempting to use too-low or"
/// " too-high values)"
#[repr(C)]
pub struct APIError {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnAPIError,
}

#[no_mangle]
pub extern "C" fn APIError_free(this_ptr: APIError) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnAPIError) };
}
