use crate::c_types::htlc_commitment_output::HTLCOutputInCommitmentResponse;
use crate::buffer::BufferResponse;
use lightning::ln::chan_utils::{HTLCOutputInCommitment as RawHTLCOutputInCommitment, HTLCOutputInCommitment};
use bitcoin::secp256k1::{Signature as RawSignature, Signature};
use bitcoin::consensus::serialize;

#[no_mangle]
pub struct HTLCCommitmentResponse {
	output_in_commitment: HTLCOutputInCommitmentResponse,
	has_signature: bool,
	signature: BufferResponse,
}

#[no_mangle]
pub struct HTLCCommitmentResponseArray {
	htlc_commitments: *const HTLCCommitmentResponse,
	length: usize,
}

impl From<Vec<(RawHTLCOutputInCommitment, Option<RawSignature>)>> for HTLCCommitmentResponseArray {
	fn from(commitments: Vec<(HTLCOutputInCommitment, Option<Signature>)>) -> Self {
		let mut commitment_responses = Vec::new();
		for current_commitment in commitments.iter() {
			let current_commitment: HTLCCommitmentResponse = current_commitment.clone().into();
			commitment_responses.push(current_commitment);
		}

		let htlc_commitments = commitment_responses.as_ptr();
		let length = commitment_responses.len();
		Self { htlc_commitments, length }
	}
}

impl From<(RawHTLCOutputInCommitment, Option<RawSignature>)> for HTLCCommitmentResponse {
	fn from((commitment_output, signature_option): (HTLCOutputInCommitment, Option<RawSignature>)) -> Self {
		let commitment_output: HTLCOutputInCommitmentResponse = commitment_output.into();
		let has_signature = signature_option.is_some();
		let buffer_response: BufferResponse = match signature_option {
			None => { vec![].into() }
			Some(signature) => {
				signature.into()
			}
		};
		HTLCCommitmentResponse {
			output_in_commitment: commitment_output,
			has_signature,
			signature: buffer_response,
		}
	}
}
