use std::ffi::c_void;

use lightning::chain::transaction::OutPoint;
use lightning::ln::channelmonitor::{ChannelMonitor as lnChannelMonitor, ChannelMonitorUpdate, ChannelMonitorUpdateErr, HTLCUpdate, ManyChannelMonitor as lnManyChannelMonitor, ChannelMonitor};

use crate::channels::keys_interface::ChannelKeys;

#[repr(C)]
pub struct ManyChannelMonitor {
	pub this_arg: *const c_void,
	//TODO: Add fns for callbacks
}

unsafe impl Sync for ManyChannelMonitor {}
unsafe impl Send for ManyChannelMonitor {}

impl lnManyChannelMonitor<ChannelKeys> for ManyChannelMonitor {
	fn add_monitor(&self, funding_txo: OutPoint, monitor: ChannelMonitor<ChannelKeys>) -> Result<(), ChannelMonitorUpdateErr> {
		unimplemented!()
	}

	fn update_monitor(&self, funding_txo: OutPoint, monitor: ChannelMonitorUpdate) -> Result<(), ChannelMonitorUpdateErr> {
		unimplemented!()
	}

	fn get_and_clear_pending_htlcs_updated(&self) -> Vec<HTLCUpdate> {
		unimplemented!()
	}
}

impl std::ops::Deref for ManyChannelMonitor {
	type Target = Self;
	fn deref(&self) -> &Self {
		self
	}
}