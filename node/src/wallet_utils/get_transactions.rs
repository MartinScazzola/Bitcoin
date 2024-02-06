use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::read_from_bytes::read_u32_from_bytes;
use crate::messages::{
    message_error::MessageError,
    read_from_bytes::{fill_command, read_vec_from_bytes},
};
use std::io::Read;

use super::wallet_utils_constants::GET_TX_COMMAND;

///Represents a command to request transactions from the wallet.
#[derive(Debug)]
pub struct GetTransactions {
    command_name: String,
    pk_script: Vec<u8>,
    public_key: Vec<u8>,
    last_update: u32,
}

impl GetTransactions {
    /// Creates a new `GetTransactions` instance.
    ///
    /// # Arguments
    ///
    /// * `pk_script` - The public key script.
    /// * `public_key` - The public key.
    /// * `last_update` - The last update timestamp.
    ///
    /// # Returns
    ///
    /// A new `GetTransactions` instance.
    pub fn new(pk_script: Vec<u8>, public_key: Vec<u8>, last_update: u32) -> GetTransactions {
        GetTransactions {
            command_name: GET_TX_COMMAND.to_string(),
            pk_script,
            public_key,
            last_update,
        }
    }

    /// Converts the `GetTransactions` struct into a byte vector.
    ///
    /// The resulting byte vector represents the serialized form of the `GetTransactions` struct,
    /// suitable for network transmission or storage.
    ///
    /// # Returns
    ///
    /// A byte vector containing the serialized representation of the `GetTransactions` struct.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = fill_command(self.command_name.as_str()).as_bytes().to_vec();
        buffer.extend(CompactSizeUInt::from_number(self.pk_script.len() as u64).as_bytes());
        buffer.extend(&self.pk_script);
        buffer.extend(&self.public_key);
        buffer.extend(self.last_update.to_le_bytes());
        buffer
    }

    /// Parses a byte stream into a `GetTransactions` struct.
    ///
    /// This function attempts to parse a byte stream into a `GetTransactions` struct,
    /// extracting the `pk_script`, `public_key`, and `last_update` fields.
    ///
    /// # Arguments
    ///
    /// * `command_name` - The name of the command.
    /// * `stream` - A mutable reference to a byte stream implementing the `Read` trait.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `GetTransactions` struct if successful,
    /// or a `MessageError` if an error occurs during parsing.
    pub fn from_bytes(
        command_name: String,
        stream: &mut dyn Read,
    ) -> Result<GetTransactions, MessageError> {
        let pk_script_bytes = CompactSizeUInt::from_bytes(stream)?;
        let pk_script = read_vec_from_bytes(stream, pk_script_bytes.value() as usize)?;
        let public_key = read_vec_from_bytes(stream, 33)?;
        let last_update = read_u32_from_bytes(stream, true)?;

        Ok(GetTransactions {
            command_name,
            pk_script,
            public_key,
            last_update,
        })
    }

    pub fn get_last_update(&self) -> u32 {
        self.last_update
    }

    pub fn get_pk_script(&self) -> &Vec<u8> {
        &self.pk_script
    }

    pub fn get_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }
}
