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
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnUserConfig,
}

#[no_mangle]
pub extern "C" fn UserConfig_free(this_ptr: UserConfig) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnUserConfig) };
}
#[no_mangle]
pub extern "C" fn UserConfig_default() -> UserConfig {
	UserConfig { inner: Box::into_raw(Box::new(Default::default())) }
}

use lightning::util::config::ChannelHandshakeConfig as lnChannelHandshakeConfigImport;
type lnChannelHandshakeConfig = lnChannelHandshakeConfigImport;

/// " Configuration we set when applicable."
/// ""
/// " Default::default() provides sane defaults."
#[repr(C)]
pub struct ChannelHandshakeConfig {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelHandshakeConfig,
}

#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_free(this_ptr: ChannelHandshakeConfig) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelHandshakeConfig) };
}
/// " Confirmations we will wait for before considering the channel locked in."
/// " Applied only for inbound channels (see ChannelHandshakeLimits::max_minimum_depth for the"
/// " equivalent limit applied to outbound channels)."
/// ""
/// " Default value: 6."
#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_set_minimum_depth(this_ptr: &mut ChannelHandshakeConfig, val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeConfig) }.minimum_depth = val;
}
/// " Set to the amount of time we require our counterparty to wait to claim their money."
/// ""
/// " It's one of the main parameter of our security model. We (or one of our watchtowers) MUST"
/// " be online to check for peer having broadcast a revoked transaction to steal our funds"
/// " at least once every our_to_self_delay blocks."
/// ""
/// " Meanwhile, asking for a too high delay, we bother peer to freeze funds for nothing in"
/// " case of an honest unilateral channel close, which implicitly decrease the economic value of"
/// " our channel."
/// ""
/// " Default value: BREAKDOWN_TIMEOUT (currently 144), we enforce it as a minimum at channel"
/// " opening so you can tweak config to ask for more security, not less."
#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_set_our_to_self_delay(this_ptr: &mut ChannelHandshakeConfig, val: u16) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeConfig) }.our_to_self_delay = val;
}
/// " Set to the smallest value HTLC we will accept to process."
/// ""
/// " This value is sent to our counterparty on channel-open and we close the channel any time"
/// " our counterparty misbehaves by sending us an HTLC with a value smaller than this."
/// ""
/// " Default value: 1. If the value is less than 1, it is ignored and set to 1, as is required"
/// " by the protocol."
#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_set_our_htlc_minimum_msat(this_ptr: &mut ChannelHandshakeConfig, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeConfig) }.our_htlc_minimum_msat = val;
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_new(minimum_depth_arg: u32, our_to_self_delay_arg: u16, our_htlc_minimum_msat_arg: u64) -> ChannelHandshakeConfig {
	ChannelHandshakeConfig { inner: Box::into_raw(Box::new(lnChannelHandshakeConfig {
		minimum_depth: minimum_depth_arg,
		our_to_self_delay: our_to_self_delay_arg,
		our_htlc_minimum_msat: our_htlc_minimum_msat_arg,
	}))}
}
#[no_mangle]
pub extern "C" fn ChannelHandshakeConfig_default() -> ChannelHandshakeConfig {
	ChannelHandshakeConfig { inner: Box::into_raw(Box::new(Default::default())) }
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
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelHandshakeLimits,
}

#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_free(this_ptr: ChannelHandshakeLimits) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelHandshakeLimits) };
}
/// " Minimum allowed satoshis when a channel is funded, this is supplied by the sender and so"
/// " only applies to inbound channels."
/// ""
/// " Default value: 0."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_min_funding_satoshis(this_ptr: &mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.min_funding_satoshis = val;
}
/// " The remote node sets a limit on the minimum size of HTLCs we can send to them. This allows"
/// " you to limit the maximum minimum-size they can require."
/// ""
/// " Default value: u64::max_value."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_max_htlc_minimum_msat(this_ptr: &mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.max_htlc_minimum_msat = val;
}
/// " The remote node sets a limit on the maximum value of pending HTLCs to them at any given"
/// " time to limit their funds exposure to HTLCs. This allows you to set a minimum such value."
/// ""
/// " Default value: 0."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_min_max_htlc_value_in_flight_msat(this_ptr: &mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.min_max_htlc_value_in_flight_msat = val;
}
/// " The remote node will require we keep a certain amount in direct payment to ourselves at all"
/// " time, ensuring that we are able to be punished if we broadcast an old state. This allows to"
/// " you limit the amount which we will have to keep to ourselves (and cannot use for HTLCs)."
/// ""
/// " Default value: u64::max_value."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_max_channel_reserve_satoshis(this_ptr: &mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.max_channel_reserve_satoshis = val;
}
/// " The remote node sets a limit on the maximum number of pending HTLCs to them at any given"
/// " time. This allows you to set a minimum such value."
/// ""
/// " Default value: 0."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_min_max_accepted_htlcs(this_ptr: &mut ChannelHandshakeLimits, val: u16) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.min_max_accepted_htlcs = val;
}
/// " Outputs below a certain value will not be added to on-chain transactions. The dust value is"
/// " required to always be higher than this value so this only applies to HTLC outputs (and"
/// " potentially to-self outputs before any payments have been made)."
/// " Thus, HTLCs below this amount plus HTLC transaction fees are not enforceable on-chain."
/// " This setting allows you to set a minimum dust limit for their commitment transactions,"
/// " reflecting the reality that tiny outputs are not considered standard transactions and will"
/// " not propagate through the Bitcoin network."
/// ""
/// " Default value: 546, the current dust limit on the Bitcoin network."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_min_dust_limit_satoshis(this_ptr: &mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.min_dust_limit_satoshis = val;
}
/// " Maximum allowed threshold above which outputs will not be generated in their commitment"
/// " transactions."
/// " HTLCs below this amount plus HTLC transaction fees are not enforceable on-chain."
/// ""
/// " Default value: u64::max_value."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_max_dust_limit_satoshis(this_ptr: &mut ChannelHandshakeLimits, val: u64) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.max_dust_limit_satoshis = val;
}
/// " Before a channel is usable the funding transaction will need to be confirmed by at least a"
/// " certain number of blocks, specified by the node which is not the funder (as the funder can"
/// " assume they aren't going to double-spend themselves)."
/// " This config allows you to set a limit on the maximum amount of time to wait."
/// ""
/// " Default value: 144, or roughly one day and only applies to outbound channels."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_max_minimum_depth(this_ptr: &mut ChannelHandshakeLimits, val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.max_minimum_depth = val;
}
/// " Set to force the incoming channel to match our announced channel preference in"
/// " ChannelConfig."
/// ""
/// " Default value: true, to make the default that no announced channels are possible (which is"
/// " appropriate for any nodes which are not online very reliably)."
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_force_announced_channel_preference(this_ptr: &mut ChannelHandshakeLimits, val: bool) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.force_announced_channel_preference = val;
}
/// " Set to the amount of time we're willing to wait to claim money back to us."
/// ""
/// " Not checking this value would be a security issue, as our peer would be able to set it to"
/// " max relative lock-time (a year) and we would \"lose\" money as it would be locked for a long time."
/// ""
/// " Default value: MAX_LOCAL_BREAKDOWN_TIMEOUT (1008), which we also enforce as a maximum value"
/// " so you can tweak config to reduce the loss of having useless locked funds (if your peer accepts)"
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_set_their_to_self_delay(this_ptr: &mut ChannelHandshakeLimits, val: u16) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelHandshakeLimits) }.their_to_self_delay = val;
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
#[no_mangle]
pub extern "C" fn ChannelHandshakeLimits_default() -> ChannelHandshakeLimits {
	ChannelHandshakeLimits { inner: Box::into_raw(Box::new(Default::default())) }
}

use lightning::util::config::ChannelConfig as lnChannelConfigImport;
type lnChannelConfig = lnChannelConfigImport;

/// " Options which apply on a per-channel basis and may change at runtime or based on negotiation"
/// " with our counterparty."
#[repr(C)]
pub struct ChannelConfig {
	/// Nearly everyhwere, inner must be non-null, however in places where
	///the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *const lnChannelConfig,
}

#[no_mangle]
pub extern "C" fn ChannelConfig_free(this_ptr: ChannelConfig) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnChannelConfig) };
}
/// " Amount (in millionths of a satoshi) the channel will charge per transferred satoshi."
/// " This may be allowed to change at runtime in a later update, however doing so must result in"
/// " update messages sent to notify all nodes of our updated relay fee."
/// ""
/// " Default value: 0."
#[no_mangle]
pub extern "C" fn ChannelConfig_set_fee_proportional_millionths(this_ptr: &mut ChannelConfig, val: u32) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelConfig) }.fee_proportional_millionths = val;
}
/// " Set to announce the channel publicly and notify all nodes that they can route via this"
/// " channel."
/// ""
/// " This should only be set to true for nodes which expect to be online reliably."
/// ""
/// " As the node which funds a channel picks this value this will only apply for new outbound"
/// " channels unless ChannelHandshakeLimits::force_announced_channel_preferences is set."
/// ""
/// " This cannot be changed after the initial channel handshake."
/// ""
/// " Default value: false."
#[no_mangle]
pub extern "C" fn ChannelConfig_set_announced_channel(this_ptr: &mut ChannelConfig, val: bool) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelConfig) }.announced_channel = val;
}
/// " When set, we commit to an upfront shutdown_pubkey at channel open. If our counterparty"
/// " supports it, they will then enforce the mutual-close output to us matches what we provided"
/// " at intialization, preventing us from closing to an alternate pubkey."
/// ""
/// " This is set to true by default to provide a slight increase in security, though ultimately"
/// " any attacker who is able to take control of a channel can just as easily send the funds via"
/// " lightning payments, so we never require that our counterparties support this option."
/// ""
/// " This cannot be changed after a channel has been initialized."
/// ""
/// " Default value: true."
#[no_mangle]
pub extern "C" fn ChannelConfig_set_commit_upfront_shutdown_pubkey(this_ptr: &mut ChannelConfig, val: bool) {
	unsafe { &mut *(this_ptr.inner as *mut lnChannelConfig) }.commit_upfront_shutdown_pubkey = val;
}
#[no_mangle]
pub extern "C" fn ChannelConfig_new(fee_proportional_millionths_arg: u32, announced_channel_arg: bool, commit_upfront_shutdown_pubkey_arg: bool) -> ChannelConfig {
	ChannelConfig { inner: Box::into_raw(Box::new(lnChannelConfig {
		fee_proportional_millionths: fee_proportional_millionths_arg,
		announced_channel: announced_channel_arg,
		commit_upfront_shutdown_pubkey: commit_upfront_shutdown_pubkey_arg,
	}))}
}
#[no_mangle]
pub extern "C" fn ChannelConfig_default() -> ChannelConfig {
	ChannelConfig { inner: Box::into_raw(Box::new(Default::default())) }
}
