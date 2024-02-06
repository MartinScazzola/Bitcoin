use crate::block_mod::tx_in::TxIn;
use crate::block_mod::tx_out::TxOut;
use crate::messages::compact_size::CompactSizeUInt;
use crate::messages::message_error::MessageError;
use crate::messages::read_from_bytes::{read_i32_from_bytes, read_u32_from_bytes};
use bitcoin_hashes::Hash;
use bitcoin_hashes::{sha256, sha256d};
use std::io::Read;

/// Represents a Transaction in the Bitcoin protocol.
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    version: i32,
    tx_in_count: CompactSizeUInt,
    tx_in_list: Vec<TxIn>,
    tx_out_count: CompactSizeUInt,
    tx_out_list: Vec<TxOut>,
    lock_time: u32,
}

impl Transaction {
    pub fn new(
        version: i32,
        tx_in_list: Vec<TxIn>,
        tx_out_list: Vec<TxOut>,
        lock_time: u32,
    ) -> Transaction {
        Transaction {
            version,
            tx_in_count: CompactSizeUInt::from_number(tx_in_list.len() as u64),
            tx_in_list,
            tx_out_count: CompactSizeUInt::from_number(tx_out_list.len() as u64),
            tx_out_list,
            lock_time,
        }
    }
    /// Creates a new `Transaction` instance from the provided byte stream.
    ///
    /// # Arguments
    /// * `stream` - A mutable reference to the byte stream.
    ///
    /// # Returns
    /// A `Result` containing the parsed `Transaction` instance or a `MessageError` if parsing fails.
    pub fn from_bytes(stream: &mut dyn Read) -> Result<Transaction, MessageError> {
        let version = read_i32_from_bytes(stream, true)?;
        let tx_in_count = CompactSizeUInt::from_bytes(stream)?;
        let mut tx_in_list: Vec<TxIn> = Vec::new();

        for _i in 0..tx_in_count.value() {
            tx_in_list.push(TxIn::from_bytes(stream)?);
        }
        let tx_out_count = CompactSizeUInt::from_bytes(stream)?;
        let mut tx_out_list: Vec<TxOut> = Vec::new();

        for _i in 0..tx_out_count.value() {
            tx_out_list.push(TxOut::from_bytes(stream)?);
        }
        let lock_time = read_u32_from_bytes(stream, true)?;

        Ok(Transaction {
            version,
            tx_in_count,
            tx_in_list,
            tx_out_count,
            tx_out_list,
            lock_time,
        })
    }

    /// Converts the `Transaction` instance to a byte representation.
    ///
    /// # Returns
    /// A vector of bytes representing the `Transaction` instance.
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();

        buff.extend(self.version.to_le_bytes());
        buff.extend(self.tx_in_count.as_bytes());

        for txin in self.tx_in_list.iter() {
            //for each?
            buff.extend(&txin.as_bytes());
        }
        buff.extend(self.tx_out_count.as_bytes());

        for txout in self.tx_out_list.iter() {
            buff.extend(&txout.as_bytes());
        }
        buff.extend(self.lock_time.to_le_bytes());

        buff
    }

    ///Calculates the transaction ID by hashing the serialized bytes of the `BlockHeader`.
    ///
    /// # Returns
    /// A vector of bytes representing the transaction.
    pub fn get_id(&self) -> Vec<u8> {
        sha256d::Hash::hash(&self.as_bytes())
            .to_byte_array()
            .to_vec()
    }

    /// Returns a reference to the list of transaction inputs.
    pub fn get_tx_in_list(&self) -> &Vec<TxIn> {
        &self.tx_in_list
    }

    /// Returns a reference to the list of transaction outputs.
    pub fn get_tx_out_list(&self) -> &Vec<TxOut> {
        &self.tx_out_list
    }

    /// Computes the signature hash for the transaction at the given input index with the provided public key script.
    /// The signature hash is used for generating a digital signature that verifies the integrity of the transaction.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the transaction input for which to compute the signature hash.
    /// * `pk_script` - The public key script associated with the transaction input.
    ///
    /// # Returns
    ///
    /// A vector of bytes representing the computed signature hash.
    pub fn sig_hash(&self, index: usize, pk_script: &[u8]) -> Vec<u8> {
        let mut buffer = self.version.to_le_bytes().to_vec();
        buffer.extend(self.tx_in_count.as_bytes());

        for (i, txin) in self.tx_in_list.iter().enumerate() {
            if i == index {
                let aux_txin = TxIn::new(
                    txin.get_prev_output().get_tx_id().clone(),
                    txin.get_prev_output().get_index(),
                    pk_script.to_vec(),
                    txin.get_sequence(),
                );
                buffer.extend(&aux_txin.as_bytes());
            } else {
                let aux_txin = TxIn::new(
                    txin.get_prev_output().get_tx_id().clone(),
                    txin.get_prev_output().get_index(),
                    vec![],
                    txin.get_sequence(),
                );
                buffer.extend(&aux_txin.as_bytes());
            }
        }
        buffer.extend(self.tx_out_count.as_bytes());

        for txout in self.tx_out_list.iter() {
            buffer.extend(txout.as_bytes());
        }

        buffer.extend(self.lock_time.to_le_bytes());
        buffer.extend((1_u32).to_le_bytes());

        sha256::Hash::hash(&buffer).as_byte_array().to_vec()
    }

    /// Sets the signature for the transaction input at the given index with the provided signature script.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the transaction input for which to set the signature.
    /// * `signature_script` - The signature script to set for the transaction input.
    pub fn set_signature(&mut self, index: usize, signature_script: Vec<u8>) {
        self.tx_in_list[index].set_signature(signature_script);
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "      Version: {}", self.version)?;
        writeln!(f, "      Txin")?;

        for (i, txin) in self.tx_in_list.iter().enumerate() {
            writeln!(f, "          TxIn: {}", i)?;
            writeln!(f, "          {:?}", txin)?;
        }

        writeln!(f, "      TxOut")?;

        for (i, txout) in self.tx_out_list.iter().enumerate() {
            writeln!(f, "          TxOut: {}", i)?;
            writeln!(f, "          {:?}", txout)?;
        }

        writeln!(f, "LockTime: {}", self.lock_time)?;
        Ok(())
    }
}
