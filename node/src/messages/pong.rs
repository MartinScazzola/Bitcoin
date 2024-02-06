use super::message_constants::HEADER_BYTES_SIZE;
use super::message_error::MessageError;
use crate::messages::header::MessageHeader;
use crate::messages::message_constants::PONG_COMMAND;
use crate::messages::read_from_bytes::*;
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::io::Read;

/// Represents the "Pong" message in the Bitcoin protocol.
#[derive(Debug, PartialEq)]
pub struct Pong {
    header: MessageHeader,
    nonce: u64,
}

impl Pong {
    /// Constructs a new Pong message with the given start string and nonce.
    ///
    /// # Arguments
    ///
    /// * `start_string` - The start string of the message.
    /// * `nonce` - The nonce value.
    ///
    /// # Returns
    ///
    /// A new Pong message.
    pub fn new(start_string: Vec<u8>, nonce: u64) -> Pong {
        let header = MessageHeader::new(start_string, PONG_COMMAND.to_string());

        let mut pong = Pong { header, nonce };

        let stream = pong.as_bytes();
        let payload_size = stream.len() - HEADER_BYTES_SIZE;
        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        pong.header.update_payload(payload_size as u32, checksum);
        pong
    }

    /// Constructs a Pong message from the given header and byte stream.
    ///
    /// # Arguments
    ///
    /// * `header` - The message header.
    /// * `stream` - The byte stream to read the message from.
    ///
    /// # Returns
    ///
    /// A Result containing the constructed Pong message or an error if parsing fails.
    pub fn from_bytes(header: MessageHeader, stream: &mut dyn Read) -> Result<Pong, MessageError> {
        if header.get_command_name() != PONG_COMMAND {
            return Err(MessageError::InvalidInputPong);
        }

        let nonce = read_u64_from_bytes(stream, true)?;

        Ok(Pong { header, nonce })
    }

    /// Serializes the Pong message into a byte vector.
    ///
    /// # Returns
    ///
    /// The serialized byte vector representation of the Pong message.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_bytes();
        buffer.extend(self.nonce.to_le_bytes());
        buffer
    }
}

#[cfg(test)]
mod pong_test {
    use super::*;

    #[test]
    fn test_new_pong_from_bytes() -> Result<(), MessageError> {
        let start_string = vec![11, 17, 9, 7];

        let pong_env = Pong::new(start_string, 123456);
        let pong_env_bytes = pong_env.as_bytes();
        let mut stream = pong_env_bytes.as_slice();

        let header = MessageHeader::from_bytes(&mut stream)?;
        let pong_recv = Pong::from_bytes(header, &mut stream)?;

        assert_eq!(pong_env, pong_recv);

        Ok(())
    }
}
