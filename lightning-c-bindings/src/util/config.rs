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

#[no_mangle]
pub extern "C" fn UserConfig_free(this_ptr: UserConfig) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUserConfig) };
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
pub extern "C" fn ChannelHandshakeConfig_free(this_ptr: ChannelHandshakeConfig) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelHandshakeConfig) };
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
#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_new(minimum_depth_arg: u32, our_to_self_delay_arg: u16, our_htlc_minimum_msat_arg: u64) -> ChannelHandshakeConfig {
	ChannelHandshakeConfig { inner: Box::into_raw(Box::new(lnChannelHandshakeConfig {
		minimum_depth: minimum_depth_arg,
		our_to_self_delay: our_to_self_delay_arg,
		our_htlc_minimum_msat: our_htlc_minimum_msat_arg,
	}))}
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
pub extern "C" fn ChannelHandshakeLimits_free(this_ptr: ChannelHandshakeLimits) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelHandshakeLimits) };
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
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_new(min_funding_satoshis_arg: u64, max_htlc_minimum_msat_arg: u64, min_max_htlc_value_in_flight_msat_arg: u64, max_channel_reserve_satoshis_arg: u64, min_max_accepted_htlcs_arg: u16, min_dust_limit_satoshis_arg: u64, max_dust_limit_satoshis_arg: u64, max_minimum_depth_arg: u32, force_announced_channel_preference_arg: bool, their_to_self_delay_arg: u16) -> ChannelHandshakeLimits {
	ChannelHandshakeLimits { inner: Box::into_raw(Box::new(lnChannelHandshakeLimits {
		min_funding_satoshis: min_funding_satoshis_arg,
		max_htlc_minimum_msat: max_htlc_minimum_msat_arg,
		min_max_htlc_value_in_flight_msat: min_max_htlc_value_in_flight_msat_arg,
		max_channel_reserve_satoshis: max_channel_reserve_satoshis_arg,
		min_max_accepted_htlcs: min_max_accepted_htlcs_arg,
		min_dust_limit_satoshis: min_dust_limit_satoshis_arg,
		max_dust_limit_satoshis: max_dust_limit_satoshis_arg,
		max_minimum_depth: max_minimum_depth_arg,
		force_announced_channel_preference: force_announced_channel_preference_arg,
		their_to_self_delay: their_to_self_delay_arg,
	}))}
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
pub extern "C" fn ChannelConfig_free(this_ptr: ChannelConfig) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelConfig) };
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
#[no_mangle]
pub extern "C" fn ChannelConfig_new(fee_proportional_millionths_arg: u32, announced_channel_arg: bool, commit_upfront_shutdown_pubkey_arg: bool) -> ChannelConfig {
	ChannelConfig { inner: Box::into_raw(Box::new(lnChannelConfig {
		fee_proportional_millionths: fee_proportional_millionths_arg,
		announced_channel: announced_channel_arg,
		commit_upfront_shutdown_pubkey: commit_upfront_shutdown_pubkey_arg,
	}))}
}
