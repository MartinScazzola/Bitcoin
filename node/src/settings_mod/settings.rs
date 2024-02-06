use crate::messages::read_from_bytes::decode_hex;
use crate::settings_mod::settings_constants::*;
use crate::settings_mod::settings_error::SettingError;
use std::collections::HashMap;
use std::fs;
use std::net::Ipv6Addr;
use std::str::FromStr;

/// Configuration settings for network communication.
#[derive(Debug)]
pub struct Settings {
    dns_seed: Option<String>,
    ips_to_connect: Option<Vec<Ipv6Addr>>,
    protocol_version: i32,
    services: u64,
    port: u16,
    ip: Ipv6Addr,
    user_agent: String,
    start_height: i32,
    relay: bool,
    start_string: Vec<u8>,
    date_limit: String,
    wallet_connection_addr: String,
    headers_path: String,
    server_addr: String,
    blocks_path: String,
}

impl Settings {
    /// Loads the settings from a file.
    ///
    /// This function reads the settings from a file located at the specified path and returns a
    /// `Result<Settings, SettingError>` representing the loaded settings if successful, or an error if
    /// there was a problem in reading or parsing the file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice representing the path to the settings file.
    ///
    /// # Returns
    ///
    /// - `Ok(settings)`: The loaded settings if successful.
    /// - `Err(err)`: If there was an error in reading or parsing the settings file.
    ///
    /// # Errors
    ///
    /// The function can return the following errors:
    ///
    /// - `SettingError::FileNotFound`: If the settings file was not found.
    /// - `SettingError::TokenNotFound`: If a required token is missing in the settings file.
    /// - `SettingError::FieldNotFound`: If a required field is missing in the settings file.
    /// - `SettingError::ParseError`: If there was an error in parsing a field value from the settings file.
    /// - `SettingError::DecodeError`: If there was an error in decoding a hex string from the settings file.
    pub fn from_file(path: &str) -> Result<Settings, SettingError> {
        let mut parser_config: HashMap<String, String> = HashMap::new();
        let file = fs::read_to_string(path)?;

        for line in file.lines() {
            let token: Vec<&str> = line.split(EQUAL).collect();

            if matches!(
                token[0],
                DNS_SEED
                    | PROCOCOL_VERSION
                    | SERVICES
                    | PORT
                    | IP
                    | USER_AGENT
                    | START_HEIGHT
                    | RELAY
                    | START_STRING
                    | IPS_TO_CONNECT
                    | DATE_LIMIT
                    | WALLET_CONNECTION_ADDR
                    | HEADERS_PATH
                    | SERVER_ADDR
                    | BLOCKS_PATH
            ) {
                parser_config.insert(token[0].to_string(), token[1].to_string());
            } else {
                return Err(SettingError::TokenNotFound);
            }
        }

        Ok(Settings {
            dns_seed: parser_config.get(DNS_SEED).cloned(),
            ips_to_connect: parser_config.get(IPS_TO_CONNECT).map(|ip_str| {
                ip_str
                    .split(',')
                    .filter_map(|ip| match Ipv6Addr::from_str(ip) {
                        Ok(ip) => Some(ip),
                        Err(_) => None,
                    })
                    .collect()
            }),
            protocol_version: i32::from_str(
                parser_config
                    .get(PROCOCOL_VERSION)
                    .ok_or(SettingError::FieldNotFound)?,
            )?,
            services: parser_config
                .get(SERVICES)
                .ok_or(SettingError::FieldNotFound)?
                .parse()?,
            port: parser_config
                .get(PORT)
                .ok_or(SettingError::FieldNotFound)?
                .parse()?,
            ip: Ipv6Addr::from_str(parser_config.get(IP).ok_or(SettingError::FieldNotFound)?)?,
            user_agent: parser_config
                .get(USER_AGENT)
                .ok_or(SettingError::FieldNotFound)?
                .to_string(),
            start_height: parser_config
                .get(START_HEIGHT)
                .ok_or(SettingError::FieldNotFound)?
                .parse()?,
            relay: parser_config
                .get(RELAY)
                .ok_or(SettingError::FieldNotFound)?
                .parse()?,
            start_string: decode_hex(
                parser_config
                    .get(START_STRING)
                    .ok_or(SettingError::FieldNotFound)?,
            )?,
            date_limit: parser_config
                .get(DATE_LIMIT)
                .ok_or(SettingError::FieldNotFound)?
                .to_string(),
            wallet_connection_addr: parser_config
                .get(WALLET_CONNECTION_ADDR)
                .ok_or(SettingError::FieldNotFound)?
                .to_string(),
            headers_path: parser_config
                .get(HEADERS_PATH)
                .ok_or(SettingError::FieldNotFound)?
                .to_string(),
            server_addr: parser_config
                .get(SERVER_ADDR)
                .ok_or(SettingError::FieldNotFound)?
                .to_string(),
            blocks_path: parser_config
                .get(BLOCKS_PATH)
                .ok_or(SettingError::FieldNotFound)?
                .to_string(),
        })
    }

    pub fn get_dns_seed(&self) -> &Option<String> {
        &self.dns_seed
    }
    pub fn get_ips_to_connect(&self) -> &Option<Vec<Ipv6Addr>> {
        &self.ips_to_connect
    }
    pub fn get_protocol_version(&self) -> i32 {
        self.protocol_version
    }
    pub fn get_services(&self) -> u64 {
        self.services
    }
    pub fn get_port(&self) -> u16 {
        self.port
    }
    pub fn get_ip(&self) -> Ipv6Addr {
        self.ip
    }
    pub fn get_user_agent(&self) -> String {
        self.user_agent.clone()
    }
    pub fn get_start_height(&self) -> i32 {
        self.start_height
    }
    pub fn get_relay(&self) -> bool {
        self.relay
    }
    pub fn get_start_string(&self) -> Vec<u8> {
        self.start_string.clone()
    }
    pub fn get_date_limit(&self) -> &str {
        &self.date_limit
    }
    pub fn get_wallet_connection_address(&self) -> &str {
        &self.wallet_connection_addr
    }
    pub fn get_headers_path(&self) -> &str {
        &self.headers_path
    }
    pub fn get_server_addr(&self) -> &str {
        &self.server_addr
    }
    pub fn get_blocks_path(&self) -> &str {
        &self.blocks_path
    }
}
