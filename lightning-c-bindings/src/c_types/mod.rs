pub mod derived;

use bitcoin::Script as BitcoinScript;
use bitcoin::Transaction as BitcoinTransaction;
use bitcoin::secp256k1::key::PublicKey as SecpPublicKey;
use bitcoin::secp256k1::key::SecretKey as SecpSecretKey;
use bitcoin::secp256k1::Signature as SecpSignature;

#[repr(C)]
pub struct PublicKey {
	pub compressed_form: [u8; 33],
}
impl PublicKey {
	pub(crate) fn from_rust(pk: &SecpPublicKey) -> Self {
		Self {
			compressed_form: pk.serialize(),
		}
	}
	pub(crate) fn into_rust(&self) -> SecpPublicKey {
		SecpPublicKey::from_slice(&self.compressed_form).unwrap()
	}
}

#[repr(C)]
pub struct SecretKey {
	pub bytes: [u8; 32],
}
impl SecretKey {
	// from_rust isn't implemented since we jsut return byte array refs directly
	pub(crate) fn into_rust(&self) -> SecpSecretKey {
		SecpSecretKey::from_slice(&self.bytes).unwrap()
	}
}

#[repr(C)]
pub struct Signature {
	pub compact_form: [u8; 64],
}
impl Signature {
	pub(crate) fn from_rust(pk: &SecpSignature) -> Self {
		Self {
			compact_form: pk.serialize_compact(),
		}
	}
	pub(crate) fn into_rust(&self) -> SecpSignature {
		SecpSignature::from_compact(&self.compact_form).unwrap()
	}
}

#[repr(C)]
/// A reference to a serialized transaction, in (pointer, length) form.
/// This type does *not* own its own memory, so access to it after, eg, the call in which it was
/// provided to you are invalid.
pub struct Transaction {
	pub data: *const u8,
	pub datalen: usize
}
impl Transaction {
	pub(crate) fn into_bitcoin(&self) -> BitcoinTransaction {
		::bitcoin::consensus::encode::deserialize(unsafe { std::slice::from_raw_parts(self.data, self.datalen) }).unwrap()
	}
	pub(crate) fn from_slice(s: &[u8]) -> Self {
		Self {
			data: s.as_ptr(),
			datalen: s.len(),
		}
	}
}

#[repr(C)]
pub struct u8slice {
	pub data: *const u8,
	pub datalen: usize
}
impl u8slice {
	pub(crate) fn from_slice(s: &[u8]) -> Self {
		Self {
			data: s.as_ptr(),
			datalen: s.len(),
		}
	}
	pub(crate) fn to_slice(&self) -> &[u8] {
		unsafe { std::slice::from_raw_parts(self.data, self.datalen) }
	}
}

#[repr(C)]
/// Arbitrary 32 bytes, which could represent one of a few different things. You probably want to
/// look up the corresponding function in rust-lightning's docs.
pub struct ThirtyTwoBytes {
	pub data: [u8; 32],
}

#[repr(C)]
pub struct ThreeBytes {
	pub data: [u8; 3],
}

// Note that the C++ headers memset(0) all the Templ types to avoid deallocation!
// Thus, they must gracefully handle being completely null in _free.

#[repr(C)]
pub union CResultPtr<O, E> {
	pub result: *mut O,
	pub err: *mut E,
}
#[repr(C)]
pub struct CResultTempl<O, E> {
	pub contents: CResultPtr<O, E>,
	pub result_good: bool,
}
impl<O, E> CResultTempl<O, E> {
	pub(crate) extern "C" fn good(o: O) -> Self {
		CResultTempl {
			contents: CResultPtr {
				result: Box::into_raw(Box::new(o)),
			},
			result_good: true,
		}
	}
	pub(crate) extern "C" fn err(e: E) -> Self {
		CResultTempl {
			contents: CResultPtr {
				err: Box::into_raw(Box::new(e)),
			},
			result_good: false,
		}
	}
}
pub extern "C" fn CResultTempl_free<O, E>(_res: CResultTempl<O, E>) { }
impl<O, E> Drop for CResultTempl<O, E> {
	fn drop(&mut self) {
		if self.result_good {
			unsafe { Box::from_raw(self.contents.result) };
		} else if unsafe { !self.contents.err.is_null() } {
			unsafe { Box::from_raw(self.contents.err) };
		}
	}
}

// TODO: auto-generate these like we do the types:
#[no_mangle]
pub extern "C" fn CResult_NoneChannelMonitorUpdateErrZ_good() -> derived::CResult_NoneChannelMonitorUpdateErrZ {
	CResultTempl::good(0)
}
#[no_mangle]
pub static CResultNoneChannelMonitorUpdateErr_err:
	extern"C" fn(crate::ln::channelmonitor::ChannelMonitorUpdateErr) -> derived::CResult_NoneChannelMonitorUpdateErrZ =
	CResultTempl::<u8, crate::ln::channelmonitor::ChannelMonitorUpdateErr>::err;

#[repr(C)]
pub struct CVecTempl<T> {
	pub data: *mut T,
	pub datalen: usize
}
impl<T> CVecTempl<T> {
	pub(crate) fn into_rust(self) -> Vec<T> {
		unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.datalen)) }.into()
	}
}
impl<T> From<Vec<T>> for CVecTempl<T> {
	fn from(v: Vec<T>) -> Self {
		let datalen = v.len();
		let data = v.into_boxed_slice().as_mut_ptr();
		CVecTempl { datalen, data }
	}
}
pub extern "C" fn CVecTempl_free<T>(_res: CVecTempl<T>) { }
impl<T> Drop for CVecTempl<T> {
	fn drop(&mut self) {
		// datalen == 0 is will gracefully be ignored, so we don't have to handle data == null
		// here.
		unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.datalen)) };
	}
}

#[repr(C)]
pub struct C2TupleTempl<A, B> {
	pub a: *mut A,
	pub b: *mut B,
}
impl<A, B> From<(A, B)> for C2TupleTempl<A, B> {
	fn from(tup: (A, B)) -> Self {
		Self {
			a: Box::into_raw(Box::new(tup.0)),
			b: Box::into_raw(Box::new(tup.1)),
		}
	}
}
impl<A, B> C2TupleTempl<A, B> {
	pub(crate) fn to_rust(self) -> (A, B) {
		(unsafe { *Box::from_raw(self.a) }, unsafe { *Box::from_raw(self.b) })
	}
}
pub extern "C" fn C2TupleTempl_free<A, B>(_res: C2TupleTempl<A, B>) { }
impl<A, B> Drop for C2TupleTempl<A, B> {
	fn drop(&mut self) {
		if !self.a.is_null() {
			unsafe { Box::from_raw(self.a) };
		}
		if !self.b.is_null() {
			unsafe { Box::from_raw(self.b) };
		}
	}
}

/// Utility to make it easy to set a pointer to null and get its original value in line.
pub(crate) trait TakePointer<T> {
	fn take_ptr(&mut self) -> *const T;
}
impl<T> TakePointer<T> for *const T {
	fn take_ptr(&mut self) -> *const T {
		let ret = *self;
		*self = std::ptr::null();
		ret
	}
}
