use crate::ln::channelmanager::ChannelManager;
use std::slice;
use bitcoin::secp256k1::PublicKey;
use crate::error::Error;

#[no_mangle]
pub extern "C" fn channel_manager_open_channel(this_arg: &ChannelManager, their_network_key: *const u8, channel_value_satoshis: u64, push_msat: u64, user_id: u64, error: *mut Error) {
	let public_key_slice = unsafe {
		assert!(!their_network_key.is_null());
		slice::from_raw_parts(their_network_key, 33)
	};
	let public_key_object = PublicKey::from_slice(public_key_slice).unwrap();
	let channel = unsafe { &*this_arg.inner }.create_channel(public_key_object, channel_value_satoshis, push_msat, user_id, None);
	if channel.is_err() {
		let ffi_error: Error = channel.err().unwrap().into();
		unsafe { std::ptr::write(error, ffi_error); }
	}
}
