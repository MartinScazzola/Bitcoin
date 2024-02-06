use crate::messages::{
    compact_size::CompactSizeUInt,
    message_error::MessageError,
    read_from_bytes::{read_i64_from_bytes, read_vec_from_bytes},
};
use std::io::Read;

/// Represents a transaction output (TxOut) in a transaction.
#[derive(Debug, Clone, PartialEq)]
pub struct TxOut {
    value: i64,
    pk_script_bytes: CompactSizeUInt,
    pk_script: Vec<u8>,
}

impl TxOut {
    pub fn new(value: i64, pk_script: Vec<u8>) -> TxOut {
        TxOut {
            value,
            pk_script_bytes: CompactSizeUInt::from_number(pk_script.len() as u64),
            pk_script,
        }
    }
    /// Parses a byte stream and constructs a `TxOut` (transaction output) from it.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a byte stream implementing the `Read` trait.
    ///
    /// # Returns
    ///
    /// - `Ok(TxOut)` if parsing is successful.
    /// - `Err(MessageError)` if an error occurs during parsing.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<TxOut, MessageError> {
        let value = read_i64_from_bytes(stream, true)?;
        let pk_script_bytes = CompactSizeUInt::from_bytes(stream)?;
        let pk_script = read_vec_from_bytes(stream, pk_script_bytes.value() as usize)?;

        Ok(TxOut {
            value,
            pk_script_bytes,
            pk_script,
        })
    }

    /// Converts the `TxOut` (transaction output) into a byte representation.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the byte representation of the `TxOut`.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.extend(self.value.to_le_bytes());
        buff.extend(self.pk_script_bytes.as_bytes());
        buff.extend(&self.pk_script);

        buff
    }

    /// Returns a reference to the value of the transaction output.
    pub fn get_value(&self) -> i64 {
        self.value
    }

    pub fn get_pk_script(&self) -> Vec<u8> {
        self.pk_script.clone()
    }
}
