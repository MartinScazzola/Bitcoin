use super::message_error::MessageError;
use super::read_from_bytes::*;
use crate::messages::message_constants::*;
use std::io::Read;

/// Represents a variable-size unsigned integer using compact size encoding.
#[derive(Debug, PartialEq, Clone)]
pub struct CompactSizeUInt {
    value: u64,
}

impl CompactSizeUInt {
    /// Creates a `CompactSizeUInt` object from the given `number`.
    ///
    /// # Arguments
    ///
    /// * `number` - The value of the unsigned integer.
    ///
    /// # Returns
    ///
    /// A `CompactSizeUInt` object initialized with the provided `number`.
    pub fn from_number(number: u64) -> CompactSizeUInt {
        CompactSizeUInt { value: number }
    }

    /// Reads a `CompactSizeUInt` object from a byte stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a byte stream implementing the `Read` trait.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `CompactSizeUInt` object on success, or a `MessageError` on failure.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<CompactSizeUInt, MessageError> {
        let value = read_u8_from_bytes(stream)?;

        if value <= BYTE_SIZE as u8 {
            return Ok(CompactSizeUInt::from_number(value as u64));
        } else if value == TWO_BYTE_SIZE {
            let value16 = read_u16_from_bytes(stream, true)?;
            return Ok(CompactSizeUInt::from_number(value16 as u64));
        } else if value == FOUR_BYTE_SIZE {
            let value32 = read_u32_from_bytes(stream, true)?;
            return Ok(CompactSizeUInt::from_number(value32 as u64));
        }

        let value64 = read_u64_from_bytes(stream, true)?;
        Ok(CompactSizeUInt::from_number(value64))
    }

    /// Checks the size category of the `CompactSizeUInt` object.
    ///
    /// # Returns
    ///
    /// A `u8` value representing the size category of the `CompactSizeUInt`:
    /// * If the value is less than or equal to `BYTE_SIZE`, it returns the value as a `u8`.
    /// * If the value is between 253 (inclusive) and 0xffff (inclusive), it returns `TWO_BYTE_SIZE`.
    /// * If the value is between 0x10000 (inclusive) and 0xffffffff (inclusive), it returns `FOUR_BYTE_SIZE`.
    /// * Otherwise, it returns `EIGHT_BYTE_SIZE`.
    fn check_size(&self) -> u8 {
        if self.value <= BYTE_SIZE {
            return self.value as u8;
        } else if self.value >= 253 && self.value <= 0xffff {
            return TWO_BYTE_SIZE;
        } else if self.value >= 0x10000 && self.value <= 0xffffffff {
            return FOUR_BYTE_SIZE;
        }
        EIGHT_BYTE_SIZE
    }

    /// Converts the `CompactSizeUInt` object into a byte vector representation.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the byte representation of the `CompactSizeUInt` object.
    /// The byte vector includes the size prefix based on the value of the `CompactSizeUInt`.
    /// If the size is less than or equal to `BYTE_SIZE`, it includes the size as a single byte.
    /// If the size is `TWO_BYTE_SIZE`, it includes a size prefix followed by a 16-bit value in little-endian format.
    /// If the size is `FOUR_BYTE_SIZE`, it includes a size prefix followed by a 32-bit value in little-endian format.
    /// If the size is `EIGHT_BYTE_SIZE`, it includes a size prefix followed by a 64-bit value in little-endian format.
    pub fn as_bytes(&self) -> Vec<u8> {
        let size = self.check_size();
        let mut buffer = Vec::new();

        buffer.extend(&size.to_le_bytes());

        if size == TWO_BYTE_SIZE {
            let value16 = (self.value as u16).to_le_bytes();
            buffer.extend(&value16);
        } else if size == FOUR_BYTE_SIZE {
            let value32 = (self.value as u32).to_le_bytes();
            buffer.extend(&value32);
        } else if size == EIGHT_BYTE_SIZE {
            let value64 = self.value.to_le_bytes();
            buffer.extend(&value64);
        }
        buffer
    }

    /// Returns the value of the `CompactSizeUInt` object.
    ///
    /// # Returns
    ///
    /// The unsigned 64-bit integer value stored in the `CompactSizeUInt`.
    pub fn value(&self) -> u64 {
        self.value
    }
}

#[cfg(test)]
mod test_compact_size {
    use super::*;
    #[test]
    fn test_new_little_compact_size_from_bytes() -> Result<(), MessageError> {
        let number_env = CompactSizeUInt::from_number(56);

        let number_env_bytes = number_env.as_bytes();

        let mut stream = number_env_bytes.as_slice();

        let number_recv = CompactSizeUInt::from_bytes(&mut stream)?;

        assert_eq!(number_env, number_recv);
        Ok(())
    }

    #[test]
    fn test_new_medium_compact_size_from_bytes() -> Result<(), MessageError> {
        let number_env = CompactSizeUInt::from_number(10000);

        let number_env_bytes = number_env.as_bytes();

        let mut stream = number_env_bytes.as_slice();

        let number_recv = CompactSizeUInt::from_bytes(&mut stream)?;

        assert_eq!(number_env, number_recv);
        Ok(())
    }

    #[test]
    fn test_new_large_compact_size_from_bytes() -> Result<(), MessageError> {
        let number_env = CompactSizeUInt::from_number(1000000000);

        let number_env_bytes = number_env.as_bytes();

        let mut stream = number_env_bytes.as_slice();

        let number_recv = CompactSizeUInt::from_bytes(&mut stream)?;

        assert_eq!(number_env, number_recv);
        Ok(())
    }
}
