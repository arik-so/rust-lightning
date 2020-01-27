pub use ln::messaging::messages::ping::PingMessage as Ping;
pub use ln::messaging::messages::pong::PongMessage as Pong;
pub use ln::messaging::messages::query_channel_range::QueryChannelRangeMessage as QueryChannelRange;
use ln::messaging::serde::Serde;

mod ping;
mod pong;
mod query_channel_range;

pub enum LightningMessageId {
	Init = 16,
	Error = 17,
	Ping = 18,
	Pong = 19,

	OpenChannel = 32,
	AcceptChannel = 33,
	FundingCreated = 34,
	FundingSigned = 35,
	FundingLocked = 36,

	Shutdown = 38,
	ClosingSigned = 39,

	UpdateAddHtlc = 128,
	UpdateFulfillHtlc = 130,
	UpdateFailHtlc = 131,
	UpdateFailMalformedHtlc = 135,

	ChannelAnnouncement = 256,
	NodeAnnouncement = 257,
	ChannelUpdate = 258,

	QueryShortChannelIds = 261,
	ReplyShortChannelIdsEnd = 262,
	QueryChannelRange = 263,
	ReplyChannelRange = 264,
}

#[derive(Debug)]
pub enum LightningMessage {
	Ping(Ping),
	Pong(Pong),
	QueryChannelRange(QueryChannelRange),
}

impl LightningMessage {
	pub fn parse(buffer: &[u8]) -> LightningMessage {
		let id_slice = &buffer[0..2];
		let mut id_bytes = [0; 2];
		id_bytes.copy_from_slice(id_slice);

		let id = u16::from_be_bytes(id_bytes);
		match id {
			id if id == LightningMessageId::Ping as u16 => LightningMessage::Ping(*Ping::parse(buffer)),
			id if id == LightningMessageId::Pong as u16 => LightningMessage::Pong(*Pong::parse(buffer)),
			id if id == LightningMessageId::QueryChannelRange as u16 => LightningMessage::QueryChannelRange(*QueryChannelRange::parse(buffer)),
			_ => {
				unimplemented!()
			}
		}
	}

	pub fn serialize(&self) -> Vec<u8>{
		match self {
			LightningMessage::Ping(m) => m.serialize(),
			LightningMessage::Pong(m) => m.serialize(),
			LightningMessage::QueryChannelRange(m) => m.serialize(),
			_ => panic!()
		}
	}


}