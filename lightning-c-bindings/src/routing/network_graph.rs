//! " The top-level network map tracking logic lives here."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::TakePointer;


use lightning::routing::network_graph::NetGraphMsgHandler as lnNetGraphMsgHandlerImport;
type lnNetGraphMsgHandler = lnNetGraphMsgHandlerImport<crate::chain::chaininterface::ChainWatchInterface, crate::util::logger::Logger>;

/// " Receives and validates network updates from peers,"
/// " stores authentic and relevant data as a network graph."
/// " This network graph is then used for routing payments."
/// " Provides interface to help with initial routing sync by"
/// " serving historical announcements."
#[repr(C)]
pub struct NetGraphMsgHandler {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnNetGraphMsgHandler,
}

impl Drop for NetGraphMsgHandler {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnNetGraphMsgHandler) };
		}
	}
}
#[no_mangle]
pub extern "C" fn NetGraphMsgHandler_free(this_ptr: NetGraphMsgHandler) { }
/// " Creates a new tracker of the actual state of the network of channels and nodes,"
/// " assuming a fresh network graph."
/// " Chain monitor is used to make sure announced channels exist on-chain,"
/// " channel data is correct, and that the announcement is signed with"
/// " channel owners' keys."
#[no_mangle]
pub extern "C" fn NetGraphMsgHandler_new(mut chain_monitor: crate::chain::chaininterface::ChainWatchInterface, mut logger: crate::util::logger::Logger) -> NetGraphMsgHandler {
	let mut ret = lightning::routing::network_graph::NetGraphMsgHandler::new(chain_monitor, logger);
	NetGraphMsgHandler { inner: Box::into_raw(Box::new(ret)) }
}

#[no_mangle]
pub extern "C" fn NetGraphMsgHandler_as_RoutingMessageHandler(this_arg: *const NetGraphMsgHandler) -> crate::ln::msgs::RoutingMessageHandler {
	crate::ln::msgs::RoutingMessageHandler {
		this_arg: unsafe { (*this_arg).inner as *mut c_void },
		handle_node_announcement: NetGraphMsgHandler_RoutingMessageHandler_handle_node_announcement,
		handle_channel_announcement: NetGraphMsgHandler_RoutingMessageHandler_handle_channel_announcement,
		handle_channel_update: NetGraphMsgHandler_RoutingMessageHandler_handle_channel_update,
		handle_htlc_fail_channel_update: NetGraphMsgHandler_RoutingMessageHandler_handle_htlc_fail_channel_update,
		//XXX: Need to export get_next_channel_announcements
		//XXX: Need to export get_next_node_announcements
		should_request_full_sync: NetGraphMsgHandler_RoutingMessageHandler_should_request_full_sync,
	}
}
use lightning::ln::msgs::RoutingMessageHandler as RoutingMessageHandlerTraitImport;
extern "C" fn NetGraphMsgHandler_RoutingMessageHandler_handle_node_announcement(this_arg: *const c_void, msg: &crate::ln::msgs::NodeAnnouncement) -> crate::c_types::derived::CResult_boolLightningErrorZ {
	let mut ret = unsafe { &mut *(this_arg as *mut lnNetGraphMsgHandler) }.handle_node_announcement(unsafe { &*msg.inner });
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { o }), Err(mut e) => crate::c_types::CResultTempl::err( { crate::ln::msgs::LightningError { inner: Box::into_raw(Box::new(e)) } }) };
	local_ret
}
extern "C" fn NetGraphMsgHandler_RoutingMessageHandler_handle_channel_announcement(this_arg: *const c_void, msg: &crate::ln::msgs::ChannelAnnouncement) -> crate::c_types::derived::CResult_boolLightningErrorZ {
	let mut ret = unsafe { &mut *(this_arg as *mut lnNetGraphMsgHandler) }.handle_channel_announcement(unsafe { &*msg.inner });
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { o }), Err(mut e) => crate::c_types::CResultTempl::err( { crate::ln::msgs::LightningError { inner: Box::into_raw(Box::new(e)) } }) };
	local_ret
}
extern "C" fn NetGraphMsgHandler_RoutingMessageHandler_handle_htlc_fail_channel_update(this_arg: *const c_void, update: &crate::ln::msgs::HTLCFailChannelUpdate) {
	unsafe { &mut *(this_arg as *mut lnNetGraphMsgHandler) }.handle_htlc_fail_channel_update(unsafe { &*update.inner })
}
extern "C" fn NetGraphMsgHandler_RoutingMessageHandler_handle_channel_update(this_arg: *const c_void, msg: &crate::ln::msgs::ChannelUpdate) -> crate::c_types::derived::CResult_boolLightningErrorZ {
	let mut ret = unsafe { &mut *(this_arg as *mut lnNetGraphMsgHandler) }.handle_channel_update(unsafe { &*msg.inner });
	let mut local_ret = match ret{ Ok(mut o) => crate::c_types::CResultTempl::good( { o }), Err(mut e) => crate::c_types::CResultTempl::err( { crate::ln::msgs::LightningError { inner: Box::into_raw(Box::new(e)) } }) };
	local_ret
}
extern "C" fn NetGraphMsgHandler_RoutingMessageHandler_should_request_full_sync(this_arg: *const c_void, _node_id: crate::c_types::PublicKey) -> bool {
	let mut ret = unsafe { &mut *(this_arg as *mut lnNetGraphMsgHandler) }.should_request_full_sync(&_node_id.into_rust());
	ret
}


use lightning::routing::network_graph::DirectionalChannelInfo as lnDirectionalChannelInfoImport;
type lnDirectionalChannelInfo = lnDirectionalChannelInfoImport;

/// " Details about one direction of a channel. Received"
/// " within a channel update."
#[repr(C)]
pub struct DirectionalChannelInfo {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnDirectionalChannelInfo,
}

impl Drop for DirectionalChannelInfo {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnDirectionalChannelInfo) };
		}
	}
}
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_free(this_ptr: DirectionalChannelInfo) { }
/// " When the last update to the channel direction was issued."
/// " Value is opaque, as set in the announcement."
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_get_last_update(this_ptr: &DirectionalChannelInfo) -> u32 {
	let inner_val = &unsafe { &*this_ptr.inner }.last_update;
	(*inner_val)
}
/// " When the last update to the channel direction was issued."
/// " Value is opaque, as set in the announcement."
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_set_last_update(this_ptr: &mut DirectionalChannelInfo, mut val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnDirectionalChannelInfo) }.last_update = val;
}
/// " Whether the channel can be currently used for payments (in this one direction)."
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_get_enabled(this_ptr: &DirectionalChannelInfo) -> bool {
	let inner_val = &unsafe { &*this_ptr.inner }.enabled;
	(*inner_val)
}
/// " Whether the channel can be currently used for payments (in this one direction)."
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_set_enabled(this_ptr: &mut DirectionalChannelInfo, mut val: bool) {
	unsafe { &mut *(this_ptr.inner as *mut lnDirectionalChannelInfo) }.enabled = val;
}
/// " The difference in CLTV values that you must have when routing through this channel."
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_get_cltv_expiry_delta(this_ptr: &DirectionalChannelInfo) -> u16 {
	let inner_val = &unsafe { &*this_ptr.inner }.cltv_expiry_delta;
	(*inner_val)
}
/// " The difference in CLTV values that you must have when routing through this channel."
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_set_cltv_expiry_delta(this_ptr: &mut DirectionalChannelInfo, mut val: u16) {
	unsafe { &mut *(this_ptr.inner as *mut lnDirectionalChannelInfo) }.cltv_expiry_delta = val;
}
/// " The minimum value, which must be relayed to the next hop via the channel"
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_get_htlc_minimum_msat(this_ptr: &DirectionalChannelInfo) -> u64 {
	let inner_val = &unsafe { &*this_ptr.inner }.htlc_minimum_msat;
	(*inner_val)
}
/// " The minimum value, which must be relayed to the next hop via the channel"
#[no_mangle]
pub extern "C" fn DirectionalChannelInfo_set_htlc_minimum_msat(this_ptr: &mut DirectionalChannelInfo, mut val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnDirectionalChannelInfo) }.htlc_minimum_msat = val;
}

use lightning::routing::network_graph::ChannelInfo as lnChannelInfoImport;
type lnChannelInfo = lnChannelInfoImport;

/// " Details about a channel (both directions)."
/// " Received within a channel announcement."
#[repr(C)]
pub struct ChannelInfo {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelInfo,
}

impl Drop for ChannelInfo {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnChannelInfo) };
		}
	}
}
#[no_mangle]
pub extern "C" fn ChannelInfo_free(this_ptr: ChannelInfo) { }
/// " Source node of the first direction of a channel"
#[no_mangle]
pub extern "C" fn ChannelInfo_get_node_one(this_ptr: &ChannelInfo) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.node_one;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " Source node of the first direction of a channel"
#[no_mangle]
pub extern "C" fn ChannelInfo_set_node_one(this_ptr: &mut ChannelInfo, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelInfo) }.node_one = val.into_rust();
}
/// " Details about the first direction of a channel"
#[no_mangle]
pub extern "C" fn ChannelInfo_get_one_to_two(this_ptr: &ChannelInfo) -> *const DirectionalChannelInfo {
	let inner_val = &unsafe { &*this_ptr.inner }.one_to_two;
	let mut local_inner_val = if inner_val.is_none() { return std::ptr::null(); } else {  { Box::into_raw(Box::new(crate::routing::network_graph::DirectionalChannelInfo { inner: &(*inner_val.as_ref().unwrap()) } )) } };
	local_inner_val
}
/// " Details about the first direction of a channel"
#[no_mangle]
pub extern "C" fn ChannelInfo_set_one_to_two(this_ptr: &mut ChannelInfo, mut val: DirectionalChannelInfo) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) } }) };
	unsafe { &mut *(this_ptr.inner as *mut lnChannelInfo) }.one_to_two = local_val;
}
/// " Source node of the second direction of a channel"
#[no_mangle]
pub extern "C" fn ChannelInfo_get_node_two(this_ptr: &ChannelInfo) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.node_two;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " Source node of the second direction of a channel"
#[no_mangle]
pub extern "C" fn ChannelInfo_set_node_two(this_ptr: &mut ChannelInfo, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelInfo) }.node_two = val.into_rust();
}
/// " Details about the second direction of a channel"
#[no_mangle]
pub extern "C" fn ChannelInfo_get_two_to_one(this_ptr: &ChannelInfo) -> *const DirectionalChannelInfo {
	let inner_val = &unsafe { &*this_ptr.inner }.two_to_one;
	let mut local_inner_val = if inner_val.is_none() { return std::ptr::null(); } else {  { Box::into_raw(Box::new(crate::routing::network_graph::DirectionalChannelInfo { inner: &(*inner_val.as_ref().unwrap()) } )) } };
	local_inner_val
}
/// " Details about the second direction of a channel"
#[no_mangle]
pub extern "C" fn ChannelInfo_set_two_to_one(this_ptr: &mut ChannelInfo, mut val: DirectionalChannelInfo) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) } }) };
	unsafe { &mut *(this_ptr.inner as *mut lnChannelInfo) }.two_to_one = local_val;
}

use lightning::routing::network_graph::RoutingFees as lnRoutingFeesImport;
type lnRoutingFees = lnRoutingFeesImport;

/// " Fees for routing via a given channel or a node"
#[repr(C)]
pub struct RoutingFees {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRoutingFees,
}

impl Drop for RoutingFees {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnRoutingFees) };
		}
	}
}
#[no_mangle]
pub extern "C" fn RoutingFees_free(this_ptr: RoutingFees) { }
/// " Flat routing fee in satoshis"
#[no_mangle]
pub extern "C" fn RoutingFees_get_base_msat(this_ptr: &RoutingFees) -> u32 {
	let inner_val = &unsafe { &*this_ptr.inner }.base_msat;
	(*inner_val)
}
/// " Flat routing fee in satoshis"
#[no_mangle]
pub extern "C" fn RoutingFees_set_base_msat(this_ptr: &mut RoutingFees, mut val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnRoutingFees) }.base_msat = val;
}
/// " Liquidity-based routing fee in millionths of a routed amount."
/// " In other words, 10000 is 1%."
#[no_mangle]
pub extern "C" fn RoutingFees_get_proportional_millionths(this_ptr: &RoutingFees) -> u32 {
	let inner_val = &unsafe { &*this_ptr.inner }.proportional_millionths;
	(*inner_val)
}
/// " Liquidity-based routing fee in millionths of a routed amount."
/// " In other words, 10000 is 1%."
#[no_mangle]
pub extern "C" fn RoutingFees_set_proportional_millionths(this_ptr: &mut RoutingFees, mut val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnRoutingFees) }.proportional_millionths = val;
}
#[no_mangle]
pub extern "C" fn RoutingFees_new(mut base_msat_arg: u32, mut proportional_millionths_arg: u32) -> RoutingFees {
	RoutingFees { inner: Box::into_raw(Box::new(lnRoutingFees {
		base_msat: base_msat_arg,
		proportional_millionths: proportional_millionths_arg,
	}))}
}

use lightning::routing::network_graph::NodeAnnouncementInfo as lnNodeAnnouncementInfoImport;
type lnNodeAnnouncementInfo = lnNodeAnnouncementInfoImport;

/// " Information received in the latest node_announcement from this node."
#[repr(C)]
pub struct NodeAnnouncementInfo {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnNodeAnnouncementInfo,
}

impl Drop for NodeAnnouncementInfo {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnNodeAnnouncementInfo) };
		}
	}
}
#[no_mangle]
pub extern "C" fn NodeAnnouncementInfo_free(this_ptr: NodeAnnouncementInfo) { }
/// " When the last known update to the node state was issued."
/// " Value is opaque, as set in the announcement."
#[no_mangle]
pub extern "C" fn NodeAnnouncementInfo_get_last_update(this_ptr: &NodeAnnouncementInfo) -> u32 {
	let inner_val = &unsafe { &*this_ptr.inner }.last_update;
	(*inner_val)
}
/// " When the last known update to the node state was issued."
/// " Value is opaque, as set in the announcement."
#[no_mangle]
pub extern "C" fn NodeAnnouncementInfo_set_last_update(this_ptr: &mut NodeAnnouncementInfo, mut val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnNodeAnnouncementInfo) }.last_update = val;
}
/// " Color assigned to the node"
#[no_mangle]
pub extern "C" fn NodeAnnouncementInfo_get_rgb(this_ptr: &NodeAnnouncementInfo) -> *const [u8; 3] {
	let inner_val = &unsafe { &*this_ptr.inner }.rgb;
	&(*inner_val)
}
/// " Color assigned to the node"
#[no_mangle]
pub extern "C" fn NodeAnnouncementInfo_set_rgb(this_ptr: &mut NodeAnnouncementInfo, mut val: crate::c_types::ThreeBytes) {
	unsafe { &mut *(this_ptr.inner as *mut lnNodeAnnouncementInfo) }.rgb = val.data;
}
/// " Moniker assigned to the node."
/// " May be invalid or malicious (eg control chars),"
/// " should not be exposed to the user."
#[no_mangle]
pub extern "C" fn NodeAnnouncementInfo_get_alias(this_ptr: &NodeAnnouncementInfo) -> *const [u8; 32] {
	let inner_val = &unsafe { &*this_ptr.inner }.alias;
	&(*inner_val)
}
/// " Moniker assigned to the node."
/// " May be invalid or malicious (eg control chars),"
/// " should not be exposed to the user."
#[no_mangle]
pub extern "C" fn NodeAnnouncementInfo_set_alias(this_ptr: &mut NodeAnnouncementInfo, mut val: crate::c_types::ThirtyTwoBytes) {
	unsafe { &mut *(this_ptr.inner as *mut lnNodeAnnouncementInfo) }.alias = val.data;
}
/// " Internet-level addresses via which one can connect to the node"
#[no_mangle]
pub extern "C" fn NodeAnnouncementInfo_set_addresses(this_ptr: &mut NodeAnnouncementInfo, mut val: crate::c_types::derived::CVec_NetAddressZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); };
	unsafe { &mut *(this_ptr.inner as *mut lnNodeAnnouncementInfo) }.addresses = local_val;
}

use lightning::routing::network_graph::NodeInfo as lnNodeInfoImport;
type lnNodeInfo = lnNodeInfoImport;

/// " Details about a node in the network, known from the network announcement."
#[repr(C)]
pub struct NodeInfo {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnNodeInfo,
}

impl Drop for NodeInfo {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnNodeInfo) };
		}
	}
}
#[no_mangle]
pub extern "C" fn NodeInfo_free(this_ptr: NodeInfo) { }
/// " All valid channels a node has announced"
#[no_mangle]
pub extern "C" fn NodeInfo_set_channels(this_ptr: &mut NodeInfo, mut val: crate::c_types::derived::CVec_u64Z) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { item }); };
	unsafe { &mut *(this_ptr.inner as *mut lnNodeInfo) }.channels = local_val;
}
/// " Lowest fees enabling routing via any of the enabled, known channels to a node."
/// " The two fields (flat and proportional fee) are independent,"
/// " meaning they don't have to refer to the same channel."
#[no_mangle]
pub extern "C" fn NodeInfo_get_lowest_inbound_channel_fees(this_ptr: &NodeInfo) -> *const RoutingFees {
	let inner_val = &unsafe { &*this_ptr.inner }.lowest_inbound_channel_fees;
	let mut local_inner_val = if inner_val.is_none() { return std::ptr::null(); } else {  { Box::into_raw(Box::new(crate::routing::network_graph::RoutingFees { inner: &(*inner_val.as_ref().unwrap()) } )) } };
	local_inner_val
}
/// " Lowest fees enabling routing via any of the enabled, known channels to a node."
/// " The two fields (flat and proportional fee) are independent,"
/// " meaning they don't have to refer to the same channel."
#[no_mangle]
pub extern "C" fn NodeInfo_set_lowest_inbound_channel_fees(this_ptr: &mut NodeInfo, mut val: RoutingFees) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) } }) };
	unsafe { &mut *(this_ptr.inner as *mut lnNodeInfo) }.lowest_inbound_channel_fees = local_val;
}
/// " More information about a node from node_announcement."
/// " Optional because we store a Node entry after learning about it from"
/// " a channel announcement, but before receiving a node announcement."
#[no_mangle]
pub extern "C" fn NodeInfo_get_announcement_info(this_ptr: &NodeInfo) -> *const NodeAnnouncementInfo {
	let inner_val = &unsafe { &*this_ptr.inner }.announcement_info;
	let mut local_inner_val = if inner_val.is_none() { return std::ptr::null(); } else {  { Box::into_raw(Box::new(crate::routing::network_graph::NodeAnnouncementInfo { inner: &(*inner_val.as_ref().unwrap()) } )) } };
	local_inner_val
}
/// " More information about a node from node_announcement."
/// " Optional because we store a Node entry after learning about it from"
/// " a channel announcement, but before receiving a node announcement."
#[no_mangle]
pub extern "C" fn NodeInfo_set_announcement_info(this_ptr: &mut NodeInfo, mut val: NodeAnnouncementInfo) {
	let mut local_val = if val.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) } }) };
	unsafe { &mut *(this_ptr.inner as *mut lnNodeInfo) }.announcement_info = local_val;
}
#[no_mangle]
pub extern "C" fn NodeInfo_new(mut channels_arg: crate::c_types::derived::CVec_u64Z, mut lowest_inbound_channel_fees_arg: RoutingFees, mut announcement_info_arg: NodeAnnouncementInfo) -> NodeInfo {
	let mut local_channels_arg = Vec::new(); for mut item in channels_arg.into_rust().drain(..) { local_channels_arg.push( { item }); };
	let mut local_lowest_inbound_channel_fees_arg = if lowest_inbound_channel_fees_arg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(lowest_inbound_channel_fees_arg.inner.take_ptr() as *mut _) } }) };
	let mut local_announcement_info_arg = if announcement_info_arg.inner.is_null() { None } else { Some( { *unsafe { Box::from_raw(announcement_info_arg.inner.take_ptr() as *mut _) } }) };
	NodeInfo { inner: Box::into_raw(Box::new(lnNodeInfo {
		channels: local_channels_arg,
		lowest_inbound_channel_fees: local_lowest_inbound_channel_fees_arg,
		announcement_info: local_announcement_info_arg,
	}))}
}

use lightning::routing::network_graph::NetworkGraph as lnNetworkGraphImport;
type lnNetworkGraph = lnNetworkGraphImport;

/// " Represents the network as nodes and channels between them"
#[repr(C)]
pub struct NetworkGraph {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnNetworkGraph,
}

impl Drop for NetworkGraph {
	fn drop(&mut self) {
		if !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnNetworkGraph) };
		}
	}
}
#[no_mangle]
pub extern "C" fn NetworkGraph_free(this_ptr: NetworkGraph) { }
/// " Close a channel if a corresponding HTLC fail was sent."
/// " If permanent, removes a channel from the local storage."
/// " May cause the removal of nodes too, if this was their last channel."
/// " If not permanent, makes channels unavailable for routing."
#[no_mangle]
pub extern "C" fn NetworkGraph_close_channel_from_update(this_arg: &mut NetworkGraph, mut short_channel_id: u64, mut is_permanent: bool) {
	unsafe { &mut (*(this_arg.inner as *mut lnNetworkGraph)) }.close_channel_from_update(short_channel_id, is_permanent)
}

