use std::cell::Cell;
use std::ops::Deref;

use bitcoin_hashes::{Hash, HashEngine, Hmac, HmacEngine};
use bitcoin_hashes::sha256::Hash as Sha256;
use secp256k1::{PublicKey, SecretKey};

use ln::peers::handshake::acts::{ActOne, ActThree, ActTwo};
use ln::peers::handshake::states::{ActOneExpectation, HandshakeState, LastIncomingMessageExpectation};
use ln::peers::peer::ConnectedPeer;
use util::hkdf;

mod acts;
mod states;

struct PeerHandshake {
	state: Option<HandshakeState>,
	private_key: [u8; 32],

	// hash: HandshakeHash,
	// chaining_key: [u8; 32],
}

impl PeerHandshake {
	pub fn new(private_key: [u8; 32]) -> Self {
		let mut handshake = PeerHandshake {
			state: Some(HandshakeState::Blank),
			private_key,
			// hash: HandshakeHash::new(initial_hash_preimage.as_slice()),
			// chaining_key,
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


	pub fn initiate(&mut self, ephemeral_private_key: [u8; 32], remote_public_key: [u8; 33]) -> Result<ActOne, String> {
		if let Some(HandshakeState::Blank) = &self.state {} else {
			return Err("incorrect state".to_string());
		}

		let (hash, chaining_key) = Self::initialize_state(&remote_public_key);

		/*
		pre-process and serialize act one here
		*/

		self.state = Some(HandshakeState::AwaitingActTwo(LastIncomingMessageExpectation {
			hash,
			chaining_key,
			temporary_key: [0; 32],
		}));

		Ok(ActOne([0; 50]))
	}

	/*
	pub fn process(&mut self, data: &[u8]) -> Result<(Option<Act>, Option<ConnectedPeer>), String> {
		if let HandshakeState::Blank = self.state {
			self.state = self.state.advance(); // we are now awaiting act one
		}

		unimplemented!()
	}
	*/


	pub fn process_act_one(&mut self, act: ActOne) -> ActTwo {
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


		let version = act.0[0];

		let mut ephemeral_public_key = [0u8; 33];
		ephemeral_public_key.copy_from_slice(&act.0[1..34]);

		let mut chacha_tag = [0u8; 16];
		chacha_tag.copy_from_slice(&act.0[34..50]);

		// update the hash with the own public key
		// self.hash.update(Self::private_key_to_public_key(self.private_key).as_ref());

		// process the act message

		// update hash with partner's pubkey
		let mut hash = act_one_expectation.hash;
		hash.update(&ephemeral_public_key);

		// calculate ECDH with partner's pubkey and local privkey
		let ecdh = Self::ecdh(&self.private_key, &ephemeral_public_key);

		// HKDF(chaining key, ECDH) -> chaining key' + next temporary key
		let (chaining_key, temporary_key) = hkdf::derive(&act_one_expectation.chaining_key, &ecdh);

		// Validate chacha tag (temporary key, 0, self.hash, chacha_tag)
		// TODO: arik


		self.state = Some(HandshakeState::AwaitingActThree(LastIncomingMessageExpectation {
			hash,
			chaining_key,
			temporary_key,
		}));


		// serialize_act_two

		ActTwo([0; 50])
	}

	pub fn process_act_two(&mut self, act: ActTwo) -> (ActThree, ConnectedPeer) {
		unimplemented!()
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
		hash.update(first_input);
		hash
	}

	fn update(&mut self, input: &[u8]) {
		let mut sha = Sha256::engine();
		sha.input(self.value.as_ref());
		sha.input(input);
		self.value = Sha256::from_engine(sha).into_inner();
	}
}

#[cfg(test)]
mod tests {
	use secp256k1::key::{PublicKey, SecretKey};

	use ln::peers::handshake::{HandshakeState, PeerHandshake};

	#[test]
	fn test_exchange() {
		let local_private_key = [0x_11_u8; 32];
		let remote_private_key = [0x_21_u8; 32];

		let mut local_handshake = PeerHandshake::new(local_private_key);
		let mut remote_handshake = PeerHandshake::new(remote_private_key);

		let local_ephemeral_private_key = [0x_12_u8; 32];
		let remote_ephemeral_private_key = [0x_22_u8; 32];

		let remote_public_key = public_key_from_private_key(remote_private_key);

		let act_1_message = local_handshake.initiate(local_ephemeral_private_key, remote_public_key);
		let act_2_message = remote_handshake.process_act_one(act_1_message.unwrap());
		let act_3_message = local_handshake.process_act_two(act_2_message);
		remote_handshake.process_act_three(act_3_message.0);
	}

	fn public_key_from_private_key(private_key: [u8; 32]) -> [u8; 33] {
		let curve = secp256k1::Secp256k1::new();
		let sk_object = SecretKey::from_slice(private_key.as_ref()).unwrap();
		let pk_object = PublicKey::from_secret_key(&curve, &sk_object);
		pk_object.serialize()
	}
}