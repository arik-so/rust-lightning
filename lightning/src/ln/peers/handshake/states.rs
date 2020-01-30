use ln::peers::handshake::HandshakeHash;

// #[derive(Clone)]
pub enum HandshakeState {
	Blank,
	AwaitingActOne(ActOneExpectation),
	AwaitingActTwo(ActTwoExpectation),
	AwaitingActThree(ActThreeExpectation),
	Complete,
}

/*
impl HandshakeState {
	pub fn initiate(&self) -> Result<Self, String> {
		if let HandshakeState::Blank = self {
			return Ok(HandshakeState::AwaitingActTwo);
		}
		Err("Handshake can only be initiated from blank state".to_string())
	}

	pub fn advance(&self) -> Self {
		match self {
			HandshakeState::Blank => { HandshakeState::AwaitingActOne }
			HandshakeState::AwaitingActOne => { HandshakeState::AwaitingActThree }
			HandshakeState::AwaitingActTwo => { HandshakeState::Complete }
			_ => { HandshakeState::Complete }
		}
	}
}
*/

// #[derive(Clone)]
pub struct ActOneExpectation {
	pub(crate) hash: HandshakeHash,
	pub(crate) chaining_key: [u8; 32],
}

pub struct ActTwoExpectation {
	pub(crate) hash: HandshakeHash,
	pub(crate) chaining_key: [u8; 32],
	pub(crate) temporary_key: [u8; 32],
	pub(crate) ephemeral_private_key: [u8; 32],
}

// #[derive(Clone)]
pub struct ActThreeExpectation {
	pub(crate) hash: HandshakeHash,
	pub(crate) chaining_key: [u8; 32],
	pub(crate) temporary_key: [u8; 32],
	pub(crate) remote_ephemeral_public_key: [u8; 33],
}