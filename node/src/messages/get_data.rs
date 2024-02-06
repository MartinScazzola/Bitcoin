use super::inventory::Inventory;
use super::message_constants::HEADER_BYTES_SIZE;
use super::message_error::MessageError;
use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::header::MessageHeader;
use crate::messages::message_constants::GET_DATA_COMMAND;
use bitcoin_hashes::sha256d;
use bitcoin_hashes::Hash;
use std::io::Read;

/// Represents a "getdata" message in the Bitcoin protocol.
/// It is used to request specific data (e.g., blocks or transactions) from a peer.
#[derive(Debug, PartialEq)]
pub struct GetData {
    header: MessageHeader,
    count: CompactSizeUInt,
    inventory_list: Vec<Inventory>,
}

impl GetData {
    /// Creates a new `GetData` message with the provided start string and inventory list.
    ///
    /// # Arguments
    ///
    /// * `start_string` - The start string of the message.
    /// * `inventory_list` - The list of inventory items.
    ///
    /// # Returns
    ///
    /// A new `GetData` message.
    pub fn new(start_string: Vec<u8>, inventory_list: Vec<Inventory>) -> GetData {
        let header = MessageHeader::new(start_string, GET_DATA_COMMAND.to_string());

        let mut get_data = GetData {
            header,
            count: CompactSizeUInt::from_number(inventory_list.len() as u64),
            inventory_list,
        };

        let stream: Vec<u8> = get_data.as_bytes();

        let payload_size = stream.len() - HEADER_BYTES_SIZE;

        let checksum =
            sha256d::Hash::hash(&stream[HEADER_BYTES_SIZE..]).to_byte_array()[..4].to_vec();

        get_data
            .header
            .update_payload(payload_size as u32, checksum);

        get_data
    }

    /// Creates a `GetData` message by decoding the byte stream.
    ///
    /// # Arguments
    ///
    /// * `header` - The message header.
    /// * `stream` - A mutable reference to a `dyn Read` trait object representing the byte stream to decode.
    ///
    /// # Returns
    ///
    /// A result containing the decoded `GetData` message if successful, or a `MessageError` if an error occurs during decoding.
    pub fn from_bytes(
        header: MessageHeader,
        stream: &mut dyn Read,
    ) -> Result<GetData, MessageError> {
        if header.get_command_name() != GET_DATA_COMMAND {
            return Err(MessageError::InvalidInputGetData);
        }

        let count = CompactSizeUInt::from_bytes(stream)?;
        let mut inventory_list = Vec::new();

        for _i in 0..count.value() {
            inventory_list.push(Inventory::from_bytes(stream)?);
        }
        Ok(GetData {
            header,
            count,
            inventory_list,
        })
    }

    /// Converts the `GetData` message to its byte representation.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the `GetData` message.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = self.header.as_bytes();

        buffer.extend(self.count.as_bytes());

        for inv in self.inventory_list.iter() {
            buffer.extend(inv.as_bytes())
        }

        buffer
    }

    pub fn get_inv_list(&self) -> &Vec<Inventory> {
        &self.inventory_list
    }
}

#[cfg(test)]
mod get_data_test {
    use super::*;
    #[test]
    fn test_new_get_data_from_bytes() -> Result<(), MessageError> {
        let start_string = vec![11u8, 17, 9, 7];

        let inv1 = Inventory::new(2, vec![1u8; 32]);
        let inv2 = Inventory::new(2, vec![3u8; 32]);

        let inventory_list = vec![inv1, inv2];

        let get_data_env = GetData::new(start_string, inventory_list);

        let get_data_env_bytes = get_data_env.as_bytes();

        let mut stream = get_data_env_bytes.as_slice();

        let header = MessageHeader::from_bytes(&mut stream)?;
        let get_data_recv = GetData::from_bytes(header, &mut stream)?;

        assert_eq!(get_data_env, get_data_recv);
        Ok(())
    }
}
