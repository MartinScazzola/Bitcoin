use super::{message_error::MessageError, read_from_bytes::read_u8_from_bytes};
use std::io::Read;

/// This structure represents a number which can variate in byte length, and the size is set by the first byte
#[derive(Debug, Clone)]
pub struct Script {
    number: u64,
    bytes: u8,
    read_bytes: Vec<u8>,
}

impl Script {
    /// Parses a Script from a byte stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - The byte stream to read from.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed Script or an error if parsing fails.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Script, MessageError> {
        let bytes = read_u8_from_bytes(stream)?;
        let mut read_bytes = Vec::new();
        let mut number: u64 = 0;
        for i in 0..bytes {
            let byte_i = read_u8_from_bytes(stream)?;

            number += (u64::pow(256, i as u32)) * byte_i as u64;

            read_bytes.push(byte_i);
        }
        Ok(Script {
            number,
            bytes,
            read_bytes,
        })
    }

    /// Returns the value of the Script.
    ///
    /// # Returns
    ///
    /// The value of the Script.
    pub fn value(&self) -> u64 {
        self.number
    }

    /// Returns the number of bytes occupied by the Script, including the byte indicating the number of bytes.
    ///
    /// # Returns
    ///
    /// The number of bytes occupied by the Script.
    pub fn cant_bytes(&self) -> u8 {
        self.bytes + 1
    }

    /// Converts the Script to its byte representation.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the Script.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = vec![self.bytes];

        buff.extend(&self.read_bytes);

        buff
    }
}

#[cfg(test)]
mod script_test {
    use super::*;

    #[test]
    fn test_script_from_bytes() -> Result<(), MessageError> {
        let bytes: Vec<u8> = vec![3, 78, 1, 5]; //

        let mut stream = bytes.as_slice();

        let number_recv = Script::from_bytes(&mut stream)?;

        assert_eq!(328014, number_recv.value());

        Ok(())
    }
}
