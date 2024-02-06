use super::compact_size::CompactSizeUInt;
use super::header::MessageHeader;
use super::ip::Ip;
use super::message_constants::{ADDR_COMMAND, HEADER_BYTES_SIZE};
use super::message_error::MessageError;
use super::read_from_bytes::{
    read_ipv6_from_bytes, read_u16_from_bytes, read_u32_from_bytes, read_u64_from_bytes,
};
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::io::Read;

/// Represents a network address message in the Bitcoin protocol.
#[derive(Debug, PartialEq)]
pub struct Addr {
    header: MessageHeader,
    ip_address_count: CompactSizeUInt,
    ip_address_list: Vec<Ip>,
}

impl Addr {
    /// Creates a new `Addr` object with the given start string and IP address list.
    ///
    /// # Arguments
    ///
    /// * `start_string` - The start string used to construct the message header.
    /// * `ip_address_list` - The list of IP addresses to be included in the `Addr` object.
    ///
    /// # Returns
    ///
    /// A new `Addr` object.
    pub fn new(start_string: Vec<u8>, ip_address_list: Vec<Ip>) -> Addr {
        let header = MessageHeader::new(start_string, ADDR_COMMAND.to_string());

        let mut addr = Addr {
            header,
            ip_address_count: CompactSizeUInt::from_number(ip_address_list.len() as u64),
            ip_address_list,
        };

        let stream: Vec<u8> = addr.as_bytes();

        let payload_size = stream.len() - HEADER_BYTES_SIZE;

        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        addr.header.update_payload(payload_size as u32, checksum);

        addr
    }

    /// Creates a new `Addr` object by parsing the serialized byte stream.
    ///
    /// # Arguments
    ///
    /// * `header` - The message header associated with the `Addr` object.
    /// * `stream` - A mutable reference to the byte stream to be parsed.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Addr` object if successful, or a `MessageError` if an error occurs during parsing.
    pub fn from_bytes(header: MessageHeader, stream: &mut dyn Read) -> Result<Addr, MessageError> {
        if header.get_command_name() != ADDR_COMMAND {
            return Err(MessageError::InvalidInputAddr);
        }

        let ip_address_count = CompactSizeUInt::from_bytes(stream)?;
        let mut ip_address_list: Vec<Ip> = Vec::new();

        for _i in 0..ip_address_count.value() {
            let time = read_u32_from_bytes(stream, true)?;
            let services = read_u64_from_bytes(stream, true)?;
            let ip_address = read_ipv6_from_bytes(stream)?;
            let port = read_u16_from_bytes(stream, false)?;

            ip_address_list.push(Ip::new(time, services, ip_address, port));
        }

        Ok(Addr {
            header,
            ip_address_count,
            ip_address_list,
        })
    }

    /// Serializes the `Addr` object into a byte stream.
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` containing the serialized byte stream of the `Addr` object.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = self.header.as_bytes();

        buff.extend(self.ip_address_count.as_bytes());
        self.ip_address_list
            .iter()
            .for_each(|ip| buff.extend(ip.as_bytes()));
        buff
    }
}

#[cfg(test)]
mod addr_test {
    use std::net::Ipv6Addr;

    use super::*;

    #[test]
    fn test_new_addr_message_from_bytes() -> Result<(), MessageError> {
        let start_string = vec![11u8, 17, 9, 7];

        let ip1 = Ip::new(1234, 1024, Ipv6Addr::new(1, 2, 3, 1, 2, 3, 1, 1), 18333);
        let ip2 = Ip::new(222, 1024, Ipv6Addr::new(4, 5, 6, 4, 5, 6, 4, 5), 18333);

        let ip_address_list = vec![ip1, ip2];

        let addr_env = Addr::new(start_string, ip_address_list);

        let addr_env_bytes = addr_env.as_bytes();

        let mut stream = addr_env_bytes.as_slice();

        let header_addr_recv = MessageHeader::from_bytes(&mut stream)?;

        let addr_recv = Addr::from_bytes(header_addr_recv, &mut stream)?;

        assert_eq!(addr_env, addr_recv);

        Ok(())
    }
}
