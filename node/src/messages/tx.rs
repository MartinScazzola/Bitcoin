use super::{
    header::MessageHeader,
    message_constants::{HEADER_BYTES_SIZE, TX_COMMAND},
};
use crate::block_mod::transaction::Transaction;
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;

/// Represents a transaction message.
#[derive(Debug)]
pub struct Tx {
    header: MessageHeader,
    transaction: Transaction,
}

impl Tx {
    /// Creates a new `Tx` with the given start string and transaction.
    pub fn new(start_string: Vec<u8>, transaction: Transaction) -> Tx {
        let header = MessageHeader::new(start_string, TX_COMMAND.to_string());
        let mut tx = Tx {
            header,
            transaction,
        };

        let stream: Vec<u8> = tx.as_bytes();

        let payload_size = stream.len() - HEADER_BYTES_SIZE;

        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        tx.header.update_payload(payload_size as u32, checksum);

        tx
    }

    /// Converts the `Tx` object to its byte representation.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = self.header.as_bytes();
        buff.extend(self.transaction.as_bytes());

        buff
    }
}
