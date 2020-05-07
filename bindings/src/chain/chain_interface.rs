use std::ffi::c_void;
use bitcoin::consensus::encode;
use bitcoin::Transaction;
use lightning::chain::chaininterface::{BroadcasterInterface, ConfirmationTarget, FeeEstimator};
#[repr(C)]
pub struct lightning_BroadcasterInterface {
	pub this_arg: *const c_void,
	pub broadcast_transaction_fn: extern "C" fn(this_arg: *const c_void, serialized_tx: *const u8),
}
unsafe impl Sync for lightning_BroadcasterInterface {}
unsafe impl Send for lightning_BroadcasterInterface {}
impl BroadcasterInterface for lightning_BroadcasterInterface {
	fn broadcast_transaction(&self, tx: &Transaction) {
		let serialized_tx = encode::serialize(tx);
		unsafe {
			(self.broadcast_transaction_fn)(self.this_arg, serialized_tx[..].as_ptr());
		}
	}
}
#[no_mangle]
pub static lightning_ConfirmationTarget_Background: u8 = 1;
#[no_mangle]
pub static lightning_ConfirmationTarget_Normal: u8 = 2;
#[no_mangle]
pub static lightning_ConfirmationTarget_HighPriority: u8 = 3;
#[repr(C)]
pub struct lightning_FeeEstimator {
	pub this_arg: *const c_void,
	pub get_est_sat_per_1000_weight_fn: extern "C" fn(this_arg: *const c_void, confirmation_target: u8) -> u64,
}
unsafe impl Sync for lightning_FeeEstimator {}
unsafe impl Send for lightning_FeeEstimator {}
impl FeeEstimator for lightning_FeeEstimator {
	fn get_est_sat_per_1000_weight(&self, confirmation_target: ConfirmationTarget) -> u64 {
		let conf_target_flag = match confirmation_target {
			ConfirmationTarget::Background => lightning_ConfirmationTarget_Background,
			ConfirmationTarget::Normal => lightning_ConfirmationTarget_Normal,
			ConfirmationTarget::HighPriority => lightning_ConfirmationTarget_HighPriority,
		};
		unsafe { (self.get_est_sat_per_1000_weight_fn)(self.this_arg, conf_target_flag) }
	}
}