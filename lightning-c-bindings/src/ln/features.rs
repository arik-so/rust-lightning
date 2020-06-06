//! " Feature flag definitions for the Lightning protocol according to [BOLT #9]."
//! ""
//! " Lightning nodes advertise a supported set of operation through feature flags. Features are"
//! " applicable for a specific context as indicated in some [messages]. [`Features`] encapsulates"
//! " behavior for specifying and checking feature flags for a particular context. Each feature is"
//! " defined internally by a trait specifying the corresponding flags (i.e., even and odd bits). A"
//! " [`Context`] is used to parameterize [`Features`] and defines which features it can support."
//! ""
//! " Whether a feature is considered \"known\" or \"unknown\" is relative to the implementation, whereas"
//! " the term \"supports\" is used in reference to a particular set of [`Features`]. That is, a node"
//! " supports a feature if it advertises the feature (as either required or optional) to its peers."
//! " And the implementation can interpret a feature if the feature is known to it."
//! ""
//! " [BOLT #9]: https://github.com/lightningnetwork/lightning-rfc/blob/master/09-features.md"
//! " [messages]: ../msgs/index.html"
//! " [`Features`]: struct.Features.html"
//! " [`Context`]: sealed/trait.Context.html"

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::*;


use lightning::ln::features::InitFeatures as lnInitFeaturesImport;
type lnInitFeatures = lnInitFeaturesImport;

/// " Features used within an `init` message."
#[repr(C)]
pub struct InitFeatures {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnInitFeatures,
	pub _underlying_ref: bool,
}

impl Drop for InitFeatures {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnInitFeatures) };
		}
	}
}
#[no_mangle]
pub extern "C" fn InitFeatures_free(this_ptr: InitFeatures) { }

use lightning::ln::features::NodeFeatures as lnNodeFeaturesImport;
type lnNodeFeatures = lnNodeFeaturesImport;

/// " Features used within a `node_announcement` message."
#[repr(C)]
pub struct NodeFeatures {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnNodeFeatures,
	pub _underlying_ref: bool,
}

impl Drop for NodeFeatures {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnNodeFeatures) };
		}
	}
}
#[no_mangle]
pub extern "C" fn NodeFeatures_free(this_ptr: NodeFeatures) { }

use lightning::ln::features::ChannelFeatures as lnChannelFeaturesImport;
type lnChannelFeatures = lnChannelFeaturesImport;

/// " Features used within a `channel_announcement` message."
#[repr(C)]
pub struct ChannelFeatures {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelFeatures,
	pub _underlying_ref: bool,
}

impl Drop for ChannelFeatures {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelFeatures) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelFeatures_free(this_ptr: ChannelFeatures) { }
