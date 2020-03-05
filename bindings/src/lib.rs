extern crate lightning;

mod util;

pub use lightning::ln::peers::conduit::Conduit;
use lightning::ln::peer_handler::PeerManager;
use std::slice;
use secp256k1::PublicKey;
use lightning::ln::peers::handshake::PeerHandshake;

#[no_mangle]
pub extern fn conduit_encrypt() -> u8 {
	5
}

#[no_mangle]
pub extern fn peer_new_outbound(public_key: *const u8) -> Vec<u8>{
	let public_key_slice = unsafe { slice::from_raw_parts(public_key, 33) };
	let public_key_object = PublicKey::from_slice(public_key_slice).unwrap();
//	let peer_manager = PeerManager::new();
//	let peer_manager = PeerManager::new_outbound_connection(public_key_object)
//	let peer_handshake = PeerHandshake::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
