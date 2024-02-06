use std::collections::HashMap;

use crate::network::network_constants::{
    GENESIS_MERKLE_ROOT_HASH, GENESIS_NBITS, GENESIS_NONCE, GENESIS_PREVIOUS_BLOCK_HEADER_HASH,
    GENESIS_TIME, GENESIS_VERSION,
};

use super::{block::Block, block_header::BlockHeader};

/// Represents a blockchain and maintains information about blocks and the last block header.
pub struct BlockChain {
    blocks: HashMap<Vec<u8>, Block>,
    last_block_header: BlockHeader,
}

impl BlockChain {
    /// Creates a new `BlockChain` object with default values for the genesis block.
    pub fn new() -> BlockChain {
        BlockChain {
            blocks: HashMap::new(),
            last_block_header: BlockHeader::new(
                GENESIS_VERSION,
                GENESIS_PREVIOUS_BLOCK_HEADER_HASH.to_vec(),
                GENESIS_MERKLE_ROOT_HASH.to_vec(),
                GENESIS_TIME,
                GENESIS_NBITS,
                GENESIS_NONCE,
            ),
        }
    }

    /// Adds a new block to the blockchain.
    ///
    /// # Arguments
    ///
    /// * `block` - The `Block` object to be added to the blockchain.
    pub fn add(&mut self, block: Block) {
        let header = block.get_header();

        if self.last_block_header.get_time() < header.get_time() {
            self.last_block_header = header.clone();
        }

        self.blocks.insert(header.get_header(), block);
    }

    /// Returns the number of blocks in the blockchain.
    ///
    /// # Returns
    ///
    /// The number of blocks in the blockchain.
    pub fn get_cant_act_blocks(&self) -> usize {
        self.blocks.len()
    }

    /// Returns a reference to the internal HashMap containing the blocks in the blockchain.
    ///
    /// # Returns
    ///
    /// A reference to the HashMap containing the blocks in the blockchain.
    pub fn get_blocks(&self) -> &HashMap<Vec<u8>, Block> {
        //tratar de sacar
        &self.blocks
    }

    /// Returns the header of the last block in the blockchain.
    ///
    /// # Returns
    ///
    /// The header of the last block as a byte vector.
    pub fn get_last_block_header(&self) -> Vec<u8> {
        self.last_block_header.get_header()
    }

    /// Returns a reference to the block associated with the given block header.
    ///
    /// # Arguments
    ///
    /// * `block_header` - The block header as a byte vector.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the block if found, or `None` if the block is not present in the blockchain.
    pub fn get_block(&self, block_header: &Vec<u8>) -> Option<&Block> {
        self.blocks.get(block_header)
    }
}

impl Default for BlockChain {
    fn default() -> Self {
        Self::new()
    }
}
