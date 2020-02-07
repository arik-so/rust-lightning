use bitcoin_hashes::{Hash, HashEngine};
use bitcoin_hashes::sha256::Hash as Sha256;

pub(crate) struct HandshakeHash {
	pub(crate) value: [u8; 32]
}

impl HandshakeHash {
	pub(crate) fn new(first_input: &[u8]) -> Self {
		let mut hash = Self {
			value: [0; 32]
		};
		let mut sha = Sha256::engine();
		sha.input(first_input);
		hash.value = Sha256::from_engine(sha).into_inner();
		hash
	}

	pub(crate) fn update(&mut self, input: &[u8]) {
		let mut sha = Sha256::engine();
		sha.input(self.value.as_ref());
		sha.input(input);
		self.value = Sha256::from_engine(sha).into_inner();
	}
}