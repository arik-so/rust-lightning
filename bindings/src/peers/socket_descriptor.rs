use std::hash::{Hash, Hasher};
use std::ffi::c_void;
use crate::buffer::BufferResponse;

use lightning::ln::peer_handler::SocketDescriptor as RawSocketDescriptor;

#[derive(Clone)]
pub struct SocketDescriptor {
	pub(crate) socket_id: u8,
	pub(crate) host_instance_pointer: *const c_void,
	pub(crate) send_data_callback: fn(*const c_void, *mut BufferResponse) -> usize
}

impl RawSocketDescriptor for SocketDescriptor{
	fn send_data(&mut self, data: &[u8], resume_read: bool) -> usize {
		let buffer: BufferResponse = data.into();
		let callback = self.send_data_callback;
		let read_offset = callback(self.host_instance_pointer, Box::into_raw(Box::new(buffer)));
		read_offset
	}

	fn disconnect_socket(&mut self) {
		unimplemented!()
	}
}

impl Hash for SocketDescriptor{
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.socket_id.hash(state)
	}
}

impl PartialEq for SocketDescriptor{
	fn eq(&self, other: &Self) -> bool {
		return self.socket_id == other.socket_id
	}
}
impl Eq for SocketDescriptor{}
