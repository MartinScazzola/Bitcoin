use super::super::messages::message_error::MessageError;
use std::{
    net::TcpStream,
    string::FromUtf8Error,
    sync::{MutexGuard, PoisonError},
};

#[derive(Debug)]
pub enum NetworkError {
    HandShake,
    HeaderDownload,
    BlockDownload,
    Broadcasting,
    PeerConnection,
}

impl From<std::io::Error> for NetworkError {
    fn from(_: std::io::Error) -> NetworkError {
        NetworkError::HandShake
    }
}

impl From<FromUtf8Error> for NetworkError {
    fn from(_: FromUtf8Error) -> NetworkError {
        NetworkError::HandShake
    }
}

impl From<MessageError> for NetworkError {
    fn from(_: MessageError) -> NetworkError {
        NetworkError::HandShake
    }
}

impl From<PoisonError<MutexGuard<'_, TcpStream>>> for NetworkError {
    fn from(_: PoisonError<MutexGuard<'_, TcpStream>>) -> NetworkError {
        NetworkError::Broadcasting
    }
}
