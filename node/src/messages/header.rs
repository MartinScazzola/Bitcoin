use super::read_from_bytes::fill_command;
use super::{
    message_error::MessageError,
    read_from_bytes::{read_string_from_bytes, read_u32_from_bytes, read_vec_from_bytes},
};
use crate::messages::message_constants::CHECKSUM_EMPTY_MSG;
use crate::messages::message_constants::PAYLOAD_EMPTY_MSG;
use std::io::Read;

/// Represents the header of a Bitcoin protocol message.
#[derive(Debug, PartialEq, Clone)]
pub struct MessageHeader {
    start_string: Vec<u8>,
    command_name: String,
    payload_size: u32,
    checksum: Vec<u8>,
}

impl MessageHeader {
    /// Creates a new `MessageHeader` with the specified start string and command name.
    ///
    /// # Arguments
    ///
    /// * `start_string` - The start string of the message.
    /// * `command_name` - The command name of the message.
    pub fn new(start_string: Vec<u8>, command_name: String) -> MessageHeader {
        MessageHeader {
            start_string,
            command_name,
            payload_size: PAYLOAD_EMPTY_MSG,
            checksum: CHECKSUM_EMPTY_MSG.to_vec(),
        }
    }

    /// Parses a `MessageHeader` from a byte stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The byte stream to read the header from.
    ///
    /// # Returns
    ///
    /// * `Result<MessageHeader, MessageError>` - The parsed `MessageHeader` if successful, or an error
    ///   if the stream does not contain a valid header.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<MessageHeader, MessageError> {
        let start_string = read_vec_from_bytes(stream, 4)?;
        let command_name = read_string_from_bytes(stream, 12)?;
        let payload_size = read_u32_from_bytes(stream, true)?;
        let checksum = read_vec_from_bytes(stream, 4)?;

        Ok(MessageHeader {
            start_string,
            command_name,
            payload_size,
            checksum,
        })
    }

    /// Converts the `MessageHeader` to a byte representation.
    ///
    /// The byte representation follows the Bitcoin protocol message header format.
    ///
    /// # Returns
    ///
    /// * `Vec<u8>` - The byte representation of the `MessageHeader`.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff: Vec<u8> = vec![];

        buff.extend(self.get_start_string());
        buff.extend(fill_command(self.get_command_name().as_str()).as_bytes());
        buff.extend(self.get_payload_size().to_le_bytes()); //Tiene que ser Little Endian
        buff.extend(self.get_checksum());

        buff
    }

    /// Updates the payload size and checksum of the `MessageHeader`.
    ///
    /// # Arguments
    ///
    /// * `payload_size` - The size of the payload.
    /// * `checksum` - The checksum of the payload.
    pub fn update_payload(&mut self, payload_size: u32, checksum: Vec<u8>) {
        self.payload_size = payload_size;
        self.checksum = checksum;
    }

    /// Returns a reference to the start string of the `MessageHeader`.
    ///
    /// # Returns
    ///
    /// A reference to the start string.
    pub fn get_start_string(&self) -> &Vec<u8> {
        &self.start_string
    }

    /// Returns a reference to the command name of the `MessageHeader`.
    ///
    /// # Returns
    ///
    /// A reference to the command name.
    pub fn get_command_name(&self) -> &String {
        &self.command_name
    }

    /// Returns the payload size of the `MessageHeader`.
    ///
    /// # Returns
    ///
    /// The payload size.
    pub fn get_payload_size(&self) -> u32 {
        self.payload_size
    }

    /// Returns the checksum of the `MessageHeader`.
    ///
    /// # Returns
    ///
    /// A reference to the checksum.
    pub fn get_checksum(&self) -> &Vec<u8> {
        &self.checksum
    }
}

#[cfg(test)]
mod header_test {
    use super::MessageHeader;
    use crate::messages::{message_constants::VERACK_COMMAND, message_error::MessageError};

    #[test]
    fn test_new_header_from_bytes() -> Result<(), MessageError> {
        let start_string = vec![11u8, 17, 9, 7];
        let command_name = VERACK_COMMAND.to_string();

        let mut header_env = MessageHeader::new(start_string, command_name);
        let payload_size = 50;
        let checksum = vec![4u8, 8, 12, 53];
        header_env.update_payload(payload_size, checksum);

        let header_env_bytes = header_env.as_bytes();

        let mut stream = header_env_bytes.as_slice();

        let header_recv = MessageHeader::from_bytes(&mut stream)?;

        assert_eq!(header_env, header_recv);
        Ok(())
    }
}
