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
use crate::c_types::*;


use lightning::ln::msgs::DecodeError as lnDecodeErrorImport;
type lnDecodeError = lnDecodeErrorImport;

/// " An error in decoding a message or struct."
#[must_use]
#[repr(C)]
pub struct DecodeError {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnDecodeError,
	pub _underlying_ref: bool,
}

impl Drop for DecodeError {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnDecodeError) };
		}
	}
}
#[no_mangle]
pub extern "C" fn DecodeError_free(this_ptr: DecodeError) { }

use lightning::ln::msgs::Init as lnInitImport;
type lnInit = lnInitImport;

/// " An init message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct Init {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnInit,
	pub _underlying_ref: bool,
}

impl Drop for Init {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnInit) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Init_free(this_ptr: Init) { }

use lightning::ln::msgs::ErrorMessage as lnErrorMessageImport;
type lnErrorMessage = lnErrorMessageImport;

/// " An error message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct ErrorMessage {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnErrorMessage,
	pub _underlying_ref: bool,
}

impl Drop for ErrorMessage {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnErrorMessage) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ErrorMessage_free(this_ptr: ErrorMessage) { }
impl Clone for ErrorMessage {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::Ping as lnPingImport;
type lnPing = lnPingImport;

/// " A ping message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct Ping {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnPing,
	pub _underlying_ref: bool,
}

impl Drop for Ping {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnPing) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Ping_free(this_ptr: Ping) { }

use lightning::ln::msgs::Pong as lnPongImport;
type lnPong = lnPongImport;

/// " A pong message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct Pong {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnPong,
	pub _underlying_ref: bool,
}

impl Drop for Pong {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnPong) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Pong_free(this_ptr: Pong) { }

use lightning::ln::msgs::OpenChannel as lnOpenChannelImport;
type lnOpenChannel = lnOpenChannelImport;

/// " An open_channel message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct OpenChannel {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnOpenChannel,
	pub _underlying_ref: bool,
}

impl Drop for OpenChannel {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnOpenChannel) };
		}
	}
}
#[no_mangle]
pub extern "C" fn OpenChannel_free(this_ptr: OpenChannel) { }
impl Clone for OpenChannel {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::AcceptChannel as lnAcceptChannelImport;
type lnAcceptChannel = lnAcceptChannelImport;

/// " An accept_channel message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct AcceptChannel {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnAcceptChannel,
	pub _underlying_ref: bool,
}

impl Drop for AcceptChannel {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnAcceptChannel) };
		}
	}
}
#[no_mangle]
pub extern "C" fn AcceptChannel_free(this_ptr: AcceptChannel) { }
impl Clone for AcceptChannel {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::FundingCreated as lnFundingCreatedImport;
type lnFundingCreated = lnFundingCreatedImport;

/// " A funding_created message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct FundingCreated {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnFundingCreated,
	pub _underlying_ref: bool,
}

impl Drop for FundingCreated {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnFundingCreated) };
		}
	}
}
#[no_mangle]
pub extern "C" fn FundingCreated_free(this_ptr: FundingCreated) { }
impl Clone for FundingCreated {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::FundingSigned as lnFundingSignedImport;
type lnFundingSigned = lnFundingSignedImport;

/// " A funding_signed message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct FundingSigned {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnFundingSigned,
	pub _underlying_ref: bool,
}

impl Drop for FundingSigned {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnFundingSigned) };
		}
	}
}
#[no_mangle]
pub extern "C" fn FundingSigned_free(this_ptr: FundingSigned) { }
impl Clone for FundingSigned {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::FundingLocked as lnFundingLockedImport;
type lnFundingLocked = lnFundingLockedImport;

/// " A funding_locked message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct FundingLocked {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnFundingLocked,
	pub _underlying_ref: bool,
}

impl Drop for FundingLocked {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnFundingLocked) };
		}
	}
}
#[no_mangle]
pub extern "C" fn FundingLocked_free(this_ptr: FundingLocked) { }
impl Clone for FundingLocked {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
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
#[must_use]
#[no_mangle]
pub extern "C" fn FundingLocked_new(mut channel_id_arg: crate::c_types::ThirtyTwoBytes, mut next_per_commitment_point_arg: crate::c_types::PublicKey) -> FundingLocked {
	FundingLocked { inner: Box::into_raw(Box::new(lnFundingLocked {
		channel_id: channel_id_arg.data,
		next_per_commitment_point: next_per_commitment_point_arg.into_rust(),
	})), _underlying_ref: false }
}

use lightning::ln::msgs::Shutdown as lnShutdownImport;
type lnShutdown = lnShutdownImport;

/// " A shutdown message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct Shutdown {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnShutdown,
	pub _underlying_ref: bool,
}

impl Drop for Shutdown {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnShutdown) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Shutdown_free(this_ptr: Shutdown) { }
impl Clone for Shutdown {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::ClosingSigned as lnClosingSignedImport;
type lnClosingSigned = lnClosingSignedImport;

/// " A closing_signed message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct ClosingSigned {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnClosingSigned,
	pub _underlying_ref: bool,
}

impl Drop for ClosingSigned {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnClosingSigned) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ClosingSigned_free(this_ptr: ClosingSigned) { }
impl Clone for ClosingSigned {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::UpdateAddHTLC as lnUpdateAddHTLCImport;
type lnUpdateAddHTLC = lnUpdateAddHTLCImport;

/// " An update_add_htlc message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct UpdateAddHTLC {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateAddHTLC,
	pub _underlying_ref: bool,
}

impl Drop for UpdateAddHTLC {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateAddHTLC) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_free(this_ptr: UpdateAddHTLC) { }
impl Clone for UpdateAddHTLC {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::UpdateFulfillHTLC as lnUpdateFulfillHTLCImport;
type lnUpdateFulfillHTLC = lnUpdateFulfillHTLCImport;

/// " An update_fulfill_htlc message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct UpdateFulfillHTLC {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateFulfillHTLC,
	pub _underlying_ref: bool,
}

impl Drop for UpdateFulfillHTLC {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateFulfillHTLC) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_free(this_ptr: UpdateFulfillHTLC) { }
impl Clone for UpdateFulfillHTLC {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::UpdateFailHTLC as lnUpdateFailHTLCImport;
type lnUpdateFailHTLC = lnUpdateFailHTLCImport;

/// " An update_fail_htlc message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct UpdateFailHTLC {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateFailHTLC,
	pub _underlying_ref: bool,
}

impl Drop for UpdateFailHTLC {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateFailHTLC) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_free(this_ptr: UpdateFailHTLC) { }
impl Clone for UpdateFailHTLC {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::UpdateFailMalformedHTLC as lnUpdateFailMalformedHTLCImport;
type lnUpdateFailMalformedHTLC = lnUpdateFailMalformedHTLCImport;

/// " An update_fail_malformed_htlc message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct UpdateFailMalformedHTLC {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateFailMalformedHTLC,
	pub _underlying_ref: bool,
}

impl Drop for UpdateFailMalformedHTLC {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateFailMalformedHTLC) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_free(this_ptr: UpdateFailMalformedHTLC) { }
impl Clone for UpdateFailMalformedHTLC {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::CommitmentSigned as lnCommitmentSignedImport;
type lnCommitmentSigned = lnCommitmentSignedImport;

/// " A commitment_signed message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct CommitmentSigned {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnCommitmentSigned,
	pub _underlying_ref: bool,
}

impl Drop for CommitmentSigned {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnCommitmentSigned) };
		}
	}
}
#[no_mangle]
pub extern "C" fn CommitmentSigned_free(this_ptr: CommitmentSigned) { }
impl Clone for CommitmentSigned {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::RevokeAndACK as lnRevokeAndACKImport;
type lnRevokeAndACK = lnRevokeAndACKImport;

/// " A revoke_and_ack message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct RevokeAndACK {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRevokeAndACK,
	pub _underlying_ref: bool,
}

impl Drop for RevokeAndACK {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnRevokeAndACK) };
		}
	}
}
#[no_mangle]
pub extern "C" fn RevokeAndACK_free(this_ptr: RevokeAndACK) { }
impl Clone for RevokeAndACK {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::UpdateFee as lnUpdateFeeImport;
type lnUpdateFee = lnUpdateFeeImport;

/// " An update_fee message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct UpdateFee {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUpdateFee,
	pub _underlying_ref: bool,
}

impl Drop for UpdateFee {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUpdateFee) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UpdateFee_free(this_ptr: UpdateFee) { }
impl Clone for UpdateFee {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::ChannelReestablish as lnChannelReestablishImport;
type lnChannelReestablish = lnChannelReestablishImport;

/// " A channel_reestablish message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct ChannelReestablish {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelReestablish,
	pub _underlying_ref: bool,
}

impl Drop for ChannelReestablish {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelReestablish) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelReestablish_free(this_ptr: ChannelReestablish) { }
impl Clone for ChannelReestablish {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::AnnouncementSignatures as lnAnnouncementSignaturesImport;
type lnAnnouncementSignatures = lnAnnouncementSignaturesImport;

/// " An announcement_signatures message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct AnnouncementSignatures {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnAnnouncementSignatures,
	pub _underlying_ref: bool,
}

impl Drop for AnnouncementSignatures {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnAnnouncementSignatures) };
		}
	}
}
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_free(this_ptr: AnnouncementSignatures) { }
impl Clone for AnnouncementSignatures {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
/// " An address which can be used to connect to a remote peer"
#[must_use]
#[derive(Clone)]
#[repr(C)]
pub enum NetAddress {
	/// " An IPv4 address/port on which the peer is listening."
	IPv4 {
		addr: crate::c_types::FourBytes,
		port: u16,
	},
	/// " An IPv6 address/port on which the peer is listening."
	IPv6 {
		addr: crate::c_types::SixteenBytes,
		port: u16,
	},
	/// " An old-style Tor onion address/port on which the peer is listening."
	OnionV2 {
		addr: crate::c_types::TenBytes,
		port: u16,
	},
	/// " A new-style Tor onion address/port on which the peer is listening."
	/// " To create the human-readable \"hostname\", concatenate ed25519_pubkey, checksum, and version,"
	/// " wrap as base32 and append \".onion\"."
	OnionV3 {
		ed25519_pubkey: crate::c_types::ThirtyTwoBytes,
		checksum: u16,
		version: u8,
		port: u16,
	},
}
use lightning::ln::msgs::NetAddress as lnNetAddress;
impl NetAddress {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnNetAddress {
		match self {
			NetAddress::IPv4 {ref addr, ref port, } => {
				let mut addr_nonref = (*addr).clone();
				let mut port_nonref = (*port).clone();
				lnNetAddress::IPv4 {
					addr: addr_nonref.data,
					port: port_nonref,
				}
			},
			NetAddress::IPv6 {ref addr, ref port, } => {
				let mut addr_nonref = (*addr).clone();
				let mut port_nonref = (*port).clone();
				lnNetAddress::IPv6 {
					addr: addr_nonref.data,
					port: port_nonref,
				}
			},
			NetAddress::OnionV2 {ref addr, ref port, } => {
				let mut addr_nonref = (*addr).clone();
				let mut port_nonref = (*port).clone();
				lnNetAddress::OnionV2 {
					addr: addr_nonref.data,
					port: port_nonref,
				}
			},
			NetAddress::OnionV3 {ref ed25519_pubkey, ref checksum, ref version, ref port, } => {
				let mut ed25519_pubkey_nonref = (*ed25519_pubkey).clone();
				let mut checksum_nonref = (*checksum).clone();
				let mut version_nonref = (*version).clone();
				let mut port_nonref = (*port).clone();
				lnNetAddress::OnionV3 {
					ed25519_pubkey: ed25519_pubkey_nonref.data,
					checksum: checksum_nonref,
					version: version_nonref,
					port: port_nonref,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_ln(self) -> lnNetAddress {
		match self {
			NetAddress::IPv4 {mut addr, mut port, } => {
				lnNetAddress::IPv4 {
					addr: addr.data,
					port: port,
				}
			},
			NetAddress::IPv6 {mut addr, mut port, } => {
				lnNetAddress::IPv6 {
					addr: addr.data,
					port: port,
				}
			},
			NetAddress::OnionV2 {mut addr, mut port, } => {
				lnNetAddress::OnionV2 {
					addr: addr.data,
					port: port,
				}
			},
			NetAddress::OnionV3 {mut ed25519_pubkey, mut checksum, mut version, mut port, } => {
				lnNetAddress::OnionV3 {
					ed25519_pubkey: ed25519_pubkey.data,
					checksum: checksum,
					version: version,
					port: port,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: &lnNetAddress) -> Self {
		match lnt {
			lnNetAddress::IPv4 {ref addr, ref port, } => {
				let mut addr_nonref = (*addr).clone();
				let mut port_nonref = (*port).clone();
				NetAddress::IPv4 {
					addr: crate::c_types::FourBytes { data: addr_nonref },
					port: port_nonref,
				}
			},
			lnNetAddress::IPv6 {ref addr, ref port, } => {
				let mut addr_nonref = (*addr).clone();
				let mut port_nonref = (*port).clone();
				NetAddress::IPv6 {
					addr: crate::c_types::SixteenBytes { data: addr_nonref },
					port: port_nonref,
				}
			},
			lnNetAddress::OnionV2 {ref addr, ref port, } => {
				let mut addr_nonref = (*addr).clone();
				let mut port_nonref = (*port).clone();
				NetAddress::OnionV2 {
					addr: crate::c_types::TenBytes { data: addr_nonref },
					port: port_nonref,
				}
			},
			lnNetAddress::OnionV3 {ref ed25519_pubkey, ref checksum, ref version, ref port, } => {
				let mut ed25519_pubkey_nonref = (*ed25519_pubkey).clone();
				let mut checksum_nonref = (*checksum).clone();
				let mut version_nonref = (*version).clone();
				let mut port_nonref = (*port).clone();
				NetAddress::OnionV3 {
					ed25519_pubkey: crate::c_types::ThirtyTwoBytes { data: ed25519_pubkey_nonref },
					checksum: checksum_nonref,
					version: version_nonref,
					port: port_nonref,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn ln_into(lnt: lnNetAddress) -> Self {
		match lnt {
			lnNetAddress::IPv4 {mut addr, mut port, } => {
				NetAddress::IPv4 {
					addr: crate::c_types::FourBytes { data: addr },
					port: port,
				}
			},
			lnNetAddress::IPv6 {mut addr, mut port, } => {
				NetAddress::IPv6 {
					addr: crate::c_types::SixteenBytes { data: addr },
					port: port,
				}
			},
			lnNetAddress::OnionV2 {mut addr, mut port, } => {
				NetAddress::OnionV2 {
					addr: crate::c_types::TenBytes { data: addr },
					port: port,
				}
			},
			lnNetAddress::OnionV3 {mut ed25519_pubkey, mut checksum, mut version, mut port, } => {
				NetAddress::OnionV3 {
					ed25519_pubkey: crate::c_types::ThirtyTwoBytes { data: ed25519_pubkey },
					checksum: checksum,
					version: version,
					port: port,
				}
			},
		}
	}
}
#[no_mangle]
pub extern "C" fn NetAddress_free(this_ptr: NetAddress) { }

use lightning::ln::msgs::UnsignedNodeAnnouncement as lnUnsignedNodeAnnouncementImport;
type lnUnsignedNodeAnnouncement = lnUnsignedNodeAnnouncementImport;

/// " The unsigned part of a node_announcement"
#[must_use]
#[repr(C)]
pub struct UnsignedNodeAnnouncement {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUnsignedNodeAnnouncement,
	pub _underlying_ref: bool,
}

impl Drop for UnsignedNodeAnnouncement {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUnsignedNodeAnnouncement) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_free(this_ptr: UnsignedNodeAnnouncement) { }
impl Clone for UnsignedNodeAnnouncement {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
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
#[must_use]
#[repr(C)]
pub struct NodeAnnouncement {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnNodeAnnouncement,
	pub _underlying_ref: bool,
}

impl Drop for NodeAnnouncement {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnNodeAnnouncement) };
		}
	}
}
#[no_mangle]
pub extern "C" fn NodeAnnouncement_free(this_ptr: NodeAnnouncement) { }
impl Clone for NodeAnnouncement {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::UnsignedChannelAnnouncement as lnUnsignedChannelAnnouncementImport;
type lnUnsignedChannelAnnouncement = lnUnsignedChannelAnnouncementImport;

/// " The unsigned part of a channel_announcement"
#[must_use]
#[repr(C)]
pub struct UnsignedChannelAnnouncement {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUnsignedChannelAnnouncement,
	pub _underlying_ref: bool,
}

impl Drop for UnsignedChannelAnnouncement {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnUnsignedChannelAnnouncement) };
		}
	}
}
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_free(this_ptr: UnsignedChannelAnnouncement) { }
impl Clone for UnsignedChannelAnnouncement {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
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
#[must_use]
#[repr(C)]
pub struct ChannelAnnouncement {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelAnnouncement,
	pub _underlying_ref: bool,
}

impl Drop for ChannelAnnouncement {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelAnnouncement) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_free(this_ptr: ChannelAnnouncement) { }
impl Clone for ChannelAnnouncement {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}

use lightning::ln::msgs::ChannelUpdate as lnChannelUpdateImport;
type lnChannelUpdate = lnChannelUpdateImport;

/// " A channel_update message to be sent or received from a peer"
#[must_use]
#[repr(C)]
pub struct ChannelUpdate {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelUpdate,
	pub _underlying_ref: bool,
}

impl Drop for ChannelUpdate {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelUpdate) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelUpdate_free(this_ptr: ChannelUpdate) { }
impl Clone for ChannelUpdate {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
/// " Used to put an error message in a LightningError"
#[must_use]
#[derive(Clone)]
#[repr(C)]
pub enum ErrorAction {
	/// " The peer took some action which made us think they were useless. Disconnect them."
	DisconnectPeer {
		msg: crate::ln::msgs::ErrorMessage,
	},
	/// " The peer did something harmless that we weren't able to process, just log and ignore"
	IgnoreError,
	/// " The peer did something incorrect. Tell them."
	SendErrorMessage {
		msg: crate::ln::msgs::ErrorMessage,
	},
}
use lightning::ln::msgs::ErrorAction as lnErrorAction;
impl ErrorAction {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnErrorAction {
		match self {
			ErrorAction::DisconnectPeer {ref msg, } => {
				let mut msg_nonref = (*msg).clone();let mut local_msg_nonref = if msg_nonref.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) } }) };
				lnErrorAction::DisconnectPeer {
					msg: local_msg_nonref,
				}
			},
			ErrorAction::IgnoreError => lnErrorAction::IgnoreError,
			ErrorAction::SendErrorMessage {ref msg, } => {
				let mut msg_nonref = (*msg).clone();
				lnErrorAction::SendErrorMessage {
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_ln(self) -> lnErrorAction {
		match self {
			ErrorAction::DisconnectPeer {mut msg, } => {
				let mut local_msg = if msg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) } }) };
				lnErrorAction::DisconnectPeer {
					msg: local_msg,
				}
			},
			ErrorAction::IgnoreError => lnErrorAction::IgnoreError,
			ErrorAction::SendErrorMessage {mut msg, } => {
				lnErrorAction::SendErrorMessage {
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: &lnErrorAction) -> Self {
		match lnt {
			lnErrorAction::DisconnectPeer {ref msg, } => {
				let mut msg_nonref = (*msg).clone();let mut local_msg_nonref = crate::ln::msgs::ErrorMessage { inner: if msg_nonref.is_none() { std::ptr::null_mut() } else { Box::into_raw(Box::new( { (msg_nonref.unwrap()) })) }, _underlying_ref: false };
				ErrorAction::DisconnectPeer {
					msg: local_msg_nonref,
				}
			},
			lnErrorAction::IgnoreError => ErrorAction::IgnoreError,
			lnErrorAction::SendErrorMessage {ref msg, } => {
				let mut msg_nonref = (*msg).clone();
				ErrorAction::SendErrorMessage {
					msg: crate::ln::msgs::ErrorMessage { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn ln_into(lnt: lnErrorAction) -> Self {
		match lnt {
			lnErrorAction::DisconnectPeer {mut msg, } => {
				let mut local_msg = crate::ln::msgs::ErrorMessage { inner: if msg.is_none() { std::ptr::null_mut() } else { Box::into_raw(Box::new( { (msg.unwrap()) })) }, _underlying_ref: false };
				ErrorAction::DisconnectPeer {
					msg: local_msg,
				}
			},
			lnErrorAction::IgnoreError => ErrorAction::IgnoreError,
			lnErrorAction::SendErrorMessage {mut msg, } => {
				ErrorAction::SendErrorMessage {
					msg: crate::ln::msgs::ErrorMessage { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
		}
	}
}
#[no_mangle]
pub extern "C" fn ErrorAction_free(this_ptr: ErrorAction) { }

use lightning::ln::msgs::LightningError as lnLightningErrorImport;
type lnLightningError = lnLightningErrorImport;

/// " An Err type for failure to process messages."
#[must_use]
#[repr(C)]
pub struct LightningError {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnLightningError,
	pub _underlying_ref: bool,
}

impl Drop for LightningError {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnLightningError) };
		}
	}
}
#[no_mangle]
pub extern "C" fn LightningError_free(this_ptr: LightningError) { }
/// " A human-readable message describing the error"
#[no_mangle]
pub extern "C" fn LightningError_get_err(this_ptr: &LightningError) -> crate::c_types::Str {
	let inner_val = &unsafe { &*this_ptr.inner }.err;
	(*inner_val).into()
}
/// " A human-readable message describing the error"
#[no_mangle]
pub extern "C" fn LightningError_set_err(this_ptr: &mut LightningError, mut val: crate::c_types::Str) {
	unsafe { &mut *(this_ptr.inner as *mut lnLightningError) }.err = val.into();
}
/// " The action which should be taken against the offending peer."
#[no_mangle]
pub extern "C" fn LightningError_get_action(this_ptr: &LightningError) -> *const crate::ln::msgs::ErrorAction {
	let inner_val = &unsafe { &*this_ptr.inner }.action;
	&crate::ln::msgs::ErrorAction::from_ln(&(*inner_val))
}
/// " The action which should be taken against the offending peer."
#[no_mangle]
pub extern "C" fn LightningError_set_action(this_ptr: &mut LightningError, mut val: crate::ln::msgs::ErrorAction) {
	unsafe { &mut *(this_ptr.inner as *mut lnLightningError) }.action = val.into_ln();
}
#[must_use]
#[no_mangle]
pub extern "C" fn LightningError_new(mut err_arg: crate::c_types::Str, mut action_arg: crate::ln::msgs::ErrorAction) -> LightningError {
	LightningError { inner: Box::into_raw(Box::new(lnLightningError {
		err: err_arg.into(),
		action: action_arg.into_ln(),
	})), _underlying_ref: false }
}

use lightning::ln::msgs::CommitmentUpdate as lnCommitmentUpdateImport;
type lnCommitmentUpdate = lnCommitmentUpdateImport;

/// " Struct used to return values from revoke_and_ack messages, containing a bunch of commitment"
/// " transaction updates if they were pending."
#[must_use]
#[repr(C)]
pub struct CommitmentUpdate {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnCommitmentUpdate,
	pub _underlying_ref: bool,
}

impl Drop for CommitmentUpdate {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnCommitmentUpdate) };
		}
	}
}
#[no_mangle]
pub extern "C" fn CommitmentUpdate_free(this_ptr: CommitmentUpdate) { }
impl Clone for CommitmentUpdate {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
/// " update_add_htlc messages which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_add_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::derived::CVec_UpdateAddHTLCZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_add_htlcs = local_val;
}
/// " update_fulfill_htlc messages which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fulfill_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::derived::CVec_UpdateFulfillHTLCZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_fulfill_htlcs = local_val;
}
/// " update_fail_htlc messages which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fail_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::derived::CVec_UpdateFailHTLCZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_fail_htlcs = local_val;
}
/// " update_fail_malformed_htlc messages which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fail_malformed_htlcs(this_ptr: &mut CommitmentUpdate, mut val: crate::c_types::derived::CVec_UpdateFailMalformedHTLCZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_fail_malformed_htlcs = local_val;
}
/// " An update_fee message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_update_fee(this_ptr: &CommitmentUpdate) -> *const crate::ln::msgs::UpdateFee {
	let inner_val = &unsafe { &*this_ptr.inner }.update_fee;
	let mut local_inner_val = &crate::ln::msgs::UpdateFee { inner: if inner_val.is_none() { std::ptr::null() } else {  { (inner_val.as_ref().unwrap()) } }, _underlying_ref: true };
	local_inner_val
}
/// " An update_fee message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_update_fee(this_ptr: &mut CommitmentUpdate, mut val: crate::ln::msgs::UpdateFee) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) } }) };
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.update_fee = local_val;
}
/// " Finally, the commitment_signed message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_get_commitment_signed(this_ptr: &CommitmentUpdate) -> *const crate::ln::msgs::CommitmentSigned {
	let inner_val = &unsafe { &*this_ptr.inner }.commitment_signed;
	Box::into_raw(Box::new(crate::ln::msgs::CommitmentSigned { inner: &(*inner_val), _underlying_ref: true } ))
}
/// " Finally, the commitment_signed message which should be sent"
#[no_mangle]
pub extern "C" fn CommitmentUpdate_set_commitment_signed(this_ptr: &mut CommitmentUpdate, mut val: crate::ln::msgs::CommitmentSigned) {
	unsafe { &mut *(this_ptr.inner as *mut lnCommitmentUpdate) }.commitment_signed = *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) };
}
#[must_use]
#[no_mangle]
pub extern "C" fn CommitmentUpdate_new(mut update_add_htlcs_arg: crate::c_types::derived::CVec_UpdateAddHTLCZ, mut update_fulfill_htlcs_arg: crate::c_types::derived::CVec_UpdateFulfillHTLCZ, mut update_fail_htlcs_arg: crate::c_types::derived::CVec_UpdateFailHTLCZ, mut update_fail_malformed_htlcs_arg: crate::c_types::derived::CVec_UpdateFailMalformedHTLCZ, mut update_fee_arg: crate::ln::msgs::UpdateFee, mut commitment_signed_arg: crate::ln::msgs::CommitmentSigned) -> CommitmentUpdate {
	let mut local_update_add_htlcs_arg = Vec::new(); for mut item in update_add_htlcs_arg.into_rust().drain(..) { local_update_add_htlcs_arg.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	let mut local_update_fulfill_htlcs_arg = Vec::new(); for mut item in update_fulfill_htlcs_arg.into_rust().drain(..) { local_update_fulfill_htlcs_arg.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	let mut local_update_fail_htlcs_arg = Vec::new(); for mut item in update_fail_htlcs_arg.into_rust().drain(..) { local_update_fail_htlcs_arg.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	let mut local_update_fail_malformed_htlcs_arg = Vec::new(); for mut item in update_fail_malformed_htlcs_arg.into_rust().drain(..) { local_update_fail_malformed_htlcs_arg.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	let mut local_update_fee_arg = if update_fee_arg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(update_fee_arg.inner.take_ptr() as *mut _) } }) };
	CommitmentUpdate { inner: Box::into_raw(Box::new(lnCommitmentUpdate {
		update_add_htlcs: local_update_add_htlcs_arg,
		update_fulfill_htlcs: local_update_fulfill_htlcs_arg,
		update_fail_htlcs: local_update_fail_htlcs_arg,
		update_fail_malformed_htlcs: local_update_fail_malformed_htlcs_arg,
		update_fee: local_update_fee_arg,
		commitment_signed: *unsafe { Box::from_raw(commitment_signed_arg.inner.take_ptr() as *mut _) },
	})), _underlying_ref: false }
}
/// " The information we received from a peer along the route of a payment we originated. This is"
/// " returned by ChannelMessageHandler::handle_update_fail_htlc to be passed into"
/// " RoutingMessageHandler::handle_htlc_fail_channel_update to update our network map."
#[must_use]
#[derive(Clone)]
#[repr(C)]
pub enum HTLCFailChannelUpdate {
	/// " We received an error which included a full ChannelUpdate message."
	ChannelUpdateMessage {
		msg: crate::ln::msgs::ChannelUpdate,
	},
	/// " We received an error which indicated only that a channel has been closed"
	ChannelClosed {
		short_channel_id: u64,
		is_permanent: bool,
	},
	/// " We received an error which indicated only that a node has failed"
	NodeFailure {
		node_id: crate::c_types::PublicKey,
		is_permanent: bool,
	},
}
use lightning::ln::msgs::HTLCFailChannelUpdate as lnHTLCFailChannelUpdate;
impl HTLCFailChannelUpdate {
	#[allow(unused)]
	pub(crate) fn to_ln(&self) -> lnHTLCFailChannelUpdate {
		match self {
			HTLCFailChannelUpdate::ChannelUpdateMessage {ref msg, } => {
				let mut msg_nonref = (*msg).clone();
				lnHTLCFailChannelUpdate::ChannelUpdateMessage {
					msg: *unsafe { Box::from_raw(msg_nonref.inner.take_ptr() as *mut _) },
				}
			},
			HTLCFailChannelUpdate::ChannelClosed {ref short_channel_id, ref is_permanent, } => {
				let mut short_channel_id_nonref = (*short_channel_id).clone();
				let mut is_permanent_nonref = (*is_permanent).clone();
				lnHTLCFailChannelUpdate::ChannelClosed {
					short_channel_id: short_channel_id_nonref,
					is_permanent: is_permanent_nonref,
				}
			},
			HTLCFailChannelUpdate::NodeFailure {ref node_id, ref is_permanent, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut is_permanent_nonref = (*is_permanent).clone();
				lnHTLCFailChannelUpdate::NodeFailure {
					node_id: node_id_nonref.into_rust(),
					is_permanent: is_permanent_nonref,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn into_ln(self) -> lnHTLCFailChannelUpdate {
		match self {
			HTLCFailChannelUpdate::ChannelUpdateMessage {mut msg, } => {
				lnHTLCFailChannelUpdate::ChannelUpdateMessage {
					msg: *unsafe { Box::from_raw(msg.inner.take_ptr() as *mut _) },
				}
			},
			HTLCFailChannelUpdate::ChannelClosed {mut short_channel_id, mut is_permanent, } => {
				lnHTLCFailChannelUpdate::ChannelClosed {
					short_channel_id: short_channel_id,
					is_permanent: is_permanent,
				}
			},
			HTLCFailChannelUpdate::NodeFailure {mut node_id, mut is_permanent, } => {
				lnHTLCFailChannelUpdate::NodeFailure {
					node_id: node_id.into_rust(),
					is_permanent: is_permanent,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn from_ln(lnt: &lnHTLCFailChannelUpdate) -> Self {
		match lnt {
			lnHTLCFailChannelUpdate::ChannelUpdateMessage {ref msg, } => {
				let mut msg_nonref = (*msg).clone();
				HTLCFailChannelUpdate::ChannelUpdateMessage {
					msg: crate::ln::msgs::ChannelUpdate { inner: Box::into_raw(Box::new(msg_nonref)), _underlying_ref: false },
				}
			},
			lnHTLCFailChannelUpdate::ChannelClosed {ref short_channel_id, ref is_permanent, } => {
				let mut short_channel_id_nonref = (*short_channel_id).clone();
				let mut is_permanent_nonref = (*is_permanent).clone();
				HTLCFailChannelUpdate::ChannelClosed {
					short_channel_id: short_channel_id_nonref,
					is_permanent: is_permanent_nonref,
				}
			},
			lnHTLCFailChannelUpdate::NodeFailure {ref node_id, ref is_permanent, } => {
				let mut node_id_nonref = (*node_id).clone();
				let mut is_permanent_nonref = (*is_permanent).clone();
				HTLCFailChannelUpdate::NodeFailure {
					node_id: crate::c_types::PublicKey::from_rust(&node_id_nonref),
					is_permanent: is_permanent_nonref,
				}
			},
		}
	}
	#[allow(unused)]
	pub(crate) fn ln_into(lnt: lnHTLCFailChannelUpdate) -> Self {
		match lnt {
			lnHTLCFailChannelUpdate::ChannelUpdateMessage {mut msg, } => {
				HTLCFailChannelUpdate::ChannelUpdateMessage {
					msg: crate::ln::msgs::ChannelUpdate { inner: Box::into_raw(Box::new(msg)), _underlying_ref: false },
				}
			},
			lnHTLCFailChannelUpdate::ChannelClosed {mut short_channel_id, mut is_permanent, } => {
				HTLCFailChannelUpdate::ChannelClosed {
					short_channel_id: short_channel_id,
					is_permanent: is_permanent,
				}
			},
			lnHTLCFailChannelUpdate::NodeFailure {mut node_id, mut is_permanent, } => {
				HTLCFailChannelUpdate::NodeFailure {
					node_id: crate::c_types::PublicKey::from_rust(&node_id),
					is_permanent: is_permanent,
				}
			},
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
	pub handle_open_channel: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, their_features: crate::ln::features::InitFeatures, msg: &crate::ln::msgs::OpenChannel),
	/// " Handle an incoming accept_channel message from the given peer."
	pub handle_accept_channel: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, their_features: crate::ln::features::InitFeatures, msg: &crate::ln::msgs::AcceptChannel),
	/// " Handle an incoming funding_created message from the given peer."
	pub handle_funding_created: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::FundingCreated),
	/// " Handle an incoming funding_signed message from the given peer."
	pub handle_funding_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::FundingSigned),
	/// " Handle an incoming funding_locked message from the given peer."
	pub handle_funding_locked: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::FundingLocked),
	/// " Handle an incoming shutdown message from the given peer."
	pub handle_shutdown: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::Shutdown),
	/// " Handle an incoming closing_signed message from the given peer."
	pub handle_closing_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::ClosingSigned),
	/// " Handle an incoming update_add_htlc message from the given peer."
	pub handle_update_add_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::UpdateAddHTLC),
	/// " Handle an incoming update_fulfill_htlc message from the given peer."
	pub handle_update_fulfill_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::UpdateFulfillHTLC),
	/// " Handle an incoming update_fail_htlc message from the given peer."
	pub handle_update_fail_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::UpdateFailHTLC),
	/// " Handle an incoming update_fail_malformed_htlc message from the given peer."
	pub handle_update_fail_malformed_htlc: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::UpdateFailMalformedHTLC),
	/// " Handle an incoming commitment_signed message from the given peer."
	pub handle_commitment_signed: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::CommitmentSigned),
	/// " Handle an incoming revoke_and_ack message from the given peer."
	pub handle_revoke_and_ack: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::RevokeAndACK),
	/// " Handle an incoming update_fee message from the given peer."
	pub handle_update_fee: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::UpdateFee),
	/// " Handle an incoming announcement_signatures message from the given peer."
	pub handle_announcement_signatures: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::AnnouncementSignatures),
	/// " Indicates a connection to the peer failed/an existing connection was lost. If no connection"
	/// " is believed to be possible in the future (eg they're sending us messages we don't"
	/// " understand or indicate they require unknown feature bits), no_connection_possible is set"
	/// " and any outstanding channels should be failed."
	pub peer_disconnected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, no_connection_possible: bool),
	/// " Handle a peer reconnecting, possibly generating channel_reestablish message(s)."
	pub peer_connected: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::Init),
	/// " Handle an incoming channel_reestablish message from the given peer."
	pub handle_channel_reestablish: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::ChannelReestablish),
	/// " Handle an incoming error message from the given peer."
	pub handle_error: extern "C" fn (this_arg: *const c_void, their_node_id: crate::c_types::PublicKey, msg: &crate::ln::msgs::ErrorMessage),
	pub MessageSendEventsProvider: crate::util::events::MessageSendEventsProvider,
}
impl lightning::util::events::MessageSendEventsProvider for ChannelMessageHandler {
	fn get_and_clear_pending_msg_events(&self) -> Vec<lightning::util::events::MessageSendEvent> {
		<crate::util::events::MessageSendEventsProvider as lightning::util::events::MessageSendEventsProvider>::get_and_clear_pending_msg_events(&self.MessageSendEventsProvider)
	}
}
unsafe impl Send for ChannelMessageHandler {}
unsafe impl Sync for ChannelMessageHandler {}

use lightning::ln::msgs::ChannelMessageHandler as lnChannelMessageHandler;
impl lnChannelMessageHandler for ChannelMessageHandler {
	fn handle_open_channel(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, their_features: lightning::ln::features::InitFeatures, msg: &lightning::ln::msgs::OpenChannel) {
		(self.handle_open_channel)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new(their_features)), _underlying_ref: false }, &crate::ln::msgs::OpenChannel { inner: msg, _underlying_ref: true })
	}
	fn handle_accept_channel(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, their_features: lightning::ln::features::InitFeatures, msg: &lightning::ln::msgs::AcceptChannel) {
		(self.handle_accept_channel)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), crate::ln::features::InitFeatures { inner: Box::into_raw(Box::new(their_features)), _underlying_ref: false }, &crate::ln::msgs::AcceptChannel { inner: msg, _underlying_ref: true })
	}
	fn handle_funding_created(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingCreated) {
		(self.handle_funding_created)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::FundingCreated { inner: msg, _underlying_ref: true })
	}
	fn handle_funding_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingSigned) {
		(self.handle_funding_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::FundingSigned { inner: msg, _underlying_ref: true })
	}
	fn handle_funding_locked(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::FundingLocked) {
		(self.handle_funding_locked)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::FundingLocked { inner: msg, _underlying_ref: true })
	}
	fn handle_shutdown(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::Shutdown) {
		(self.handle_shutdown)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::Shutdown { inner: msg, _underlying_ref: true })
	}
	fn handle_closing_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ClosingSigned) {
		(self.handle_closing_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::ClosingSigned { inner: msg, _underlying_ref: true })
	}
	fn handle_update_add_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateAddHTLC) {
		(self.handle_update_add_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateAddHTLC { inner: msg, _underlying_ref: true })
	}
	fn handle_update_fulfill_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFulfillHTLC) {
		(self.handle_update_fulfill_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateFulfillHTLC { inner: msg, _underlying_ref: true })
	}
	fn handle_update_fail_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFailHTLC) {
		(self.handle_update_fail_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateFailHTLC { inner: msg, _underlying_ref: true })
	}
	fn handle_update_fail_malformed_htlc(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFailMalformedHTLC) {
		(self.handle_update_fail_malformed_htlc)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateFailMalformedHTLC { inner: msg, _underlying_ref: true })
	}
	fn handle_commitment_signed(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::CommitmentSigned) {
		(self.handle_commitment_signed)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::CommitmentSigned { inner: msg, _underlying_ref: true })
	}
	fn handle_revoke_and_ack(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::RevokeAndACK) {
		(self.handle_revoke_and_ack)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::RevokeAndACK { inner: msg, _underlying_ref: true })
	}
	fn handle_update_fee(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::UpdateFee) {
		(self.handle_update_fee)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::UpdateFee { inner: msg, _underlying_ref: true })
	}
	fn handle_announcement_signatures(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::AnnouncementSignatures) {
		(self.handle_announcement_signatures)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::AnnouncementSignatures { inner: msg, _underlying_ref: true })
	}
	fn peer_disconnected(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, no_connection_possible: bool) {
		(self.peer_disconnected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), no_connection_possible)
	}
	fn peer_connected(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::Init) {
		(self.peer_connected)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::Init { inner: msg, _underlying_ref: true })
	}
	fn handle_channel_reestablish(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ChannelReestablish) {
		(self.handle_channel_reestablish)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::ChannelReestablish { inner: msg, _underlying_ref: true })
	}
	fn handle_error(&self, their_node_id: &bitcoin::secp256k1::key::PublicKey, msg: &lightning::ln::msgs::ErrorMessage) {
		(self.handle_error)(self.this_arg, crate::c_types::PublicKey::from_rust(&their_node_id), &crate::ln::msgs::ErrorMessage { inner: msg, _underlying_ref: true })
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
	#[must_use]
	pub handle_node_announcement: extern "C" fn (this_arg: *const c_void, msg: &crate::ln::msgs::NodeAnnouncement) -> crate::c_types::derived::CResult_boolLightningErrorZ,
	/// " Handle a channel_announcement message, returning true if it should be forwarded on, false"
	/// " or returning an Err otherwise."
	#[must_use]
	pub handle_channel_announcement: extern "C" fn (this_arg: *const c_void, msg: &crate::ln::msgs::ChannelAnnouncement) -> crate::c_types::derived::CResult_boolLightningErrorZ,
	/// " Handle an incoming channel_update message, returning true if it should be forwarded on,"
	/// " false or returning an Err otherwise."
	#[must_use]
	pub handle_channel_update: extern "C" fn (this_arg: *const c_void, msg: &crate::ln::msgs::ChannelUpdate) -> crate::c_types::derived::CResult_boolLightningErrorZ,
	/// " Handle some updates to the route graph that we learned due to an outbound failed payment."
	pub handle_htlc_fail_channel_update: extern "C" fn (this_arg: *const c_void, update: &crate::ln::msgs::HTLCFailChannelUpdate),
	/// " Gets a subset of the channel announcements and updates required to dump our routing table"
	/// " to a remote node, starting at the short_channel_id indicated by starting_point and"
	/// " including the batch_amount entries immediately higher in numerical value than starting_point."
	#[must_use]
	pub get_next_channel_announcements: extern "C" fn (this_arg: *const c_void, starting_point: u64, batch_amount: u8) -> crate::c_types::derived::CVec_C3Tuple_ChannelAnnouncementChannelUpdateChannelUpdateZZ,
	/// " Gets a subset of the node announcements required to dump our routing table to a remote node,"
	/// " starting at the node *after* the provided publickey and including batch_amount entries"
	/// " immediately higher (as defined by <PublicKey as Ord>::cmp) than starting_point."
	/// " If None is provided for starting_point, we start at the first node."
	#[must_use]
	pub get_next_node_announcements: extern "C" fn (this_arg: *const c_void, starting_point: *const crate::c_types::PublicKey, batch_amount: u8) -> crate::c_types::derived::CVec_NodeAnnouncementZ,
	/// " Returns whether a full sync should be requested from a peer."
	#[must_use]
	pub should_request_full_sync: extern "C" fn (this_arg: *const c_void, node_id: crate::c_types::PublicKey) -> bool,
}
unsafe impl Send for RoutingMessageHandler {}
unsafe impl Sync for RoutingMessageHandler {}

use lightning::ln::msgs::RoutingMessageHandler as lnRoutingMessageHandler;
impl lnRoutingMessageHandler for RoutingMessageHandler {
	fn handle_node_announcement(&self, msg: &lightning::ln::msgs::NodeAnnouncement) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_node_announcement)(self.this_arg, &crate::ln::msgs::NodeAnnouncement { inner: msg, _underlying_ref: true });
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }) }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(ret.contents.err.take_ptr()) }).inner.take_ptr() as *mut _) } })};
		local_ret
	}
	fn handle_channel_announcement(&self, msg: &lightning::ln::msgs::ChannelAnnouncement) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_channel_announcement)(self.this_arg, &crate::ln::msgs::ChannelAnnouncement { inner: msg, _underlying_ref: true });
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }) }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(ret.contents.err.take_ptr()) }).inner.take_ptr() as *mut _) } })};
		local_ret
	}
	fn handle_channel_update(&self, msg: &lightning::ln::msgs::ChannelUpdate) -> Result<bool, lightning::ln::msgs::LightningError> {
		let mut ret = (self.handle_channel_update)(self.this_arg, &crate::ln::msgs::ChannelUpdate { inner: msg, _underlying_ref: true });
		let mut local_ret = match ret.result_good { true => Ok( { (*unsafe { Box::from_raw(ret.contents.result.take_ptr()) }) }), false => Err( { *unsafe { Box::from_raw((*unsafe { Box::from_raw(ret.contents.err.take_ptr()) }).inner.take_ptr() as *mut _) } })};
		local_ret
	}
	fn handle_htlc_fail_channel_update(&self, update: &lightning::ln::msgs::HTLCFailChannelUpdate) {
		(self.handle_htlc_fail_channel_update)(self.this_arg, &crate::ln::msgs::HTLCFailChannelUpdate::from_ln(&update))
	}
	fn get_next_channel_announcements(&self, starting_point: u64, batch_amount: u8) -> Vec<(lightning::ln::msgs::ChannelAnnouncement, Option<lightning::ln::msgs::ChannelUpdate>, Option<lightning::ln::msgs::ChannelUpdate>)> {
		let mut ret = (self.get_next_channel_announcements)(self.this_arg, starting_point, batch_amount);
		let mut local_ret = Vec::new(); for mut item in ret.into_rust().drain(..) { local_ret.push( { let (mut orig_ret_0_0, mut orig_ret_0_1, mut orig_ret_0_2) = item.to_rust(); let mut local_orig_ret_0_1 = if orig_ret_0_1.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(orig_ret_0_1.inner.take_ptr() as *mut _) } }) }; let mut local_orig_ret_0_2 = if orig_ret_0_2.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(orig_ret_0_2.inner.take_ptr() as *mut _) } }) }; let local_ret_0 = (*unsafe { Box::from_raw(orig_ret_0_0.inner.take_ptr() as *mut _) }, local_orig_ret_0_1, local_orig_ret_0_2); local_ret_0 }); };
		local_ret
	}
	fn get_next_node_announcements(&self, starting_point: Option<&bitcoin::secp256k1::key::PublicKey>, batch_amount: u8) -> Vec<lightning::ln::msgs::NodeAnnouncement> {
		let mut local_starting_point = if starting_point.is_none() { std::ptr::null() } else {  { Box::into_raw(Box::new(crate::c_types::PublicKey::from_rust(&(*starting_point.as_ref().unwrap())))) } };
		let mut ret = (self.get_next_node_announcements)(self.this_arg, local_starting_point, batch_amount);
		let mut local_ret = Vec::new(); for mut item in ret.into_rust().drain(..) { local_ret.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
		local_ret
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
#[no_mangle]
pub extern "C" fn AcceptChannel_write(obj: *const AcceptChannel) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn AcceptChannel_read(ser: crate::c_types::u8slice) -> AcceptChannel {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		AcceptChannel { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		AcceptChannel { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_write(obj: *const AnnouncementSignatures) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn AnnouncementSignatures_read(ser: crate::c_types::u8slice) -> AnnouncementSignatures {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		AnnouncementSignatures { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		AnnouncementSignatures { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn ChannelReestablish_write(obj: *const ChannelReestablish) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn ChannelReestablish_read(ser: crate::c_types::u8slice) -> ChannelReestablish {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		ChannelReestablish { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		ChannelReestablish { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn ClosingSigned_write(obj: *const ClosingSigned) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn ClosingSigned_read(ser: crate::c_types::u8slice) -> ClosingSigned {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		ClosingSigned { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		ClosingSigned { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn CommitmentSigned_write(obj: *const CommitmentSigned) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn CommitmentSigned_read(ser: crate::c_types::u8slice) -> CommitmentSigned {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		CommitmentSigned { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		CommitmentSigned { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn FundingCreated_write(obj: *const FundingCreated) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn FundingCreated_read(ser: crate::c_types::u8slice) -> FundingCreated {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		FundingCreated { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		FundingCreated { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn FundingSigned_write(obj: *const FundingSigned) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn FundingSigned_read(ser: crate::c_types::u8slice) -> FundingSigned {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		FundingSigned { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		FundingSigned { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn FundingLocked_write(obj: *const FundingLocked) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn FundingLocked_read(ser: crate::c_types::u8slice) -> FundingLocked {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		FundingLocked { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		FundingLocked { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn Init_write(obj: *const Init) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn Init_read(ser: crate::c_types::u8slice) -> Init {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		Init { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		Init { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn OpenChannel_write(obj: *const OpenChannel) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn OpenChannel_read(ser: crate::c_types::u8slice) -> OpenChannel {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		OpenChannel { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		OpenChannel { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn RevokeAndACK_write(obj: *const RevokeAndACK) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn RevokeAndACK_read(ser: crate::c_types::u8slice) -> RevokeAndACK {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		RevokeAndACK { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		RevokeAndACK { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn Shutdown_write(obj: *const Shutdown) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn Shutdown_read(ser: crate::c_types::u8slice) -> Shutdown {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		Shutdown { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		Shutdown { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_write(obj: *const UpdateFailHTLC) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn UpdateFailHTLC_read(ser: crate::c_types::u8slice) -> UpdateFailHTLC {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		UpdateFailHTLC { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		UpdateFailHTLC { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_write(obj: *const UpdateFailMalformedHTLC) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn UpdateFailMalformedHTLC_read(ser: crate::c_types::u8slice) -> UpdateFailMalformedHTLC {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		UpdateFailMalformedHTLC { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		UpdateFailMalformedHTLC { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn UpdateFee_write(obj: *const UpdateFee) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn UpdateFee_read(ser: crate::c_types::u8slice) -> UpdateFee {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		UpdateFee { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		UpdateFee { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_write(obj: *const UpdateFulfillHTLC) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn UpdateFulfillHTLC_read(ser: crate::c_types::u8slice) -> UpdateFulfillHTLC {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		UpdateFulfillHTLC { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		UpdateFulfillHTLC { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_write(obj: *const UpdateAddHTLC) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn UpdateAddHTLC_read(ser: crate::c_types::u8slice) -> UpdateAddHTLC {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		UpdateAddHTLC { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		UpdateAddHTLC { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn Ping_write(obj: *const Ping) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn Ping_read(ser: crate::c_types::u8slice) -> Ping {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		Ping { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		Ping { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn Pong_write(obj: *const Pong) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn Pong_read(ser: crate::c_types::u8slice) -> Pong {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		Pong { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		Pong { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_write(obj: *const UnsignedChannelAnnouncement) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn UnsignedChannelAnnouncement_read(ser: crate::c_types::u8slice) -> UnsignedChannelAnnouncement {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		UnsignedChannelAnnouncement { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		UnsignedChannelAnnouncement { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_write(obj: *const ChannelAnnouncement) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn ChannelAnnouncement_read(ser: crate::c_types::u8slice) -> ChannelAnnouncement {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		ChannelAnnouncement { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		ChannelAnnouncement { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn ChannelUpdate_write(obj: *const ChannelUpdate) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn ChannelUpdate_read(ser: crate::c_types::u8slice) -> ChannelUpdate {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		ChannelUpdate { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		ChannelUpdate { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn ErrorMessage_write(obj: *const ErrorMessage) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn ErrorMessage_read(ser: crate::c_types::u8slice) -> ErrorMessage {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		ErrorMessage { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		ErrorMessage { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_write(obj: *const UnsignedNodeAnnouncement) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn UnsignedNodeAnnouncement_read(ser: crate::c_types::u8slice) -> UnsignedNodeAnnouncement {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		UnsignedNodeAnnouncement { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		UnsignedNodeAnnouncement { inner: std::ptr::null(), _underlying_ref: false }
	}
}
#[no_mangle]
pub extern "C" fn NodeAnnouncement_write(obj: *const NodeAnnouncement) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn NodeAnnouncement_read(ser: crate::c_types::u8slice) -> NodeAnnouncement {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		NodeAnnouncement { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		NodeAnnouncement { inner: std::ptr::null(), _underlying_ref: false }
	}
}
