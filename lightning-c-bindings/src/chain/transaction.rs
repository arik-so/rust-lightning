//! " Contains simple structs describing parts of transactions on the chain."

use std::ffi::c_void;
use bitcoin::hashes::Hash;

use bitcoin::hash_types::Txid as lnTxid;

use lightning::chain::transaction::OutPoint as lnOutPointImport;
type lnOutPoint = lnOutPointImport;

/// " A reference to a transaction output."
/// ""
/// " Differs from bitcoin::blockdata::transaction::OutPoint as the index is a u16 instead of u32"
/// " due to LN's restrictions on index values. Should reduce (possibly) unsafe conversions this way."
#[repr(C)]
pub struct OutPoint {
	pub(crate) inner: *const lnOutPoint,
}

#[no_mangle]
pub extern "C" fn OutPoint_free(this_ptr: OutPoint) {
	let _ = unsafe { Box::from_raw(this_ptr.inner as *mut lnOutPoint) };
}
/// " The referenced transaction's txid."
#[no_mangle]
pub extern "C" fn OutPoint_get_txid(this_ptr: &OutPoint) -> *const [u8; 32] {
	&unsafe { &*this_ptr.inner }.txid.into_inner()
}
/// " The referenced transaction's txid."
#[no_mangle]
pub extern "C" fn OutPoint_set_txid(this_ptr: &mut OutPoint, val: [u8; 32]) {
	unsafe { &mut *(this_ptr.inner as *mut lnOutPoint) }.txid = ::bitcoin::hash_types::Txid::from_slice(&val[..]).unwrap();
}
/// " The index of the referenced output in its transaction's vout."
#[no_mangle]
pub extern "C" fn OutPoint_set_index(this_ptr: &mut OutPoint, val: u16) {
	unsafe { &mut *(this_ptr.inner as *mut lnOutPoint) }.index = val;
}
#[no_mangle]
pub extern "C" fn OutPoint_new(txid_arg: [u8; 32], index_arg: u16) -> OutPoint {
	OutPoint { inner: Box::into_raw(Box::new(lnOutPoint {
		txid: ::bitcoin::hash_types::Txid::from_slice(&txid_arg[..]).unwrap(),
		index: index_arg,
	}))}
}
/// " Convert an `OutPoint` to a lightning channel id."
#[no_mangle]
pub extern "C" fn OutPoint_to_channel_id(this_arg: & OutPoint) -> crate::c_types::ThirtyTwoBytes {
	crate::c_types::ThirtyTwoBytes { data: unsafe { &*this_arg.inner }.to_channel_id() }
}

