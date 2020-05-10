//! " Various user-configurable channel limits and settings which ChannelManager"
//! " applies for you."

use std::ffi::c_void;
use bitcoin::hashes::Hash;


use lightning::util::config::UserConfig as lnUserConfigImport;
type lnUserConfig = lnUserConfigImport;

/// " Top-level config which holds ChannelHandshakeLimits and ChannelConfig."
/// ""
/// " Default::default() provides sane defaults for most configurations"
/// " (but currently with 0 relay fees!)"
#[repr(C)]
pub struct UserConfig {
	pub(crate) inner: *const lnUserConfig,
}


use lightning::util::config::ChannelHandshakeConfig as lnChannelHandshakeConfigImport;
type lnChannelHandshakeConfig = lnChannelHandshakeConfigImport;

/// " Configuration we set when applicable."
/// ""
/// " Default::default() provides sane defaults."
#[repr(C)]
pub struct ChannelHandshakeConfig {
	pub(crate) inner: *const lnChannelHandshakeConfig,
}

#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_set_minimum_depth(this_ptr: *mut ChannelHandshakeConfig, val: u32) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeConfig) }.minimum_depth = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_set_our_to_self_delay(this_ptr: *mut ChannelHandshakeConfig, val: u16) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeConfig) }.our_to_self_delay = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_set_our_htlc_minimum_msat(this_ptr: *mut ChannelHandshakeConfig, val: u64) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeConfig) }.our_htlc_minimum_msat = val;
}

use lightning::util::config::ChannelHandshakeLimits as lnChannelHandshakeLimitsImport;
type lnChannelHandshakeLimits = lnChannelHandshakeLimitsImport;

/// " Optional channel limits which are applied during channel creation."
/// ""
/// " These limits are only applied to our counterparty's limits, not our own."
/// ""
/// " Use 0/<type>::max_value() as appropriate to skip checking."
/// ""
/// " Provides sane defaults for most configurations."
/// ""
/// " Most additional limits are disabled except those with which specify a default in individual"
/// " field documentation. Note that this may result in barely-usable channels, but since they"
/// " are applied mostly only to incoming channels that's not much of a problem."
#[repr(C)]
pub struct ChannelHandshakeLimits {
	pub(crate) inner: *const lnChannelHandshakeLimits,
}

#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_min_funding_satoshis(this_ptr: *mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.min_funding_satoshis = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_max_htlc_minimum_msat(this_ptr: *mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.max_htlc_minimum_msat = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_min_max_htlc_value_in_flight_msat(this_ptr: *mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.min_max_htlc_value_in_flight_msat = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_max_channel_reserve_satoshis(this_ptr: *mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.max_channel_reserve_satoshis = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_min_max_accepted_htlcs(this_ptr: *mut ChannelHandshakeLimits, val: u16) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.min_max_accepted_htlcs = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_min_dust_limit_satoshis(this_ptr: *mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.min_dust_limit_satoshis = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_max_dust_limit_satoshis(this_ptr: *mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.max_dust_limit_satoshis = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_max_minimum_depth(this_ptr: *mut ChannelHandshakeLimits, val: u32) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.max_minimum_depth = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_force_announced_channel_preference(this_ptr: *mut ChannelHandshakeLimits, val: bool) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.force_announced_channel_preference = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_their_to_self_delay(this_ptr: *mut ChannelHandshakeLimits, val: u16) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelHandshakeLimits) }.their_to_self_delay = val;
}

use lightning::util::config::ChannelConfig as lnChannelConfigImport;
type lnChannelConfig = lnChannelConfigImport;

/// " Options which apply on a per-channel basis and may change at runtime or based on negotiation"
/// " with our counterparty."
#[repr(C)]
pub struct ChannelConfig {
	pub(crate) inner: *const lnChannelConfig,
}

#[no_mangle]
pub extern "C" fn ChannelConfig_set_fee_proportional_millionths(this_ptr: *mut ChannelConfig, val: u32) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelConfig) }.fee_proportional_millionths = val;
}
#[no_mangle]
pub extern "C" fn ChannelConfig_set_announced_channel(this_ptr: *mut ChannelConfig, val: bool) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelConfig) }.announced_channel = val;
}
#[no_mangle]
pub extern "C" fn ChannelConfig_set_commit_upfront_shutdown_pubkey(this_ptr: *mut ChannelConfig, val: bool) {
	unsafe { &mut *((*this_ptr).inner as *mut lnChannelConfig) }.commit_upfront_shutdown_pubkey = val;
}
