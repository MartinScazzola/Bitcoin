use crate::{
    block_mod::transaction::Transaction,
    messages::{message_error::MessageError, read_from_bytes::read_string_from_bytes},
};
use std::io::Read;

/// Represents a wallet transaction.
#[derive(Clone, Debug, PartialEq)]
pub struct WalletTx {
    transaction: Transaction,
    date: String,
}

impl WalletTx {
    /// Creates a new `WalletTx` object.
    ///
    /// # Arguments
    ///
    /// * `transaction`: A `Transaction` object representing the underlying transaction.
    /// * `date`: A `String` representing the date of the transaction.
    ///
    /// # Returns
    ///
    /// A `WalletTx` object initialized with the provided transaction and date.
    pub fn new(transaction: Transaction, date: String) -> WalletTx {
        WalletTx { transaction, date }
    }

    /// Creates a `WalletTx` object by deserializing it from a byte stream.
    ///
    /// # Arguments
    ///
    /// * `stream`: A mutable reference to a type that implements the `Read` trait, providing the byte stream to deserialize from.
    ///
    /// # Returns
    ///
    /// A `Result` containing either the deserialized `WalletTx` object or a `MessageError` if deserialization fails.
    ///
    /// # Errors
    ///
    /// The function can return a `MessageError` if there is an error during deserialization.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<WalletTx, MessageError> {
        let transaction = Transaction::from_bytes(stream)?;
        let date = read_string_from_bytes(stream, 17)?;

        Ok(WalletTx { transaction, date })
    }

    /// Serializes the `WalletTx` object into a byte vector.
    ///
    /// # Returns
    ///
    /// A byte vector containing the serialized representation of the `WalletTx` object.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend(&self.transaction.as_bytes());
        buffer.extend(self.date.as_bytes());

        buffer
    }

    pub fn get_tx(&self) -> &Transaction {
        &self.transaction
    }

    pub fn get_date(&self) -> &String {
        &self.date
    }
}

#[cfg(test)]
mod header_test {
    use chrono::{Local, NaiveDateTime};

    #[test]
    fn test_date_time() {
        let datetime = NaiveDateTime::from_timestamp_opt(963916800, 0).unwrap();
        let _date = datetime.date();

        println!(
            "Date: {:?}",
            datetime.format("%Y-%m-%d %H-%M-%S").to_string()
        );
    }

    #[test]
    fn test_date_time2() {
        let current_datetime = Local::now();
        let formatted_datetime = current_datetime.format("%Y-%m-%d %H:%M:%S");
        println!("{}", formatted_datetime);
    }
}
