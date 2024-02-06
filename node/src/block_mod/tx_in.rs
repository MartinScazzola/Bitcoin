use super::outpoint::Outpoint;
use crate::messages::{
    compact_size::CompactSizeUInt,
    message_error::MessageError,
    read_from_bytes::{read_u32_from_bytes, read_vec_from_bytes},
};
use std::io::Read;

/// Represents a transaction input (TxIn) in a transaction.
#[derive(Debug, Clone, PartialEq)]
pub struct TxIn {
    previous_output: Outpoint,
    script_bytes: CompactSizeUInt,
    signature_script: Vec<u8>,
    sequence: u32,
}

impl TxIn {
    /// Parses a byte stream and constructs a `TxIn` (transaction input) from it.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a byte stream implementing the `Read` trait.
    ///
    /// # Returns
    ///
    /// - `Ok(TxIn)` if parsing is successful.
    /// - `Err(MessageError)` if an error occurs during parsing.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<TxIn, MessageError> {
        let previous_output = Outpoint::from_bytes(stream)?;
        let script_bytes = CompactSizeUInt::from_bytes(stream)?;
        let signature_script = read_vec_from_bytes(stream, script_bytes.value() as usize)?;
        let sequence = read_u32_from_bytes(stream, true)?;

        Ok(TxIn {
            previous_output,
            script_bytes,
            signature_script,
            sequence,
        })
    }

    /// Converts the `TxIn` (transaction input) into a byte representation.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the byte representation of the `TxIn`.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.extend(self.previous_output.as_bytes());
        buff.extend(self.script_bytes.as_bytes());
        buff.extend(&self.signature_script);
        buff.extend(self.sequence.to_le_bytes());

        buff
    }

    /// Returns a reference to the previous output being spent by this input.
    pub fn get_prev_output(&self) -> &Outpoint {
        &self.previous_output
    }

    pub fn new(
        prev_tx: Vec<u8>,
        prev_index: u32,
        signature_script: Vec<u8>,
        sequence: u32,
    ) -> TxIn {
        let outpoint = Outpoint::new(prev_tx, prev_index);

        TxIn {
            previous_output: outpoint,
            script_bytes: CompactSizeUInt::from_number(signature_script.len() as u64),
            signature_script,
            sequence,
        }
    }

    pub fn set_signature(&mut self, signature: Vec<u8>) {
        self.script_bytes = CompactSizeUInt::from_number(signature.len() as u64);
        self.signature_script = signature;
    }

    pub fn get_sequence(&self) -> u32 {
        self.sequence
    }

    pub fn get_signature_script(&self) -> &Vec<u8> {
        &self.signature_script
    }
}
