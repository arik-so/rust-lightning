pub use ln::messaging::messages::ping::PingMessage as Ping;
pub use ln::messaging::messages::pong::PongMessage as Pong;

mod ping;
mod pong;

pub enum LightningMessage {
	Ping(Ping),
	Pong(Pong),
}