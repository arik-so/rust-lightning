#[repr(C)]
pub struct BufferArgument {
	data: *const u8,
	length: usize,
}

#[repr(C)]
pub struct BufferArgumentArray {
	buffers: *mut BufferArgument,
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

impl BufferArgumentArray {
	pub unsafe fn to_vec(&self) -> Vec<Vec<u8>> {
		if self.buffers.is_null() {
			return vec![];
		}
		let buffers = Vec::from_raw_parts(self.buffers, self.length, self.length);
		let mut data = vec![];
		for buffer in buffers {
			data.push(buffer.to_vec());
		}
		data
	}
}

#[repr(C)]
pub struct BufferResponse {
	data: *mut u8,
	length: usize,
}

#[repr(C)]
pub struct BufferResponseArray {
	buffers: *const BufferResponse,
	length: usize,
}

impl BufferResponse {
	pub fn into_mut_ptr(self) -> *mut Self {
		Box::into_raw(Box::new(self))
	}
}

impl BufferResponseArray {
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

impl From<&[u8]> for BufferResponse {
	fn from(bytes: &[u8]) -> Self {
		let bytes = bytes.to_vec();
		bytes.into()
	}
}

impl From<Vec<Vec<u8>>> for BufferResponseArray {
	fn from(byte_arrays: Vec<Vec<u8>>) -> Self {
		let mut buffer_response_arrays = Vec::new();
		for bytes in byte_arrays.iter() {
			let buffer_response: BufferResponse = bytes.clone().into();
			buffer_response_arrays.push(buffer_response);
		}

		let buffers = buffer_response_arrays.as_ptr();
		let length = buffer_response_arrays.len();
		Self { buffers, length }
	}
}

#[no_mangle]
pub extern "C" fn buffer_response_free(raw_buffer: *mut BufferResponse) {
	unsafe {
		if raw_buffer.is_null() { return; }
		let buffer = Box::from_raw(raw_buffer);
		let _ = std::slice::from_raw_parts_mut(buffer.data, buffer.length);
	};
}