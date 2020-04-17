use bitcoin::Transaction;

pub struct Broadcaster{}

impl lightning::chain::chaininterface::BroadcasterInterface for Broadcaster{
	fn broadcast_transaction(&self, tx: &Transaction) {
		unimplemented!()
	}
}