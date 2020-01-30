pub(crate) struct ConnectedPeer {
	pub(crate) sending_key: [u8; 32],
	pub(crate) receiving_key: [u8; 32],
	pub(crate) chaining_key: [u8; 32],

	pub(crate) receiving_nonce: u32,
	pub(crate) sending_nonce: u32,
}

impl ConnectedPeer {
	pub fn encrypt(buffer: &[u8]) -> Vec<u8> {
		unimplemented!()
	}

	pub fn decrypt(buffer: &[u8]) -> (Vec<u8>, &[u8]) {

		// unimplemented!()
		(Vec::new(), &buffer[2..])
	}
}