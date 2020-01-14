pub(crate) struct ConnectedPeer {
	sending_key: [u8; 32],
	receiving_key: [u8; 32],
	chaining_key: [u8; 32],

	receiving_nonce: u32,
	sending_nonce: u32,
}

impl ConnectedPeer {
	pub fn encrypt(buffer: &[u8]) -> Vec<u8> {}

	pub fn decrypt(buffer: &[u8]) -> Vec<u8> {}
}