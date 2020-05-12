#[no_mangle]
pub type CVec_SpendableOutputDescriptorZ = crate::c_types::CVecTempl<crate::chain::keysinterface::SpendableOutputDescriptor>;
#[no_mangle]
pub static CVec_SpendableOutputDescriptorZ_free: extern "C" fn(CVec_SpendableOutputDescriptorZ) = crate::c_types::CVecTempl_free::<crate::chain::keysinterface::SpendableOutputDescriptor>;

#[no_mangle]
pub type CVec_MessageSendEventZ = crate::c_types::CVecTempl<crate::util::events::MessageSendEvent>;
#[no_mangle]
pub static CVec_MessageSendEventZ_free: extern "C" fn(CVec_MessageSendEventZ) = crate::c_types::CVecTempl_free::<crate::util::events::MessageSendEvent>;

#[no_mangle]
pub type CVec_EventZ = crate::c_types::CVecTempl<crate::util::events::Event>;
#[no_mangle]
pub static CVec_EventZ_free: extern "C" fn(CVec_EventZ) = crate::c_types::CVecTempl_free::<crate::util::events::Event>;

#[no_mangle]
pub type C2Tuple_Txidu32Z = crate::c_types::C2TupleTempl<crate::c_types::ThirtyTwoBytes, u32>;
#[no_mangle]
pub static C2Tuple_Txidu32Z_free: extern "C" fn(C2Tuple_Txidu32Z) = crate::c_types::C2TupleTempl_free::<crate::c_types::ThirtyTwoBytes, u32>;
#[no_mangle]
pub extern "C" fn C2Tuple_Txidu32Z_new(a: crate::c_types::ThirtyTwoBytes, b: u32) -> C2Tuple_Txidu32Z {
	C2Tuple_Txidu32Z {
		a: Box::into_raw(Box::new(a)),
		b: Box::into_raw(Box::new(b)),
	}
}

#[no_mangle]
pub type C2Tuple_Scriptu64Z = crate::c_types::C2TupleTempl<crate::c_types::derived::CVec_u8Z, u64>;
#[no_mangle]
pub static C2Tuple_Scriptu64Z_free: extern "C" fn(C2Tuple_Scriptu64Z) = crate::c_types::C2TupleTempl_free::<crate::c_types::derived::CVec_u8Z, u64>;
#[no_mangle]
pub extern "C" fn C2Tuple_Scriptu64Z_new(a: crate::c_types::derived::CVec_u8Z, b: u64) -> C2Tuple_Scriptu64Z {
	C2Tuple_Scriptu64Z {
		a: Box::into_raw(Box::new(a)),
		b: Box::into_raw(Box::new(b)),
	}
}

#[no_mangle]
pub type CResult_C2Tuple_Scriptu64ZChainErrorZ = crate::c_types::CResultTempl<crate::c_types::C2TupleTempl<crate::c_types::derived::CVec_u8Z, u64>, crate::chain::chaininterface::ChainError>;
#[no_mangle]
pub static CResult_C2Tuple_Scriptu64ZChainErrorZ_free: extern "C" fn(CResult_C2Tuple_Scriptu64ZChainErrorZ) = crate::c_types::CResultTempl_free::<crate::c_types::C2TupleTempl<crate::c_types::derived::CVec_u8Z, u64>, crate::chain::chaininterface::ChainError>;
#[no_mangle]
pub static CResult_C2Tuple_Scriptu64ZChainErrorZ_good: extern "C" fn (C2Tuple_Scriptu64Z) -> CResult_C2Tuple_Scriptu64ZChainErrorZ =
	crate::c_types::CResultTempl::<crate::c_types::C2TupleTempl<crate::c_types::derived::CVec_u8Z, u64>, crate::chain::chaininterface::ChainError>::good;

#[no_mangle]
pub static CResult_C2Tuple_Scriptu64ZChainErrorZ_err: extern "C" fn (crate::chain::chaininterface::ChainError) -> CResult_C2Tuple_Scriptu64ZChainErrorZ =
	crate::c_types::CResultTempl::<crate::c_types::C2TupleTempl<crate::c_types::derived::CVec_u8Z, u64>, crate::chain::chaininterface::ChainError>::err;

#[no_mangle]
pub type CVec_u32Z = crate::c_types::CVecTempl<u32>;
#[no_mangle]
pub static CVec_u32Z_free: extern "C" fn(CVec_u32Z) = crate::c_types::CVecTempl_free::<u32>;

#[no_mangle]
pub type CTransactionSlice = crate::c_types::CSliceTempl<crate::c_types::derived::CVec_u8Z>;
impl From<&[&bitcoin::blockdata::transaction::Transaction]> for CTransactionSlice {
	fn from(slice: &[&bitcoin::blockdata::transaction::Transaction]) -> Self {
		let mut v = Vec::with_capacity(slice.len());
		for e in slice.iter() {
			let local_e = ::bitcoin::consensus::encode::serialize(*e);
			v.push(local_e.into());
		}
		Self { datalen: v.len(), data: unsafe { (*Box::into_raw(v.into_boxed_slice())).as_mut_ptr() } }
	}
}
impl CTransactionSlice {
	pub(crate) fn into_vec(mut self) -> Vec<bitcoin::blockdata::transaction::Transaction> {
		let mut ret = Vec::new();
		let mut orig: Vec<_> = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.datalen)) }.into();
		for e in orig.drain(..) {
			ret.push(::bitcoin::consensus::encode::deserialize(&e.into_rust()[..]).unwrap());
		}
		// Make sure we don't try to de-allocate the things we just drain(..)ed
		self.data = std::ptr::null_mut(); self.datalen = 0;
		ret
	}
}
#[no_mangle]
pub type C2Tuple_u64u64Z = crate::c_types::C2TupleTempl<u64, u64>;
#[no_mangle]
pub static C2Tuple_u64u64Z_free: extern "C" fn(C2Tuple_u64u64Z) = crate::c_types::C2TupleTempl_free::<u64, u64>;
#[no_mangle]
pub extern "C" fn C2Tuple_u64u64Z_new(a: u64, b: u64) -> C2Tuple_u64u64Z {
	C2Tuple_u64u64Z {
		a: Box::into_raw(Box::new(a)),
		b: Box::into_raw(Box::new(b)),
	}
}

#[no_mangle]
pub type CHTLCOutputInCommitmentSlice = crate::c_types::CSliceTempl<crate::ln::chan_utils::HTLCOutputInCommitment>;
impl From<&[&lightning::ln::chan_utils::HTLCOutputInCommitment]> for CHTLCOutputInCommitmentSlice {
	fn from(slice: &[&lightning::ln::chan_utils::HTLCOutputInCommitment]) -> Self {
		let mut v = Vec::with_capacity(slice.len());
		for e in slice.iter() {
			v.push(crate::ln::chan_utils::HTLCOutputInCommitment { inner: *e, _underlying_ref: true });
		}
		Self { datalen: v.len(), data: unsafe { (*Box::into_raw(v.into_boxed_slice())).as_mut_ptr() } }
	}
}
impl CHTLCOutputInCommitmentSlice {
	pub(crate) fn into_vec(mut self) -> Vec<&'static lightning::ln::chan_utils::HTLCOutputInCommitment> {
		let mut ret = Vec::new();
		let mut orig: Vec<_> = unsafe { Box::from_raw(std::slice::from_raw_parts_mut(self.data, self.datalen)) }.into();
		for e in orig.drain(..) {
			ret.push(unsafe { &*e.inner });
		}
		// Make sure we don't try to de-allocate the things we just drain(..)ed
		self.data = std::ptr::null_mut(); self.datalen = 0;
		ret
	}
}
#[no_mangle]
pub type CVec_SignatureZ = crate::c_types::CVecTempl<crate::c_types::Signature>;
#[no_mangle]
pub static CVec_SignatureZ_free: extern "C" fn(CVec_SignatureZ) = crate::c_types::CVecTempl_free::<crate::c_types::Signature>;

#[no_mangle]
pub type C2Tuple_SignatureCVec_SignatureZZ = crate::c_types::C2TupleTempl<crate::c_types::Signature, crate::c_types::CVecTempl<crate::c_types::Signature>>;
#[no_mangle]
pub static C2Tuple_SignatureCVec_SignatureZZ_free: extern "C" fn(C2Tuple_SignatureCVec_SignatureZZ) = crate::c_types::C2TupleTempl_free::<crate::c_types::Signature, crate::c_types::CVecTempl<crate::c_types::Signature>>;
#[no_mangle]
pub extern "C" fn C2Tuple_SignatureCVec_SignatureZZ_new(a: crate::c_types::Signature, b: crate::c_types::derived::CVec_SignatureZ) -> C2Tuple_SignatureCVec_SignatureZZ {
	C2Tuple_SignatureCVec_SignatureZZ {
		a: Box::into_raw(Box::new(a)),
		b: Box::into_raw(Box::new(b)),
	}
}

#[no_mangle]
pub type CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ = crate::c_types::CResultTempl<crate::c_types::C2TupleTempl<crate::c_types::Signature, crate::c_types::CVecTempl<crate::c_types::Signature>>, u8>;
#[no_mangle]
pub static CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ_free: extern "C" fn(CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ) = crate::c_types::CResultTempl_free::<crate::c_types::C2TupleTempl<crate::c_types::Signature, crate::c_types::CVecTempl<crate::c_types::Signature>>, u8>;
#[no_mangle]
pub static CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ_good: extern "C" fn (C2Tuple_SignatureCVec_SignatureZZ) -> CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ =
	crate::c_types::CResultTempl::<crate::c_types::C2TupleTempl<crate::c_types::Signature, crate::c_types::CVecTempl<crate::c_types::Signature>>, u8>::good;

#[no_mangle]
pub extern "C" fn CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ_err() -> CResult_C2Tuple_SignatureCVec_SignatureZZNoneZ {
	crate::c_types::CResultTempl::err(0)
}

#[no_mangle]
pub type CResult_SignatureNoneZ = crate::c_types::CResultTempl<crate::c_types::Signature, u8>;
#[no_mangle]
pub static CResult_SignatureNoneZ_free: extern "C" fn(CResult_SignatureNoneZ) = crate::c_types::CResultTempl_free::<crate::c_types::Signature, u8>;
#[no_mangle]
pub static CResult_SignatureNoneZ_good: extern "C" fn (crate::c_types::Signature) -> CResult_SignatureNoneZ =
	crate::c_types::CResultTempl::<crate::c_types::Signature, u8>::good;

#[no_mangle]
pub extern "C" fn CResult_SignatureNoneZ_err() -> CResult_SignatureNoneZ {
	crate::c_types::CResultTempl::err(0)
}

#[no_mangle]
pub type C2Tuple_SecretKey_u832Z = crate::c_types::C2TupleTempl<crate::c_types::SecretKey, crate::c_types::ThirtyTwoBytes>;
#[no_mangle]
pub static C2Tuple_SecretKey_u832Z_free: extern "C" fn(C2Tuple_SecretKey_u832Z) = crate::c_types::C2TupleTempl_free::<crate::c_types::SecretKey, crate::c_types::ThirtyTwoBytes>;
#[no_mangle]
pub extern "C" fn C2Tuple_SecretKey_u832Z_new(a: crate::c_types::SecretKey, b: crate::c_types::ThirtyTwoBytes) -> C2Tuple_SecretKey_u832Z {
	C2Tuple_SecretKey_u832Z {
		a: Box::into_raw(Box::new(a)),
		b: Box::into_raw(Box::new(b)),
	}
}

#[no_mangle]
pub type CResult_NoneAPIErrorZ = crate::c_types::CResultTempl<u8, crate::util::errors::APIError>;
#[no_mangle]
pub static CResult_NoneAPIErrorZ_free: extern "C" fn(CResult_NoneAPIErrorZ) = crate::c_types::CResultTempl_free::<u8, crate::util::errors::APIError>;
#[no_mangle]
pub extern "C" fn CResult_NoneAPIErrorZ_good() -> CResult_NoneAPIErrorZ {
	crate::c_types::CResultTempl::good(0)
}

#[no_mangle]
pub static CResult_NoneAPIErrorZ_err: extern "C" fn (crate::util::errors::APIError) -> CResult_NoneAPIErrorZ =
	crate::c_types::CResultTempl::<u8, crate::util::errors::APIError>::err;

#[no_mangle]
pub type CVec_ChannelDetailsZ = crate::c_types::CVecTempl<crate::ln::channelmanager::ChannelDetails>;
#[no_mangle]
pub static CVec_ChannelDetailsZ_free: extern "C" fn(CVec_ChannelDetailsZ) = crate::c_types::CVecTempl_free::<crate::ln::channelmanager::ChannelDetails>;

#[no_mangle]
pub type CResult_NonePaymentSendFailureZ = crate::c_types::CResultTempl<u8, crate::ln::channelmanager::PaymentSendFailure>;
#[no_mangle]
pub static CResult_NonePaymentSendFailureZ_free: extern "C" fn(CResult_NonePaymentSendFailureZ) = crate::c_types::CResultTempl_free::<u8, crate::ln::channelmanager::PaymentSendFailure>;
#[no_mangle]
pub extern "C" fn CResult_NonePaymentSendFailureZ_good() -> CResult_NonePaymentSendFailureZ {
	crate::c_types::CResultTempl::good(0)
}

#[no_mangle]
pub static CResult_NonePaymentSendFailureZ_err: extern "C" fn (crate::ln::channelmanager::PaymentSendFailure) -> CResult_NonePaymentSendFailureZ =
	crate::c_types::CResultTempl::<u8, crate::ln::channelmanager::PaymentSendFailure>::err;

#[no_mangle]
pub type CVec_NetAddressZ = crate::c_types::CVecTempl<crate::ln::msgs::NetAddress>;
#[no_mangle]
pub static CVec_NetAddressZ_free: extern "C" fn(CVec_NetAddressZ) = crate::c_types::CVecTempl_free::<crate::ln::msgs::NetAddress>;

#[no_mangle]
pub type CResult_NoneChannelMonitorUpdateErrZ = crate::c_types::CResultTempl<u8, crate::ln::channelmonitor::ChannelMonitorUpdateErr>;
#[no_mangle]
pub static CResult_NoneChannelMonitorUpdateErrZ_free: extern "C" fn(CResult_NoneChannelMonitorUpdateErrZ) = crate::c_types::CResultTempl_free::<u8, crate::ln::channelmonitor::ChannelMonitorUpdateErr>;
#[no_mangle]
pub extern "C" fn CResult_NoneChannelMonitorUpdateErrZ_good() -> CResult_NoneChannelMonitorUpdateErrZ {
	crate::c_types::CResultTempl::good(0)
}

#[no_mangle]
pub static CResult_NoneChannelMonitorUpdateErrZ_err: extern "C" fn (crate::ln::channelmonitor::ChannelMonitorUpdateErr) -> CResult_NoneChannelMonitorUpdateErrZ =
	crate::c_types::CResultTempl::<u8, crate::ln::channelmonitor::ChannelMonitorUpdateErr>::err;

#[no_mangle]
pub type CVec_HTLCUpdateZ = crate::c_types::CVecTempl<crate::ln::channelmonitor::HTLCUpdate>;
#[no_mangle]
pub static CVec_HTLCUpdateZ_free: extern "C" fn(CVec_HTLCUpdateZ) = crate::c_types::CVecTempl_free::<crate::ln::channelmonitor::HTLCUpdate>;

#[no_mangle]
pub type C2Tuple_OutPointScriptZ = crate::c_types::C2TupleTempl<crate::chain::transaction::OutPoint, crate::c_types::derived::CVec_u8Z>;
#[no_mangle]
pub static C2Tuple_OutPointScriptZ_free: extern "C" fn(C2Tuple_OutPointScriptZ) = crate::c_types::C2TupleTempl_free::<crate::chain::transaction::OutPoint, crate::c_types::derived::CVec_u8Z>;
#[no_mangle]
pub extern "C" fn C2Tuple_OutPointScriptZ_new(a: crate::chain::transaction::OutPoint, b: crate::c_types::derived::CVec_u8Z) -> C2Tuple_OutPointScriptZ {
	C2Tuple_OutPointScriptZ {
		a: Box::into_raw(Box::new(a)),
		b: Box::into_raw(Box::new(b)),
	}
}

#[no_mangle]
pub type CVec_UpdateAddHTLCZ = crate::c_types::CVecTempl<crate::ln::msgs::UpdateAddHTLC>;
#[no_mangle]
pub static CVec_UpdateAddHTLCZ_free: extern "C" fn(CVec_UpdateAddHTLCZ) = crate::c_types::CVecTempl_free::<crate::ln::msgs::UpdateAddHTLC>;

#[no_mangle]
pub type CVec_UpdateFulfillHTLCZ = crate::c_types::CVecTempl<crate::ln::msgs::UpdateFulfillHTLC>;
#[no_mangle]
pub static CVec_UpdateFulfillHTLCZ_free: extern "C" fn(CVec_UpdateFulfillHTLCZ) = crate::c_types::CVecTempl_free::<crate::ln::msgs::UpdateFulfillHTLC>;

#[no_mangle]
pub type CVec_UpdateFailHTLCZ = crate::c_types::CVecTempl<crate::ln::msgs::UpdateFailHTLC>;
#[no_mangle]
pub static CVec_UpdateFailHTLCZ_free: extern "C" fn(CVec_UpdateFailHTLCZ) = crate::c_types::CVecTempl_free::<crate::ln::msgs::UpdateFailHTLC>;

#[no_mangle]
pub type CVec_UpdateFailMalformedHTLCZ = crate::c_types::CVecTempl<crate::ln::msgs::UpdateFailMalformedHTLC>;
#[no_mangle]
pub static CVec_UpdateFailMalformedHTLCZ_free: extern "C" fn(CVec_UpdateFailMalformedHTLCZ) = crate::c_types::CVecTempl_free::<crate::ln::msgs::UpdateFailMalformedHTLC>;

#[no_mangle]
pub type CResult_boolLightningErrorZ = crate::c_types::CResultTempl<bool, crate::ln::msgs::LightningError>;
#[no_mangle]
pub static CResult_boolLightningErrorZ_free: extern "C" fn(CResult_boolLightningErrorZ) = crate::c_types::CResultTempl_free::<bool, crate::ln::msgs::LightningError>;
#[no_mangle]
pub static CResult_boolLightningErrorZ_good: extern "C" fn (bool) -> CResult_boolLightningErrorZ =
	crate::c_types::CResultTempl::<bool, crate::ln::msgs::LightningError>::good;

#[no_mangle]
pub static CResult_boolLightningErrorZ_err: extern "C" fn (crate::ln::msgs::LightningError) -> CResult_boolLightningErrorZ =
	crate::c_types::CResultTempl::<bool, crate::ln::msgs::LightningError>::err;

#[no_mangle]
pub type C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZ = crate::c_types::C3TupleTempl<crate::ln::msgs::ChannelAnnouncement, crate::ln::msgs::ChannelUpdate, crate::ln::msgs::ChannelUpdate>;
#[no_mangle]
pub static C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZ_free: extern "C" fn(C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZ) = crate::c_types::C3TupleTempl_free::<crate::ln::msgs::ChannelAnnouncement, crate::ln::msgs::ChannelUpdate, crate::ln::msgs::ChannelUpdate>;
#[no_mangle]
pub extern "C" fn C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZ_new(a: crate::ln::msgs::ChannelAnnouncement, b: crate::ln::msgs::ChannelUpdate, c: crate::ln::msgs::ChannelUpdate) -> C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZ {
	C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZ {
		a: Box::into_raw(Box::new(a)),
		b: Box::into_raw(Box::new(b)),
		c: Box::into_raw(Box::new(c)),
	}
}

#[no_mangle]
pub type CVec_C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZZ = crate::c_types::CVecTempl<crate::c_types::C3TupleTempl<crate::ln::msgs::ChannelAnnouncement, crate::ln::msgs::ChannelUpdate, crate::ln::msgs::ChannelUpdate>>;
#[no_mangle]
pub static CVec_C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZZ_free: extern "C" fn(CVec_C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZZ) = crate::c_types::CVecTempl_free::<crate::c_types::C3TupleTempl<crate::ln::msgs::ChannelAnnouncement, crate::ln::msgs::ChannelUpdate, crate::ln::msgs::ChannelUpdate>>;

#[no_mangle]
pub type CVec_NodeAnnouncementZ = crate::c_types::CVecTempl<crate::ln::msgs::NodeAnnouncement>;
#[no_mangle]
pub static CVec_NodeAnnouncementZ_free: extern "C" fn(CVec_NodeAnnouncementZ) = crate::c_types::CVecTempl_free::<crate::ln::msgs::NodeAnnouncement>;

#[no_mangle]
pub type CVec_PublicKeyZ = crate::c_types::CVecTempl<crate::c_types::PublicKey>;
#[no_mangle]
pub static CVec_PublicKeyZ_free: extern "C" fn(CVec_PublicKeyZ) = crate::c_types::CVecTempl_free::<crate::c_types::PublicKey>;

#[no_mangle]
pub type CVec_u8Z = crate::c_types::CVecTempl<u8>;
#[no_mangle]
pub static CVec_u8Z_free: extern "C" fn(CVec_u8Z) = crate::c_types::CVecTempl_free::<u8>;

#[no_mangle]
pub type CResult_CVec_u8ZPeerHandleErrorZ = crate::c_types::CResultTempl<crate::c_types::CVecTempl<u8>, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResult_CVec_u8ZPeerHandleErrorZ_free: extern "C" fn(CResult_CVec_u8ZPeerHandleErrorZ) = crate::c_types::CResultTempl_free::<crate::c_types::CVecTempl<u8>, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResult_CVec_u8ZPeerHandleErrorZ_good: extern "C" fn (CVec_u8Z) -> CResult_CVec_u8ZPeerHandleErrorZ =
	crate::c_types::CResultTempl::<crate::c_types::CVecTempl<u8>, crate::ln::peer_handler::PeerHandleError>::good;

#[no_mangle]
pub static CResult_CVec_u8ZPeerHandleErrorZ_err: extern "C" fn (crate::ln::peer_handler::PeerHandleError) -> CResult_CVec_u8ZPeerHandleErrorZ =
	crate::c_types::CResultTempl::<crate::c_types::CVecTempl<u8>, crate::ln::peer_handler::PeerHandleError>::err;

#[no_mangle]
pub type CResult_NonePeerHandleErrorZ = crate::c_types::CResultTempl<u8, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResult_NonePeerHandleErrorZ_free: extern "C" fn(CResult_NonePeerHandleErrorZ) = crate::c_types::CResultTempl_free::<u8, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub extern "C" fn CResult_NonePeerHandleErrorZ_good() -> CResult_NonePeerHandleErrorZ {
	crate::c_types::CResultTempl::good(0)
}

#[no_mangle]
pub static CResult_NonePeerHandleErrorZ_err: extern "C" fn (crate::ln::peer_handler::PeerHandleError) -> CResult_NonePeerHandleErrorZ =
	crate::c_types::CResultTempl::<u8, crate::ln::peer_handler::PeerHandleError>::err;

#[no_mangle]
pub type CResult_boolPeerHandleErrorZ = crate::c_types::CResultTempl<bool, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResult_boolPeerHandleErrorZ_free: extern "C" fn(CResult_boolPeerHandleErrorZ) = crate::c_types::CResultTempl_free::<bool, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResult_boolPeerHandleErrorZ_good: extern "C" fn (bool) -> CResult_boolPeerHandleErrorZ =
	crate::c_types::CResultTempl::<bool, crate::ln::peer_handler::PeerHandleError>::good;

#[no_mangle]
pub static CResult_boolPeerHandleErrorZ_err: extern "C" fn (crate::ln::peer_handler::PeerHandleError) -> CResult_boolPeerHandleErrorZ =
	crate::c_types::CResultTempl::<bool, crate::ln::peer_handler::PeerHandleError>::err;

#[no_mangle]
pub type CVec_RouteHopZ = crate::c_types::CVecTempl<crate::routing::router::RouteHop>;
#[no_mangle]
pub static CVec_RouteHopZ_free: extern "C" fn(CVec_RouteHopZ) = crate::c_types::CVecTempl_free::<crate::routing::router::RouteHop>;

#[no_mangle]
pub type CVec_CVec_RouteHopZZ = crate::c_types::CVecTempl<crate::c_types::CVecTempl<crate::routing::router::RouteHop>>;
#[no_mangle]
pub static CVec_CVec_RouteHopZZ_free: extern "C" fn(CVec_CVec_RouteHopZZ) = crate::c_types::CVecTempl_free::<crate::c_types::CVecTempl<crate::routing::router::RouteHop>>;

#[no_mangle]
pub type CVec_u64Z = crate::c_types::CVecTempl<u64>;
#[no_mangle]
pub static CVec_u64Z_free: extern "C" fn(CVec_u64Z) = crate::c_types::CVecTempl_free::<u64>;

