use lightning::ln::channelmonitor::{HTLCUpdate, ChannelMonitorUpdate, ChannelMonitorUpdateErr};
use lightning::chain::transaction::OutPoint;
use crate::channels::channel_keys::ChannelKeys;

pub struct ChannelMonitor{}

impl<ChanSigner: lightning::chain::keysinterface::ChannelKeys> lightning::ln::channelmonitor::ManyChannelMonitor<ChanSigner> for ChannelMonitor{
	fn add_monitor(&self, funding_txo: OutPoint, monitor: lightning::ln::channelmonitor::ChannelMonitor<ChanSigner>) -> Result<(), ChannelMonitorUpdateErr> {
		unimplemented!()
	}

	fn update_monitor(&self, funding_txo: OutPoint, monitor: ChannelMonitorUpdate) -> Result<(), ChannelMonitorUpdateErr> {
		unimplemented!()
	}

	fn get_and_clear_pending_htlcs_updated(&self) -> Vec<HTLCUpdate> {
		unimplemented!()
	}
}