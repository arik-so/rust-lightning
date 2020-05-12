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
/// A reference to a script, in (pointer, length) form.
/// This type does *not* own its own memory, so access to it after, eg, the call in which it was
/// provided to you are invalid.
pub struct Script {
	pub data: *const u8,
	pub datalen: usize
}
impl Script {
	pub(crate) fn into_bitcoin(&self) -> BitcoinScript {
		BitcoinScript::from(unsafe { std::slice::from_raw_parts(self.data, self.datalen) }.to_vec())
	}
	pub(crate) fn from_bitcoin(s: &BitcoinScript) -> Self {
		Self {
			data: s.as_bytes().as_ptr(),
			datalen: s.len(),
		}
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
		} else {
			unsafe { Box::from_raw(self.contents.err) };
		}
	}
}

#[no_mangle]
pub type CResultNoneAPIError = CResultTempl<u8, crate::util::errors::APIError>;
#[no_mangle]
pub static CResultNoneAPIError_free: extern "C" fn(CResultNoneAPIError) = CResultTempl_free::<u8, crate::util::errors::APIError>;

#[no_mangle]
pub type CResultNonePeerHandleError = CResultTempl<u8, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResultNonePeerHandleError_free: extern "C" fn(CResultNonePeerHandleError) = CResultTempl_free::<u8, crate::ln::peer_handler::PeerHandleError>;

#[no_mangle]
pub type CResultNonePaymentSendFailure = CResultTempl<u8, crate::ln::channelmanager::PaymentSendFailure>;
#[no_mangle]
pub static CResultNonePaymentSendFailure_free: extern "C" fn(CResultNonePaymentSendFailure) = CResultTempl_free::<u8, crate::ln::channelmanager::PaymentSendFailure>;

#[no_mangle]
pub type CResultboolLightningError = CResultTempl<bool, crate::ln::msgs::LightningError>;
#[no_mangle]
pub static CResultboolLightningError_free: extern "C" fn(CResultboolLightningError) = CResultTempl_free::<bool, crate::ln::msgs::LightningError>;

#[no_mangle]
pub type CResultSignatureNone = CResultTempl<Signature, u8>;
#[no_mangle]
pub static CResultSignatureNone_free: extern "C" fn(CResultSignatureNone) = CResultTempl_free::<Signature, u8>;

#[no_mangle]
pub type CResultboolPeerHandleError = CResultTempl<bool, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResultboolPeerHandleError_free: extern "C" fn(CResultboolPeerHandleError) = CResultTempl_free::<bool, crate::ln::peer_handler::PeerHandleError>;

#[no_mangle]
pub type CResultNoneChannelMonitorUpdateErr = CResultTempl<u8, crate::ln::channelmonitor::ChannelMonitorUpdateErr>;
#[no_mangle]
pub static CResultNoneChannelMonitorUpdateErr_free: extern "C" fn(CResultNoneChannelMonitorUpdateErr) = CResultTempl_free::<u8, crate::ln::channelmonitor::ChannelMonitorUpdateErr>;
#[no_mangle]
pub extern "C" fn CResultNoneChannelMonitorUpdateErr_good() -> CResultNoneChannelMonitorUpdateErr {
	CResultTempl::good(0)
}
#[no_mangle]
pub static CResultNoneChannelMonitorUpdateErr_err:
	extern"C" fn(crate::ln::channelmonitor::ChannelMonitorUpdateErr) -> CResultNoneChannelMonitorUpdateErr =
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
		unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.datalen)) };
	}
}

#[no_mangle]
pub type CVecChannelDetails = CVecTempl<crate::ln::channelmanager::ChannelDetails>;
#[no_mangle]
pub static CVecChannelDetails_free: extern "C" fn(CVecChannelDetails) = CVecTempl_free::<crate::ln::channelmanager::ChannelDetails>;

#[no_mangle]
pub type CVecUpdateAddHTLC = CVecTempl<crate::ln::msgs::UpdateAddHTLC>;
#[no_mangle]
pub static CVecUpdateAddHTLC_free: extern "C" fn(CVecUpdateAddHTLC) = CVecTempl_free::<crate::ln::msgs::UpdateAddHTLC>;

#[no_mangle]
pub type CVecUpdateFulfillHTLC = CVecTempl<crate::ln::msgs::UpdateFulfillHTLC>;
#[no_mangle]
pub static CVecUpdateFulfillHTLC_free: extern "C" fn(CVecUpdateFulfillHTLC) = CVecTempl_free::<crate::ln::msgs::UpdateFulfillHTLC>;

#[no_mangle]
pub type CVecUpdateFailHTLC = CVecTempl<crate::ln::msgs::UpdateFailHTLC>;
#[no_mangle]
pub static CVecUpdateFailHTLC_free: extern "C" fn(CVecUpdateFailHTLC) = CVecTempl_free::<crate::ln::msgs::UpdateFailHTLC>;

#[no_mangle]
pub type CVecUpdateFailMalformedHTLC = CVecTempl<crate::ln::msgs::UpdateFailMalformedHTLC>;
#[no_mangle]
pub static CVecUpdateFailMalformedHTLC_free: extern "C" fn(CVecUpdateFailMalformedHTLC) = CVecTempl_free::<crate::ln::msgs::UpdateFailMalformedHTLC>;

#[no_mangle]
pub type CVecHTLCUpdate = CVecTempl<crate::ln::channelmonitor::HTLCUpdate>;
#[no_mangle]
pub static CVecHTLCUpdate_free: extern "C" fn(CVecHTLCUpdate) = CVecTempl_free::<crate::ln::channelmonitor::HTLCUpdate>;

#[no_mangle]
pub type CVecNetAddress = CVecTempl<crate::ln::msgs::NetAddress>;
#[no_mangle]
pub static CVecNetAddress_free: extern "C" fn(CVecNetAddress) = CVecTempl_free::<crate::ln::msgs::NetAddress>;

#[no_mangle]
pub type CVecPublicKey = CVecTempl<PublicKey>;
#[no_mangle]
pub static CVecPublicKey_free: extern "C" fn(CVecPublicKey) = CVecTempl_free::<PublicKey>;

#[no_mangle]
pub type CVecu64 = CVecTempl<u64>;
#[no_mangle]
pub static CVecu64_free: extern "C" fn(CVecu64) = CVecTempl_free::<u64>;


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
