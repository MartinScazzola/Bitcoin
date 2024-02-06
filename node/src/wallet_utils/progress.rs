use std::io::Read;

use crate::messages::{message_error::MessageError, read_from_bytes::read_u64_from_bytes};

pub struct Progress {
    act_blocks: u64,
    total_blocks: u64,
}

impl Progress {
    pub fn new(act_blocks: u64, total_blocks: u64) -> Progress {
        Progress {
            act_blocks,
            total_blocks,
        }
    }

    pub fn from_bytes(stream: &mut dyn Read) -> Result<Progress, MessageError> {
        let act_blocks = read_u64_from_bytes(stream, true)?;
        let total_blocks = read_u64_from_bytes(stream, true)?;

        Ok(Progress {
            act_blocks,
            total_blocks,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.act_blocks.to_le_bytes().to_vec();
        buffer.extend(self.total_blocks.to_le_bytes());
        buffer
    }

    pub fn get_progress(&self) -> f64 {
        self.act_blocks as f64 / self.total_blocks as f64
    }
}
