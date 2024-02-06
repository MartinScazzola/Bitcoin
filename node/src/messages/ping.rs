use super::message_constants::HEADER_BYTES_SIZE;
use super::message_error::MessageError;
use crate::messages::header::MessageHeader;
use crate::messages::message_constants::PING_COMMAND;
use crate::messages::read_from_bytes::*;
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::io::Read;

/// Represents a Ping message in the Bitcoin protocol.
#[derive(Debug, PartialEq)]
pub struct Ping {
    header: MessageHeader,
    nonce: u64,
}

impl Ping {
    /// Creates a new Ping message with the specified start string and nonce.
    ///
    /// # Arguments
    ///
    /// * `start_string`: The start string of the message.
    /// * `nonce`: A randomly generated nonce value.
    ///
    /// # Returns
    ///
    /// A new `Ping` message.
    pub fn new(start_string: Vec<u8>, nonce: u64) -> Ping {
        let header = MessageHeader::new(start_string, PING_COMMAND.to_string());

        let mut ping = Ping { header, nonce };

        let stream: Vec<u8> = ping.as_bytes();
        let payload_size = stream.len() - HEADER_BYTES_SIZE;
        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        ping.header.update_payload(payload_size as u32, checksum);
        ping
    }

    /// Parses a Ping message from the provided message header and byte stream.
    ///
    /// # Arguments
    ///
    /// * `header`: The message header.
    /// * `stream`: The byte stream to read the message from.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed `Ping` message or an error if parsing fails.
    pub fn from_bytes(header: MessageHeader, stream: &mut dyn Read) -> Result<Ping, MessageError> {
        if header.get_command_name() != PING_COMMAND {
            return Err(MessageError::InvalidInputPing);
        }

        let nonce = read_u64_from_bytes(stream, true)?;
        Ok(Ping { header, nonce })
    }

    /// Serializes the `Ping` message into a byte vector.
    ///
    /// # Returns
    ///
    /// A byte vector representing the serialized `Ping` message.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_bytes();
        buffer.extend(self.nonce.to_le_bytes());
        buffer
    }

    /// Returns the nonce value of the `Ping` message.
    ///
    /// # Returns
    ///
    /// The nonce value.
    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
}

#[cfg(test)]
mod ping_test {
    use super::*;

    #[test]
    fn test_new_ping_from_bytes() -> Result<(), MessageError> {
        let start_string = vec![11, 17, 9, 7];

        let ping_env = Ping::new(start_string, 1234);

        let ping_env_bytes = ping_env.as_bytes();

        let mut stream = ping_env_bytes.as_slice();

        let header = MessageHeader::from_bytes(&mut stream)?;
        let ping_recv = Ping::from_bytes(header, &mut stream)?;

        assert_eq!(ping_env, ping_recv);
        Ok(())
    }
}
