use lightning::chain::chaininterface::ConfirmationTarget;

pub struct FeeEstimator{}

impl lightning::chain::chaininterface::FeeEstimator for FeeEstimator{
	fn get_est_sat_per_1000_weight(&self, confirmation_target: ConfirmationTarget) -> u64 {
		unimplemented!()
	}
}