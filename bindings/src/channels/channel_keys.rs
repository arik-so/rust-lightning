use secp256k1::{SecretKey, Signature, Secp256k1, PublicKey};
use lightning::ln::msgs::UnsignedChannelAnnouncement;
use bitcoin::{Transaction, Script};
use lightning::ln::chan_utils::{TxCreationKeys, ChannelPublicKeys, HTLCOutputInCommitment};
use lightning::chain::keysinterface::KeysInterface;

#[derive(Clone)]
pub struct ChannelKeys{}

impl lightning::chain::keysinterface::ChannelKeys for ChannelKeys{
	fn funding_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn revocation_base_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn payment_base_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn delayed_payment_base_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn htlc_base_key<'a>(&'a self) -> &'a SecretKey {
		unimplemented!()
	}

	fn commitment_seed<'a>(&'a self) -> &'a [u8; 32] {
		unimplemented!()
	}

	fn pubkeys<'a>(&'a self) -> &'a ChannelPublicKeys {
		unimplemented!()
	}

	fn sign_remote_commitment<T: secp256k1::Signing + secp256k1::Verification>(&self, feerate_per_kw: u64, commitment_tx: &Transaction, keys: &TxCreationKeys, htlcs: &[&HTLCOutputInCommitment], to_self_delay: u16, secp_ctx: &Secp256k1<T>) -> Result<(Signature, Vec<Signature>), ()> {
		unimplemented!()
	}

	fn sign_closing_transaction<T: secp256k1::Signing>(&self, closing_tx: &Transaction, secp_ctx: &Secp256k1<T>) -> Result<Signature, ()> {
		unimplemented!()
	}

	fn sign_channel_announcement<T: secp256k1::Signing>(&self, msg: &UnsignedChannelAnnouncement, secp_ctx: &Secp256k1<T>) -> Result<Signature, ()> {
		unimplemented!()
	}

	fn set_remote_channel_pubkeys(&mut self, channel_points: &ChannelPublicKeys) {
		unimplemented!()
	}
}

impl lightning::chain::keysinterface::KeysInterface for ChannelKeys{
	type ChanKeySigner = ChannelKeys;

	fn get_node_secret(&self) -> SecretKey {
		unimplemented!()
	}

	fn get_destination_script(&self) -> Script {
		unimplemented!()
	}

	fn get_shutdown_pubkey(&self) -> PublicKey {
		unimplemented!()
	}

	fn get_channel_keys(&self, inbound: bool, channel_value_satoshis: u64) -> Self::ChanKeySigner {
		unimplemented!()
	}

	fn get_onion_rand(&self) -> (SecretKey, [u8; 32]) {
		unimplemented!()
	}

	fn get_channel_id(&self) -> [u8; 32] {
		unimplemented!()
	}
}