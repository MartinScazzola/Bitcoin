use std::net::AddrParseError;
use std::num::ParseIntError;
use std::str::ParseBoolError;

#[derive(Debug)]
pub enum SettingError {
    TokenNotFound,
    FileNotFound,
    FieldNotFound,
}

impl From<std::io::Error> for SettingError {
    fn from(_: std::io::Error) -> SettingError {
        SettingError::FileNotFound
    }
}

impl From<ParseIntError> for SettingError {
    fn from(_: ParseIntError) -> SettingError {
        SettingError::FieldNotFound
    }
}

impl From<AddrParseError> for SettingError {
    fn from(_: AddrParseError) -> SettingError {
        SettingError::FieldNotFound
    }
}

impl From<ParseBoolError> for SettingError {
    fn from(_: ParseBoolError) -> SettingError {
        SettingError::FieldNotFound
    }
}
