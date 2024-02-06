use crate::messages::{
    message_error::MessageError,
    read_from_bytes::{read_u32_from_bytes, read_vec_from_bytes},
};
use std::io::Read;

/// Represents an Outpoint in the Bitcoin protocol.
/// An Outpoint is a structure that contains a reference to a TxOut used in the transaction
#[derive(Debug, Clone, PartialEq)]
pub struct Outpoint {
    tx_id: Vec<u8>, //ID of the transaction that has the output for this transaction
    index: u32,     //Index of the output (TxOut)
}

impl Outpoint {
    /// Creates a new `Outpoint` instance from the provided byte stream.
    ///
    /// # Arguments
    /// * `stream` - A mutable reference to the byte stream.
    ///
    /// # Returns
    /// A `Result` containing the parsed `Outpoint` instance or a `MessageError` if parsing fails.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Outpoint, MessageError> {
        let tx_id = read_vec_from_bytes(stream, 32)?;
        let index = read_u32_from_bytes(stream, true)?;

        Ok(Outpoint { tx_id, index })
    }

    /// Converts the `Outpoint` instance to a byte representation.
    ///
    /// # Returns
    /// A vector of bytes representing the `Outpoint` instance.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.extend(&self.tx_id);
        buff.extend(self.index.to_le_bytes());

        buff
    }

    /// A reference to the transaction ID associated with the outpoint.
    pub fn get_tx_id(&self) -> &Vec<u8> {
        &self.tx_id
    }

    /// Returns a reference to the output index.
    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn new(tx_id: Vec<u8>, index: u32) -> Outpoint {
        Outpoint { tx_id, index }
    }
}
