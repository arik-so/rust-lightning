use ln::peers::handshake::PeerHandshake;
use ln::peers::conduit::Conduit;
use secp256k1::{PublicKey, SecretKey};
use rand::{thread_rng, Rng};

enum PeerState {
	Handshake(PeerHandshake),
	Connected(Conduit),
}

struct Peer {
	ephemeral_private_key: SecretKey,
	remote_public_key: Option<PublicKey>,
	state: PeerState,

	pending_read_buffer: Vec<u8>,
	pending_write_buffer: Vec<u8>,

	inbox: Vec<Vec<u8>>,
}

impl Peer {
	pub fn handshake_complete(&self) -> bool {
		if let PeerState::Handshake(_) = &self.state {
			false
		} else {
			true
		}
	}

	pub fn new_inbound(private_key: SecretKey, ephemeral_private_key: Option<SecretKey>) -> Self {
		let handshake = PeerHandshake::new(&private_key);
		let ephemeral_private_key = ephemeral_private_key.unwrap_or(Self::generate_ephemeral_private_key());

		Peer {
			ephemeral_private_key,
			remote_public_key: None,
			state: PeerState::Handshake(handshake),

			pending_read_buffer: Vec::new(),
			pending_write_buffer: Vec::new(),
			inbox: Vec::new(),
		}
	}

	pub fn new_outbound(private_key: SecretKey, remote_pubkey: PublicKey, ephemeral_private_key: Option<SecretKey>) -> Self {
		let handshake = PeerHandshake::new(&private_key);
		let ephemeral_private_key = ephemeral_private_key.unwrap_or(Self::generate_ephemeral_private_key());

		Peer {
			ephemeral_private_key,
			remote_public_key: Some(remote_pubkey),
			state: PeerState::Handshake(handshake),

			pending_read_buffer: Vec::new(),
			pending_write_buffer: Vec::new(),
			inbox: Vec::new(),
		}
	}

	pub fn read_data(&mut self, buffer: &[u8]) {
		/*
		* First: is the handshake complete?
		* If not, give the data to the handshake object, having it return the overflow index
		* If the handshake completes the handshake, change state to the returned conduit
		* Second: if the handshake is now complete, feed data into its buffer
		* If it was just completed, use only overflow data
		* Otherwise, feed the entire buffer
		*/

		self.pending_read_buffer.extend_from_slice(buffer);

		// we need `&mut self.state` in order to use `ref mut`
		if let PeerState::Handshake(ref mut handshake) = &mut self.state {
			let result = handshake.process_act(&buffer, &self.ephemeral_private_key, self.remote_public_key.as_ref()).unwrap();

			let output = result.0;
			let offset = result.1;

			self.pending_write_buffer.extend_from_slice(&output);
			self.pending_read_buffer.drain(0..offset);

			if let Some(conduit) = result.2 {
				// update conduit
				self.state = PeerState::Connected(conduit);
			}
		}

		if let PeerState::Connected(ref mut conduit) = &mut self.state {
			let mut new_messages = conduit.decrypt_message_stream(Some(&self.pending_read_buffer));
			self.inbox.append(&mut new_messages);
		}
	}

	fn generate_ephemeral_private_key() -> SecretKey {
		let mut rng = thread_rng();
		let mut ephemeral_bytes = [0; 32];
		rng.fill_bytes(&mut ephemeral_bytes);
		SecretKey::from_slice(&ephemeral_bytes).expect("You broke elliptic curve cryptography")
	}
}