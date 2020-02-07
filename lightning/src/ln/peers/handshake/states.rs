use ln::peers::handshake::hash::HandshakeHash;

pub enum HandshakeState {
	Blank,
	AwaitingActOne(ActOneExpectation),
	AwaitingActTwo(ActTwoExpectation),
	AwaitingActThree(ActThreeExpectation),
	Complete,
}

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

pub struct ActThreeExpectation {
	pub(crate) hash: HandshakeHash,
	pub(crate) chaining_key: [u8; 32],
	pub(crate) temporary_key: [u8; 32],
	pub(crate) ephemeral_private_key: [u8; 32],
	pub(crate) remote_ephemeral_public_key: [u8; 33],
}