use ln::peers::handshake::hash::HandshakeHash;

pub enum HandshakeState {
	Blank,
	AwaitingActOne(ActOneExpectation),
	AwaitingActTwo(ActTwoExpectation),
	AwaitingActThree(ActThreeExpectation),
	Complete,
}

pub struct ActOneExpectation {
	pub(super) hash: HandshakeHash,
	pub(super) chaining_key: [u8; 32],
}

pub struct ActTwoExpectation {
	pub(super) hash: HandshakeHash,
	pub(super) chaining_key: [u8; 32],
	pub(super) temporary_key: [u8; 32],
	pub(super) ephemeral_private_key: [u8; 32],
}

pub struct ActThreeExpectation {
	pub(super) hash: HandshakeHash,
	pub(super) chaining_key: [u8; 32],
	pub(super) temporary_key: [u8; 32],
	pub(super) ephemeral_private_key: [u8; 32],
	pub(super) remote_ephemeral_public_key: [u8; 33],
}