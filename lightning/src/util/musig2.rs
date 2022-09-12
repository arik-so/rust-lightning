//! Musig2 module
//!
//! We use properly spelled out variable names here, so here's a reference of the BIP equivalents
//! Q: aggregate_public_key
//! aggnonce: aggregate_nonce (technically a compound nonce, prior to the linear combination)
//! R: public_nonce (aggregate_nonce.0 + b * aggregate_nonce.1)
use core::borrow::Borrow;
use bitcoin::hashes::{Hash, HashEngine};
use bitcoin::hashes::sha256::Hash as Sha256;
use bitcoin::hashes::sha256::HashEngine as ShaEngine;
use bitcoin::secp256k1::{Parity, PublicKey, Scalar, Secp256k1, SecretKey};
use bitcoin::XOnlyPublicKey;
use util::ser::Writeable;

/// A public nonce used for Musig2, comprised of two ECPoints
#[derive(Clone, Debug, PartialEq)]
pub struct Musig2PublicNonce(pub(crate) PublicKey, pub(crate) PublicKey);

/// A secret nonce used for Musig2, comprised of two SecretKeys
#[derive(Clone, Debug, PartialEq)]
pub struct Musig2SecretNonce(pub(crate) SecretKey, pub(crate) SecretKey);

pub(crate) type Musig2AggregateKey = (XOnlyPublicKey, Parity);
pub type Musig2PartialSignature = SecretKey;

impl Musig2SecretNonce {
	pub(crate) fn to_public(&self) -> Musig2PublicNonce {
		let secp = Secp256k1::new();
		Musig2PublicNonce(
			PublicKey::from_secret_key(&secp, &self.0),
			PublicKey::from_secret_key(&secp, &self.1),
		)
	}

	pub(crate) fn negate(&self) -> Self {
		let first_nonce = self.0.negate();
		let second_nonce = self.1.negate();
		Musig2SecretNonce(first_nonce, second_nonce)
	}
}

impl Into<Musig2PublicNonce> for Musig2SecretNonce {
	fn into(self) -> Musig2PublicNonce {
		self.to_public()
	}
}

pub(crate) fn aggregate_keys<K: Borrow<PublicKey>>(sorted_keys: &[K]) -> Musig2AggregateKey {
	let secp = Secp256k1::new();

	let mut key_summands = Vec::with_capacity(sorted_keys.len());

	for i in 0..sorted_keys.len() {
		let current_key = sorted_keys[i].borrow();
		let current_coefficient = key_aggregation_coefficient_internal(sorted_keys, &current_key);

		let current_summand = current_key.mul_tweak(&secp, &current_coefficient).unwrap();

		key_summands.push(current_summand);
	}

	let summand_references: Vec<&PublicKey> = key_summands.iter().collect();
	let combined_pubkey = PublicKey::combine_keys(&summand_references).unwrap();
	combined_pubkey.x_only_public_key()
}

fn fake_aggregate<K: Borrow<PublicKey>>(array: &[K]) {
	for current_key in array {
		println!("key: {:?}", current_key.borrow().serialize());
	}
}

pub(crate) fn aggregate_nonces<N: Borrow<Musig2PublicNonce>>(nonces: &[N]) -> Musig2PublicNonce {

	let mut first_nonce_summands = Vec::with_capacity(nonces.len());
	let mut second_nonce_summands = Vec::with_capacity(nonces.len());

	for current_nonce in nonces {
		let first_nonce = &current_nonce.borrow().0;
		let second_nonce = &current_nonce.borrow().1;

		first_nonce_summands.push(first_nonce);
		second_nonce_summands.push(second_nonce);
	}

	let first_nonce = PublicKey::combine_keys(&first_nonce_summands).unwrap();
	let second_nonce = PublicKey::combine_keys(&second_nonce_summands).unwrap();
	Musig2PublicNonce(first_nonce, second_nonce)
}

pub(crate) fn calculate_final_nonce(aggregate_nonce: &Musig2PublicNonce, aggregate_public_key: &Musig2AggregateKey, message: &[u8]) -> PublicKey {
	let secp = Secp256k1::new();

	let nonce_coefficient = nonce_linear_combination_coefficient(&aggregate_public_key.0, &aggregate_nonce, message);
	let nonce_summand = aggregate_nonce.1.mul_tweak(&secp, &nonce_coefficient).unwrap();
	let public_nonce = PublicKey::combine_keys(vec![&aggregate_nonce.0, &nonce_summand].as_slice()).unwrap();
	public_nonce
}

pub(crate) fn sign<K: Borrow<PublicKey>>(secret_key: &SecretKey, secret_nonce: &Musig2SecretNonce, aggregate_nonce: &Musig2PublicNonce, sorted_keys: &[K], message: &[u8]) -> Musig2PartialSignature {
	let aggregate_public_key = self::aggregate_keys(sorted_keys);
	let public_nonce = calculate_final_nonce(aggregate_nonce, &aggregate_public_key, message);

	let final_secret_nonce = if has_even_y(&public_nonce) {
		secret_nonce.clone()
	} else {
		secret_nonce.negate()
	};

	// determine whether the aggregate public key has an even y coordinate
	let final_secret_key = if let Parity::Even = aggregate_public_key.1 {
		secret_key.clone()
	} else {
		secret_key.negate()
	};

	// m = hash_BIP0340/challenge(public_nonce || aggregate_key || message)
	let mut sha_engine = tagged_sha_engine("BIP0340/challenge");
	sha_engine.input(&public_nonce.x_only_public_key().0.serialize());
	sha_engine.input(&aggregate_public_key.0.serialize());
	sha_engine.input(message);
	let output_hash = Sha256::from_engine(sha_engine).into_inner();
	let m = Scalar::from_be_bytes(output_hash)
		.expect("SHA256 appears to be broken due to the heat death of the universe. If the user has a cat, please make sure to kill it.");

	let secp = Secp256k1::new();
	let nonce_coefficient = nonce_linear_combination_coefficient(&aggregate_public_key.0, &aggregate_nonce, message);

	let a = key_aggregation_coefficient_internal(&sorted_keys, &secret_key.public_key(&secp));
	// m*a*d
	let summand_a = final_secret_key.mul_tweak(&m).unwrap().mul_tweak(&a).unwrap();
	// b*k2
	let summand_b = final_secret_nonce.1.mul_tweak(&nonce_coefficient).unwrap();
	// s = m*a*d + k1 + b*k2
	let s = summand_a.add_tweak(&Scalar::from(summand_b)).unwrap().add_tweak(&Scalar::from(final_secret_nonce.0)).unwrap();
	s
}

pub(crate) fn aggregate_partial_signatures<K: Borrow<SecretKey>>(partial_signatures: &[K], final_nonce: &PublicKey) -> Vec<u8> {
	let mut aggregate_signature = Scalar::ZERO;
	for current_signature in partial_signatures {
		aggregate_signature = Scalar::from(current_signature.borrow().add_tweak(&aggregate_signature).unwrap());
	}
	let mut serialized_signature = Vec::with_capacity(64);
	final_nonce.x_only_public_key().0.serialize().write(&mut serialized_signature).unwrap();
	aggregate_signature.to_be_bytes().write(&mut serialized_signature);
	serialized_signature
}

fn verify_partial_signature_internal(partial_signature: &SecretKey, public_nonce: &PublicKey, partial_public_key: &PublicKey, aggregate_public_key: &Musig2AggregateKey, ) {
	unimplemented!()
	// let a = key_aggregation_coefficient_internal(&sorted_keys, &secret_key.public_key(&secp));
}

fn has_even_y(public_key: &PublicKey) -> bool {
	let first_byte = public_key.serialize()[0];
	assert!([2, 3].contains(&first_byte));
	first_byte == 2
}

pub(crate) fn sort_keys<K: Borrow<PublicKey>>(keys: &[K]) -> Vec<PublicKey> {
	let mut sorted_keys = Vec::with_capacity(keys.len());
	for current_key in keys {
		sorted_keys.push(current_key.borrow().clone());
	}
	sorted_keys.sort_by(|key_a, key_b| {
		let slice_a = key_a.serialize();
		let slice_b = key_b.serialize();
		slice_a.cmp(&slice_b)
	});
	sorted_keys
}

fn hash_keys<K: Borrow<PublicKey>>(sorted_keys: &[K]) -> [u8; 32] {
	let mut sha_engine = tagged_sha_engine("KeyAgg list");
	for current_key in sorted_keys {
		sha_engine.input(&current_key.borrow().serialize());
	}
	Sha256::from_engine(sha_engine).into_inner()
}

fn get_second_key_serialization<K: Borrow<PublicKey>>(sorted_keys: &[K]) -> [u8; 33] {
	let first_key = sorted_keys[0].borrow();
	for j in 1..sorted_keys.len() {
		let current_key = sorted_keys[j].borrow();
		if current_key != first_key {
			return current_key.serialize();
		}
	}
	[0; 33]
}

fn key_aggregation_coefficient_internal<K: Borrow<PublicKey>>(sorted_keys: &[K], public_key: &PublicKey) -> Scalar {
	// could be memoized
	let second_key = get_second_key_serialization(sorted_keys);
	let referenced_key = public_key.serialize();
	if referenced_key == second_key {
		return Scalar::ONE
	}

	// could be memoized
	let key_list_hash = hash_keys(sorted_keys);
	tagged_hash_scalar("KeyAgg coefficient", &[&key_list_hash, &referenced_key])
}

fn nonce_linear_combination_coefficient(aggregate_public_key: &XOnlyPublicKey, aggregate_nonce: &Musig2PublicNonce, message: &[u8]) -> Scalar {
	tagged_hash_scalar("MuSig/noncecoef", &[&aggregate_nonce.encode(), &aggregate_public_key.serialize(), message])
}

fn tagged_sha_engine(tag: &str) -> ShaEngine {
	let tag_hash = Sha256::hash(tag.as_bytes()).into_inner();
	let mut sha_engine = Sha256::engine();
	sha_engine.input(&tag_hash);
	sha_engine.input(&tag_hash);
	sha_engine
}

fn tagged_hash_scalar(tag: &str, inputs: &[&[u8]]) -> Scalar {
	let mut sha_engine = tagged_sha_engine(tag);
	for current_input in inputs {
		sha_engine.input(current_input);
	}
	let output_hash = Sha256::from_engine(sha_engine).into_inner();
	Scalar::from_be_bytes(output_hash)
		.expect("SHA256 appears to be broken due to the heat death of the universe. If the user has a cat, please make sure to kill it.")
}

#[cfg(test)]
mod tests {
	use std::io::Cursor;
	use bitcoin::secp256k1::{PublicKey, SecretKey};
	use util::musig2::{aggregate_keys, aggregate_nonces, aggregate_partial_signatures, calculate_final_nonce, fake_aggregate, key_aggregation_coefficient_internal, Musig2PublicNonce, sort_keys};
	use util::ser::{Readable, Writeable};

	#[test]
	fn test_reference_passing() {
		let keys = vec![
			PublicKey::from_slice(&hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap()).unwrap(),
		];
		fake_aggregate(keys.as_slice());

		let reference_keys: Vec<&PublicKey> = keys.iter().collect();
		fake_aggregate(reference_keys.as_slice());
	}

	#[test]
	fn test_key_sorting() {
		let unsorted_keys = vec![
			PublicKey::from_slice(&hex::decode("02dd308afec5777e13121fa72b9cc1b7cc0139715309b086c960e18fd969774eb8").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("03dff1d77f2a671c5f36183726db2341be58feae1da2deced843240f7b502ba659").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("023590a94e768f8e1815c2f24b4d80a8e3149316c3518ce7b7ad338368d038ca66").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("02dd308afec5777e13121fa72b9cc1b7cc0139715309b086c960e18fd969774eb8").unwrap()).unwrap()
		];

		let sorted_keys = sort_keys(unsorted_keys.as_slice());

		assert_eq!(hex::encode(sorted_keys[0].serialize()), "023590a94e768f8e1815c2f24b4d80a8e3149316c3518ce7b7ad338368d038ca66");
		assert_eq!(hex::encode(sorted_keys[1].serialize()), "02dd308afec5777e13121fa72b9cc1b7cc0139715309b086c960e18fd969774eb8");
		assert_eq!(hex::encode(sorted_keys[2].serialize()), "02dd308afec5777e13121fa72b9cc1b7cc0139715309b086c960e18fd969774eb8");
		assert_eq!(hex::encode(sorted_keys[3].serialize()), "02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9");
		assert_eq!(hex::encode(sorted_keys[4].serialize()), "03dff1d77f2a671c5f36183726db2341be58feae1da2deced843240f7b502ba659");
	}

	#[test]
	fn test_key_aggregation_coefficient_on_distinct_key() {
		let keys = vec![
			PublicKey::from_slice(&hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("03dff1d77f2a671c5f36183726db2341be58feae1da2deced843240f7b502ba659").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("023590a94e768f8e1815c2f24b4d80a8e3149316c3518ce7b7ad338368d038ca66").unwrap()).unwrap(),
		];

		let coefficients = [
			key_aggregation_coefficient_internal(&keys, &keys[0]),
			key_aggregation_coefficient_internal(&keys, &keys[1]),
			key_aggregation_coefficient_internal(&keys, &keys[2]),
		];

		assert_eq!(hex::encode(coefficients[0].to_be_bytes()), "ad0537c883813849e3b95ce5db1d45eb25cc5fae197c4e8759719065932aa183");
		assert_eq!(hex::encode(coefficients[1].to_be_bytes()), "0000000000000000000000000000000000000000000000000000000000000001");
		assert_eq!(hex::encode(coefficients[2].to_be_bytes()), "f45bb025038e68f050ce4dbf8c63e613678892e2a3e930780afe6293376005c6");
	}

	#[test]
	fn test_key_aggregation_coefficient_on_repeated_key() {
		let keys = vec![
			PublicKey::from_slice(&hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap()).unwrap(),
			PublicKey::from_slice(&hex::decode("02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9").unwrap()).unwrap(),
		];

		let coefficients = [
			key_aggregation_coefficient_internal(&keys, &keys[0]),
			key_aggregation_coefficient_internal(&keys, &keys[1]),
			key_aggregation_coefficient_internal(&keys, &keys[2]),
		];

		assert_eq!(hex::encode(coefficients[0].to_be_bytes()), "d18a55755a3b6aa3b171bd484eb508318abd01a3cd5e6c78aca613dbd72fa061");
		assert_eq!(hex::encode(coefficients[1].to_be_bytes()), "d18a55755a3b6aa3b171bd484eb508318abd01a3cd5e6c78aca613dbd72fa061");
		assert_eq!(hex::encode(coefficients[2].to_be_bytes()), "d18a55755a3b6aa3b171bd484eb508318abd01a3cd5e6c78aca613dbd72fa061");

		let aggregate_key = aggregate_keys(keys.as_slice());
		assert_eq!(hex::encode(aggregate_key.0.serialize()), "b436e3bad62b8cd409969a224731c193d051162d8c5ae8b109306127da3aa935");
	}

	#[test]
	fn test_nonce_aggregation() {

		let public_nonce_hexes = vec![
			"020151c80f435648df67a22b749cd798ce54e0321d034b92b709b567d60a42e66603ba47fbc1834437b3212e89a84d8425e7bf12e0245d98262268ebdcb385d50641",
			"03ff406ffd8adb9cd29877e4985014f66a59f6cd01c0e88caa8e5f3166b1f676a60248c264cdd57d3c24d79990b0f865674eb62a0f9018277a95011b41bfc193b833"
		];

		let mut public_nonces = Vec::with_capacity(public_nonce_hexes.len());
		for current_nonce_hex in public_nonce_hexes {
			let mut cursor = Cursor::new(hex::decode(current_nonce_hex).unwrap());
			let current_nonce = Musig2PublicNonce::read(&mut cursor).unwrap();
			public_nonces.push(current_nonce);
		}

		assert_eq!(public_nonces.len(), 2);

		let aggregate_nonce = aggregate_nonces(&public_nonces);
		let mut aggregate_nonce_bytes = Vec::with_capacity(66);
		aggregate_nonce.write(&mut aggregate_nonce_bytes).unwrap();
		let aggregate_nonce_hex = hex::encode(aggregate_nonce_bytes);

		assert_eq!(aggregate_nonce_hex, "035fe1873b4f2967f52fea4a06ad5a8eccbe9d0fd73068012c894e2e87ccb5804b024725377345bde0e9c33af3c43c0a29a9249f2f2956fa8cfeb55c8573d0262dc8")
	}

	#[test]
	#[should_panic]
	// TODO: make it such that this doesn't panic
	fn test_nonce_aggregation_at_infinity_point() {

		let public_nonce_hexes = vec![
			"020151c80f435648df67a22b749cd798ce54e0321d034b92b709b567d60a42e6660279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
			"03ff406ffd8adb9cd29877e4985014f66a59f6cd01c0e88caa8e5f3166b1f676a60379be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798"
		];

		let mut public_nonces = Vec::with_capacity(public_nonce_hexes.len());
		for current_nonce_hex in public_nonce_hexes {
			let mut cursor = Cursor::new(hex::decode(current_nonce_hex).unwrap());
			let current_nonce = Musig2PublicNonce::read(&mut cursor).unwrap();
			public_nonces.push(current_nonce);
		}

		assert_eq!(public_nonces.len(), 2);

		let aggregate_nonce = aggregate_nonces(&public_nonces);
		let mut aggregate_nonce_bytes = Vec::with_capacity(66);
		aggregate_nonce.write(&mut aggregate_nonce_bytes).unwrap();
		let aggregate_nonce_hex = hex::encode(aggregate_nonce_bytes);

		assert_eq!(aggregate_nonce_hex, "035fe1873b4f2967f52fea4a06ad5a8eccbe9d0fd73068012c894e2e87ccb5804b000000000000000000000000000000000000000000000000000000000000000000")
	}

	#[test]
	fn test_signature_aggregation() {
		let pubkey_hexes = vec![
			"03935f972da013f80ae011890fa89b67a27b7be6ccb24d3274d18b2d4067f261a9",
			"02d2dc6f5df7c56acf38c7fa0ae7a759ae30e19b37359dfde015872324c7ef6e05"
		];

		let public_nonce_hexes = vec![
			"0300a32f8548f59c533f55db9754e3c0ba3c2544f085649fdce42b8bd3f244c2ca0384449bed61004e8863452a38534e91875516c3cc543122ce2be1f31845025588",
			"03f66b072a869bc2a57d776d487151d707e82b4f1b885066a589858c1bf3871db603ed391c9658ab6031a96acbd5e2d9fec465efdc8c0d0b765c9b9f3579d520fb6f"
		];

		let partial_signature_hexes = vec![
			"7918521f42e5727fe2e82d802876e0c8844336fda1b58c82696a55b0188c8b3d",
			"599044037ae15c4a99fb94f022b48e7ab215bf703954ec0b83d0e06230476001"
		];

		let mut public_keys = Vec::with_capacity(pubkey_hexes.len());
		for current_pubkey_hex in pubkey_hexes {
			let current_key = PublicKey::from_slice(&hex::decode(current_pubkey_hex).unwrap()).unwrap();
			public_keys.push(current_key);
		}

		let mut public_nonces = Vec::with_capacity(public_nonce_hexes.len());
		for current_nonce_hex in public_nonce_hexes {
			let mut cursor = Cursor::new(hex::decode(current_nonce_hex).unwrap());
			let current_nonce = Musig2PublicNonce::read(&mut cursor).unwrap();
			public_nonces.push(current_nonce);
		}

		let mut partial_signatures = Vec::with_capacity(partial_signature_hexes.len());
		for current_signature_hex in partial_signature_hexes {
			let current_signature = SecretKey::from_slice(&hex::decode(current_signature_hex).unwrap()).unwrap();
			partial_signatures.push(current_signature);
		}

		assert_eq!(public_keys.len(), 2);
		assert_eq!(public_nonces.len(), 2);
		assert_eq!(partial_signatures.len(), 2);

		let message = hex::decode("599c67ea410d005b9da90817cf03ed3b1c868e4da4edf00a5880b0082c237869").unwrap();

		let aggregate_nonce = aggregate_nonces(&public_nonces);
		let aggregate_nonce_hex = hex::encode(aggregate_nonce.encode());
		assert_eq!(aggregate_nonce_hex, "02bc34cdf6fa1298d7b6a126812fad0739005bc44e45c21276eefe41aaf841c86f03f3562aed52243bb99f43d1677db59f0fefb961633997f7ac924b78fbd0b0334f");

		let aggregate_public_key = aggregate_keys(public_keys.as_slice());
		println!("aggregate pubkey (Q): {}", hex::encode(aggregate_public_key.0.serialize()));

		let final_nonce = calculate_final_nonce(&aggregate_nonce, &aggregate_public_key, &message);
		println!("final nonce (R): {}", hex::encode(final_nonce.serialize()));

		let signature = aggregate_partial_signatures(partial_signatures.as_slice(), &final_nonce);
		let signature_hex = hex::encode(signature);
		assert_eq!(signature_hex, "ca3c28729659e50f829f55dc5db1de88a05d1702b4165b85f95b627fc57733f8d2a89622bdc6ceca7ce3c2704b2b6f433658f66ddb0a788ded3b361248d3eb3e")
	}
}
