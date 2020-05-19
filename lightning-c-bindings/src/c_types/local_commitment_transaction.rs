use lightning::ln::chan_utils::{LocalCommitmentTransaction as RawLocalCommitmentTransaction, LocalCommitmentTransaction};
use crate::buffer::BufferResponse;
use crate::c_types::tx_creation_keys::TxCreationKeySetResponse;
use crate::c_types::htlc_commitment::HTLCCommitmentResponseArray;

pub struct LocalCommitmentTransactionResponse {
	pub unsigned_tx: BufferResponse,
	pub their_sig: BufferResponse,
	pub our_sig_first: bool,
	pub local_keys: TxCreationKeySetResponse,
	pub feerate_per_kw: u64,
	pub per_htlc: HTLCCommitmentResponseArray,
}

impl From<RawLocalCommitmentTransaction> for LocalCommitmentTransactionResponse {
	fn from(commitment_tx: LocalCommitmentTransaction) -> Self {
		let LocalCommitmentTransaction {
			unsigned_tx,
			their_sig,
			our_sig_first,
			local_keys,
			feerate_per_kw,
			per_htlc,
		} = commitment_tx;
		let unsigned_tx: BufferResponse = unsigned_tx.into();
		let their_sig: BufferResponse = their_sig.into();
		let local_keys: TxCreationKeySetResponse = local_keys.into();
		let per_htlc: HTLCCommitmentResponseArray = per_htlc.into();
		LocalCommitmentTransactionResponse {
			unsigned_tx,
			their_sig,
			our_sig_first,
			local_keys,
			feerate_per_kw,
			per_htlc
		}
	}
}
