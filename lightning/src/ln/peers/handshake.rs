use bitcoin_hashes::{Hash, HashEngine, Hmac, HmacEngine};
use bitcoin_hashes::sha256::Hash as Sha256;

use ln::peers::peer::ConnectedPeer;

struct PeerHandshake {
	state: HandshakeState,
	private_key: [u8; 32],

	hash: HandshakeHash,
	chaining_key: [u8; 32],
}

impl PeerHandshake {
	pub fn new(private_key: [u8; 32]) -> Self {
		let protocol_name = b"Noise_XK_secp256k1_ChaChaPoly_SHA256";
		let prologue = b"lightning";

		let mut sha = Sha256::engine();
		sha.input(protocol_name);
		let chaining_key = Sha256::from_engine(sha).into_inner();

		let mut initial_hash_preimage = chaining_key.to_vec();
		initial_hash_preimage.extend_from_slice(prologue.as_ref());

		let mut handshake = PeerHandshake {
			state: HandshakeState::Blank,
			private_key,
			hash: HandshakeHash::new(initial_hash_preimage.as_slice()),
			chaining_key,
		};
		handshake
	}


	pub fn initiate(&mut self, ephemeral_private_key: [u8; 32], remote_public_key: [u8; 33]) -> Result<ActOne, String> {
		self.state = self.state.initiate()?;
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
		self.hash.update(remote_public_key.as_ref());

		let version = act.0[0];
		let ephemeral_private_key: [u8; 33] = act.0[1..34].try_into().unwrap();
		let chacha_tag: [u8; 32] = act.0[34..50].try_into().unwrap();

		ActTwo([0; 50])
	}

	pub fn process_act_two(&mut self, act: ActTwo) -> (ActThree, ConnectedPeer) {
		unimplemented!()
	}

	pub fn process_act_tree(&mut self, act: ActThree) -> ConnectedPeer {
		unimplemented!()
	}
}

struct ActOne([u8; 50]);

struct ActTwo([u8; 50]);

struct ActThree([u8; 66]);

enum Act {
	One(ActOne),
	Two(ActTwo),
	Three(ActThree),
}

impl Act {
	pub fn serialize(&self) -> Vec<u8> {
		match self {
			Act::One(act) => {
				act.0.to_vec()
			}
			Act::Two(act) => {
				act.0.to_vec()
			}
			Act::Three(act) => {
				act.0.to_vec()
			}
		}
	}
}

enum HandshakeState {
	Blank,
	AwaitingActOne,
	AwaitingActTwo,
	AwaitingActThree,
	Complete,
}

impl HandshakeState {
	fn initiate(&self) -> Result<Self, String> {
		if let HandshakeState::Blank = self {
			return Ok(HandshakeState::AwaitingActTwo);
		}
		Err("Handshake can only be initiated from blank state".to_string())
	}

	fn advance(&self) -> Self {
		match self {
			HandshakeState::Blank => { HandshakeState::AwaitingActOne }
			HandshakeState::AwaitingActOne => { HandshakeState::AwaitingActThree }
			HandshakeState::AwaitingActTwo => { HandshakeState::Complete }
			_ => { HandshakeState::Complete }
		}
	}
}

struct HandshakeHash {
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
		remote_handshake.process_act_tree(act_3_message.0);
	}

	fn public_key_from_private_key(private_key: [u8; 32]) -> [u8; 33] {
		let curve = secp256k1::Secp256k1::new();
		let sk_object = SecretKey::from_slice(private_key.as_ref()).unwrap();
		let pk_object = PublicKey::from_secret_key(&curve, &sk_object);
		pk_object.serialize()
	}
}