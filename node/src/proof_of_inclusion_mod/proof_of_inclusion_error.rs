use std::num::ParseIntError;

use crate::messages::message_error::MessageError;

#[derive(Debug)]
pub enum ProofOfInclusionError {
    ParseMessage,
    MurMurHash,
    ParseIntError,
    HandleOtherMessageError,
    ReadMessageHeaderError,
    SendProofError,
    BlockNotFound,
    TransactionNotFound,
    LockBlockChain,
    WriteError,
}

impl From<MessageError> for ProofOfInclusionError {
    fn from(_: MessageError) -> ProofOfInclusionError {
        ProofOfInclusionError::ParseMessage
    }
}

impl From<std::io::Error> for ProofOfInclusionError {
    fn from(_: std::io::Error) -> Self {
        // Create an instance of ProofOfInclusionError based on the io::Error
        // You can extract relevant information from the io::Error or provide a custom error message
        ProofOfInclusionError::MurMurHash
    }
}

impl From<ParseIntError> for ProofOfInclusionError {
    fn from(_: ParseIntError) -> Self {
        ProofOfInclusionError::ParseIntError
    }
}
