//! " Wire messages, traits representing wire message handlers, and a few error types live here."
//! ""
//! " For a normal node you probably don't need to use anything here, however, if you wish to split a"
//! " node into an internet-facing route/message socket handling daemon and a separate daemon (or"
//! " server entirely) which handles only channel-related messages you may wish to implement"
//! " ChannelMessageHandler yourself and use it to re-serialize messages and pass them across"
//! " daemons/servers."
//! ""
//! " Note that if you go with such an architecture (instead of passing raw socket events to a"
//! " non-internet-facing system) you trust the frontend internet-facing system to not lie about the"
//! " source node_id of the message, however this does allow you to significantly reduce bandwidth"
//! " between the systems as routing messages can represent a significant chunk of bandwidth usage"
//! " (especially for non-channel-publicly-announcing nodes). As an alternate design which avoids"
//! " this issue, if you have sufficient bidirectional bandwidth between your systems, you may send"
//! " raw socket events into your non-internet-facing system and then send routing events back to"
//! " track the network on the less-secure system."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

use bitcoin::secp256k1::key::PublicKey as lnPublicKey;
use bitcoin::secp256k1::Signature as lnSignature;
use bitcoin::secp256k1 as lnsecp256k1;
use bitcoin::blockdata::script::Script as lnScript;
use bitcoin::hash_types::Txid as lnTxid;
use bitcoin::hash_types::BlockHash as lnBlockHash;

use lightning::ln::msgs::DecodeError as lnDecodeErrorImport;
type lnDecodeError = lnDecodeErrorImport;

/// " An error in decoding a message or struct."
#[repr(C)]
pub struct DecodeError {
	pub(crate) inner: *const lnDecodeError,
}

#[no_mangle]
pub extern "C" fn DecodeError_free(this_ptr: DecodeError) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnDecodeError) };
}

use lightning::ln::msgs::Init as lnInitImport;
type lnInit = lnInitImport;

/// " An init message to be sent or received from a peer"
#[repr(C)]
pub struct Init {
	pub(crate) inner: *const lnInit,
}

#[no_mangle]
pub extern "C" fn Init_free(this_ptr: Init) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnInit) };
}

use lightning::ln::msgs::ErrorMessage as lnErrorMessageImport;
type lnErrorMessage = lnErrorMessageImport;

/// " An error message to be sent or received from a peer"
#[repr(C)]
pub struct ErrorMessage {
	pub(crate) inner: *const lnErrorMessage,
}

#[no_mangle]
pub extern "C" fn ErrorMessage_free(this_ptr: ErrorMessage) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnErrorMessage) };
}

use lightning::ln::msgs::Ping as lnPingImport;
type lnPing = lnPingImport;

/// " A ping message to be sent or received from a peer"
#[repr(C)]
pub struct Ping {
	pub(crate) inner: *const lnPing,
}

#[no_mangle]
pub extern "C" fn Ping_free(this_ptr: Ping) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnPing) };
}

use lightning::ln::msgs::Pong as lnPongImport;
type lnPong = lnPongImport;

/// " A pong message to be sent or received from a peer"
#[repr(C)]
pub struct Pong {
	pub(crate) inner: *const lnPong,
}

#[no_mangle]
pub extern "C" fn Pong_free(this_ptr: Pong) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnPong) };
}

use lightning::ln::msgs::OpenChannel as lnOpenChannelImport;
type lnOpenChannel = lnOpenChannelImport;

/// " An open_channel message to be sent or received from a peer"
#[repr(C)]
pub struct OpenChannel {
	pub(crate) inner: *const lnOpenChannel,
}

#[no_mangle]
pub extern "C" fn OpenChannel_free(this_ptr: OpenChannel) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnOpenChannel) };
}

use lightning::ln::msgs::AcceptChannel as lnAcceptChannelImport;
type lnAcceptChannel = lnAcceptChannelImport;

/// " An accept_channel message to be sent or received from a peer"
#[repr(C)]
pub struct AcceptChannel {
	pub(crate) inner: *const lnAcceptChannel,
}

#[no_mangle]
pub extern "C" fn AcceptChannel_free(this_ptr: AcceptChannel) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnAcceptChannel) };
}

use lightning::ln::msgs::FundingCreated as lnFundingCreatedImport;
type lnFundingCreated = lnFundingCreatedImport;

/// " A funding_created message to be sent or received from a peer"
#[repr(C)]
pub struct FundingCreated {
	pub(crate) inner: *const lnFundingCreated,
}

#[no_mangle]
pub extern "C" fn FundingCreated_free(this_ptr: FundingCreated) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnFundingCreated) };
}

use lightning::ln::msgs::FundingSigned as lnFundingSignedImport;
type lnFundingSigned = lnFundingSignedImport;

/// " A funding_signed message to be sent or received from a peer"
#[repr(C)]
pub struct FundingSigned {
	pub(crate) inner: *const lnFundingSigned,
}

#[no_mangle]
pub extern "C" fn FundingSigned_free(this_ptr: FundingSigned) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnFundingSigned) };
}

use lightning::ln::msgs::FundingLocked as lnFundingLockedImport;
type lnFundingLocked = lnFundingLockedImport;

/// " A funding_locked message to be sent or received from a peer"
#[repr(C)]
pub struct FundingLocked {
	pub(crate) inner: *const lnFundingLocked,
}

#[no_mangle]
pub extern "C" fn FundingLocked_free(this_ptr: FundingLocked) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnFundingLocked) };
}
#[no_mangle]
pub extern "C" fn FundingLocked_get_channel_id(this_ptr: &FundingLocked) -> *const [u8; 32] {
	&unsafe { &*this_ptr.inner }.channel_id
}
#[no_mangle]
pub extern "C" fn FundingLocked_set_channel_id(this_ptr: &mut FundingLocked, val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *(this_ptr.inner as *mut lnFundingLocked) }.channel_id = val.data;
}
#[no_mangle]
pub extern "C" fn FundingLocked_get_next_per_commitment_point(this_ptr: &FundingLocked) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.next_per_commitment_point)
}
#[no_mangle]
pub extern "C" fn FundingLocked_set_next_per_commitment_point(this_ptr: &mut FundingLocked, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnFundingLocked) }.next_per_commitment_point = val.into_rust();
}
#[no_mangle]
pub extern "C" fn FundingLocked_new(channel_id_arg: crate::c_types::ThirtyTwoBytes, next_per_commitment_point_arg: crate::c_types::PublicKey) -> FundingLocked {
	FundingLocked { inner: Box::into_raw(Box::new(lnFundingLocked {
		channel_id: channel_id_arg.data,
		next_per_commitment_point: next_per_commitment_point_arg.into_rust(),
	}))}
}

use lightning::ln::msgs::Shutdown as lnShutdownImport;
type lnShutdown = lnShutdownImport;

/// " A shutdown message to be sent or received from a peer"
#[repr(C)]
pub struct Shutdown {
	pub(crate) inner: *const lnShutdown,
}

#[no_mangle]
pub extern "C" fn Shutdown_free(this_ptr: Shutdown) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnShutdown) };
}

use lightning::ln::msgs::ClosingSigned as lnClosingSignedImport;
type lnClosingSigned = lnClosingSignedImport;

/// " A closing_signed message to be sent or received from a peer"
#[repr(C)]
pub struct ClosingSigned {
	pub(crate) inner: *const lnClosingSigned,
}

#[no_mangle]
pub extern "C" fn ClosingSigned_free(this_ptr: ClosingSigned) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnClosingSigned) };
}

use lightning::ln::msgs::UpdateAddHTLC as lnUpdateAddHTLCImport;
type lnUpdateAddHTLC = lnUpdateAddHTLCImport;

/// " An update_add_htlc message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateAddHTLC {
	pub(crate) inner: *const lnUpdateAddHTLC,
}

#[no_mangle]
pub extern "C" fn UpdateAddHTLC_free(this_ptr: UpdateAddHTLC) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUpdateAddHTLC) };
}

use lightning::ln::msgs::UpdateFulfillHTLC as lnUpdateFulfillHTLCImport;
type lnUpdateFulfillHTLC = lnUpdateFulfillHTLCImport;

/// " An update_fulfill_htlc message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateFulfillHTLC {
	pub(crate) inner: *const lnUpdateFulfillHTLC,
}

#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_free(this_ptr: UpdateFulfillHTLC) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUpdateFulfillHTLC) };
}

use lightning::ln::msgs::UpdateFailHTLC as lnUpdateFailHTLCImport;
type lnUpdateFailHTLC = lnUpdateFailHTLCImport;

/// " An update_fail_htlc message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateFailHTLC {
	pub(crate) inner: *const lnUpdateFailHTLC,
}

#[no_mangle]
pub extern "C" fn UpdateFailHTLC_free(this_ptr: UpdateFailHTLC) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUpdateFailHTLC) };
}

use lightning::ln::msgs::UpdateFailMalformedHTLC as lnUpdateFailMalformedHTLCImport;
type lnUpdateFailMalformedHTLC = lnUpdateFailMalformedHTLCImport;

/// " An update_fail_malformed_htlc message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateFailMalformedHTLC {
	pub(crate) inner: *const lnUpdateFailMalformedHTLC,
}

#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_free(this_ptr: UpdateFailMalformedHTLC) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUpdateFailMalformedHTLC) };
}

use lightning::ln::msgs::CommitmentSigned as lnCommitmentSignedImport;
type lnCommitmentSigned = lnCommitmentSignedImport;

/// " A commitment_signed message to be sent or received from a peer"
#[repr(C)]
pub struct CommitmentSigned {
	pub(crate) inner: *const lnCommitmentSigned,
}

#[no_mangle]
pub extern "C" fn CommitmentSigned_free(this_ptr: CommitmentSigned) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnCommitmentSigned) };
}

use lightning::ln::msgs::RevokeAndACK as lnRevokeAndACKImport;
type lnRevokeAndACK = lnRevokeAndACKImport;

/// " A revoke_and_ack message to be sent or received from a peer"
#[repr(C)]
pub struct RevokeAndACK {
	pub(crate) inner: *const lnRevokeAndACK,
}

#[no_mangle]
pub extern "C" fn RevokeAndACK_free(this_ptr: RevokeAndACK) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnRevokeAndACK) };
}

use lightning::ln::msgs::UpdateFee as lnUpdateFeeImport;
type lnUpdateFee = lnUpdateFeeImport;

/// " An update_fee message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateFee {
	pub(crate) inner: *const lnUpdateFee,
}

#[no_mangle]
pub extern "C" fn UpdateFee_free(this_ptr: UpdateFee) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUpdateFee) };
}

use lightning::ln::msgs::ChannelReestablish as lnChannelReestablishImport;
type lnChannelReestablish = lnChannelReestablishImport;

/// " A channel_reestablish message to be sent or received from a peer"
#[repr(C)]
pub struct ChannelReestablish {
	pub(crate) inner: *const lnChannelReestablish,
}

#[no_mangle]
pub extern "C" fn ChannelReestablish_free(this_ptr: ChannelReestablish) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelReestablish) };
}

use lightning::ln::msgs::AnnouncementSignatures as lnAnnouncementSignaturesImport;
type lnAnnouncementSignatures = lnAnnouncementSignaturesImport;

/// " An announcement_signatures message to be sent or received from a peer"
#[repr(C)]
pub struct AnnouncementSignatures {
	pub(crate) inner: *const lnAnnouncementSignatures,
}

#[no_mangle]
pub extern "C" fn AnnouncementSignatures_free(this_ptr: AnnouncementSignatures) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnAnnouncementSignatures) };
}

use lightning::ln::msgs::NetAddress as lnNetAddressImport;
type lnNetAddress = lnNetAddressImport;

/// " An address which can be used to connect to a remote peer"
#[repr(C)]
pub struct NetAddress {
	pub(crate) inner: *const lnNetAddress,
}

#[no_mangle]
pub extern "C" fn NetAddress_free(this_ptr: NetAddress) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnNetAddress) };
}

use lightning::ln::msgs::UnsignedNodeAnnouncement as lnUnsignedNodeAnnouncementImport;
type lnUnsignedNodeAnnouncement = lnUnsignedNodeAnnouncementImport;

/// " The unsigned part of a node_announcement"
#[repr(C)]
pub struct UnsignedNodeAnnouncement {
	pub(crate) inner: *const lnUnsignedNodeAnnouncement,
}

#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_free(this_ptr: UnsignedNodeAnnouncement) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUnsignedNodeAnnouncement) };
}
/// " The node_id this announcement originated from (don't rebroadcast the node_announcement back"
/// " to this node)."
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_get_node_id(this_ptr: &UnsignedNodeAnnouncement) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.node_id)
}
/// " The node_id this announcement originated from (don't rebroadcast the node_announcement back"
/// " to this node)."
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_set_node_id(this_ptr: &mut UnsignedNodeAnnouncement, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnUnsignedNodeAnnouncement) }.node_id = val.into_rust();
}

use lightning::ln::msgs::NodeAnnouncement as lnNodeAnnouncementImport;
type lnNodeAnnouncement = lnNodeAnnouncementImport;

/// " A node_announcement message to be sent or received from a peer"
#[repr(C)]
pub struct NodeAnnouncement {
	pub(crate) inner: *const lnNodeAnnouncement,
}

#[no_mangle]
pub extern "C" fn NodeAnnouncement_free(this_ptr: NodeAnnouncement) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnNodeAnnouncement) };
}

use lightning::ln::msgs::UnsignedChannelAnnouncement as lnUnsignedChannelAnnouncementImport;
type lnUnsignedChannelAnnouncement = lnUnsignedChannelAnnouncementImport;

/// " The unsigned part of a channel_announcement"
#[repr(C)]
pub struct UnsignedChannelAnnouncement {
	pub(crate) inner: *const lnUnsignedChannelAnnouncement,
}

#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_free(this_ptr: UnsignedChannelAnnouncement) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUnsignedChannelAnnouncement) };
}
/// " One of the two node_ids which are endpoints of this channel"
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_node_id_1(this_ptr: &UnsignedChannelAnnouncement) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.node_id_1)
}
/// " One of the two node_ids which are endpoints of this channel"
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_node_id_1(this_ptr: &mut UnsignedChannelAnnouncement, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnUnsignedChannelAnnouncement) }.node_id_1 = val.into_rust();
}
/// " The other of the two node_ids which are endpoints of this channel"
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_node_id_2(this_ptr: &UnsignedChannelAnnouncement) -> crate::c_types::PublicKey {
	crate::c_types::PublicKey::from_rust(&unsafe { &*this_ptr.inner }.node_id_2)
}
/// " The other of the two node_ids which are endpoints of this channel"
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_node_id_2(this_ptr: &mut UnsignedChannelAnnouncement, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnUnsignedChannelAnnouncement) }.node_id_2 = val.into_rust();
}

use lightning::ln::msgs::ChannelAnnouncement as lnChannelAnnouncementImport;
type lnChannelAnnouncement = lnChannelAnnouncementImport;

/// " A channel_announcement message to be sent or received from a peer"
#[repr(C)]
pub struct ChannelAnnouncement {
	pub(crate) inner: *const lnChannelAnnouncement,
}

#[no_mangle]
pub extern "C" fn ChannelAnnouncement_free(this_ptr: ChannelAnnouncement) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelAnnouncement) };
}

use lightning::ln::msgs::ChannelUpdate as lnChannelUpdateImport;
type lnChannelUpdate = lnChannelUpdateImport;

/// " A channel_update message to be sent or received from a peer"
#[repr(C)]
pub struct ChannelUpdate {
	pub(crate) inner: *const lnChannelUpdate,
}

#[no_mangle]
pub extern "C" fn ChannelUpdate_free(this_ptr: ChannelUpdate) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelUpdate) };
}

use lightning::ln::msgs::LightningError as lnLightningErrorImport;
type lnLightningError = lnLightningErrorImport;

/// " An Err type for failure to process messages."
#[repr(C)]
pub struct LightningError {
	pub(crate) inner: *const lnLightningError,
}

#[no_mangle]
pub extern "C" fn LightningError_free(this_ptr: LightningError) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnLightningError) };
}

use lightning::ln::msgs::CommitmentUpdate as lnCommitmentUpdateImport;
type lnCommitmentUpdate = lnCommitmentUpdateImport;

/// " Struct used to return values from revoke_and_ack messages, containing a bunch of commitment"
/// " transaction updates if they were pending."
#[repr(C)]
pub struct CommitmentUpdate {
	pub(crate) inner: *const lnCommitmentUpdate,
}

#[no_mangle]
pub extern "C" fn CommitmentUpdate_free(this_ptr: CommitmentUpdate) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnCommitmentUpdate) };
}
/// " Finally, the commitment_signed message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_commitment_signed(this_ptr: &CommitmentUpdate) -> *const CommitmentSigned {
	Box::into_raw(Box::new(CommitmentSigned { inner: &unsafe { &*this_ptr.inner }.commitment_signed } ))
}
/// " Finally, the commitment_signed message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_commitment_signed(this_ptr: &mut CommitmentUpdate, val: CommitmentSigned) {
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.commitment_signed = *unsafe { Box::from_raw(val.inner as *mut _) };
}

use lightning::ln::msgs::HTLCFailChannelUpdate as lnHTLCFailChannelUpdateImport;
type lnHTLCFailChannelUpdate = lnHTLCFailChannelUpdateImport;

/// " The information we received from a peer along the route of a payment we originated. This is"
/// " returned by ChannelMessageHandler::handle_update_fail_htlc to be passed into"
/// " RoutingMessageHandler::handle_htlc_fail_channel_update to update our network map."
#[repr(C)]
pub struct HTLCFailChannelUpdate {
	pub(crate) inner: *const lnHTLCFailChannelUpdate,
}

#[no_mangle]
pub extern "C" fn HTLCFailChannelUpdate_free(this_ptr: HTLCFailChannelUpdate) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnHTLCFailChannelUpdate) };
}
/// " A trait to describe an object which can receive channel messages."
/// ""
/// " Messages MAY be called in parallel when they originate from different their_node_ids, however"
/// " they MUST NOT be called in parallel when the two calls have the same their_node_id."
#[repr(C)]
pub struct ChannelMessageHandler {
	pub this_arg: *mut c_void,
	/// " Handle an incoming open_channel message from the given peer."
	pub handle_open_channel: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, their_features: crate::ln::features::InitFeatures, msg: &OpenChannel),
	/// " Handle an incoming accept_channel message from the given peer."
	pub handle_accept_channel: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, their_features: crate::ln::features::InitFeatures, msg: &AcceptChannel),
	/// " Handle an incoming funding_created message from the given peer."
	pub handle_funding_created: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &FundingCreated),
	/// " Handle an incoming funding_signed message from the given peer."
	pub handle_funding_signed: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &FundingSigned),
	/// " Handle an incoming funding_locked message from the given peer."
	pub handle_funding_locked: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &FundingLocked),
	/// " Handle an incoming shutdown message from the given peer."
	pub handle_shutdown: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &Shutdown),
	/// " Handle an incoming closing_signed message from the given peer."
	pub handle_closing_signed: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &ClosingSigned),
	/// " Handle an incoming update_add_htlc message from the given peer."
	pub handle_update_add_htlc: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateAddHTLC),
	/// " Handle an incoming update_fulfill_htlc message from the given peer."
	pub handle_update_fulfill_htlc: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateFulfillHTLC),
	/// " Handle an incoming update_fail_htlc message from the given peer."
	pub handle_update_fail_htlc: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateFailHTLC),
	/// " Handle an incoming update_fail_malformed_htlc message from the given peer."
	pub handle_update_fail_malformed_htlc: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateFailMalformedHTLC),
	/// " Handle an incoming commitment_signed message from the given peer."
	pub handle_commitment_signed: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &CommitmentSigned),
	/// " Handle an incoming revoke_and_ack message from the given peer."
	pub handle_revoke_and_ack: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &RevokeAndACK),
	/// " Handle an incoming update_fee message from the given peer."
	pub handle_update_fee: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateFee),
	/// " Handle an incoming announcement_signatures message from the given peer."
	pub handle_announcement_signatures: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &AnnouncementSignatures),
	/// " Indicates a connection to the peer failed/an existing connection was lost. If no connection"
	/// " is believed to be possible in the future (eg they're sending us messages we don't"
	/// " understand or indicate they require unknown feature bits), no_connection_possible is set"
	/// " and any outstanding channels should be failed."
	pub peer_disconnected: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, no_connection_possible: bool),
	/// " Handle a peer reconnecting, possibly generating channel_reestablish message(s)."
	pub peer_connected: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &Init),
	/// " Handle an incoming channel_reestablish message from the given peer."
	pub handle_channel_reestablish: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &ChannelReestablish),
	/// " Handle an incoming error message from the given peer."
	pub handle_error: extern "C" fn (this_arg: *const  c_void, their_node_id: crate::c_types::PublicKey, msg: &ErrorMessage),
	pub MessageSendEventsProvider: crate::util::events::MessageSendEventsProvider,
}
impl lightning::util::events::MessageSendEventsProvider for ChannelMessageHandler {
	fn get_and_clear_pending_msg_events(&self) -> Vec<lightning::util::events::MessageSendEvent> {
		unimplemented!()
	}
}
unsafe impl Send for ChannelMessageHandler {}
unsafe impl Sync for ChannelMessageHandler {}

use lightning::ln::msgs::ChannelMessageHandler as lnChannelMessageHandler;
impl lnChannelMessageHandler for ChannelMessageHandler {
	fn handle_open_channel(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, their_features: lightning::ln::features::InitFeatures, msg: &lightning::ln::msgs::OpenChannel) {
		(self.handle_open_channel)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new(their_features)) }, &OpenChannel { inner: msg })
	}
	fn handle_accept_channel(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, their_features: lightning::ln::features::InitFeatures, msg: &lightning::ln::msgs::AcceptChannel) {
		(self.handle_accept_channel)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new(their_features)) }, &AcceptChannel { inner: msg })
	}
	fn handle_funding_created(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingCreated) {
		(self.handle_funding_created)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &FundingCreated { inner: msg })
	}
	fn handle_funding_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingSigned) {
		(self.handle_funding_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &FundingSigned { inner: msg })
	}
	fn handle_funding_locked(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingLocked) {
		(self.handle_funding_locked)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &FundingLocked { inner: msg })
	}
	fn handle_shutdown(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::Shutdown) {
		(self.handle_shutdown)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &Shutdown { inner: msg })
	}
	fn handle_closing_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ClosingSigned) {
		(self.handle_closing_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &ClosingSigned { inner: msg })
	}
	fn handle_update_add_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateAddHTLC) {
		(self.handle_update_add_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &UpdateAddHTLC { inner: msg })
	}
	fn handle_update_fulfill_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFulfillHTLC) {
		(self.handle_update_fulfill_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &UpdateFulfillHTLC { inner: msg })
	}
	fn handle_update_fail_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFailHTLC) {
		(self.handle_update_fail_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &UpdateFailHTLC { inner: msg })
	}
	fn handle_update_fail_malformed_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFailMalformedHTLC) {
		(self.handle_update_fail_malformed_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &UpdateFailMalformedHTLC { inner: msg })
	}
	fn handle_commitment_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::CommitmentSigned) {
		(self.handle_commitment_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &CommitmentSigned { inner: msg })
	}
	fn handle_revoke_and_ack(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::RevokeAndACK) {
		(self.handle_revoke_and_ack)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &RevokeAndACK { inner: msg })
	}
	fn handle_update_fee(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFee) {
		(self.handle_update_fee)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &UpdateFee { inner: msg })
	}
	fn handle_announcement_signatures(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::AnnouncementSignatures) {
		(self.handle_announcement_signatures)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &AnnouncementSignatures { inner: msg })
	}
	fn peer_disconnected(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, no_connection_possible: bool) {
		(self.peer_disconnected)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), no_connection_possible)
	}
	fn peer_connected(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::Init) {
		(self.peer_connected)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &Init { inner: msg })
	}
	fn handle_channel_reestablish(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ChannelReestablish) {
		(self.handle_channel_reestablish)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &ChannelReestablish { inner: msg })
	}
	fn handle_error(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ErrorMessage) {
		(self.handle_error)(self.this_arg, crate::c_types::PublicKey::from_rust(their_node_id), &ErrorMessage { inner: msg })
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for ChannelMessageHandler {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
/// " A trait to describe an object which can receive routing messages."
#[repr(C)]
pub struct RoutingMessageHandler {
	pub this_arg: *mut c_void,
	//XXX: Need to export handle_node_announcement
	//XXX: Need to export handle_channel_announcement
	//XXX: Need to export handle_channel_update
	//XXX: Need to export handle_htlc_fail_channel_update
	//XXX: Need to export get_next_channel_announcements
	//XXX: Need to export get_next_node_announcements
	/// " Returns whether a full sync should be requested from a peer."
	pub should_request_full_sync: extern "C" fn (this_arg: *const  c_void, node_id: crate::c_types::PublicKey) -> bool,
}
unsafe impl Send for RoutingMessageHandler {}
unsafe impl Sync for RoutingMessageHandler {}

use lightning::ln::msgs::RoutingMessageHandler as lnRoutingMessageHandler;
impl lnRoutingMessageHandler for RoutingMessageHandler {
	fn handle_node_announcement(&self, msg: &lightning::ln::msgs::NodeAnnouncement) -> std::result::Result<bool, lightning::ln::msgs::LightningError> {
		unimplemented!();
	}
	fn handle_channel_announcement(&self, msg: &lightning::ln::msgs::ChannelAnnouncement) -> std::result::Result<bool, lightning::ln::msgs::LightningError> {
		unimplemented!();
	}
	fn handle_channel_update(&self, msg: &lightning::ln::msgs::ChannelUpdate) -> std::result::Result<bool, lightning::ln::msgs::LightningError> {
		unimplemented!();
	}
	fn handle_htlc_fail_channel_update(&self, update: &lightning::ln::msgs::HTLCFailChannelUpdate) {
		unimplemented!();
	}
	fn get_next_channel_announcements(&self, starting_point: u64, batch_amount: u8) -> Vec<(lightning::ln::msgs::ChannelAnnouncement, Option<lightning::ln::msgs::ChannelUpdate>, Option<lightning::ln::msgs::ChannelUpdate>)> {
		unimplemented!();
	}
	fn get_next_node_announcements(&self, starting_point: Option<&bitcoin::secp256k1::key::PublicKey>, batch_amount: u8) -> Vec<lightning::ln::msgs::NodeAnnouncement> {
		unimplemented!();
	}
	fn should_request_full_sync(&self, node_id: &bitcoin::secp256k1::key::PublicKey) -> bool {
		(self.should_request_full_sync)(self.this_arg, crate::c_types::PublicKey::from_rust(node_id))
	}
}

// We're essentially a pointer already, or at least a set of pointers, so allow us to be used
// directly as a Deref trait in higher-level structs:
impl std::ops::Deref for RoutingMessageHandler {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}
