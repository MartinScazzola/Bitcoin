use bitcoin_hashes::{sha256d, Hash};

use super::{
    compact_size::CompactSizeUInt,
    header::MessageHeader,
    message_constants::{FILTER_LOAD_COMMAND, HEADER_BYTES_SIZE},
};
#[derive(Debug)]
pub struct FilterLoad {
    header: MessageHeader,
    n_filter_bytes: CompactSizeUInt,
    filter: Vec<u8>,
    n_hash_funcs: u32,
    n_tweak: u32,
    n_flags: u8,
}

impl FilterLoad {
    pub fn new(
        start_string: Vec<u8>,
        n_filter_bytes: CompactSizeUInt,
        filter: Vec<u8>,
        n_hash_funcs: u32,
        n_tweak: u32,
        n_flags: u8,
    ) -> FilterLoad {
        let header = MessageHeader::new(start_string, FILTER_LOAD_COMMAND.to_string());

        let mut filterload = FilterLoad {
            header,
            n_filter_bytes,
            filter,
            n_hash_funcs,
            n_tweak,
            n_flags,
        };

        let stream: Vec<u8> = filterload.as_bytes();

        let payload_size = stream.len() - HEADER_BYTES_SIZE;

        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        filterload
            .header
            .update_payload(payload_size as u32, checksum);

        filterload
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_bytes();

        buffer.extend(self.n_filter_bytes.as_bytes());
        buffer.extend(&self.filter);
        buffer.extend(self.n_hash_funcs.to_le_bytes());
        buffer.extend(self.n_tweak.to_le_bytes());
        buffer.push(self.n_flags);

        buffer
    }
}
