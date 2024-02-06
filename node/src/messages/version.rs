use super::compact_size::CompactSizeUInt;
use super::header::MessageHeader;
use super::message_constants::HEADER_BYTES_SIZE;
use super::message_constants::VERSION_COMMAND;
use super::message_error::MessageError;
use super::read_from_bytes::*;
use crate::settings_mod::settings::Settings;
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use chrono::Utc;
use std::io::Read;
use std::net::Ipv6Addr;

/// Version Message is one of the messages exchanged during the handshake process in the Bitcoin protocol.
/// This message is used for two nodes to introduce themselves and agree on connection details such as:
///
/// The version of the protocol they are using
/// The last block they have seen
/// The IP address
/// The port they are listening on
/// among other details.
#[derive(Debug, PartialEq)]
pub struct Version {
    header: MessageHeader,
    version: i32,
    services: u64,
    timestamp: i64,
    addr_recv_services: u64,
    addr_recv_ip: Ipv6Addr,
    addr_recv_port: u16,
    addr_trans_services: u64,
    addr_trans_ip: Ipv6Addr,
    addr_trans_port: u16,
    nonce: u64,
    user_agent_bytes: CompactSizeUInt,
    user_agent: String,
    start_height: i32,
    relay: bool,
}

impl Version {
    pub fn new(ip: Ipv6Addr, settings: &Settings) -> Version {
        let header = MessageHeader::new(settings.get_start_string(), VERSION_COMMAND.to_string());

        let mut version = Version {
            header,
            version: settings.get_protocol_version(),
            services: settings.get_services(),
            timestamp: Utc::now().timestamp(),
            addr_recv_services: settings.get_services(),
            addr_recv_ip: ip,
            addr_recv_port: settings.get_port(),
            addr_trans_services: settings.get_services(),
            addr_trans_ip: settings.get_ip(),
            addr_trans_port: settings.get_port(),
            nonce: rand::random(),
            user_agent_bytes: CompactSizeUInt::from_number(settings.get_user_agent().len() as u64),
            user_agent: settings.get_user_agent(),
            start_height: settings.get_start_height(),
            relay: settings.get_relay(),
        };

        let stream: Vec<u8> = version.as_bytes();

        let payload_size = stream.len() - HEADER_BYTES_SIZE;
        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        version.header.update_payload(payload_size as u32, checksum);

        version
    }

    /// Implementación del trait *FromBytes* para el mensaje ***Version***
    pub fn from_bytes(
        header: MessageHeader,
        stream: &mut dyn Read,
    ) -> Result<Version, MessageError> {
        if header.get_command_name() != VERSION_COMMAND {
            return Err(MessageError::InvalidInputVersion);
        }

        let version = read_i32_from_bytes(stream, true)?;
        let services = read_u64_from_bytes(stream, true)?;
        let timestamp = read_i64_from_bytes(stream, true)?;
        let addr_recv_services = read_u64_from_bytes(stream, true)?;
        let addr_recv_ip = read_ipv6_from_bytes(stream)?;
        let addr_recv_port = read_u16_from_bytes(stream, false)?;
        let addr_trans_services = read_u64_from_bytes(stream, true)?;
        let addr_trans_ip = read_ipv6_from_bytes(stream)?;
        let addr_trans_port = read_u16_from_bytes(stream, false)?;
        let nonce = read_u64_from_bytes(stream, true)?;
        let user_agent_bytes = CompactSizeUInt::from_bytes(stream)?;
        let user_agent = read_string_from_bytes(stream, user_agent_bytes.value() as usize)?;
        let start_height = read_i32_from_bytes(stream, true)?;

        let relay = match read_u8_from_bytes(stream) {
            Ok(value) => value == 0,
            Err(_) => false,
        };

        Ok(Version {
            header,
            version,
            services,
            timestamp,
            addr_recv_services,
            addr_recv_ip,
            addr_recv_port,
            addr_trans_services,
            addr_trans_ip,
            addr_trans_port,
            nonce,
            user_agent_bytes,
            user_agent,
            start_height,
            relay,
        })
    }

    /// Implementación del trait *AsBytes* para el mensaje ***Version***
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = self.header.as_bytes();

        buff.extend(self.version.to_le_bytes());
        buff.extend(self.services.to_le_bytes());
        buff.extend(self.timestamp.to_le_bytes());
        buff.extend(self.addr_recv_services.to_le_bytes());
        buff.extend(self.addr_recv_ip.octets());
        buff.extend(self.addr_recv_port.to_be_bytes());
        buff.extend(self.addr_trans_services.to_le_bytes());
        buff.extend(self.addr_trans_ip.octets());
        buff.extend(self.addr_trans_port.to_be_bytes());
        buff.extend(self.nonce.to_le_bytes());
        buff.extend(self.user_agent_bytes.as_bytes());
        buff.extend(self.user_agent.as_bytes());
        buff.extend(self.start_height.to_le_bytes());

        if self.relay {
            buff.extend([0u8]);
        } else {
            buff.extend([1u8]);
        }

        buff
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }
    pub fn get_services(&self) -> u64 {
        self.services
    }
}

#[cfg(test)]
mod version_test {
    use super::*;

    #[test]
    fn test_new_version_from_bytes() -> Result<(), MessageError> {
        let ip_recv = Ipv6Addr::new(2, 2, 2, 2, 2, 2, 2, 2);

        let settings =
            Settings::from_file("settings/nodo.conf").map_err(|_| MessageError::ReadFromBytes)?;

        let version_env = Version::new(ip_recv, &settings);

        let verison_env_bytes = version_env.as_bytes();

        let mut stream = verison_env_bytes.as_slice();

        let header = MessageHeader::from_bytes(&mut stream)?;
        let version_recv = Version::from_bytes(header, &mut stream)?;

        assert_eq!(version_env, version_recv);
        Ok(())
    }
}
