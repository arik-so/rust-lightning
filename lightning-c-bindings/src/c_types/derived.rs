#[no_mangle]
pub type CVec_MessageSendEventZ = crate::c_types::CVecTempl<crate::util::events::MessageSendEvent>;
#[no_mangle]
pub static CVec_MessageSendEventZ_free: extern "C" fn(CVec_MessageSendEventZ) = crate::c_types::CVecTempl_free::<crate::util::events::MessageSendEvent>;
#[no_mangle]
pub type CVec_EventZ = crate::c_types::CVecTempl<crate::util::events::Event>;
#[no_mangle]
pub static CVec_EventZ_free: extern "C" fn(CVec_EventZ) = crate::c_types::CVecTempl_free::<crate::util::events::Event>;
#[no_mangle]
pub type CTuple2_Scriptu64Z = crate::c_types::C2TupleTempl<crate::c_types::derived::CVec_u8Z, u64>;
#[no_mangle]
pub static CTuple2_Scriptu64Z_free: extern "C" fn(CTuple2_Scriptu64Z) = crate::c_types::C2TupleTempl_free::<crate::c_types::derived::CVec_u8Z, u64>;
#[no_mangle]
pub type CResult_CTuple2_Scriptu64ZChainErrorZ = crate::c_types::CResultTempl<crate::c_types::C2TupleTempl<crate::c_types::derived::CVec_u8Z, u64>, crate::chain::chaininterface::ChainError>;
#[no_mangle]
pub static CResult_CTuple2_Scriptu64ZChainErrorZ_free: extern "C" fn(CResult_CTuple2_Scriptu64ZChainErrorZ) = crate::c_types::CResultTempl_free::<crate::c_types::C2TupleTempl<crate::c_types::derived::CVec_u8Z, u64>, crate::chain::chaininterface::ChainError>;
#[no_mangle]
pub type CResult_SignatureNoneZ = crate::c_types::CResultTempl<crate::c_types::Signature, u8>;
#[no_mangle]
pub static CResult_SignatureNoneZ_free: extern "C" fn(CResult_SignatureNoneZ) = crate::c_types::CResultTempl_free::<crate::c_types::Signature, u8>;
#[no_mangle]
pub type C2Tuple_SecretKey_u832Z = crate::c_types::C2TupleTempl<crate::c_types::SecretKey, crate::c_types::ThirtyTwoBytes>;
#[no_mangle]
pub static C2Tuple_SecretKey_u832Z_free: extern "C" fn(C2Tuple_SecretKey_u832Z) = crate::c_types::C2TupleTempl_free::<crate::c_types::SecretKey, crate::c_types::ThirtyTwoBytes>;
#[no_mangle]
pub type CResult_NoneAPIErrorZ = crate::c_types::CResultTempl<u8, crate::util::errors::APIError>;
#[no_mangle]
pub static CResult_NoneAPIErrorZ_free: extern "C" fn(CResult_NoneAPIErrorZ) = crate::c_types::CResultTempl_free::<u8, crate::util::errors::APIError>;
#[no_mangle]
pub type CVec_ChannelDetailsZ = crate::c_types::CVecTempl<crate::ln::channelmanager::ChannelDetails>;
#[no_mangle]
pub static CVec_ChannelDetailsZ_free: extern "C" fn(CVec_ChannelDetailsZ) = crate::c_types::CVecTempl_free::<crate::ln::channelmanager::ChannelDetails>;
#[no_mangle]
pub type CResult_NonePaymentSendFailureZ = crate::c_types::CResultTempl<u8, crate::ln::channelmanager::PaymentSendFailure>;
#[no_mangle]
pub static CResult_NonePaymentSendFailureZ_free: extern "C" fn(CResult_NonePaymentSendFailureZ) = crate::c_types::CResultTempl_free::<u8, crate::ln::channelmanager::PaymentSendFailure>;
#[no_mangle]
pub type CVec_NetAddressZ = crate::c_types::CVecTempl<crate::ln::msgs::NetAddress>;
#[no_mangle]
pub static CVec_NetAddressZ_free: extern "C" fn(CVec_NetAddressZ) = crate::c_types::CVecTempl_free::<crate::ln::msgs::NetAddress>;
#[no_mangle]
pub type CResult_NoneChannelMonitorUpdateErrZ = crate::c_types::CResultTempl<u8, crate::ln::channelmonitor::ChannelMonitorUpdateErr>;
#[no_mangle]
pub static CResult_NoneChannelMonitorUpdateErrZ_free: extern "C" fn(CResult_NoneChannelMonitorUpdateErrZ) = crate::c_types::CResultTempl_free::<u8, crate::ln::channelmonitor::ChannelMonitorUpdateErr>;
#[no_mangle]
pub type CVec_HTLCUpdateZ = crate::c_types::CVecTempl<crate::ln::channelmonitor::HTLCUpdate>;
#[no_mangle]
pub static CVec_HTLCUpdateZ_free: extern "C" fn(CVec_HTLCUpdateZ) = crate::c_types::CVecTempl_free::<crate::ln::channelmonitor::HTLCUpdate>;
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
pub type CResult_NonePeerHandleErrorZ = crate::c_types::CResultTempl<u8, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResult_NonePeerHandleErrorZ_free: extern "C" fn(CResult_NonePeerHandleErrorZ) = crate::c_types::CResultTempl_free::<u8, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub type CResult_boolPeerHandleErrorZ = crate::c_types::CResultTempl<bool, crate::ln::peer_handler::PeerHandleError>;
#[no_mangle]
pub static CResult_boolPeerHandleErrorZ_free: extern "C" fn(CResult_boolPeerHandleErrorZ) = crate::c_types::CResultTempl_free::<bool, crate::ln::peer_handler::PeerHandleError>;
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
