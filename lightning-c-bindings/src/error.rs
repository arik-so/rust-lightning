//! Experimental error module

use std::ffi::CString;
use std::os::raw::c_char;
use lightning::ln::peer_handler::PeerHandleError;
use lightning::util::errors::APIError;

#[repr(C)]
pub struct Error {
	message: *mut c_char,
	length: usize,
}

impl From<PeerHandleError> for Error {
	fn from(error: PeerHandleError) -> Self {
		let message = error.to_string();
		let length = message.len();
		let message = CString::new(message).unwrap().into_raw();
		Self { message, length }
	}
}

impl From<APIError> for Error {
	fn from(error: APIError) -> Self {
		let message = match error {
			APIError::FeeRateTooHigh { err: _, feerate: rate } => {
				format!("error: fee rate too high ({})", rate)
			}
			APIError::RouteError { err: _ } => {
				"error: route error".to_string()
			}
			APIError::ChannelUnavailable { err: _ } => {
				"error: channel unavailable".to_string()
			}
			APIError::MonitorUpdateFailed => {
				"error: monitor update failed".to_string()
			}
			APIError::APIMisuseError { err: error } => {
				format!("error: api misuse: {}", error)
			}
		};
		let length = message.len();
		let message = CString::new(message).unwrap().into_raw();
		Self { message, length }
	}
}

impl From<String> for Error {
	fn from(error: String) -> Self {
		let length = error.len();
		let message = CString::new(error).unwrap().into_raw();
		Self { message, length }
	}
}

impl From<&str> for Error {
	fn from(error: &str) -> Self {
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
