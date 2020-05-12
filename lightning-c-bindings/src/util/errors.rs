//! " Error types live here."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::*;

/// " Indicates an error on the client's part (usually some variant of attempting to use too-low or"
/// " too-high values)"
#[must_use]
#[derive(Clone)]
#[repr(C)]
pub enum APIError {
	/// " Indicates the API was wholly misused (see err for more). Cases where these can be returned"
	/// " are documented, but generally indicates some precondition of a function was violated."
	APIMisuseError {
		err: crate::c_types::Str,
	},
	/// " Due to a high feerate, we were unable to complete the request."
	/// " For example, this may be returned if the feerate implies we cannot open a channel at the"
	/// " requested value, but opening a larger channel would succeed."
	FeeRateTooHigh {
		err: crate::c_types::derived::CVec_u8Z,
		feerate: u64,
	},
	/// " A malformed Route was provided (eg overflowed value, node id mismatch, overly-looped route,"
	/// " too-many-hops, etc)."
	RouteError {
		err: crate::c_types::Str,
	},
	/// " We were unable to complete the request as the Channel required to do so is unable to"
	/// " complete the request (or was not found). This can take many forms, including disconnected"
	/// " peer, channel at capacity, channel shutting down, etc."
	ChannelUnavailable {
		err: crate::c_types::Str,
	},
	/// " An attempt to call add/update_monitor returned an Err (ie you did this!), causing the"
	/// " attempted action to fail."
	MonitorUpdateFailed,
}
use lightning::util::errors::APIError as lnAPIError;
impl APIError {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnAPIError {
		match self {
			APIError::APIMisuseError {ref err, } => {
				let mut err_nonref = (*err).clone();
				lnAPIError::APIMisuseError {
					err: err_nonref.into(),
				}
			},
			APIError::FeeRateTooHigh {ref err, ref feerate, } => {
				let mut err_nonref = (*err).clone();
				let mut feerate_nonref = (*feerate).clone();
				lnAPIError::FeeRateTooHigh {
					err: String::from_utf8(err_nonref.into_rust()).unwrap(),
					feerate: feerate_nonref,
				}
			},
			APIError::RouteError {ref err, } => {
				let mut err_nonref = (*err).clone();
				lnAPIError::RouteError {
					err: err_nonref.into(),
				}
			},
			APIError::ChannelUnavailable {ref err, } => {
				let mut err_nonref = (*err).clone();
				lnAPIError::ChannelUnavailable {
					err: err_nonref.into(),
				}
			},
			APIError::MonitorUpdateFailed => lnAPIError::MonitorUpdateFailed,
		}
	}
	#[allow(unused)]
	pub(crate) fn into_ln(self) -> lnAPIError {
		match self {
			APIError::APIMisuseError {mut err, } => {
				lnAPIError::APIMisuseError {
					err: err.into(),
				}
			},
			APIError::FeeRateTooHigh {mut err, mut feerate, } => {
				lnAPIError::FeeRateTooHigh {
					err: String::from_utf8(err.into_rust()).unwrap(),
					feerate: feerate,
				}
			},
			APIError::RouteError {mut err, } => {
				lnAPIError::RouteError {
					err: err.into(),
				}
			},
			APIError::ChannelUnavailable {mut err, } => {
				lnAPIError::ChannelUnavailable {
					err: err.into(),
				}
			},
			APIError::MonitorUpdateFailed => lnAPIError::MonitorUpdateFailed,
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: &lnAPIError) -> Self {
		match lnt {
			lnAPIError::APIMisuseError {ref err, } => {
				let mut err_nonref = (*err).clone();
				APIError::APIMisuseError {
					err: err_nonref.into(),
				}
			},
			lnAPIError::FeeRateTooHigh {ref err, ref feerate, } => {
				let mut err_nonref = (*err).clone();
				let mut feerate_nonref = (*feerate).clone();
				APIError::FeeRateTooHigh {
					err: err_nonref.into_bytes().into(),
					feerate: feerate_nonref,
				}
			},
			lnAPIError::RouteError {ref err, } => {
				let mut err_nonref = (*err).clone();
				APIError::RouteError {
					err: err_nonref.into(),
				}
			},
			lnAPIError::ChannelUnavailable {ref err, } => {
				let mut err_nonref = (*err).clone();
				APIError::ChannelUnavailable {
					err: err_nonref.into(),
				}
			},
			lnAPIError::MonitorUpdateFailed => APIError::MonitorUpdateFailed,
		}
	}
	#[allow(unused)]
	pub(crate) fn ln_into(lnt: lnAPIError) -> Self {
		match lnt {
			lnAPIError::APIMisuseError {mut err, } => {
				APIError::APIMisuseError {
					err: err.into(),
				}
			},
			lnAPIError::FeeRateTooHigh {mut err, mut feerate, } => {
				APIError::FeeRateTooHigh {
					err: err.into_bytes().into(),
					feerate: feerate,
				}
			},
			lnAPIError::RouteError {mut err, } => {
				APIError::RouteError {
					err: err.into(),
				}
			},
			lnAPIError::ChannelUnavailable {mut err, } => {
				APIError::ChannelUnavailable {
					err: err.into(),
				}
			},
			lnAPIError::MonitorUpdateFailed => APIError::MonitorUpdateFailed,
		}
	}
}
#[no_mangle]
pub extern "C" fn APIError_free(this_ptr: APIError) { }
