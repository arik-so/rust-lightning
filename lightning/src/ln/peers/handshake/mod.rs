use std::cell::Cell;
use std::ops::Deref;

use bitcoin_hashes::{Hash, HashEngine, Hmac, HmacEngine};
use bitcoin_hashes::sha256::Hash as Sha256;
use hex;
use secp256k1::{PublicKey, SecretKey};

use ln::peers::{chacha, hkdf};
use ln::peers::handshake::acts::{ActOne, ActThree, ActTwo};
use ln::peers::handshake::states::{ActOneExpectation, ActThreeExpectation, ActTwoExpectation, HandshakeState};
use ln::peers::peer::ConnectedPeer;

mod acts;
mod states;
mod tests;

struct PeerHandshake {
	state: Option<HandshakeState>,
	private_key: [u8; 32],
}

impl PeerHandshake {
	pub fn new(private_key: [u8; 32]) -> Self {
		let mut handshake = PeerHandshake {
			state: Some(HandshakeState::Blank),
			private_key,
		};
		handshake
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

	pub fn calculate_act_message(&self, local_private_key: &[u8; 32], remote_public_key: &[u8; 33], chaining_key: [u8; 32], hash: &mut HandshakeHash) -> ([u8; 50], [u8; 32], [u8; 32]) {
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

	pub fn initiate(&mut self, ephemeral_private_key: &[u8; 32], remote_public_key: &[u8; 33]) -> Result<ActOne, String> {
		if let Some(HandshakeState::Blank) = &self.state {} else {
			return Err("incorrect state".to_string());
		}

		let (mut hash, chaining_key) = Self::initialize_state(&remote_public_key);

		let (act_one, chaining_key, temporary_key) = self.calculate_act_message(ephemeral_private_key, remote_public_key, chaining_key, &mut hash);

		self.state = Some(HandshakeState::AwaitingActTwo(ActTwoExpectation {
			hash,
			chaining_key,
			temporary_key,
			ephemeral_private_key: ephemeral_private_key.clone(),
		}));

		// serialize act one
		Ok(ActOne(act_one))
	}

	fn process_act_message(&self, act_bytes: [u8; 50], local_private_key: &[u8; 32], chaining_key: [u8; 32], hash: &mut HandshakeHash) -> ([u8; 33], [u8; 32], [u8; 32]) {
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
		let mut chacha = chacha::decrypt(&temporary_key, 0, &hash.value, &chacha_tag).unwrap();

		hash.update(&chacha_tag);

		(ephemeral_public_key, chaining_key, temporary_key)
	}

	pub fn process_act_one(&mut self, act: ActOne, ephemeral_private_key: &[u8; 32]) -> ActTwo {
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
				panic!("unexpected state")
			}
		};

		let mut hash = act_one_expectation.hash;
		let (remote_ephemeral_public_key, chaining_key, temporary_key) = self.process_act_message(act.0,  &self.private_key, act_one_expectation.chaining_key, &mut hash);

		let (act_two, chaining_key, temporary_key) = self.calculate_act_message(ephemeral_private_key, &remote_ephemeral_public_key, chaining_key, &mut hash);

		println!("act 2: {:?}", hex::encode(&act_two.to_vec()));

		self.state = Some(HandshakeState::AwaitingActThree(ActThreeExpectation {
			hash,
			chaining_key,
			temporary_key,
			remote_ephemeral_public_key,
		}));

		ActTwo(act_two)
	}

	pub fn process_act_two(&mut self, act: ActTwo) -> (ActThree, ConnectedPeer) {
		let state = self.state.take();
		let act_two_expectation = match state {
			Some(HandshakeState::AwaitingActTwo(act_state)) => act_state,
			_ => {
				self.state = state;
				panic!("unexpected state!")
			}
		};

		let mut hash = act_two_expectation.hash;
		let (_, chaining_key, temporary_key) = self.process_act_message(act.0, &act_two_expectation.ephemeral_private_key, act_two_expectation.chaining_key, &mut hash);

		self.state = Some(HandshakeState::Complete);

		// start serializing act three

		unimplemented!();
	}

	pub fn process_act_three(&mut self, act: ActThree) -> ConnectedPeer {
		unimplemented!()
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

pub(crate) struct HandshakeHash {
	value: [u8; 32]
}

impl HandshakeHash {
	fn new(first_input: &[u8]) -> Self {
		let mut hash = Self {
			value: [0; 32]
		};
		let mut sha = Sha256::engine();
		sha.input(first_input);
		hash.value = Sha256::from_engine(sha).into_inner();
		hash
	}

	fn update(&mut self, input: &[u8]) {
		let mut sha = Sha256::engine();
		sha.input(self.value.as_ref());
		sha.input(input);
		self.value = Sha256::from_engine(sha).into_inner();
	}
}