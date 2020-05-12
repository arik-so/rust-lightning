//! " The top-level routing/network map tracking logic lives here."
//! ""
//! " You probably want to create a NetGraphMsgHandler and use that as your RoutingMessageHandler and then"
//! " interrogate it to get routes for your own payments."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

use bitcoin::secp256k1::key::PublicKey as lnPublicKey;

use lightning::routing::router::RouteHop as lnRouteHopImport;
type lnRouteHop = lnRouteHopImport;

/// " A hop in a route"
#[repr(C)]
pub struct RouteHop {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRouteHop,
}

#[no_mangle]
pub extern "C" fn RouteHop_free(this_ptr: RouteHop) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnRouteHop) };
}
/// " The node_id of the node at this hop."
#[no_mangle]
pub extern "C" fn RouteHop_get_pubkey(this_ptr: &RouteHop) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.pubkey;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The node_id of the node at this hop."
#[no_mangle]
pub extern "C" fn RouteHop_set_pubkey(this_ptr: &mut RouteHop, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHop) }.pubkey = val.into_rust();
}
/// " The channel that should be used from the previous hop to reach this node."
#[no_mangle]
pub extern "C" fn RouteHop_set_short_channel_id(this_ptr: &mut RouteHop, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHop) }.short_channel_id = val;
}
/// " The fee taken on this hop. For the last hop, this should be the full value of the payment."
#[no_mangle]
pub extern "C" fn RouteHop_set_fee_msat(this_ptr: &mut RouteHop, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHop) }.fee_msat = val;
}
/// " The CLTV delta added for this hop. For the last hop, this should be the full CLTV value"
/// " expected at the destination, in excess of the current block height."
#[no_mangle]
pub extern "C" fn RouteHop_set_cltv_expiry_delta(this_ptr: &mut RouteHop, val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHop) }.cltv_expiry_delta = val;
}

use lightning::routing::router::Route as lnRouteImport;
type lnRoute = lnRouteImport;

/// " A route directs a payment from the sender (us) to the recipient. If the recipient supports MPP,"
/// " it can take multiple paths. Each path is composed of one or more hops through the network."
#[repr(C)]
pub struct Route {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRoute,
}

#[no_mangle]
pub extern "C" fn Route_free(this_ptr: Route) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnRoute) };
}

use lightning::routing::router::RouteHint as lnRouteHintImport;
type lnRouteHint = lnRouteHintImport;

/// " A channel descriptor which provides a last-hop route to get_route"
#[repr(C)]
pub struct RouteHint {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRouteHint,
}

#[no_mangle]
pub extern "C" fn RouteHint_free(this_ptr: RouteHint) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnRouteHint) };
}
/// " The node_id of the non-target end of the route"
#[no_mangle]
pub extern "C" fn RouteHint_get_src_node_id(this_ptr: &RouteHint) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.src_node_id;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The node_id of the non-target end of the route"
#[no_mangle]
pub extern "C" fn RouteHint_set_src_node_id(this_ptr: &mut RouteHint, val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.src_node_id = val.into_rust();
}
/// " The short_channel_id of this channel"
#[no_mangle]
pub extern "C" fn RouteHint_set_short_channel_id(this_ptr: &mut RouteHint, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.short_channel_id = val;
}
/// " The difference in CLTV values between this node and the next node."
#[no_mangle]
pub extern "C" fn RouteHint_set_cltv_expiry_delta(this_ptr: &mut RouteHint, val: u16) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.cltv_expiry_delta = val;
}
/// " The minimum value, in msat, which must be relayed to the next hop."
#[no_mangle]
pub extern "C" fn RouteHint_set_htlc_minimum_msat(this_ptr: &mut RouteHint, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.htlc_minimum_msat = val;
}
