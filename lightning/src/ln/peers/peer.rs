use test::Concurrent::No;

use ln::peers::{chacha, hkdf};

pub(crate) struct ConnectedPeer {
	pub(crate) sending_key: [u8; 32],
	pub(crate) receiving_key: [u8; 32],
	pub(crate) sending_chaining_key: [u8; 32],
	pub(crate) receiving_chaining_key: [u8; 32],

	pub(crate) receiving_nonce: u32,
	pub(crate) sending_nonce: u32,
}

impl ConnectedPeer {
	pub fn encrypt(&mut self, buffer: &[u8]) -> Vec<u8> {
		unimplemented!()
	}

	pub fn decrypt<'a>(&mut self, buffer: &'a [u8]) -> (Option<Vec<u8>>, &'a [u8]) { // the response slice should have the same lifetime as the argument. It's the slice data is read from
		if buffer.len() < 18 {
			return (None, buffer);
		}

		let encrypted_length = &buffer[0..18]; // todo: abort if too short
		let length_vec = chacha::decrypt(&self.receiving_key, self.receiving_nonce as u64, &[0; 0], encrypted_length).unwrap();
		let mut length_bytes = [0u8; 2];
		length_bytes.copy_from_slice(length_vec.as_slice());
		let message_length = u16::from_be_bytes(length_bytes) as usize;

		let message_end_index = message_length + 18; // todo: abort if too short
		if buffer.len() < message_end_index {
			return (None, buffer);
		}

		let encrypted_message = &buffer[18..message_end_index];

		self.increment_receiving_nonce();

		let message = chacha::decrypt(&self.receiving_key, self.receiving_nonce as u64, &[0; 0], encrypted_message).unwrap();

		self.increment_receiving_nonce();

		let unread_buffer = &buffer[message_end_index..];

		(Some(message), unread_buffer)
	}

	fn increment_sending_nonce(&mut self) {
		Self::increment_nonce(&mut self.sending_nonce, &mut self.sending_chaining_key, &mut self.sending_key);
	}

	fn increment_receiving_nonce(&mut self) {
		Self::increment_nonce(&mut self.receiving_nonce, &mut self.receiving_chaining_key, &mut self.receiving_key);
	}

	fn increment_nonce(nonce: &mut u32, chaining_key: &mut [u8; 32], key: &mut [u8; 32]) {
		*nonce += 1;
		if *nonce == 1000 {
			Self::rotate_key(chaining_key, key);
			*nonce = 0;
		}
	}

	fn rotate_key(chaining_key: &mut [u8; 32], key: &mut [u8; 32]) {
		let (new_chaining_key, new_key) = hkdf::derive(chaining_key, key);
		chaining_key.copy_from_slice(&new_chaining_key);
		key.copy_from_slice(&new_key);
	}
}