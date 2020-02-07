use bitcoin_hashes::{Hash, HashEngine};
use bitcoin_hashes::sha256::Hash as Sha256;
use secp256k1::{PublicKey, SecretKey};

use ln::peers::{chacha, hkdf};
use ln::peers::handshake::acts::{ActOne, ActThree, ActTwo};
use ln::peers::handshake::hash::HandshakeHash;
use ln::peers::handshake::states::{ActOneExpectation, ActThreeExpectation, ActTwoExpectation, HandshakeState};
use ln::peers::peer::ConnectedPeer;

mod acts;
mod hash;
mod states;
mod tests;

pub struct PeerHandshake {
	state: Option<HandshakeState>,
	private_key: [u8; 32],
	ephemeral_private_key: Option<[u8; 32]>,
	remote_public_key: Option<[u8; 33]>,
}

impl PeerHandshake {
	pub fn new(private_key: [u8; 32]) -> Self {
		let handshake = PeerHandshake {
			state: Some(HandshakeState::Blank),
			private_key,
			ephemeral_private_key: None,
			remote_public_key: None,
		};
		handshake
	}

	pub fn make_initiator(&mut self, ephemeral_private_key: &[u8; 32], remote_public_key: &[u8; 33]) {
		self.ephemeral_private_key = Some(ephemeral_private_key.clone());
		self.remote_public_key = Some(remote_public_key.clone());
	}

	pub fn make_responder(&mut self, ephemeral_private_key: &[u8; 32]) {
		self.ephemeral_private_key = Some(ephemeral_private_key.clone());

		let public_key = Self::private_key_to_public_key(&self.private_key);
		let (hash, chaining_key) = Self::initialize_state(&public_key);
		self.state = Some(HandshakeState::AwaitingActOne(ActOneExpectation {
			hash,
			chaining_key,
		}))
	}

	fn initialize_state(public_key: &[u8; 33]) -> (HandshakeHash, [u8; 32]) {
		// do the proper initialization
		let protocol_name = b"Noise_XK_secp256k1_ChaChaPoly_SHA256";
		let prologue = b"lightning";

		let mut sha = Sha256::engine();
		sha.input(protocol_name);
		let chaining_key = Sha256::from_engine(sha).into_inner();

		let mut initial_hash_preimage = chaining_key.to_vec();
		initial_hash_preimage.extend_from_slice(prologue.as_ref());

		let mut hash = HandshakeHash::new(initial_hash_preimage.as_slice());
		hash.update(public_key);

		(hash, chaining_key) // hash, chaining_key
	}

	/// Process act dynamically
	/// The role must be set before this method can be called
	pub fn process_act(&mut self, input: &[u8]) -> Result<(Vec<u8>, usize, Option<ConnectedPeer>), String> {
		let ephemeral_private_key = if let Some(ephemeral_private_key) = self.ephemeral_private_key {
			ephemeral_private_key
		} else {
			return Err("Call make_initiator() or make_responder() to configure role first".to_string());
		};

		let mut response: Vec<u8> = Vec::new();
		let mut offset = 0usize;
		let input_length = input.len();
		let mut connected_peer = None;

		let act_response = match &self.state {
			Some(HandshakeState::Blank) => {
				let remote_public_key = self.remote_public_key.ok_or("Call make_initiator() first")?;
				let act_one = self.initiate(&ephemeral_private_key, &remote_public_key)?;
				response = act_one.0.to_vec();
			}
			Some(HandshakeState::AwaitingActOne(_)) => {
				if input_length < 50 {
					return Err("need at least 50 bytes".to_string());
				}

				offset = 50;

				let mut act_one_buffer = [0u8; 50];
				act_one_buffer.copy_from_slice(&input);

				let act_two = self.process_act_one(ActOne(act_one_buffer), &ephemeral_private_key)?;
				response = act_two.0.to_vec();
			}
			Some(HandshakeState::AwaitingActTwo(_)) => {
				if input_length < 50 {
					return Err("need at least 50 bytes".to_string());
				}

				offset = 50;

				let mut act_two_buffer = [0u8; 50];
				act_two_buffer.copy_from_slice(&input);

				let (act_three, peer) = self.process_act_two(ActTwo(act_two_buffer))?;
				response = act_three.0.to_vec();
				connected_peer = Some(peer);
			}
			Some(HandshakeState::AwaitingActThree(_)) => {
				if input_length < 66 {
					return Err("need at least 50 bytes".to_string());
				}

				offset = 66;

				let mut act_three_buffer = [0u8; 66];
				act_three_buffer.copy_from_slice(&input);

				let peer = self.process_act_three(ActThree(act_three_buffer))?;
				connected_peer = Some(peer);
			}
			_ => {
				return Err("no acts left to process".to_string());
			}
		};
		Ok((response, offset, connected_peer))
	}

	pub fn initiate(&mut self, ephemeral_private_key: &[u8; 32], remote_public_key: &[u8; 33]) -> Result<ActOne, String> {
		if let Some(HandshakeState::Blank) = &self.state {} else {
			return Err("incorrect state".to_string());
		}

		let (mut hash, chaining_key) = Self::initialize_state(&remote_public_key);

		// serialize act one
		let (act_one, chaining_key, temporary_key) = self.calculate_act_message(
			ephemeral_private_key,
			remote_public_key,
			chaining_key,
			&mut hash,
		);

		self.state = Some(HandshakeState::AwaitingActTwo(ActTwoExpectation {
			hash,
			chaining_key,
			temporary_key,
			ephemeral_private_key: ephemeral_private_key.clone(),
		}));

		Ok(ActOne(act_one))
	}

	pub(crate) fn process_act_one(&mut self, act: ActOne, ephemeral_private_key: &[u8; 32]) -> Result<ActTwo, String> {
		let state = self.state.take();
		let act_one_expectation = match state {
			Some(HandshakeState::AwaitingActOne(act_state)) => act_state,
			Some(HandshakeState::Blank) => {
				// this can also be initiated from a blank state
				// public key
				let public_key = Self::private_key_to_public_key(&self.private_key);
				let (hash, chaining_key) = Self::initialize_state(&public_key);
				ActOneExpectation {
					hash,
					chaining_key,
				}
			}
			_ => {
				self.state = state;
				return Err("unexpected state".to_string());
			}
		};

		let mut hash = act_one_expectation.hash;
		let (remote_ephemeral_public_key, chaining_key, _) = self.process_act_message(
			act.0,
			&self.private_key,
			act_one_expectation.chaining_key,
			&mut hash,
		)?;

		let (act_two, chaining_key, temporary_key) = self.calculate_act_message(
			ephemeral_private_key,
			&remote_ephemeral_public_key,
			chaining_key,
			&mut hash,
		);

		self.state = Some(HandshakeState::AwaitingActThree(ActThreeExpectation {
			hash,
			chaining_key,
			temporary_key,
			ephemeral_private_key: ephemeral_private_key.clone(),
			remote_ephemeral_public_key,
		}));

		Ok(ActTwo(act_two))
	}

	pub(crate) fn process_act_two(&mut self, act: ActTwo) -> Result<(ActThree, ConnectedPeer), String> {
		let state = self.state.take();
		let act_two_expectation = match state {
			Some(HandshakeState::AwaitingActTwo(act_state)) => act_state,
			_ => {
				self.state = state;
				return Err("unexpected state".to_string());
			}
		};

		let mut hash = act_two_expectation.hash;
		let (remote_ephemeral_public_key, chaining_key, temporary_key) = self.process_act_message(
			act.0,
			&act_two_expectation.ephemeral_private_key,
			act_two_expectation.chaining_key,
			&mut hash,
		)?;

		self.state = Some(HandshakeState::Complete);

		// start serializing act three

		let static_public_key = Self::private_key_to_public_key(&self.private_key);
		let tagged_encrypted_pubkey = chacha::encrypt(&temporary_key, 1, &hash.value, &static_public_key);
		hash.update(&tagged_encrypted_pubkey);

		let ecdh = Self::ecdh(&self.private_key, &remote_ephemeral_public_key);
		let (chaining_key, temporary_key) = hkdf::derive(&chaining_key, &ecdh);
		let authentication_tag = chacha::encrypt(&temporary_key, 0, &hash.value, &[0; 0]);
		let (sending_key, receiving_key) = hkdf::derive(&chaining_key, &[0; 0]);

		let mut act_three_vec = [0u8].to_vec();
		act_three_vec.extend_from_slice(&tagged_encrypted_pubkey);
		act_three_vec.extend_from_slice(authentication_tag.as_slice());
		let mut act_three = [0u8; 66];
		act_three.copy_from_slice(act_three_vec.as_slice());

		let connected_peer = ConnectedPeer {
			sending_key,
			receiving_key,
			sending_chaining_key: chaining_key,
			receiving_chaining_key: chaining_key,
			sending_nonce: 0,
			receiving_nonce: 0,
		};
		Ok((ActThree(act_three), connected_peer))
	}

	pub(crate) fn process_act_three(&mut self, act: ActThree) -> Result<ConnectedPeer, String> {
		let state = self.state.take();
		let act_three_expectation = match state {
			Some(HandshakeState::AwaitingActThree(act_state)) => act_state,
			_ => {
				self.state = state;
				return Err("unexpected state".to_string());
			}
		};

		let version = act.0[0];

		let mut tagged_encrypted_pubkey = [0u8; 49];
		tagged_encrypted_pubkey.copy_from_slice(&act.0[1..50]);

		let mut chacha_tag = [0u8; 16];
		chacha_tag.copy_from_slice(&act.0[50..66]);

		let mut hash = act_three_expectation.hash;

		let remote_pubkey_vec = chacha::decrypt(&act_three_expectation.temporary_key, 1, &hash.value, &tagged_encrypted_pubkey)?;
		let mut remote_pubkey = [0u8; 33];
		remote_pubkey.copy_from_slice(remote_pubkey_vec.as_slice());

		hash.update(&tagged_encrypted_pubkey);

		let ecdh = Self::ecdh(&act_three_expectation.ephemeral_private_key, &remote_pubkey);
		let (chaining_key, temporary_key) = hkdf::derive(&act_three_expectation.chaining_key, &ecdh);
		let tag_check = chacha::decrypt(&temporary_key, 0, &hash.value, &chacha_tag)?;
		let (receiving_key, sending_key) = hkdf::derive(&chaining_key, &[0; 0]);

		let connected_peer = ConnectedPeer {
			sending_key,
			receiving_key,
			sending_chaining_key: chaining_key,
			receiving_chaining_key: chaining_key,
			sending_nonce: 0,
			receiving_nonce: 0,
		};
		Ok(connected_peer)
	}

	fn calculate_act_message(&self, local_private_key: &[u8; 32], remote_public_key: &[u8; 33], chaining_key: [u8; 32], hash: &mut HandshakeHash) -> ([u8; 50], [u8; 32], [u8; 32]) {
		let local_public_key = Self::private_key_to_public_key(local_private_key);

		hash.update(&local_public_key);

		let ecdh = Self::ecdh(local_private_key, &remote_public_key);
		let (chaining_key, temporary_key) = hkdf::derive(&chaining_key, &ecdh);
		let tagged_ciphertext = chacha::encrypt(&temporary_key, 0, &hash.value, &[0; 0]);

		hash.update(&tagged_ciphertext);

		let mut act_vec = [0u8].to_vec();
		act_vec.extend_from_slice(&local_public_key);
		act_vec.extend_from_slice(tagged_ciphertext.as_slice());
		let mut act = [0u8; 50];
		act.copy_from_slice(act_vec.as_slice());
		(act, chaining_key, temporary_key)
	}

	fn process_act_message(&self, act_bytes: [u8; 50], local_private_key: &[u8; 32], chaining_key: [u8; 32], hash: &mut HandshakeHash) -> Result<([u8; 33], [u8; 32], [u8; 32]), String> {
		let version = act_bytes[0];

		let mut ephemeral_public_key = [0u8; 33];
		ephemeral_public_key.copy_from_slice(&act_bytes[1..34]);

		let mut chacha_tag = [0u8; 16];
		chacha_tag.copy_from_slice(&act_bytes[34..50]);

		// process the act message

		// update hash with partner's pubkey
		hash.update(&ephemeral_public_key);

		// calculate ECDH with partner's pubkey and local privkey
		let ecdh = Self::ecdh(local_private_key, &ephemeral_public_key);

		// HKDF(chaining key, ECDH) -> chaining key' + next temporary key
		let (chaining_key, temporary_key) = hkdf::derive(&chaining_key, &ecdh);

		// Validate chacha tag (temporary key, 0, self.hash, chacha_tag)
		let mut chacha = chacha::decrypt(&temporary_key, 0, &hash.value, &chacha_tag)?;

		hash.update(&chacha_tag);

		Ok((ephemeral_public_key, chaining_key, temporary_key))
	}

	fn private_key_to_public_key(private_key: &[u8; 32]) -> [u8; 33] {
		let curve = secp256k1::Secp256k1::new();
		let sk_object = SecretKey::from_slice(private_key).unwrap();
		let pk_object = PublicKey::from_secret_key(&curve, &sk_object);
		pk_object.serialize()
	}

	fn ecdh(private_key: &[u8; 32], public_key: &[u8; 33]) -> [u8; 32] {
		let curve = secp256k1::Secp256k1::new();
		let mut pk_object = PublicKey::from_slice(public_key).unwrap();
		pk_object.mul_assign(&curve, private_key);

		let preimage = pk_object.serialize();
		let mut sha = Sha256::engine();
		sha.input(preimage.as_ref());
		Sha256::from_engine(sha).into_inner()
	}
}