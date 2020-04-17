use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct SocketDescriptor {
	socket_id: u8
}

impl lightning::ln::peer_handler::SocketDescriptor for SocketDescriptor{
	fn send_data(&mut self, data: &[u8], resume_read: bool) -> usize {
		unimplemented!()
	}

	fn disconnect_socket(&mut self) {
		unimplemented!()
	}
}

impl Hash for SocketDescriptor{
	fn hash<H: Hasher>(&self, state: &mut H) {
		unimplemented!()
	}
}

impl PartialEq for SocketDescriptor{
	fn eq(&self, other: &Self) -> bool {
		return self.socket_id == other.socket_id
	}
}
impl Eq for SocketDescriptor{}
