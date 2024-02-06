use super::compact_size::CompactSizeUInt;
use super::header::MessageHeader;
use super::message_constants::{GET_HEADERS_COMMAND, HEADER_BYTES_SIZE};
use super::message_error::MessageError;
use super::read_from_bytes::{read_i32_from_bytes, read_vec_from_bytes};
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::io::Read;

/// Represents the `getheaders` message in the Bitcoin protocol.
#[derive(Debug, PartialEq)]
pub struct GetHeaders {
    header: MessageHeader,
    version: i32,
    hash_count: CompactSizeUInt,
    last_block_header: Vec<u8>,
    stopping_hash: Vec<u8>,
}

impl GetHeaders {
    /// Creates a new `GetHeaders` message with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `start_string` - The start string of the message.
    /// * `version` - The version number.
    /// * `last_block_header` - The hash of the last known block header.
    /// * `stopping_hash` - The hash of the block to stop at.
    ///
    /// # Returns
    ///
    /// A new `GetHeaders` message.
    pub fn new(
        start_string: Vec<u8>,
        version: i32,
        last_block_header: Vec<u8>,
        stopping_hash: Vec<u8>,
    ) -> GetHeaders {
        let header = MessageHeader::new(start_string, GET_HEADERS_COMMAND.to_string());

        let mut get_headers = GetHeaders {
            header,
            version,
            hash_count: CompactSizeUInt::from_number(1),
            last_block_header,
            stopping_hash,
        };

        let stream: Vec<u8> = get_headers.as_bytes();
        let payload_size = stream.len() - HEADER_BYTES_SIZE;
        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        get_headers
            .header
            .update_payload(payload_size as u32, checksum);
        get_headers
    }

    /// Creates a `GetHeaders` message by decoding the raw bytes.
    ///
    /// # Arguments
    ///
    /// * `header` - The message header.
    /// * `stream` - The byte stream to decode.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decoded `GetHeaders` message or an error.
    pub fn from_bytes(
        header: MessageHeader,
        stream: &mut dyn Read,
    ) -> Result<GetHeaders, MessageError> {
        if header.get_command_name() != GET_HEADERS_COMMAND {
            return Err(MessageError::InvalidInputHeaders);
        }

        let version = read_i32_from_bytes(stream, true)?;
        let hash_count = CompactSizeUInt::from_bytes(stream)?;
        let last_block_header = read_vec_from_bytes(stream, (hash_count.value() * 32) as usize)?;
        let stopping_hash = read_vec_from_bytes(stream, 32)?;

        Ok(GetHeaders {
            header,
            version,
            hash_count,
            last_block_header,
            stopping_hash,
        })
    }

    /// Converts the `GetHeaders` message to its raw byte representation.
    ///
    /// # Returns
    ///
    /// The byte representation of the `GetHeaders` message.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_bytes();

        buffer.extend(self.version.to_le_bytes());
        buffer.extend(self.hash_count.as_bytes());
        buffer.extend(&self.last_block_header);
        buffer.extend(&self.stopping_hash);

        buffer
    }

    pub fn get_last_block_header(&self) -> &Vec<u8> {
        &self.last_block_header
    }

    pub fn get_stop_hash(&self) -> &Vec<u8> {
        &self.stopping_hash
    }
}

#[cfg(test)]
mod get_data_test {
    use super::*;
    #[test]
    fn test_new_get_headers_from_bytes() -> Result<(), MessageError> {
        let start_string = vec![11u8, 17, 9, 7];
        let version = 70016 as i32;
        let last_block_header = [0u8; 32].to_vec();
        let stopping_hash = [0; 32].to_vec();

        let get_headers = GetHeaders::new(start_string, version, last_block_header, stopping_hash);
        let get_headers_bytes = get_headers.as_bytes();
        let mut stream = get_headers_bytes.as_slice();

        let header = MessageHeader::from_bytes(&mut stream)?;
        let get_header_recv = GetHeaders::from_bytes(header, &mut stream)?;

        assert_eq!(get_headers, get_header_recv);
        Ok(())
    }
}
