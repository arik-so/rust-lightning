//! " The top-level routing/network map tracking logic lives here."
//! ""
//! " You probably want to create a NetGraphMsgHandler and use that as your RoutingMessageHandler and then"
//! " interrogate it to get routes for your own payments."

use std::ffi::c_void;
use bitcoin::hashes::Hash;
use crate::c_types::*;


use lightning::routing::router::RouteHop as lnRouteHopImport;
type lnRouteHop = lnRouteHopImport;

/// " A hop in a route"
#[must_use]
#[repr(C)]
pub struct RouteHop {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRouteHop,
	pub _underlying_ref: bool,
}

impl Drop for RouteHop {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnRouteHop) };
		}
	}
}
#[no_mangle]
pub extern "C" fn RouteHop_free(this_ptr: RouteHop) { }
impl Clone for RouteHop {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
/// " The node_id of the node at this hop."
#[no_mangle]
pub extern "C" fn RouteHop_get_pubkey(this_ptr: &RouteHop) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.pubkey;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The node_id of the node at this hop."
#[no_mangle]
pub extern "C" fn RouteHop_set_pubkey(this_ptr: &mut RouteHop, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHop) }.pubkey = val.into_rust();
}
/// " The channel that should be used from the previous hop to reach this node."
#[no_mangle]
pub extern "C" fn RouteHop_get_short_channel_id(this_ptr: &RouteHop) -> u64 {
	let inner_val = &unsafe { &*this_ptr.inner }.short_channel_id;
	(*inner_val)
}
/// " The channel that should be used from the previous hop to reach this node."
#[no_mangle]
pub extern "C" fn RouteHop_set_short_channel_id(this_ptr: &mut RouteHop, mut val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHop) }.short_channel_id = val;
}
/// " The fee taken on this hop. For the last hop, this should be the full value of the payment."
#[no_mangle]
pub extern "C" fn RouteHop_get_fee_msat(this_ptr: &RouteHop) -> u64 {
	let inner_val = &unsafe { &*this_ptr.inner }.fee_msat;
	(*inner_val)
}
/// " The fee taken on this hop. For the last hop, this should be the full value of the payment."
#[no_mangle]
pub extern "C" fn RouteHop_set_fee_msat(this_ptr: &mut RouteHop, mut val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHop) }.fee_msat = val;
}
/// " The CLTV delta added for this hop. For the last hop, this should be the full CLTV value"
/// " expected at the destination, in excess of the current block height."
#[no_mangle]
pub extern "C" fn RouteHop_get_cltv_expiry_delta(this_ptr: &RouteHop) -> u32 {
	let inner_val = &unsafe { &*this_ptr.inner }.cltv_expiry_delta;
	(*inner_val)
}
/// " The CLTV delta added for this hop. For the last hop, this should be the full CLTV value"
/// " expected at the destination, in excess of the current block height."
#[no_mangle]
pub extern "C" fn RouteHop_set_cltv_expiry_delta(this_ptr: &mut RouteHop, mut val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHop) }.cltv_expiry_delta = val;
}

use lightning::routing::router::Route as lnRouteImport;
type lnRoute = lnRouteImport;

/// " A route directs a payment from the sender (us) to the recipient. If the recipient supports MPP,"
/// " it can take multiple paths. Each path is composed of one or more hops through the network."
#[must_use]
#[repr(C)]
pub struct Route {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRoute,
	pub _underlying_ref: bool,
}

impl Drop for Route {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnRoute) };
		}
	}
}
#[no_mangle]
pub extern "C" fn Route_free(this_ptr: Route) { }
impl Clone for Route {
	fn clone(&self) -> Self {
		Self {
			inner: Box::into_raw(Box::new(unsafe { &*self.inner }.clone())),
			_underlying_ref: false,
		}
	}
}
/// " The list of routes taken for a single (potentially-)multi-part payment. The pubkey of the"
/// " last RouteHop in each path must be the same."
/// " Each entry represents a list of hops, NOT INCLUDING our own, where the last hop is the"
/// " destination. Thus, this must always be at least length one. While the maximum length of any"
/// " given path is variable, keeping the length of any path to less than 20 should currently"
/// " ensure it is viable."
#[no_mangle]
pub extern "C" fn Route_set_paths(this_ptr: &mut Route, mut val: crate::c_types::derived::CVec_CVec_RouteHopZZ) {
	let mut local_val = Vec::new(); for mut item in val.into_rust().drain(..) { local_val.push( { let mut local_val_0 = Vec::new(); for mut item in item.into_rust().drain(..) { local_val_0.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); }; local_val_0 }); };
	unsafe { &mut *(this_ptr.inner as *mut lnRoute) }.paths = local_val;
}
#[must_use]
#[no_mangle]
pub extern "C" fn Route_new(mut paths_arg: crate::c_types::derived::CVec_CVec_RouteHopZZ) -> Route {
	let mut local_paths_arg = Vec::new(); for mut item in paths_arg.into_rust().drain(..) { local_paths_arg.push( { let mut local_paths_arg_0 = Vec::new(); for mut item in item.into_rust().drain(..) { local_paths_arg_0.push( { *unsafe { Box::from_raw(item.inner.take_ptr() as *mut _) } }); }; local_paths_arg_0 }); };
	Route { inner: Box::into_raw(Box::new(lnRoute {
		paths: local_paths_arg,
	})), _underlying_ref: false }
}
#[no_mangle]
pub extern "C" fn Route_write(obj: *const Route) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &(*(*obj).inner) })
}
#[no_mangle]
pub extern "C" fn Route_read(ser: crate::c_types::u8slice) -> Route {
	if let Ok(res) = crate::c_types::deserialize_obj(ser) {
		Route { inner: Box::into_raw(Box::new(res)), _underlying_ref: false }
	} else {
		Route { inner: std::ptr::null(), _underlying_ref: false }
	}
}

use lightning::routing::router::RouteHint as lnRouteHintImport;
type lnRouteHint = lnRouteHintImport;

/// " A channel descriptor which provides a last-hop route to get_route"
#[must_use]
#[repr(C)]
pub struct RouteHint {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnRouteHint,
	pub _underlying_ref: bool,
}

impl Drop for RouteHint {
	fn drop(&mut self) {
		if !self._underlying_ref && !self.inner.is_null() {
			let _ = unsafe { Box::from_raw(self.inner as *mut lnRouteHint) };
		}
	}
}
#[no_mangle]
pub extern "C" fn RouteHint_free(this_ptr: RouteHint) { }
/// " The node_id of the non-target end of the route"
#[no_mangle]
pub extern "C" fn RouteHint_get_src_node_id(this_ptr: &RouteHint) -> crate::c_types::PublicKey {
	let inner_val = &unsafe { &*this_ptr.inner }.src_node_id;
	crate::c_types::PublicKey::from_rust(&(*inner_val))
}
/// " The node_id of the non-target end of the route"
#[no_mangle]
pub extern "C" fn RouteHint_set_src_node_id(this_ptr: &mut RouteHint, mut val: crate::c_types::PublicKey) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.src_node_id = val.into_rust();
}
/// " The short_channel_id of this channel"
#[no_mangle]
pub extern "C" fn RouteHint_get_short_channel_id(this_ptr: &RouteHint) -> u64 {
	let inner_val = &unsafe { &*this_ptr.inner }.short_channel_id;
	(*inner_val)
}
/// " The short_channel_id of this channel"
#[no_mangle]
pub extern "C" fn RouteHint_set_short_channel_id(this_ptr: &mut RouteHint, mut val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.short_channel_id = val;
}
/// " The fees which must be paid to use this channel"
#[no_mangle]
pub extern "C" fn RouteHint_get_fees(this_ptr: &RouteHint) -> *const crate::routing::network_graph::RoutingFees {
	let inner_val = &unsafe { &*this_ptr.inner }.fees;
	Box::into_raw(Box::new(crate::routing::network_graph::RoutingFees { inner: &(*inner_val), _underlying_ref: true } ))
}
/// " The fees which must be paid to use this channel"
#[no_mangle]
pub extern "C" fn RouteHint_set_fees(this_ptr: &mut RouteHint, mut val: crate::routing::network_graph::RoutingFees) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.fees = *unsafe { Box::from_raw(val.inner.take_ptr() as *mut _) };
}
/// " The difference in CLTV values between this node and the next node."
#[no_mangle]
pub extern "C" fn RouteHint_get_cltv_expiry_delta(this_ptr: &RouteHint) -> u16 {
	let inner_val = &unsafe { &*this_ptr.inner }.cltv_expiry_delta;
	(*inner_val)
}
/// " The difference in CLTV values between this node and the next node."
#[no_mangle]
pub extern "C" fn RouteHint_set_cltv_expiry_delta(this_ptr: &mut RouteHint, mut val: u16) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.cltv_expiry_delta = val;
}
/// " The minimum value, in msat, which must be relayed to the next hop."
#[no_mangle]
pub extern "C" fn RouteHint_get_htlc_minimum_msat(this_ptr: &RouteHint) -> u64 {
	let inner_val = &unsafe { &*this_ptr.inner }.htlc_minimum_msat;
	(*inner_val)
}
/// " The minimum value, in msat, which must be relayed to the next hop."
#[no_mangle]
pub extern "C" fn RouteHint_set_htlc_minimum_msat(this_ptr: &mut RouteHint, mut val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnRouteHint) }.htlc_minimum_msat = val;
}
#[must_use]
#[no_mangle]
pub extern "C" fn RouteHint_new(mut src_node_id_arg: crate::c_types::PublicKey, mut short_channel_id_arg: u64, mut fees_arg: crate::routing::network_graph::RoutingFees, mut cltv_expiry_delta_arg: u16, mut htlc_minimum_msat_arg: u64) -> RouteHint {
	RouteHint { inner: Box::into_raw(Box::new(lnRouteHint {
		src_node_id: src_node_id_arg.into_rust(),
		short_channel_id: short_channel_id_arg,
		fees: *unsafe { Box::from_raw(fees_arg.inner.take_ptr() as *mut _) },
		cltv_expiry_delta: cltv_expiry_delta_arg,
		htlc_minimum_msat: htlc_minimum_msat_arg,
	})), _underlying_ref: false }
}
