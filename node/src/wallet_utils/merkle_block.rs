use std::io::Read;

use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use crate::messages::read_from_bytes::{fill_command, read_u8_from_bytes, read_vec_from_bytes};

use super::wallet_utils_constants::MERKLE_BLOCK_COMMAND;

/// Represents a MerkleBlock message.
#[derive(Debug)]
pub struct MerkleBlock {
    command_name: String,
    merkle_root: Vec<u8>,
    hash_count: CompactSizeUInt,
    hashes: Vec<Vec<u8>>,
    flag_byte_count: CompactSizeUInt,
    flags: Vec<u8>,
}

impl MerkleBlock {
    /// Creates a new `MerkleBlock` instance with the provided data.
    ///
    /// # Arguments
    ///
    /// * `hashes` - The list of transaction hashes in the Merkle block.
    /// * `flags` - The transaction flag bytes in the Merkle block.
    /// * `merkle_root` - The root hash of the Merkle tree.
    ///
    /// # Returns
    ///
    /// A new `MerkleBlock` instance initialized with the given data.
    pub fn new(hashes: Vec<Vec<u8>>, flags: Vec<u8>, merkle_root: Vec<u8>) -> MerkleBlock {
        MerkleBlock {
            command_name: MERKLE_BLOCK_COMMAND.to_string(),
            merkle_root,
            hash_count: CompactSizeUInt::from_number(hashes.len() as u64),
            hashes,
            flag_byte_count: CompactSizeUInt::from_number(flags.len() as u64),
            flags,
        }
    }
    /// Creates a `MerkleBlock` instance by parsing the data from a byte stream.
    ///
    /// # Arguments
    ///
    /// * `command_name` - The command name associated with the message.
    /// * `stream` - A mutable reference to the byte stream to read the data from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `MerkleBlock` instance or a `MessageError` if the parsing fails.
    pub fn from_bytes(
        command_name: String,
        stream: &mut dyn Read,
    ) -> Result<MerkleBlock, MessageError> {
        let merkle_root = read_vec_from_bytes(stream, 32)?;
        let hash_count = CompactSizeUInt::from_bytes(stream)?;

        let mut hashes = Vec::new();
        for _ in 0..hash_count.value() {
            hashes.push(read_vec_from_bytes(stream, 32)?);
        }

        let mut flags = Vec::new();
        let flag_byte_count = CompactSizeUInt::from_bytes(stream)?;
        for _ in 0..flag_byte_count.value() {
            flags.push(read_u8_from_bytes(stream)?);
        }

        Ok(MerkleBlock {
            command_name,
            merkle_root,
            hash_count,
            hashes,
            flag_byte_count,
            flags,
        })
    }

    /// Converts the `MerkleBlock` instance into a byte representation.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the byte representation of the `MerkleBlock` instance.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = fill_command(self.command_name.as_str()).as_bytes().to_vec();
        buffer.extend(&self.merkle_root);
        buffer.extend(self.hash_count.as_bytes());

        for hash in self.hashes.iter() {
            buffer.extend(hash);
        }

        buffer.extend(self.flag_byte_count.as_bytes());
        buffer.extend(&self.flags);

        buffer
    }

    pub fn hashes(&self) -> Vec<Vec<u8>> {
        self.hashes.clone()
    }

    pub fn flags(&mut self) -> &Vec<u8> {
        &self.flags
    }

    pub fn get_merkle_root(&self) -> &Vec<u8> {
        &self.merkle_root
    }
}
