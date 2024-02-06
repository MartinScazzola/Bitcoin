use super::{
    message_error::MessageError,
    read_from_bytes::{read_u32_from_bytes, read_vec_from_bytes},
};
use std::io::Read;

/// Represents an inventory item in the Bitcoin protocol.
#[derive(Debug, PartialEq, Clone)]
pub struct Inventory {
    data_type: u32,
    hash: Vec<u8>,
}

impl Inventory {
    /// Creates a new `Inventory` item with the specified data type and hash.
    ///
    /// # Arguments
    ///
    /// * `data_type` - The data type of the inventory item.
    /// * `hash` - The hash associated with the inventory item.
    ///
    /// # Returns
    ///
    /// A new `Inventory` instance.
    pub fn new(data_type: u32, hash: Vec<u8>) -> Inventory {
        Inventory { data_type, hash }
    }

    /// Reads the inventory item from the byte stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - A mutable reference to the byte stream to read from.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed `Inventory` item or an error if the parsing failed.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Inventory, MessageError> {
        let data_type = read_u32_from_bytes(stream, true)?;
        let hash = read_vec_from_bytes(stream, 32)?;

        Ok(Inventory { data_type, hash })
    }

    /// Serializes the inventory item into a byte vector.
    ///
    /// # Returns
    ///
    /// A byte vector representing the serialized inventory item.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(self.data_type.to_le_bytes());
        buffer.extend(&self.hash);
        buffer
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.hash.clone()
    }
    pub fn get_type(&self) -> u32 {
        self.data_type
    }
}

#[cfg(test)]
mod inventory_test {
    use super::*;

    #[test]
    fn test_new_inventory_from_bytes() -> Result<(), MessageError> {
        let inv_env = Inventory::new(2, vec![12; 32]);

        let inv_env_bytes = inv_env.as_bytes();

        let mut stream = inv_env_bytes.as_slice();

        let inv_recv = Inventory::from_bytes(&mut stream)?;

        assert_eq!(inv_env, inv_recv);

        Ok(())
    }
}
