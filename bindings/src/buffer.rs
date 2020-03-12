#[repr(C)]
pub struct BufferArgument {
	data: *const u8,
	length: usize,
}

impl BufferArgument {
	pub fn into_mut_ptr(self) -> *mut Self {
		Box::into_raw(Box::new(self))
	}

	pub unsafe fn to_vec(&self) -> Vec<u8> {
		if self.data.is_null() {
			return vec![];
		}
		let data = std::slice::from_raw_parts(self.data, self.length);
		data.to_vec()
	}
}

#[repr(C)]
pub struct BufferResponse {
	data: *mut u8,
	length: usize,
}

impl BufferResponse {
	pub fn into_mut_ptr(self) -> *mut Self {
		Box::into_raw(Box::new(self))
	}
}

impl From<Vec<u8>> for BufferResponse {
	fn from(bytes: Vec<u8>) -> Self {
		let mut slice = bytes.into_boxed_slice();
		let data = slice.as_mut_ptr();
		let length = slice.len();

		std::mem::forget(slice);
		Self { data, length }
	}
}

#[no_mangle]
pub extern "C" fn free_buffer(raw_buffer: *mut BufferResponse) {
	unsafe {
		if raw_buffer.is_null() { return; }
		let buffer = Box::from_raw(raw_buffer);
		let _ = std::slice::from_raw_parts_mut(buffer.data, buffer.length);
	};
}