#![allow(dead_code)]
#![allow(unused_imports)]

use bitcoin::hashes::Hash as BitcoinHash;
use electrum_client::bitcoin::hashes::Hash as ElectrumHash;

use bitcoin::consensus::Decodable as BitcoinDecodable;
use bitcoin::consensus::Encodable as BitcoinEncodable;
use electrum_client::bitcoin::consensus::Decodable as ElectrumDecodable;
use electrum_client::bitcoin::consensus::Encodable as ElectrumEncodable;

pub(crate) fn bitcoin_to_esplora_block_hash(
	hash: &bitcoin::block::BlockHash,
) -> electrum_client::bitcoin::BlockHash {
	electrum_client::bitcoin::BlockHash::from_slice(hash.as_ref()).unwrap()
}

pub(crate) fn esplora_to_bitcoin_block_hash(
	hash: &electrum_client::bitcoin::block::BlockHash,
) -> bitcoin::block::BlockHash {
	bitcoin::BlockHash::from_slice(hash.as_byte_array()).unwrap()
}

pub(crate) fn bitcoin_to_esplora_block_header(
	header: &bitcoin::block::Header,
) -> electrum_client::bitcoin::block::Header {
	let mut buffer = [0u8; bitcoin::block::Header::SIZE];
	let mut buffer_slice = &mut buffer[..];
	header.consensus_encode(&mut buffer_slice).unwrap();
	electrum_client::bitcoin::block::Header::consensus_decode(&mut &buffer[..]).unwrap()
}

pub(crate) fn esplora_to_bitcoin_block_header(
	header: &electrum_client::bitcoin::block::Header,
) -> bitcoin::block::Header {
	let mut buffer = [0u8; bitcoin::block::Header::SIZE];
	let mut buffer_slice = &mut buffer[..];
	header.consensus_encode(&mut buffer_slice).unwrap();
	bitcoin::block::Header::consensus_decode(&mut &buffer[..]).unwrap()
}

pub(crate) fn bitcoin_to_esplora_outpoint(
	outpoint: &bitcoin::OutPoint,
) -> electrum_client::bitcoin::OutPoint {
	let mut buffer = [0u8; 36];
	let mut buffer_slice = &mut buffer[..];
	outpoint.consensus_encode(&mut buffer_slice).unwrap();
	electrum_client::bitcoin::OutPoint::consensus_decode(&mut &buffer[..]).unwrap()
}

pub(crate) fn esplora_to_bitcoin_transaction(
	tx: &electrum_client::bitcoin::Transaction,
) -> bitcoin::Transaction {
	let mut buffer = Vec::new();
	tx.consensus_encode(&mut buffer).unwrap();
	bitcoin::Transaction::consensus_decode(&mut &buffer[..]).unwrap()
}

pub(crate) fn bitcoin_to_esplora_txid(txid: &bitcoin::Txid) -> electrum_client::bitcoin::Txid {
	electrum_client::bitcoin::Txid::from_slice(txid.as_byte_array()).unwrap()
}

pub(crate) fn esplora_to_bitcoin_txid(txid: &electrum_client::bitcoin::Txid) -> bitcoin::Txid {
	bitcoin::Txid::from_slice(txid.as_byte_array()).unwrap()
}

#[cfg(test)]
mod tests {
	use crate::util;

	#[test]
	fn test_bitcoin_to_esplora_block_hash() {
		use bitcoin::hashes::Hash as BitcoinHash;
		use electrum_client::bitcoin::hashes::Hash as ElectrumHash;

		let input_hash = [1u8; 32];
		let bitcoin_hash = bitcoin::block::BlockHash::from_slice(&input_hash).unwrap();
		let esplora_hash = util::bitcoin_to_esplora_block_hash(&bitcoin_hash);
		let output_hash = esplora_hash.as_byte_array();
		assert_eq!(&input_hash, output_hash);
	}

	#[test]
	fn test_esplora_to_bitcoin_block_hash() {
		use bitcoin::hashes::Hash as BitcoinHash;
		use electrum_client::bitcoin::hashes::Hash as ElectrumHash;

		let input_hash = [1u8; 32];
		let esplora_hash =
			electrum_client::bitcoin::block::BlockHash::from_slice(&input_hash).unwrap();
		let bitcoin_hash = util::esplora_to_bitcoin_block_hash(&esplora_hash);
		let output_hash = bitcoin_hash.as_byte_array();
		assert_eq!(&input_hash, output_hash);
	}

	#[test]
	fn test_bitcoin_to_esplora_block_header() {
		use bitcoin::consensus::Decodable;
		use electrum_client::bitcoin::consensus::Encodable;

		let input_header = [1u8; bitcoin::block::Header::SIZE];
		let mut input_header_copy = input_header.clone();
		let input_header_slice = &mut input_header_copy[..];
		let bitcoin_header =
			bitcoin::block::Header::consensus_decode(&mut &input_header_slice[..]).unwrap();
		let esplora_header = util::bitcoin_to_esplora_block_header(&bitcoin_header);
		let mut output_header = Vec::with_capacity(bitcoin::block::Header::SIZE);
		esplora_header.consensus_encode(&mut output_header).unwrap();
		assert_eq!(&input_header[..], &output_header[..]);
	}

	#[test]
	fn test_esplora_to_bitcoin_block_header() {
		use bitcoin::consensus::Encodable;
		use electrum_client::bitcoin::consensus::Decodable;

		let input_header = [1u8; bitcoin::block::Header::SIZE];
		let mut input_header_copy = input_header.clone();
		let input_header_slice = &mut input_header_copy[..];
		let esplora_header =
			electrum_client::bitcoin::block::Header::consensus_decode(&mut &input_header_slice[..])
				.unwrap();
		let bitcoin_header = util::esplora_to_bitcoin_block_header(&esplora_header);
		let mut output_header = Vec::with_capacity(bitcoin::block::Header::SIZE);
		bitcoin_header.consensus_encode(&mut output_header).unwrap();
		assert_eq!(&input_header[..], &output_header[..]);
	}

	#[test]
	fn test_bitcoin_to_esplora_outpoint() {
		use bitcoin::consensus::Decodable;
		use electrum_client::bitcoin::consensus::Encodable;

		let input_utxo = [1u8; 36];
		let mut input_utxo_copy = input_utxo.clone();
		let input_utxo_slice = &mut input_utxo_copy[..];
		let bitcoin_utxo = bitcoin::OutPoint::consensus_decode(&mut &input_utxo_slice[..]).unwrap();
		let esplora_utxo = util::bitcoin_to_esplora_outpoint(&bitcoin_utxo);
		let mut output_utxo = Vec::with_capacity(36);
		esplora_utxo.consensus_encode(&mut output_utxo).unwrap();
		assert_eq!(&input_utxo[..], &output_utxo[..]);
	}

	#[test]
	fn test_esplora_to_bitcoin_transaction() {
		use bitcoin::consensus::Encodable;
		use electrum_client::bitcoin::consensus::Decodable;

		let input_transaction = [
			2, 0, 0, 0, 1, 136, 87, 158, 19, 169, 236, 213, 200, 199, 157, 12, 47, 57, 211, 10,
			122, 224, 44, 11, 59, 86, 160, 255, 44, 99, 7, 83, 190, 67, 23, 8, 15, 0, 0, 0, 0, 107,
			72, 48, 69, 2, 33, 0, 144, 190, 96, 174, 37, 149, 110, 82, 111, 254, 42, 6, 55, 6, 80,
			152, 98, 177, 143, 174, 247, 31, 78, 52, 57, 100, 76, 133, 194, 158, 73, 68, 2, 32,
			106, 28, 120, 231, 93, 68, 26, 195, 123, 115, 116, 1, 51, 243, 242, 254, 73, 234, 140,
			38, 106, 30, 65, 173, 86, 59, 196, 124, 195, 13, 238, 7, 1, 33, 3, 27, 141, 63, 51, 73,
			241, 109, 107, 78, 134, 251, 52, 142, 74, 87, 95, 245, 165, 140, 212, 158, 139, 51,
			155, 19, 49, 189, 47, 41, 136, 161, 6, 255, 255, 255, 255, 2, 126, 130, 43, 6, 0, 0, 0,
			0, 25, 118, 169, 20, 14, 215, 175, 19, 11, 222, 150, 233, 243, 143, 202, 175, 250, 188,
			52, 107, 207, 217, 26, 29, 136, 172, 0, 0, 0, 0, 0, 0, 0, 0, 83, 106, 76, 80, 72, 69,
			77, 73, 1, 0, 99, 68, 115, 0, 19, 221, 190, 95, 199, 3, 232, 160, 197, 249, 111, 76,
			126, 67, 210, 245, 224, 55, 14, 129, 171, 226, 33, 169, 48, 185, 131, 155, 164, 116,
			246, 68, 79, 162, 85, 19, 199, 197, 31, 114, 92, 144, 250, 136, 24, 174, 212, 51, 65,
			109, 10, 173, 250, 139, 66, 236, 1, 220, 190, 139, 243, 144, 243, 178, 197, 48, 91, 64,
			105, 58, 94, 217, 43, 0,
		];
		let mut input_tx_copy = input_transaction.clone();
		let input_tx_slice = &mut input_tx_copy[..];
		let esplora_tx =
			electrum_client::bitcoin::Transaction::consensus_decode(&mut &input_tx_slice[..])
				.unwrap();
		let bitcoin_tx = util::esplora_to_bitcoin_transaction(&esplora_tx);
		let mut output_transaction = Vec::new();
		bitcoin_tx.consensus_encode(&mut output_transaction).unwrap();
		assert_eq!(&input_transaction[..], &output_transaction[..]);
	}

	#[test]
	fn test_bitcoin_to_esplora_txid() {
		use bitcoin::hashes::Hash as BitcoinHash;
		use electrum_client::bitcoin::hashes::Hash as ElectrumHash;

		let input_txid = [1u8; 32];
		let bitcoin_txid = bitcoin::Txid::from_slice(&input_txid).unwrap();
		let esplora_txid = util::bitcoin_to_esplora_txid(&bitcoin_txid);
		let output_txid = esplora_txid.as_byte_array();
		assert_eq!(&input_txid, output_txid);
	}

	#[test]
	fn test_esplora_to_bitcoin_txid() {
		use bitcoin::hashes::Hash as BitcoinHash;
		use electrum_client::bitcoin::hashes::Hash as ElectrumHash;

		let input_txid = [1u8; 32];
		let esplora_txid = electrum_client::bitcoin::Txid::from_slice(&input_txid).unwrap();
		let bitcoin_txid = util::esplora_to_bitcoin_txid(&esplora_txid);
		let output_txid = bitcoin_txid.as_byte_array();
		assert_eq!(&input_txid, output_txid);
	}
}
