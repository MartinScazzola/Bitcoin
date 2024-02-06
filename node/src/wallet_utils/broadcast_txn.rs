use crate::{
    block_mod::transaction::Transaction,
    messages::{message_error::MessageError, read_from_bytes::fill_command},
};
use std::io::Read;

use super::wallet_utils_constants::BROADCAST_TX_COMMAND;

/// Represents a transaction to be broadcasted.
///
/// This struct contains information about a transaction to be broadcasted over the network. It
/// includes the command name and the transaction itself.
///
/// # Fields
///
/// * `command_name` - A `String` representing the command name associated with the transaction.
/// * `transaction` - The `Transaction` to be broadcasted.
#[derive(Debug)]
pub struct BroadcastTxn {
    command_name: String,
    transaction: Transaction,
}

impl BroadcastTxn {
    /// Creates a new `BroadcastTxn` with the given transaction.
    ///
    /// This function constructs a new `BroadcastTxn` object with the provided transaction. It sets
    /// the command name to "broadcast_tx".
    ///
    /// # Arguments
    ///
    /// * `transaction` - The `Transaction` to be broadcasted.
    ///
    /// # Returns
    ///
    /// A new `BroadcastTxn` object with the specified transaction.
    pub fn new(transaction: Transaction) -> BroadcastTxn {
        let command_name = BROADCAST_TX_COMMAND.to_string();

        BroadcastTxn {
            command_name,
            transaction,
        }
    }

    /// Constructs a `BroadcastTxn` object from the provided bytes.
    ///
    /// This function reads the bytes from the provided stream and constructs a new `BroadcastTxn`
    /// object with the given command name and transaction.
    ///
    /// # Arguments
    ///
    /// * `command_name` - The command name associated with the broadcast transaction.
    /// * `stream` - A mutable reference to a stream implementing the `Read` trait, from which the bytes
    ///              are read to construct the transaction.
    ///
    /// # Returns
    ///
    /// A `Result` containing the constructed `BroadcastTxn` object if successful, or an error of type
    /// `MessageError` if the bytes could not be read or the construction fails.
    pub fn from_bytes(
        command_name: String,
        stream: &mut dyn Read,
    ) -> Result<BroadcastTxn, MessageError> {
        let transaction = Transaction::from_bytes(stream)?;

        Ok(BroadcastTxn {
            command_name,
            transaction,
        })
    }

    /// Converts the `BroadcastTxn` object into a byte representation.
    ///
    /// This function converts the `BroadcastTxn` object into a byte representation by serializing
    /// the command name and transaction into bytes and concatenating them together.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the byte representation of the `BroadcastTxn` object.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = fill_command(self.command_name.as_str()).as_bytes().to_vec();
        buffer.extend(&self.transaction.as_bytes());

        buffer
    }

    pub fn get_txn(&self) -> Transaction {
        self.transaction.clone()
    }
}
