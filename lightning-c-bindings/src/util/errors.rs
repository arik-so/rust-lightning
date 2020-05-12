//! " Error types live here."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::TakePointer;


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

impl Drop for APIError {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnAPIError) };
		}
	}
}
#[no_mangle]
pub extern "C" fn APIError_free(this_ptr: APIError) { }
