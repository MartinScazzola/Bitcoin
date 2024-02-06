use super::super::messages::read_from_bytes::*;
use crate::messages::message_error::MessageError;
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::cmp::Ordering;
use std::io::Read;

/// Represents a block header in the Bitcoin protocol.
#[derive(Debug, PartialEq, Clone)]
pub struct BlockHeader {
    block_version: i32,
    previous_block_header_hash: Vec<u8>,
    merkle_root_hash: Vec<u8>,
    time: u32,
    nbits: u32,
    nonce: u32,
    next_header_hash: Option<Vec<u8>>,
}

impl BlockHeader {
    /// Creates a new block header with the specified properties.
    ///
    /// # Arguments
    ///
    /// * `block_version` - The version of the block.
    /// * `previous_block_header_hash` - The hash of the previous block's header.
    /// * `merkle_root_hash` - The Merkle root hash of the transactions in the block.
    /// * `time` - The timestamp of the block in Unix time format.
    /// * `nbits` - The encoded difficulty target for mining the block.
    /// * `nonce` - The nonce used for mining the block.
    ///
    /// # Returns
    ///
    /// A new `BlockHeader` with the specified properties.
    pub fn new(
        block_version: i32,
        previous_block_header_hash: Vec<u8>,
        merkle_root_hash: Vec<u8>,
        time: u32,
        nbits: u32,
        nonce: u32,
    ) -> BlockHeader {
        BlockHeader {
            block_version,
            previous_block_header_hash,
            merkle_root_hash,
            time,
            nbits,
            nonce,
            next_header_hash: None,
        }
    }

    /// Parses a byte stream and constructs a `BlockHeader` from it.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a byte stream implementing the `Read` trait.
    ///
    /// # Returns
    ///
    /// - `Ok(BlockHeader)` if parsing is successful.
    /// - `Err(MessageError)` if an error occurs during parsing.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<BlockHeader, MessageError> {
        let block_version = read_i32_from_bytes(stream, true)?;
        let previous_block_header_hash = read_vec_from_bytes(stream, 32)?;
        let merkle_root_hash = read_vec_from_bytes(stream, 32)?;
        let time = read_u32_from_bytes(stream, true)?;
        let nbits = read_u32_from_bytes(stream, true)?;
        let nonce = read_u32_from_bytes(stream, true)?;

        Ok(BlockHeader {
            block_version,
            previous_block_header_hash,
            merkle_root_hash,
            time,
            nbits,
            nonce,
            next_header_hash: None,
        })
    }

    /// Converts the `BlockHeader` into a byte representation.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the byte representation of the `BlockHeader`.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(self.block_version.to_le_bytes());
        buffer.extend(&self.previous_block_header_hash);
        buffer.extend(&self.merkle_root_hash);
        buffer.extend(self.time.to_le_bytes());
        buffer.extend(self.nbits.to_le_bytes());
        buffer.extend(self.nonce.to_le_bytes());

        buffer
    }

    /// Computes the header hash of the `BlockHeader`.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` representing the header hash of the `BlockHeader`.
    pub fn get_header(&self) -> Vec<u8> {
        sha256d::Hash::hash(&self.as_bytes())
            .to_byte_array()
            .to_vec()
    }

    /// Returns the timestamp of the block in Unix time format.
    pub fn get_time(&self) -> u32 {
        self.time
    }

    /// Returns a reference to the Merkle root hash of the transactions in the block.
    pub fn get_merkle_root(&self) -> &Vec<u8> {
        &self.merkle_root_hash
    }

    /// Returns the nbits value of the block header.
    pub fn get_nbits(&self) -> u32 {
        self.nbits
    }

    pub fn get_previuos_block_header(&self) -> &Vec<u8> {
        &self.previous_block_header_hash
    }

    pub fn set_next_block_header(&mut self, next_block_header: Vec<u8>) {
        self.next_header_hash = Some(next_block_header);
    }

    pub fn get_next_block_header(&self) -> &Option<Vec<u8>> {
        &self.next_header_hash
    }

    /// Performs a proof-of-work check on the block header.
    ///
    /// The proof-of-work check compares the header hash of the block with a target threshold
    /// to determine if the block satisfies the desired difficulty level.
    ///
    /// # Returns
    ///
    /// - `true` if the block satisfies the proof-of-work requirement.
    /// - `false` if the block does not satisfy the proof-of-work requirement.
    pub fn proof_of_work(&self) -> bool {
        let nbits = self.nbits.to_be_bytes();
        let exp = nbits[0];
        let mantissa: Vec<u8> = nbits[1..].to_vec();

        let mut threshold = vec![0u8; (32 - exp) as usize];
        threshold.extend(mantissa);
        threshold.extend(&vec![0u8; (exp - 3) as usize]);

        let mut block_header = self.get_header();
        block_header.reverse();

        for i in 0..32 {
            match block_header[i].cmp(&threshold[i]) {
                Ordering::Greater => return false,
                Ordering::Less => return true,
                Ordering::Equal => {}
            }
        }
        true
    }
}

impl std::fmt::Display for BlockHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut header = self.get_header();
        header.reverse();
        writeln!(f, "Header: {}", encode_hex(&header)?)?;
        writeln!(f, "Version: {}", self.block_version)?;
        writeln!(f, "Previous: {:?}", self.previous_block_header_hash)?;
        writeln!(f, "Merkle root: {:?}", self.merkle_root_hash)?;
        writeln!(f, "Time: {}", self.time)?;
        writeln!(f, "Number of bits: {}", self.nbits)?;
        writeln!(f, "Nonce: {}", self.nonce)?;
        writeln!(f, "---------------------------------")?;

        Ok(())
    }
}

#[cfg(test)]
mod block_header_test {
    use std::num::ParseIntError;

    use crate::network::network_constants::{
        GENESIS_MERKLE_ROOT_HASH, GENESIS_NBITS, GENESIS_NONCE, GENESIS_PREVIOUS_BLOCK_HEADER_HASH,
        GENESIS_TIME, GENESIS_VERSION,
    };

    use super::*;

    #[test]
    fn test_get_header_block() {
        let block_version = 1 as i32;
        let previous_block_header_hash: Vec<u8> = vec![
            21u8, 22, 27, 172, 94, 200, 11, 10, 177, 57, 15, 60, 95, 231, 146, 151, 1, 105, 197,
            155, 71, 243, 191, 178, 89, 144, 41, 166, 0, 0, 0, 0,
        ];
        let merkle_root_hash: Vec<u8> = vec![
            3u8, 108, 22, 118, 5, 21, 57, 95, 31, 54, 134, 115, 30, 27, 216, 28, 24, 234, 226, 37,
            252, 169, 241, 45, 191, 240, 76, 18, 25, 40, 150, 48,
        ];
        let time = 1337966311 as u32;
        let nbits = 486604799 as u32;
        let nonce = 1288107791 as u32;

        let _block = BlockHeader {
            block_version,
            previous_block_header_hash,
            merkle_root_hash,
            time,
            nbits,
            nonce,
            next_header_hash: None,
        };

        assert!(true);
    }
    #[test]
    fn test_get_header_block_genesis() -> Result<(), ParseIntError> {
        let block_version = 1 as i32;
        let previous_block_header_hash = vec![0u8; 32];
        let mut merkle_root_hash =
            decode_hex("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b")?;

        merkle_root_hash.reverse();
        let time = 1296688602 as u32;
        let nbits = 486604799 as u32;
        let nonce = 414098458 as u32;

        let block = BlockHeader {
            block_version,
            previous_block_header_hash,
            merkle_root_hash,
            time,
            nbits,
            nonce,
            next_header_hash: None,
        };

        println!("{:?}", block);
        println!("{:?}", block.get_header());
        Ok(())
    }

    #[test]
    fn test_proof_of_work_of_genesis_block() {
        let genesis = BlockHeader::new(
            GENESIS_VERSION,
            GENESIS_PREVIOUS_BLOCK_HEADER_HASH.to_vec(),
            GENESIS_MERKLE_ROOT_HASH.to_vec(),
            GENESIS_TIME,
            GENESIS_NBITS,
            GENESIS_NONCE,
        );

        println!("{}", genesis);

        assert!(genesis.proof_of_work());
    }
}
