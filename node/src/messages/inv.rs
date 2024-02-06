use std::io::Read;

use bitcoin_hashes::{sha256d, Hash};

use super::{
    compact_size::CompactSizeUInt,
    header::MessageHeader,
    message_constants::{HEADER_BYTES_SIZE, INV_COMMAND},
    message_error::MessageError,
};
use crate::messages::inventory::Inventory;

/// Represents an inventory message.
#[derive(Debug)]
pub struct Inv {
    header: MessageHeader,
    count: CompactSizeUInt,
    inventory_list: Vec<Inventory>,
    data_type: u32,
}

impl Inv {
    pub fn new(start_string: Vec<u8>, inventory_list: Vec<Inventory>, data_type: u32) -> Inv {
        let header = MessageHeader::new(start_string, INV_COMMAND.to_string());
        let count = CompactSizeUInt::from_number(inventory_list.len() as u64);

        let mut inv = Inv {
            header,
            count,
            inventory_list,
            data_type,
        };

        let stream: Vec<u8> = inv.as_bytes();

        let payload_size = stream.len() - HEADER_BYTES_SIZE;
        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        inv.header.update_payload(payload_size as u32, checksum);

        inv
    }

    /// Parses an inventory message from the provided byte stream.
    pub fn from_bytes(header: MessageHeader, stream: &mut dyn Read) -> Result<Inv, MessageError> {
        if header.get_command_name() != INV_COMMAND {
            return Err(MessageError::InvalidInputInv);
        }

        let count = CompactSizeUInt::from_bytes(stream)?;
        let mut inventory_list = Vec::new();

        for _ in 0..count.value() {
            inventory_list.push(Inventory::from_bytes(stream)?)
        }
        let data_type = inventory_list
            .last()
            .ok_or(MessageError::ReadFromBytes)?
            .get_type();

        Ok(Inv {
            header,
            count,
            inventory_list,
            data_type,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_bytes();
        buffer.extend(self.count.as_bytes());

        for inventory in self.inventory_list.iter() {
            buffer.extend(inventory.as_bytes());
        }
        buffer
    }

    /// Returns a clone of the list of inventory items.
    pub fn get_inventories(&self) -> Vec<Inventory> {
        self.inventory_list.clone()
    }

    /// Returns a clone of the list of inventory items.
    pub fn get_type(&self) -> u32 {
        self.data_type
    }
}
