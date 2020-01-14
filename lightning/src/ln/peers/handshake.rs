use ln::peers::peer::ConnectedPeer;

struct ActOne([u8; 50]);

struct ActTwo([u8; 50]);

struct ActThree([u8; 50]);

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

struct PeerHandshake {
	state: HandshakeState,

	private_key: [u8; 32],
	ephemeral_private_key: [u8; 32],
}

impl PeerHandshake {
	pub fn new() -> Self {
		PeerHandshake {
			state: HandshakeState::Blank,
			private_key: [0; 32],
			ephemeral_private_key: [0; 32],
		}
	}

	pub fn initiate(&mut self) -> Result<ActOne, String> {
		self.state = self.state.initiate()?;
		Ok(ActOne([0; 50]))
	}

	pub fn from_data(data: &[u8]) -> (Self, Option<Act>) {
		// filter first 50 bytes
		unimplemented!()
	}

	pub fn process(&self, data: &[u8]) -> (Option<Act>, Option<ConnectedPeer>) {
		// filter first 50 bytes
		// data.len()
		unimplemented!()
	}

	pub fn process_act_one(act: ActOne) -> ActTwo {
		unimplemented!()
	}

	pub fn process_act_two(act: ActTwo) -> (ActThree, ConnectedPeer) {
		unimplemented!()
	}

	pub fn process_act_tree(act: ActThree) -> ConnectedPeer {
		unimplemented!()
	}
}