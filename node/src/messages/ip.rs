use super::message_error::MessageError;
use super::read_from_bytes::*;
use std::io::Read;
use std::net::Ipv6Addr;

/// Represents an IP address with associated information.
#[derive(Debug, PartialEq)]
pub struct Ip {
    time: u32,
    services: u64,
    ip: Ipv6Addr,
    port: u16,
}

impl Ip {
    /// Creates a new `Ip` instance with the specified parameters.
    pub fn new(time: u32, services: u64, ip: Ipv6Addr, port: u16) -> Ip {
        Ip {
            time,
            services,
            ip,
            port,
        }
    }

    /// Parses a byte stream and constructs an `Ip` instance from it.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to a byte stream implementing the `Read` trait.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `Ip` instance if successful, or a `MessageError` if an error occurred during parsing.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Ip, MessageError> {
        let time = read_u32_from_bytes(stream, true)?;
        let services = read_u64_from_bytes(stream, true)?;
        let ip = read_ipv6_from_bytes(stream)?;
        let port = read_u16_from_bytes(stream, false)?;

        Ok(Ip {
            time,
            services,
            ip,
            port,
        })
    }

    /// Serializes the `Ip` instance into a byte vector.
    ///
    /// # Returns
    ///
    /// A byte vector representing the serialized `Ip` instance.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff: Vec<u8> = vec![];

        buff.extend(self.time.to_le_bytes());
        buff.extend(self.services.to_le_bytes());
        buff.extend(self.ip.octets()); //Tiene que ser Little Endian
        buff.extend(self.port.to_be_bytes());

        buff
    }
}

#[cfg(test)]
mod ip_test {
    use super::*;
    use chrono::Utc;
    use std::net::Ipv4Addr;

    #[test]
    fn test_new_ip_from_bytes() -> Result<(), MessageError> {
        let time = Utc::now().timestamp() as u32;
        let services = 0 as u64;
        let ip_address = Ipv4Addr::new(127, 0, 0, 1).to_ipv6_mapped();
        let port = 25000 as u16;

        let ip_env = Ip::new(time, services, ip_address, port); // start_string testnet
        let ip_env_bytes = ip_env.as_bytes();

        let mut stream = ip_env_bytes.as_slice();

        let ip_recv = Ip::from_bytes(&mut stream)?;

        assert_eq!(ip_env, ip_recv);

        Ok(())
    }

    #[test]
    fn test_ip_size() {
        let time = Utc::now().timestamp() as u32;
        let services = 0 as u64;
        let ip_address = Ipv4Addr::new(127, 0, 0, 1).to_ipv6_mapped();
        let port = 25000 as u16;

        let ip = Ip::new(time, services, ip_address, port); // start_string testnet
        let ip_bytes = ip.as_bytes();

        println!("Ip byte size: {:?}\n", ip_bytes);

        assert_eq!(ip_bytes.len(), 30);
    }
}
