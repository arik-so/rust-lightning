use bitcoin::Script as BitcoinScript;
use bitcoin::secp256k1::key::PublicKey as SecpPublicKey;
use bitcoin::secp256k1::key::SecretKey as SecpSecretKey;

#[repr(C)]
pub struct PublicKey {
	pub compressed_form: [u8; 33],
}
impl PublicKey {
	pub(crate) fn from_rust(pk: &SecpPublicKey) -> Self {
		Self {
			compressed_form: pk.serialize(),
		}
	}
	pub(crate) fn into_rust(&self) -> SecpPublicKey {
		SecpPublicKey::from_slice(&self.compressed_form).unwrap()
	}
}

#[repr(C)]
pub struct SecretKey {
	pub bytes: [u8; 32],
}
impl SecretKey {
	// from_rust isn't implemented since we jsut return byte array refs directly
	pub(crate) fn into_rust(&self) -> SecpSecretKey {
		SecpSecretKey::from_slice(&self.bytes).unwrap()
	}
}

#[repr(C)]
pub struct Script {
	pub data: *const u8,
	pub datalen: usize
}
impl Script {
	pub(crate) fn from_slice(s: &[u8]) -> Self {
		Self {
			data: s.as_ptr(),
			datalen: s.len(),
		}
	}
	pub(crate) fn into_bitcoin(&self) -> BitcoinScript {
		BitcoinScript::from(unsafe { std::slice::from_raw_parts(self.data, self.datalen) }.to_vec())
	}
}

#[repr(C)]
pub struct Transaction {
	pub data: *const u8,
	pub datalen: usize
}
impl Transaction {
	pub(crate) fn from_slice(s: &[u8]) -> Self {
		Self {
			data: s.as_ptr(),
			datalen: s.len(),
		}
	}
}

#[repr(C)]
pub struct u8slice {
	pub data: *const u8,
	pub datalen: usize
}
impl u8slice {
	pub(crate) fn from_slice(s: &[u8]) -> Self {
		Self {
			data: s.as_ptr(),
			datalen: s.len(),
		}
	}
}

#[repr(C)]
pub struct ThirtyTwoBytes {
	pub data: [u8; 32],
}
