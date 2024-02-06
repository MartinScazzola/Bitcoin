use std::io::Read;

use crate::messages::{
    message_error::MessageError,
    read_from_bytes::{fill_command, read_vec_from_bytes},
};

use super::wallet_utils_constants::GET_PROOF_COMMAND;

/// Represents a "get_proof" message in the blockchain network.
///
/// The `GetProof` struct contains the necessary fields to construct a "get_proof" message,
/// including the command name, block identifier, and transaction identifier.
///
/// # Fields
///
/// - `command_name`: A `String` representing the command name of the message.
/// - `block`: A `Vec<u8>` representing the identifier of the block.
/// - `txn`: A `Vec<u8>` representing the identifier of the transaction.
#[derive(Debug)]
pub struct GetProof {
    command_name: String,
    block: Vec<u8>,
    txn: Vec<u8>,
}

impl GetProof {
    /// Creates a new `GetProof` message with the specified block and transaction identifiers.
    ///
    /// The `new` function takes the block and transaction identifiers as parameters and returns
    /// a `Result` indicating success or failure. If successful, it constructs a `GetProof` message
    /// with the provided identifiers and returns it wrapped in `Ok`. If any error occurs, it
    /// returns an appropriate `MessageError` wrapped in `Err`.
    ///
    /// # Arguments
    ///
    /// - `block`: A `Vec<u8>` representing the identifier of the block.
    /// - `txn`: A `Vec<u8>` representing the identifier of the transaction.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `GetProof` message if successful, or a `MessageError`
    /// indicating the reason for failure.
    pub fn new(block: Vec<u8>, txn: Vec<u8>) -> GetProof {
        let command_name = GET_PROOF_COMMAND.to_string();

        GetProof {
            command_name,
            block,
            txn,
        }
    }

    /// Parses a `GetProof` message from the given byte stream.
    ///
    /// The `from_bytes` function takes a command name and a mutable reference to a byte stream
    /// (`dyn Read`) as input. It attempts to read the block and transaction identifiers from the
    /// byte stream and constructs a `GetProof` message. The constructed message is returned wrapped
    /// in a `Result` indicating success or failure. If successful, `Ok` is returned with the
    /// constructed `GetProof` message. If any error occurs during parsing or reading from the byte
    /// stream, an appropriate `MessageError` is returned wrapped in `Err`.
    ///
    /// # Arguments
    ///
    /// - `command_name`: A `String` representing the command name of the message.
    /// - `stream`: A mutable reference to a byte stream (`dyn Read`) from which to parse the message.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `GetProof` message if successful, or a `MessageError`
    /// indicating the reason for failure.
    pub fn from_bytes(
        command_name: String,
        stream: &mut dyn Read,
    ) -> Result<GetProof, MessageError> {
        let block = read_vec_from_bytes(stream, 32)?;
        let txn = read_vec_from_bytes(stream, 32)?;

        Ok(GetProof {
            command_name,
            block,
            txn,
        })
    }

    /// Serializes the `GetProof` message into a byte vector.
    ///
    /// The `as_bytes` function serializes the `GetProof` message into a byte vector. It constructs
    /// the byte vector by concatenating the serialized command name, block, and txn fields of the
    /// `GetProof` message. The resulting byte vector represents the binary encoding of the message
    /// that can be sent over the network.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the serialized byte representation of the `GetProof` message.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = fill_command(self.command_name.as_str()).as_bytes().to_vec();
        buffer.extend(&self.block);
        buffer.extend(&self.txn);

        buffer
    }

    pub fn get_block_header(&self) -> Vec<u8> {
        self.block.clone()
    }

    pub fn get_tx_id(&self) -> Vec<u8> {
        self.txn.clone()
    }
}
