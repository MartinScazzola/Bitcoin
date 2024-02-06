use std::num::ParseIntError;

#[derive(Debug)]
pub enum TransactionCreateError {
    InsufficientFounds,
    PrivateKey,
    Decode58,
    DecodeHex,
    EncodeHex,
    GetPrivateKey,
}

impl From<ParseIntError> for TransactionCreateError {
    fn from(_: ParseIntError) -> TransactionCreateError {
        TransactionCreateError::DecodeHex
    }
}

impl From<std::fmt::Error> for TransactionCreateError {
    fn from(_: std::fmt::Error) -> TransactionCreateError {
        TransactionCreateError::EncodeHex
    }
}

impl From<secp256k1::Error> for TransactionCreateError {
    fn from(_: secp256k1::Error) -> TransactionCreateError {
        TransactionCreateError::GetPrivateKey
    }
}
