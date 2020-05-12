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
use crate::c_types::TakePointer;

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
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnDecodeError,
}

impl Drop for DecodeError {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnDecodeError) };
		}
	}
}
#[no_mangle]
pub extern "C" fn DecodeError_free(this_ptr: DecodeError) { }

use lightning::ln::msgs::Init as lnInitImport;
type lnInit = lnInitImport;

/// " An init message to be sent or received from a peer"
#[repr(C)]
pub struct Init {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnInit,
}

impl Drop for Init {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnInit) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Init_free(this_ptr: Init) { }

use lightning::ln::msgs::ErrorMessage as lnErrorMessageImport;
type lnErrorMessage = lnErrorMessageImport;

/// " An error message to be sent or received from a peer"
#[repr(C)]
pub struct ErrorMessage {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnErrorMessage,
}

impl Drop for ErrorMessage {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnErrorMessage) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ErrorMessage_free(this_ptr: ErrorMessage) { }

use lightning::ln::msgs::Ping as lnPingImport;
type lnPing = lnPingImport;

/// " A ping message to be sent or received from a peer"
#[repr(C)]
pub struct Ping {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnPing,
}

impl Drop for Ping {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnPing) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Ping_free(this_ptr: Ping) { }

use lightning::ln::msgs::Pong as lnPongImport;
type lnPong = lnPongImport;

/// " A pong message to be sent or received from a peer"
#[repr(C)]
pub struct Pong {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnPong,
}

impl Drop for Pong {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnPong) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Pong_free(this_ptr: Pong) { }

use lightning::ln::msgs::OpenChannel as lnOpenChannelImport;
type lnOpenChannel = lnOpenChannelImport;

/// " An open_channel message to be sent or received from a peer"
#[repr(C)]
pub struct OpenChannel {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnOpenChannel,
}

impl Drop for OpenChannel {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnOpenChannel) };
		}
	}
}
#[no_mangle]
pub extern "C" fn OpenChannel_free(this_ptr: OpenChannel) { }

use lightning::ln::msgs::AcceptChannel as lnAcceptChannelImport;
type lnAcceptChannel = lnAcceptChannelImport;

/// " An accept_channel message to be sent or received from a peer"
#[repr(C)]
pub struct AcceptChannel {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnAcceptChannel,
}

impl Drop for AcceptChannel {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnAcceptChannel) };
		}
	}
}
#[no_mangle]
pub extern "C" fn AcceptChannel_free(this_ptr: AcceptChannel) { }

use lightning::ln::msgs::FundingCreated as lnFundingCreatedImport;
type lnFundingCreated = lnFundingCreatedImport;

/// " A funding_created message to be sent or received from a peer"
#[repr(C)]
pub struct FundingCreated {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnFundingCreated,
}

impl Drop for FundingCreated {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnFundingCreated) };
		}
	}
}
#[no_mangle]
pub extern "C" fn FundingCreated_free(this_ptr: FundingCreated) { }

use lightning::ln::msgs::FundingSigned as lnFundingSignedImport;
type lnFundingSigned = lnFundingSignedImport;

/// " A funding_signed message to be sent or received from a peer"
#[repr(C)]
pub struct FundingSigned {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnFundingSigned,
}

impl Drop for FundingSigned {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnFundingSigned) };
		}
	}
}
#[no_mangle]
pub extern "C" fn FundingSigned_free(this_ptr: FundingSigned) { }

use lightning::ln::msgs::FundingLocked as lnFundingLockedImport;
type lnFundingLocked = lnFundingLockedImport;

/// " A funding_locked message to be sent or received from a peer"
#[repr(C)]
pub struct FundingLocked {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnFundingLocked,
}

impl Drop for FundingLocked {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnFundingLocked) };
		}
	}
}
#[no_mangle]
pub extern "C" fn FundingLocked_free(this_ptr: FundingLocked) { }
#[no_mangle]
pub extern "C" fn FundingLocked_get_channel_id(this_ptr: &FundingLocked) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.channel_id;
	&(*inner_val)
}
#[no_mangle]
pub extern "C" fn FundingLocked_set_channel_id(this_ptr: &mut FundingLocked, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *(this_ptr.inner as *mut lnFundingLocked) }.channel_id = val.data;
}
#[no_mangle]
pub extern "C" fn FundingLocked_get_next_per_commitment_point(this_ptr: &FundingLocked) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.next_per_commitment_point;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
#[no_mangle]
pub extern "C" fn FundingLocked_set_next_per_commitment_point(this_ptr: &mut FundingLocked, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnFundingLocked) }.next_per_commitment_point = val.into_rust();
}
#[no_mangle]
pub extern "C" fn FundingLocked_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut next_per_commitment_point_arg: crate::c_types::PublicKey) -> FundingLocked {
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
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnShutdown,
}

impl Drop for Shutdown {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnShutdown) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Shutdown_free(this_ptr: Shutdown) { }

use lightning::ln::msgs::ClosingSigned as lnClosingSignedImport;
type lnClosingSigned = lnClosingSignedImport;

/// " A closing_signed message to be sent or received from a peer"
#[repr(C)]
pub struct ClosingSigned {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnClosingSigned,
}

impl Drop for ClosingSigned {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnClosingSigned) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ClosingSigned_free(this_ptr: ClosingSigned) { }

use lightning::ln::msgs::UpdateAddHTLC as lnUpdateAddHTLCImport;
type lnUpdateAddHTLC = lnUpdateAddHTLCImport;

/// " An update_add_htlc message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateAddHTLC {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateAddHTLC,
}

impl Drop for UpdateAddHTLC {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateAddHTLC) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_free(this_ptr: UpdateAddHTLC) { }

use lightning::ln::msgs::UpdateFulfillHTLC as lnUpdateFulfillHTLCImport;
type lnUpdateFulfillHTLC = lnUpdateFulfillHTLCImport;

/// " An update_fulfill_htlc message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateFulfillHTLC {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateFulfillHTLC,
}

impl Drop for UpdateFulfillHTLC {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateFulfillHTLC) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_free(this_ptr: UpdateFulfillHTLC) { }

use lightning::ln::msgs::UpdateFailHTLC as lnUpdateFailHTLCImport;
type lnUpdateFailHTLC = lnUpdateFailHTLCImport;

/// " An update_fail_htlc message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateFailHTLC {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateFailHTLC,
}

impl Drop for UpdateFailHTLC {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateFailHTLC) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_free(this_ptr: UpdateFailHTLC) { }

use lightning::ln::msgs::UpdateFailMalformedHTLC as lnUpdateFailMalformedHTLCImport;
type lnUpdateFailMalformedHTLC = lnUpdateFailMalformedHTLCImport;

/// " An update_fail_malformed_htlc message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateFailMalformedHTLC {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateFailMalformedHTLC,
}

impl Drop for UpdateFailMalformedHTLC {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateFailMalformedHTLC) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_free(this_ptr: UpdateFailMalformedHTLC) { }

use lightning::ln::msgs::CommitmentSigned as lnCommitmentSignedImport;
type lnCommitmentSigned = lnCommitmentSignedImport;

/// " A commitment_signed message to be sent or received from a peer"
#[repr(C)]
pub struct CommitmentSigned {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnCommitmentSigned,
}

impl Drop for CommitmentSigned {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnCommitmentSigned) };
		}
	}
}
#[no_mangle]
pub extern "C" fn CommitmentSigned_free(this_ptr: CommitmentSigned) { }

use lightning::ln::msgs::RevokeAndACK as lnRevokeAndACKImport;
type lnRevokeAndACK = lnRevokeAndACKImport;

/// " A revoke_and_ack message to be sent or received from a peer"
#[repr(C)]
pub struct RevokeAndACK {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRevokeAndACK,
}

impl Drop for RevokeAndACK {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnRevokeAndACK) };
		}
	}
}
#[no_mangle]
pub extern "C" fn RevokeAndACK_free(this_ptr: RevokeAndACK) { }

use lightning::ln::msgs::UpdateFee as lnUpdateFeeImport;
type lnUpdateFee = lnUpdateFeeImport;

/// " An update_fee message to be sent or received from a peer"
#[repr(C)]
pub struct UpdateFee {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateFee,
}

impl Drop for UpdateFee {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateFee) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateFee_free(this_ptr: UpdateFee) { }

use lightning::ln::msgs::ChannelReestablish as lnChannelReestablishImport;
type lnChannelReestablish = lnChannelReestablishImport;

/// " A channel_reestablish message to be sent or received from a peer"
#[repr(C)]
pub struct ChannelReestablish {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelReestablish,
}

impl Drop for ChannelReestablish {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelReestablish) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelReestablish_free(this_ptr: ChannelReestablish) { }

use lightning::ln::msgs::AnnouncementSignatures as lnAnnouncementSignaturesImport;
type lnAnnouncementSignatures = lnAnnouncementSignaturesImport;

/// " An announcement_signatures message to be sent or received from a peer"
#[repr(C)]
pub struct AnnouncementSignatures {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnAnnouncementSignatures,
}

impl Drop for AnnouncementSignatures {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnAnnouncementSignatures) };
		}
	}
}
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_free(this_ptr: AnnouncementSignatures) { }

use lightning::ln::msgs::NetAddress as lnNetAddressImport;
type lnNetAddress = lnNetAddressImport;

/// " An address which can be used to connect to a remote peer"
#[repr(C)]
pub struct NetAddress {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnNetAddress,
}

impl Drop for NetAddress {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnNetAddress) };
		}
	}
}
#[no_mangle]
pub extern "C" fn NetAddress_free(this_ptr: NetAddress) { }

use lightning::ln::msgs::UnsignedNodeAnnouncement as lnUnsignedNodeAnnouncementImport;
type lnUnsignedNodeAnnouncement = lnUnsignedNodeAnnouncementImport;

/// " The unsigned part of a node_announcement"
#[repr(C)]
pub struct UnsignedNodeAnnouncement {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUnsignedNodeAnnouncement,
}

impl Drop for UnsignedNodeAnnouncement {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUnsignedNodeAnnouncement) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_free(this_ptr: UnsignedNodeAnnouncement) { }
/// " The node_id this announcement originated from (don't rebroadcast the node_announcement back"
/// " to this node)."
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_get_node_id(this_ptr: &UnsignedNodeAnnouncement) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.node_id;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The node_id this announcement originated from (don't rebroadcast the node_announcement back"
/// " to this node)."
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_set_node_id(this_ptr: &mut UnsignedNodeAnnouncement, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnUnsignedNodeAnnouncement) }.node_id = val.into_rust();
}

use lightning::ln::msgs::NodeAnnouncement as lnNodeAnnouncementImport;
type lnNodeAnnouncement = lnNodeAnnouncementImport;

/// " A node_announcement message to be sent or received from a peer"
#[repr(C)]
pub struct NodeAnnouncement {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnNodeAnnouncement,
}

impl Drop for NodeAnnouncement {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnNodeAnnouncement) };
		}
	}
}
#[no_mangle]
pub extern "C" fn NodeAnnouncement_free(this_ptr: NodeAnnouncement) { }

use lightning::ln::msgs::UnsignedChannelAnnouncement as lnUnsignedChannelAnnouncementImport;
type lnUnsignedChannelAnnouncement = lnUnsignedChannelAnnouncementImport;

/// " The unsigned part of a channel_announcement"
#[repr(C)]
pub struct UnsignedChannelAnnouncement {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUnsignedChannelAnnouncement,
}

impl Drop for UnsignedChannelAnnouncement {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUnsignedChannelAnnouncement) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_free(this_ptr: UnsignedChannelAnnouncement) { }
/// " One of the two node_ids which are endpoints of this channel"
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_node_id_1(this_ptr: &UnsignedChannelAnnouncement) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.node_id_1;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " One of the two node_ids which are endpoints of this channel"
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_node_id_1(this_ptr: &mut UnsignedChannelAnnouncement, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnUnsignedChannelAnnouncement) }.node_id_1 = val.into_rust();
}
/// " The other of the two node_ids which are endpoints of this channel"
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_get_node_id_2(this_ptr: &UnsignedChannelAnnouncement) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.node_id_2;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The other of the two node_ids which are endpoints of this channel"
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_set_node_id_2(this_ptr: &mut UnsignedChannelAnnouncement, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnUnsignedChannelAnnouncement) }.node_id_2 = val.into_rust();
}

use lightning::ln::msgs::ChannelAnnouncement as lnChannelAnnouncementImport;
type lnChannelAnnouncement = lnChannelAnnouncementImport;

/// " A channel_announcement message to be sent or received from a peer"
#[repr(C)]
pub struct ChannelAnnouncement {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelAnnouncement,
}

impl Drop for ChannelAnnouncement {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelAnnouncement) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_free(this_ptr: ChannelAnnouncement) { }

use lightning::ln::msgs::ChannelUpdate as lnChannelUpdateImport;
type lnChannelUpdate = lnChannelUpdateImport;

/// " A channel_update message to be sent or received from a peer"
#[repr(C)]
pub struct ChannelUpdate {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelUpdate,
}

impl Drop for ChannelUpdate {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelUpdate) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelUpdate_free(this_ptr: ChannelUpdate) { }

use lightning::ln::msgs::LightningError as lnLightningErrorImport;
type lnLightningError = lnLightningErrorImport;

/// " An Err type for failure to process messages."
#[repr(C)]
pub struct LightningError {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnLightningError,
}

impl Drop for LightningError {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnLightningError) };
		}
	}
}
#[no_mangle]
pub extern "C" fn LightningError_free(this_ptr: LightningError) { }

use lightning::ln::msgs::CommitmentUpdate as lnCommitmentUpdateImport;
type lnCommitmentUpdate = lnCommitmentUpdateImport;

/// " Struct used to return values from revoke_and_ack messages, containing a bunch of commitment"
/// " transaction updates if they were pending."
#[repr(C)]
pub struct CommitmentUpdate {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnCommitmentUpdate,
}

impl Drop for CommitmentUpdate {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnCommitmentUpdate) };
		}
	}
}
#[no_mangle]
pub extern "C" fn CommitmentUpdate_free(this_ptr: CommitmentUpdate) { }
/// " update_add_htlc messages which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_add_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::CVecUpdateAddHTLC) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push(*unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) }); };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_add_htlcs = local_val;
}
/// " update_fulfill_htlc messages which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fulfill_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::CVecUpdateFulfillHTLC) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push(*unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) }); };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_fulfill_htlcs = local_val;
}
/// " update_fail_htlc messages which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fail_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::CVecUpdateFailHTLC) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push(*unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) }); };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_fail_htlcs = local_val;
}
/// " update_fail_malformed_htlc messages which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fail_malformed_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::CVecUpdateFailMalformedHTLC) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push(*unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) }); };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_fail_malformed_htlcs = local_val;
}
/// " An update_fee message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_update_fee(this_ptr: &CommitmentUpdate) -> *const UpdateFee {
	let inner_val = &unsafe { &*this_ptr.inner }.update_fee;
	let mut local_inner_val = if inner_val.is_none() { return std::ptr::null(); } else { Box::into_raw(Box::new(crate::ln::msgs::UpdateFee { inner: &(*inner_val.as_ref().unwrap()) } )) };
	local_inner_val
}
/// " An update_fee message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fee(this_ptr: &mut CommitmentUpdate, mut val: UpdateFee) {
	let mut local_val = if val.inner.is_null() { None } else { Some(*unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) }) };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_fee = local_val;
}
/// " Finally, the commitment_signed message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_commitment_signed(this_ptr: &CommitmentUpdate) -> *const CommitmentSigned {
	let inner_val = &unsafe { &*this_ptr.inner }.commitment_signed;
	Box::into_raw(Box::new(crate::ln::msgs::CommitmentSigned { inner: &(*inner_val) } ))
}
/// " Finally, the commitment_signed message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_commitment_signed(this_ptr: &mut CommitmentUpdate, mut val: CommitmentSigned) {
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.commitment_signed = *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) };
}
#[no_mangle]
pub extern "C" fn CommitmentUpdate_new(mut update_add_htlcs_arg: crate::c_types::CVecUpdateAddHTLC, mut update_fulfill_htlcs_arg: crate::c_types::CVecUpdateFulfillHTLC, mut update_fail_htlcs_arg: crate::c_types::CVecUpdateFailHTLC, mut update_fail_malformed_htlcs_arg: crate::c_types::CVecUpdateFailMalformedHTLC, mut update_fee_arg: UpdateFee, mut commitment_signed_arg: CommitmentSigned) -> CommitmentUpdate {
	let mut local_update_add_htlcs_arg = Vec::new(); for mut item in update_add_htlcs_arg.into_rust().drain(..) { local_update_add_htlcs_arg.push(*unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) }); };
	let mut local_update_fulfill_htlcs_arg = Vec::new(); for mut item in update_fulfill_htlcs_arg.into_rust().drain(..) { local_update_fulfill_htlcs_arg.push(*unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) }); };
	let mut local_update_fail_htlcs_arg = Vec::new(); for mut item in update_fail_htlcs_arg.into_rust().drain(..) { local_update_fail_htlcs_arg.push(*unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) }); };
	let mut local_update_fail_malformed_htlcs_arg = Vec::new(); for mut item in update_fail_malformed_htlcs_arg.into_rust().drain(..) { local_update_fail_malformed_htlcs_arg.push(*unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) }); };
	let mut local_update_fee_arg = if update_fee_arg.inner.is_null() { None } else { Some(*unsafe { Box::from_raw(update_fee_arg.inner.take_ptr() as *mut _) }) };
	CommitmentUpdate { inner: Box::into_raw(Box::new(lnCommitmentUpdate {
		update_add_htlcs: local_update_add_htlcs_arg,
		update_fulfill_htlcs: local_update_fulfill_htlcs_arg,
		update_fail_htlcs: local_update_fail_htlcs_arg,
		update_fail_malformed_htlcs: local_update_fail_malformed_htlcs_arg,
		update_fee: local_update_fee_arg,
		commitment_signed: *unsafe { Box::from_raw(commitment_signed_arg.inner.take_ptr() as *mut _) },
	}))}
}

use lightning::ln::msgs::HTLCFailChannelUpdate as lnHTLCFailChannelUpdateImport;
type lnHTLCFailChannelUpdate = lnHTLCFailChannelUpdateImport;

/// " The information we received from a peer along the route of a payment we originated. This is"
/// " returned by ChannelMessageHandler::handle_update_fail_htlc to be passed into"
/// " RoutingMessageHandler::handle_htlc_fail_channel_update to update our network map."
#[repr(C)]
pub struct HTLCFailChannelUpdate {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnHTLCFailChannelUpdate,
}

impl Drop for HTLCFailChannelUpdate {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnHTLCFailChannelUpdate) };
		}
	}
}
#[no_mangle]
pub extern "C" fn HTLCFailChannelUpdate_free(this_ptr: HTLCFailChannelUpdate) { }
/// " A trait to describe an object which can receive channel messages."
/// ""
/// " Messages MAY be called in parallel when they originate from different their_node_ids, however"
/// " they MUST NOT be called in parallel when the two calls have the same their_node_id."
#[repr(C)]
pub struct ChannelMessageHandler {
	pub this_arg: *mut c_void,
	/// " Handle an incoming open_channel message from the given peer."
	pub handle_open_channel: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, their_features: crate::ln::features::InitFeatures, msg: &OpenChannel),
	/// " Handle an incoming accept_channel message from the given peer."
	pub handle_accept_channel: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, their_features: crate::ln::features::InitFeatures, msg: &AcceptChannel),
	/// " Handle an incoming funding_created message from the given peer."
	pub handle_funding_created: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &FundingCreated),
	/// " Handle an incoming funding_signed message from the given peer."
	pub handle_funding_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &FundingSigned),
	/// " Handle an incoming funding_locked message from the given peer."
	pub handle_funding_locked: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &FundingLocked),
	/// " Handle an incoming shutdown message from the given peer."
	pub handle_shutdown: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &Shutdown),
	/// " Handle an incoming closing_signed message from the given peer."
	pub handle_closing_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &ClosingSigned),
	/// " Handle an incoming update_add_htlc message from the given peer."
	pub handle_update_add_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateAddHTLC),
	/// " Handle an incoming update_fulfill_htlc message from the given peer."
	pub handle_update_fulfill_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateFulfillHTLC),
	/// " Handle an incoming update_fail_htlc message from the given peer."
	pub handle_update_fail_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateFailHTLC),
	/// " Handle an incoming update_fail_malformed_htlc message from the given peer."
	pub handle_update_fail_malformed_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateFailMalformedHTLC),
	/// " Handle an incoming commitment_signed message from the given peer."
	pub handle_commitment_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &CommitmentSigned),
	/// " Handle an incoming revoke_and_ack message from the given peer."
	pub handle_revoke_and_ack: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &RevokeAndACK),
	/// " Handle an incoming update_fee message from the given peer."
	pub handle_update_fee: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &UpdateFee),
	/// " Handle an incoming announcement_signatures message from the given peer."
	pub handle_announcement_signatures: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &AnnouncementSignatures),
	/// " Indicates a connection to the peer failed/an existing connection was lost. If no connection"
	/// " is believed to be possible in the future (eg they're sending us messages we don't"
	/// " understand or indicate they require unknown feature bits), no_connection_possible is set"
	/// " and any outstanding channels should be failed."
	pub peer_disconnected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, no_connection_possible: bool),
	/// " Handle a peer reconnecting, possibly generating channel_reestablish message(s)."
	pub peer_connected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &Init),
	/// " Handle an incoming channel_reestablish message from the given peer."
	pub handle_channel_reestablish: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &ChannelReestablish),
	/// " Handle an incoming error message from the given peer."
	pub handle_error: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &ErrorMessage),
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
		(self.handle_open_channel)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new(their_features)) }, &crate::ln::msgs::OpenChannel { inner: msg })
	}
	fn handle_accept_channel(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, their_features: lightning::ln::features::InitFeatures, msg: &lightning::ln::msgs::AcceptChannel) {
		(self.handle_accept_channel)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new(their_features)) }, &crate::ln::msgs::AcceptChannel { inner: msg })
	}
	fn handle_funding_created(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingCreated) {
		(self.handle_funding_created)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::FundingCreated { inner: msg })
	}
	fn handle_funding_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingSigned) {
		(self.handle_funding_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::FundingSigned { inner: msg })
	}
	fn handle_funding_locked(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingLocked) {
		(self.handle_funding_locked)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::FundingLocked { inner: msg })
	}
	fn handle_shutdown(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::Shutdown) {
		(self.handle_shutdown)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::Shutdown { inner: msg })
	}
	fn handle_closing_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ClosingSigned) {
		(self.handle_closing_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::ClosingSigned { inner: msg })
	}
	fn handle_update_add_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateAddHTLC) {
		(self.handle_update_add_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateAddHTLC { inner: msg })
	}
	fn handle_update_fulfill_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFulfillHTLC) {
		(self.handle_update_fulfill_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateFulfillHTLC { inner: msg })
	}
	fn handle_update_fail_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFailHTLC) {
		(self.handle_update_fail_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateFailHTLC { inner: msg })
	}
	fn handle_update_fail_malformed_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFailMalformedHTLC) {
		(self.handle_update_fail_malformed_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateFailMalformedHTLC { inner: msg })
	}
	fn handle_commitment_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::CommitmentSigned) {
		(self.handle_commitment_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::CommitmentSigned { inner: msg })
	}
	fn handle_revoke_and_ack(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::RevokeAndACK) {
		(self.handle_revoke_and_ack)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::RevokeAndACK { inner: msg })
	}
	fn handle_update_fee(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFee) {
		(self.handle_update_fee)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateFee { inner: msg })
	}
	fn handle_announcement_signatures(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::AnnouncementSignatures) {
		(self.handle_announcement_signatures)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::AnnouncementSignatures { inner: msg })
	}
	fn peer_disconnected(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, no_connection_possible: bool) {
		(self.peer_disconnected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), no_connection_possible)
	}
	fn peer_connected(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::Init) {
		(self.peer_connected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::Init { inner: msg })
	}
	fn handle_channel_reestablish(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ChannelReestablish) {
		(self.handle_channel_reestablish)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::ChannelReestablish { inner: msg })
	}
	fn handle_error(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ErrorMessage) {
		(self.handle_error)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::ErrorMessage { inner: msg })
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
	/// " Handle an incoming node_announcement message, returning true if it should be forwarded on,"
	/// " false or returning an Err otherwise."
	pub handle_node_announcement: extern "C" fn (this_arg: *const c_void, msg: &NodeAnnouncement) -> crate::c_types::CResultboolLightningError,
	/// " Handle a channel_announcement message, returning true if it should be forwarded on, false"
	/// " or returning an Err otherwise."
	pub handle_channel_announcement: extern "C" fn (this_arg: *const c_void, msg: &ChannelAnnouncement) -> crate::c_types::CResultboolLightningError,
	/// " Handle an incoming channel_update message, returning true if it should be forwarded on,"
	/// " false or returning an Err otherwise."
	pub handle_channel_update: extern "C" fn (this_arg: *const c_void, msg: &ChannelUpdate) -> crate::c_types::CResultboolLightningError,
	/// " Handle some updates to the route graph that we learned due to an outbound failed payment."
	pub handle_htlc_fail_channel_update: extern "C" fn (this_arg: *const c_void, update: & HTLCFailChannelUpdate),
	//XXX: Need to export get_next_channel_announcements
	//XXX: Need to export get_next_node_announcements
	/// " Returns whether a full sync should be requested from a peer."
	pub should_request_full_sync: extern "C" fn (this_arg: *const c_void, node_id: crate::c_types::PublicKey) -> bool,
}
unsafe impl Send for RoutingMessageHandler {}
unsafe impl Sync for RoutingMessageHandler {}

use lightning::ln::msgs::RoutingMessageHandler as lnRoutingMessageHandler;
impl lnRoutingMessageHandler for RoutingMessageHandler {
	fn handle_node_announcement(&self, msg: &lightning::ln::msgs::NodeAnnouncement) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_node_announcement)(self.this_arg, &crate::ln::msgs::NodeAnnouncement { inner: msg });
		let mut local_ret = match ret.result_good { true => Ok((*unsafe { &mut *ret.contents.result })), false => Err(*unsafe { Box::from_raw((*unsafe { &mut *ret.contents.err }).inner.take_ptr() as *mut _) })};
		local_ret
	}
	fn handle_channel_announcement(&self, msg: &lightning::ln::msgs::ChannelAnnouncement) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_channel_announcement)(self.this_arg, &crate::ln::msgs::ChannelAnnouncement { inner: msg });
		let mut local_ret = match ret.result_good { true => Ok((*unsafe { &mut *ret.contents.result })), false => Err(*unsafe { Box::from_raw((*unsafe { &mut *ret.contents.err }).inner.take_ptr() as *mut _) })};
		local_ret
	}
	fn handle_channel_update(&self, msg: &lightning::ln::msgs::ChannelUpdate) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_channel_update)(self.this_arg, &crate::ln::msgs::ChannelUpdate { inner: msg });
		let mut local_ret = match ret.result_good { true => Ok((*unsafe { &mut *ret.contents.result })), false => Err(*unsafe { Box::from_raw((*unsafe { &mut *ret.contents.err }).inner.take_ptr() as *mut _) })};
		local_ret
	}
	fn handle_htlc_fail_channel_update(&self, update: &lightning::ln::msgs::HTLCFailChannelUpdate) {
		(self.handle_htlc_fail_channel_update)(self.this_arg, &crate::ln::msgs::HTLCFailChannelUpdate { inner: update })
	}
	fn get_next_channel_announcements(&self, starting_point: u64, batch_amount: u8) -> Vec<(lightning::ln::msgs::ChannelAnnouncement, Option<lightning::ln::msgs::ChannelUpdate>, Option<lightning::ln::msgs::ChannelUpdate>)> {
		unimplemented!();
	}
	fn get_next_node_announcements(&self, starting_point: Option<&bitcoin::secp256k1::key::PublicKey>, batch_amount: u8) -> Vec<lightning::ln::msgs::NodeAnnouncement> {
		unimplemented!();
	}
	fn should_request_full_sync(&self, node_id: &bitcoin::secp256k1::key::PublicKey) -> bool {
		let mut ret = (self.should_request_full_sync)(self.this_arg, crate::c_types::PublicKey::from_rust(&node_id));
		ret
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
