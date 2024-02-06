use bitcoin_hashes::{sha256d, Hash};

use crate::block_mod::block::Block;

use super::{
    header::MessageHeader,
    message_constants::{BLOCK_COMMAND, HEADER_BYTES_SIZE},
};

pub struct BlockMsg {
    header: MessageHeader,
    block: Block,
}

impl BlockMsg {
    pub fn new(start_string: Vec<u8>, block: Block) -> BlockMsg {
        let header = MessageHeader::new(start_string, BLOCK_COMMAND.to_string());
        let mut block_msg = BlockMsg { header, block };

        let stream: Vec<u8> = block_msg.as_bytes();
        let payload_size = stream.len() - HEADER_BYTES_SIZE;
        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        block_msg
            .header
            .update_payload(payload_size as u32, checksum);
        block_msg
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_bytes();
        buffer.extend(self.block.as_bytes());
        buffer
    }
}
