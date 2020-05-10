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


use lightning::ln::features::InitFeatures as lnInitFeaturesImport;
type lnInitFeatures = lnInitFeaturesImport;

/// " Features used within an `init` message."
#[repr(C)]
pub struct InitFeatures {
	pub(crate) inner: *const lnInitFeatures,
}


use lightning::ln::features::NodeFeatures as lnNodeFeaturesImport;
type lnNodeFeatures = lnNodeFeaturesImport;

/// " Features used within a `node_announcement` message."
#[repr(C)]
pub struct NodeFeatures {
	pub(crate) inner: *const lnNodeFeatures,
}


use lightning::ln::features::ChannelFeatures as lnChannelFeaturesImport;
type lnChannelFeatures = lnChannelFeaturesImport;

/// " Features used within a `channel_announcement` message."
#[repr(C)]
pub struct ChannelFeatures {
	pub(crate) inner: *const lnChannelFeatures,
}

