use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use crate::messages::read_from_bytes::{read_u32_from_bytes, read_vec_from_bytes};
use crate::messages::script::Script;
use std::io::Read;

/// Represents a transaction input of the coinbase in the Bitcoin protocol.
#[derive(Debug, Clone)]
pub struct TxInCoinbase {
    hash: Vec<u8>, // Null
    index: u32,    // Null
    script_bytes: CompactSizeUInt,
    height: Script, // script
    coinbase_script: Vec<u8>,
    sequence: u32,
}

impl TxInCoinbase {
    /// Creates a new `TxInCoinbase` instance from the provided byte stream.
    ///
    /// # Arguments
    /// * `stream` - A mutable reference to the byte stream.
    ///
    /// # Returns
    /// A `Result` containing the parsed `TxInCoinbase` instance or a `MessageError` if parsing fails.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<TxInCoinbase, MessageError> {
        let hash = read_vec_from_bytes(stream, 32)?;

        let index = read_u32_from_bytes(stream, true)?;

        let script_bytes = CompactSizeUInt::from_bytes(stream)?;

        let height = Script::from_bytes(stream)?;

        let coinbase_script = read_vec_from_bytes(
            stream,
            (script_bytes.value() - height.cant_bytes() as u64) as usize,
        )?;

        let sequence = read_u32_from_bytes(stream, true)?;

        Ok(TxInCoinbase {
            hash,
            index,
            script_bytes,
            height,
            coinbase_script,
            sequence,
        })
    }

    /// Converts the `TxInCoinbase` instance to a byte representation.
    ///
    /// # Returns
    /// A vector of bytes representing the `TxInCoinbase` instance.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.extend(&self.hash);
        buff.extend(self.index.to_le_bytes());
        buff.extend(self.script_bytes.as_bytes());
        buff.extend(self.height.as_bytes());
        buff.extend(&self.coinbase_script);
        buff.extend(self.sequence.to_le_bytes());

        buff
    }
}
