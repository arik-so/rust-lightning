//! Experimental error module

use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
pub struct Error {
	message: *mut c_char,
	length: usize,
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		let length = error.len();
		let message = CString::new(error).unwrap().into_raw();
		Self { message, length }
	}
}

impl Error {
	pub fn into_mut_ptr(self) -> *mut Self {
		Box::into_raw(Box::new(self))
	}
}

//use std::error::Error;
//use std::sync::Mutex;
//
//lazy_static! {
//	static ref LAST_ERROR: Mutex<Option<Box<dyn Error>>> = Mutex::new(None);
//}
//
//pub(crate) fn update_last_error(error: impl Error + 'static) {
//
//	LAST_ERROR.lock().unwrap().replace(Box::new(error));
//
//}

// Retrieve the most recent error, clearing it in the process.
//pub fn take_last_error() -> Option<Box<dyn Error>> {
//	LAST_ERROR.borrow_mut().take()
//}