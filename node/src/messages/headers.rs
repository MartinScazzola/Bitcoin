use super::super::block_mod::block_header::BlockHeader;
use super::header::MessageHeader;
use super::message_constants::{HEADERS_COMMAND, HEADER_BYTES_SIZE};
use super::{compact_size::CompactSizeUInt, message_error::MessageError};
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::io::Read;

/// Represents the "headers" message in the Bitcoin protocol.
#[derive(Debug, PartialEq)]
pub struct Headers {
    header: MessageHeader,
    count: CompactSizeUInt,
    headers: Vec<BlockHeader>,
}

impl Headers {
    /// Creates a new `Headers` message with the given start string and block headers.
    ///
    /// # Arguments
    ///
    /// * `start_string` - The start string of the message.
    /// * `headers` - The list of block headers.
    ///
    /// # Returns
    ///
    /// A new `Headers` message.
    pub fn new(start_string: Vec<u8>, headers: Vec<BlockHeader>) -> Headers {
        let header = MessageHeader::new(start_string, HEADERS_COMMAND.to_string());

        let mut headers = Headers {
            header,
            count: CompactSizeUInt::from_number(headers.len() as u64),
            headers,
        };

        let stream: Vec<u8> = headers.as_bytes();
        let payload_size = stream.len() - HEADER_BYTES_SIZE;
        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        headers.header.update_payload(payload_size as u32, checksum);

        headers
    }
    /// Parses a `Headers` message from the byte stream.
    ///
    /// # Arguments
    ///
    /// * `header` - The message header.
    /// * `stream` - The byte stream to read from.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed `Headers` message, or a `MessageError` if parsing fails.
    pub fn from_bytes(
        header: MessageHeader,
        stream: &mut dyn Read,
    ) -> Result<Headers, MessageError> {
        if header.get_command_name() != HEADERS_COMMAND {
            return Err(MessageError::InvalidInputHeaders);
        }
        let count = CompactSizeUInt::from_bytes(stream)?;
        let mut headers: Vec<BlockHeader> = Vec::new();

        for _i in 0..count.value() {
            headers.push(BlockHeader::from_bytes(stream)?);
            stream.read_exact(&mut [0u8; 1])?;
        }
        Ok(Headers {
            header,
            count,
            headers,
        })
    }

    /// Serializes the `Headers` message into a byte vector.
    ///
    /// # Returns
    ///
    /// The byte vector representing the serialized `Headers` message.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_bytes();

        buffer.extend(self.count.as_bytes());

        for block_header in self.headers.iter() {
            buffer.extend(block_header.as_bytes());
            buffer.push(0);
        }
        buffer
    }

    /// Returns a clone of the vector containing the block headers.
    ///
    /// # Returns
    ///
    /// * A `Vec<BlockHeader>` representing the block headers.
    pub fn get_headers(&self) -> Vec<BlockHeader> {
        self.headers.clone()
    }

    /// Returns the count of block headers.
    ///
    /// # Returns
    ///
    /// * A `u64` representing the count of block headers.
    pub fn get_count(&self) -> u64 {
        self.count.value()
    }
}

#[cfg(test)]
mod headers_test {
    //use super::*;

    /* #[test]
    fn new_headers_from_bytes() -> Result<(), MessageError> {
        let start_string = vec![11, 17, 9, 7];

        let block_header_1 = BlockHeader::new(2, vec![5; 32], vec![6; 32], 1111, 2222, 3333);
        let block_header_2 = BlockHeader::new(3, vec![7; 32], vec![8; 32], 123, 321, 456);

        let block_header_list = vec![block_header_1, block_header_2];

        let headers_env = Headers::new(start_string, block_header_list);

        let header_env_bytes = headers_env.as_bytes();

        let mut stream = header_env_bytes.as_slice();

        let header = MessageHeader::from_bytes(&mut stream)?;

        let headers_recv = Headers::from_bytes(header, &mut stream)?;

        assert_eq!(headers_env, headers_recv);

        Ok(())
    } */
}
