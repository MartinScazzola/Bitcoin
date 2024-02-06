use super::message_error::MessageError;
use std::fmt::Write;
use std::num::ParseIntError;
use std::{io::Read, net::Ipv6Addr};

/// Reads a u8 value from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
///
/// # Returns
///
/// The u8 value read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the value cannot be parsed as a u8.
pub fn read_u8_from_bytes(stream: &mut dyn Read) -> Result<u8, MessageError> {
    let mut buffer = [0u8; 1];
    stream.read_exact(&mut buffer)?;

    Ok(u8::from_le_bytes(buffer))
}

/// Reads an i8 value from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
///
/// # Returns
///
/// The i8 value read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the value cannot be parsed as an i8.
pub fn read_i8_from_bytes(stream: &mut dyn Read) -> Result<i8, MessageError> {
    let mut buffer = [0u8; 1];
    stream.read_exact(&mut buffer)?;

    Ok(i8::from_le_bytes(buffer))
}

/// Reads a u16 value from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
/// * `little_endian` - Flag indicating whether the value is stored in little-endian format.
///
/// # Returns
///
/// The u16 value read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the value cannot be parsed as a u16.
pub fn read_u16_from_bytes(
    stream: &mut dyn Read,
    little_endian: bool,
) -> Result<u16, MessageError> {
    let mut buffer = [0u8; 2];
    stream.read_exact(&mut buffer)?;

    if little_endian {
        Ok(u16::from_le_bytes(buffer))
    } else {
        Ok(u16::from_be_bytes(buffer))
    }
}

/// Reads an i16 value from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
/// * `little_endian` - Flag indicating whether the value is stored in little-endian format.
///
/// # Returns
///
/// The i16 value read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the value cannot be parsed as an i16.
pub fn read_i16_from_bytes(
    stream: &mut dyn Read,
    little_endian: bool,
) -> Result<i16, MessageError> {
    let mut buffer = [0u8; 2];
    stream.read_exact(&mut buffer)?;

    if little_endian {
        Ok(i16::from_le_bytes(buffer))
    } else {
        Ok(i16::from_be_bytes(buffer))
    }
}

/// Reads a u32 value from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
/// * `little_endian` - Flag indicating whether the value is stored in little-endian format.
///
/// # Returns
///
/// The u32 value read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the value cannot be parsed as a u32.
pub fn read_u32_from_bytes(
    stream: &mut dyn Read,
    little_endian: bool,
) -> Result<u32, MessageError> {
    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;

    if little_endian {
        Ok(u32::from_le_bytes(buffer))
    } else {
        Ok(u32::from_be_bytes(buffer))
    }
}

/// Reads an i32 value from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
/// * `little_endian` - Flag indicating whether the value is stored in little-endian format.
///
/// # Returns
///
/// The i32 value read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the value cannot be parsed as an i32.
pub fn read_i32_from_bytes(
    stream: &mut dyn Read,
    little_endian: bool,
) -> Result<i32, MessageError> {
    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;

    if little_endian {
        Ok(i32::from_le_bytes(buffer))
    } else {
        Ok(i32::from_be_bytes(buffer))
    }
}

/// Reads a u64 value from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
/// * `little_endian` - Flag indicating whether the value is stored in little-endian format.
///
/// # Returns
///
/// The u64 value read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the value cannot be parsed as a u64.
pub fn read_u64_from_bytes(
    stream: &mut dyn Read,
    little_endian: bool,
) -> Result<u64, MessageError> {
    let mut buffer = [0u8; 8];
    stream.read_exact(&mut buffer)?;

    if little_endian {
        Ok(u64::from_le_bytes(buffer))
    } else {
        Ok(u64::from_be_bytes(buffer))
    }
}

/// Reads an i64 value from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
/// * `little_endian` - Flag indicating whether the value is stored in little-endian format.
///
/// # Returns
///
/// The i64 value read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the value cannot be parsed as an i64.
pub fn read_i64_from_bytes(
    stream: &mut dyn Read,
    little_endian: bool,
) -> Result<i64, MessageError> {
    let mut buffer = [0u8; 8];
    stream.read_exact(&mut buffer)?;

    if little_endian {
        Ok(i64::from_le_bytes(buffer))
    } else {
        Ok(i64::from_be_bytes(buffer))
    }
}

/// Reads a string from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
/// * `size` - The size of the string to read.
///
/// # Returns
///
/// The string read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream or if the string cannot be parsed.
pub fn read_string_from_bytes(stream: &mut dyn Read, size: usize) -> Result<String, MessageError> {
    if size == 0 {
        return Ok(String::default());
    }

    let mut buffer = vec![0u8; size];
    stream.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer)
        .map_err(|_| MessageError::ReadFromBytes)?
        .replace('\0', ""))
}

/// Reads a byte vector from the byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
/// * `size` - The size of the byte vector to read.
///
/// # Returns
///
/// The byte vector read from the byte stream.
///
/// # Errors
///
/// Returns a `MessageError` if there was an error reading from the stream.
pub fn read_vec_from_bytes(stream: &mut dyn Read, size: usize) -> Result<Vec<u8>, MessageError> {
    let mut buffer = vec![0u8; size];
    if stream.read_exact(&mut buffer).is_err() {
        return Err(MessageError::ReadFromBytes);
    }
    Ok(buffer)
}

/// Decodes a hexadecimal string into a byte vector.
///
/// # Arguments
///
/// * `s` - The hexadecimal string to decode.
///
/// # Returns
///
/// The decoded byte vector.
///
/// # Errors
///
/// Returns a `ParseIntError` if there was an error parsing the hexadecimal string.
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

/// Encodes a byte slice into a hexadecimal string.
///
/// # Arguments
///
/// * `bytes` - The byte slice to encode.
///
/// # Returns
///
/// The encoded hexadecimal string.
pub fn encode_hex(bytes: &[u8]) -> Result<String, std::fmt::Error> {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b)?;
    }
    Ok(s)
}

/// Reads an IPv6 address from a byte stream.
///
/// # Arguments
///
/// * `stream` - The byte stream to read from.
///
/// # Returns
///
/// The parsed IPv6 address.
pub fn read_ipv6_from_bytes(stream: &mut dyn Read) -> Result<Ipv6Addr, MessageError> {
    let a = read_u16_from_bytes(stream, false)?;
    let b = read_u16_from_bytes(stream, false)?;
    let c = read_u16_from_bytes(stream, false)?;
    let d = read_u16_from_bytes(stream, false)?;
    let e = read_u16_from_bytes(stream, false)?;
    let f = read_u16_from_bytes(stream, false)?;
    let g = read_u16_from_bytes(stream, false)?;
    let h = read_u16_from_bytes(stream, false)?;

    Ok(Ipv6Addr::new(a, b, c, d, e, f, g, h))
}

/// Fills a command string with null bytes to make it 12 bytes long.
///
/// # Arguments
///
/// * `string` - The command string.
///
/// # Returns
///
/// The filled command string.
pub fn fill_command(string: &str) -> String {
    let mut buffer = String::from(string);
    while buffer.len() < 12 {
        buffer.push('\0');
    }
    buffer
}

#[cfg(test)]
mod test_read_from_bytes {
    use crate::messages::message_constants::VERSION_COMMAND;
    use std::io::Write;

    use super::*;

    #[test]
    fn test_read_u8_from_bytes() -> Result<(), MessageError> {
        let mut stream = Vec::new();

        let num_env = 32 as u8;

        stream.write_all(&num_env.to_le_bytes())?;

        let num_recv = read_u8_from_bytes(&mut stream.as_slice())?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_i8_from_bytes() -> Result<(), MessageError> {
        let mut stream = Vec::new();

        let num_env = -32 as i8;

        stream.write_all(&num_env.to_le_bytes())?;

        let num_recv = read_i8_from_bytes(&mut stream.as_slice())?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_le_u16_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = 60000 as u16;

        stream.write_all(&num_env.to_le_bytes())?;

        let num_recv = read_u16_from_bytes(&mut stream.as_slice(), true)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_be_u16_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = 60000 as u16;

        stream.write_all(&num_env.to_be_bytes())?;

        let num_recv = read_u16_from_bytes(&mut stream.as_slice(), false)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_le_i16_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = -25000 as i16;

        stream.write_all(&num_env.to_le_bytes())?;

        let num_recv = read_i16_from_bytes(&mut stream.as_slice(), true)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_be_i16_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = -25000 as i16;

        stream.write_all(&num_env.to_be_bytes())?;

        let num_recv = read_i16_from_bytes(&mut stream.as_slice(), false)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_le_u32_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = 4000000000 as u32;

        stream.write_all(&num_env.to_le_bytes())?;

        let num_recv = read_u32_from_bytes(&mut stream.as_slice(), true)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_be_u32_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = 4000000000 as u32;

        stream.write_all(&num_env.to_be_bytes())?;

        let num_recv = read_u32_from_bytes(&mut stream.as_slice(), false)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_le_i32_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = -2000000000 as i32;

        stream.write_all(&num_env.to_le_bytes())?;

        let num_recv = read_i32_from_bytes(&mut stream.as_slice(), true)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_be_i32_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = -2000000000 as i32;

        stream.write_all(&num_env.to_be_bytes())?;

        let num_recv = read_i32_from_bytes(&mut stream.as_slice(), false)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_le_u64_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = 8000000000 as u64;

        stream.write_all(&num_env.to_le_bytes())?;

        let num_recv = read_u64_from_bytes(&mut stream.as_slice(), true)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_be_u64_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = 8000000000 as u64;

        stream.write_all(&num_env.to_be_bytes())?;

        let num_recv = read_u64_from_bytes(&mut stream.as_slice(), false)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_le_i64_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = -8000000000 as i64;

        stream.write_all(&num_env.to_le_bytes())?;

        let num_recv = read_i64_from_bytes(&mut stream.as_slice(), true)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_be_i64_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let num_env = -8000000000 as i64;

        stream.write_all(&num_env.to_be_bytes())?;

        let num_recv = read_i64_from_bytes(&mut stream.as_slice(), false)?;

        assert_eq!(num_recv, num_env);

        Ok(())
    }

    #[test]
    fn test_read_string_from_bytes() -> Result<(), MessageError> {
        let mut stream: Vec<u8> = Vec::new();

        let str_env = "String de prueba";

        stream.write_all(&str_env.as_bytes())?;

        let str_recv = read_string_from_bytes(&mut stream.as_slice(), str_env.len())?;

        assert_eq!(str_env, str_recv);

        Ok(())
    }

    #[test]
    fn test_read_ipv6_from_bytes() -> Result<(), MessageError> {
        let ipv6 = Ipv6Addr::new(123, 45, 67, 3, 2, 123, 678, 1111);

        let mut buffer: Vec<u8> = Vec::new();

        buffer.write_all(&(123 as u16).to_be_bytes())?;
        buffer.write_all(&(45 as u16).to_be_bytes())?;
        buffer.write_all(&(67 as u16).to_be_bytes())?;
        buffer.write_all(&(3 as u16).to_be_bytes())?;
        buffer.write_all(&(2 as u16).to_be_bytes())?;
        buffer.write_all(&(123 as u16).to_be_bytes())?;
        buffer.write_all(&(678 as u16).to_be_bytes())?;
        buffer.write_all(&(1111 as u16).to_be_bytes())?;

        let ipv6_from_bytes = read_ipv6_from_bytes(&mut buffer.as_slice())?;

        assert_eq!(ipv6, ipv6_from_bytes);
        Ok(())
    }

    #[test]
    fn test_fill_command() {
        assert_eq!(fill_command(VERSION_COMMAND), "version\0\0\0\0\0");
    }
}
