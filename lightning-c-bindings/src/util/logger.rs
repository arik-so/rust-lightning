//! " Log traits live here, which are called throughout the library to provide useful information for"
//! " debugging purposes."
//! ""
//! " There is currently 2 ways to filter log messages. First one, by using compilation features, e.g \"max_level_off\"."
//! " The second one, client-side by implementing check against Record Level field."
//! " Each module may have its own Logger or share one."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::*;

/// " An enum representing the available verbosity levels of the logger."
#[must_use]
#[derive(Clone)]
#[repr(C)]
pub enum Level {
	/// "Designates logger being silent"
	Off,
	/// " Designates very serious errors"
	Error,
	/// " Designates hazardous situations"
	Warn,
	/// " Designates useful information"
	Info,
	/// " Designates lower priority information"
	Debug,
	/// " Designates very low priority, often extremely verbose, information"
	Trace,
}
use lightning::util::logger::Level as lnLevel;
impl Level {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnLevel {
		match self {
			Level::Off => lnLevel::Off,
			Level::Error => lnLevel::Error,
			Level::Warn => lnLevel::Warn,
			Level::Info => lnLevel::Info,
			Level::Debug => lnLevel::Debug,
			Level::Trace => lnLevel::Trace,
		}
	}
	#[allow(unused)]
	pub(crate) fn into_ln(self) -> lnLevel {
		match self {
			Level::Off => lnLevel::Off,
			Level::Error => lnLevel::Error,
			Level::Warn => lnLevel::Warn,
			Level::Info => lnLevel::Info,
			Level::Debug => lnLevel::Debug,
			Level::Trace => lnLevel::Trace,
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: &lnLevel) -> Self {
		match lnt {
			lnLevel::Off => Level::Off,
			lnLevel::Error => Level::Error,
			lnLevel::Warn => Level::Warn,
			lnLevel::Info => Level::Info,
			lnLevel::Debug => Level::Debug,
			lnLevel::Trace => Level::Trace,
		}
	}
	#[allow(unused)]
	pub(crate) fn ln_into(lnt: lnLevel) -> Self {
		match lnt {
			lnLevel::Off => Level::Off,
			lnLevel::Error => Level::Error,
			lnLevel::Warn => Level::Warn,
			lnLevel::Info => Level::Info,
			lnLevel::Debug => Level::Debug,
			lnLevel::Trace => Level::Trace,
		}
	}
}
/// " Returns the most verbose logging level."
#[must_use]
#[no_mangle]
pub extern "C" fn Level_max() -> crate::util::logger::Level {
	let mut ret = lightning::util::logger::Level::max();
	crate::util::logger::Level::ln_into(ret)
}

/// " A trait encapsulating the operations required of a logger"
#[repr(C)]
pub struct Logger {
	pub this_arg: *mut c_void,
	/// " Logs the `Record`"
	pub log: extern "C" fn (this_arg: *const c_void, record: *const std::os::raw::c_char),
}
unsafe impl Sync for Logger {}
unsafe impl Send for Logger {}

use lightning::util::logger::Logger as lnLogger;
impl lnLogger for Logger {
	fn log(&self, record: &lightning::util::logger::Record) {
		let local_record = std::ffi::CString::new(format!("{}", record.args)).unwrap();
		(self.log)(self.this_arg, local_record.as_ptr())
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for Logger {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
