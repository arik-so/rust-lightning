#![cfg(test)]

use secp256k1::key::{PublicKey, SecretKey};

use ln::peers::handshake::{HandshakeState, PeerHandshake};

#[test]
fn test_exchange() {
	let local_private_key = [0x_11_u8; 32];
	let remote_private_key = [0x_21_u8; 32];

	let mut local_handshake = PeerHandshake::new(local_private_key);
	let mut remote_handshake = PeerHandshake::new(remote_private_key);

	let local_ephemeral_private_key = [0x_12_u8; 32];
	let remote_ephemeral_private_key = [0x_22_u8; 32];

	let remote_public_key = public_key_from_private_key(remote_private_key);

	let act_1_message = local_handshake.initiate(&local_ephemeral_private_key, &remote_public_key);
	let act_2_message = remote_handshake.process_act_one(act_1_message.unwrap(), &remote_ephemeral_private_key);
	let act_3_message = local_handshake.process_act_two(act_2_message);
	remote_handshake.process_act_three(act_3_message.0);
}

fn public_key_from_private_key(private_key: [u8; 32]) -> [u8; 33] {
	let curve = secp256k1::Secp256k1::new();
	let sk_object = SecretKey::from_slice(private_key.as_ref()).unwrap();
	let pk_object = PublicKey::from_secret_key(&curve, &sk_object);
	pk_object.serialize()
}