pub enum HandshakeState {
	Blank,
	AwaitingActOne,
	AwaitingActTwo,
	AwaitingActThree,
	Complete,
}

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