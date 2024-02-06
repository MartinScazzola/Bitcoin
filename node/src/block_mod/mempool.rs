use std::collections::HashMap;

use super::{block::Block, transaction::Transaction};

/// Represents a mempool of unconfirmed transactions.
pub struct Mempool {
    txs: HashMap<Vec<u8>, Transaction>,
}

impl Mempool {
    /// Creates a new instance of the Mempool struct.
    pub fn new() -> Mempool {
        Mempool {
            txs: HashMap::new(),
        }
    }

    /// Adds a transaction to the mempool.
    ///
    /// # Arguments
    ///
    /// * `tx` - The transaction to be added to the mempool.
    pub fn add(&mut self, tx: Transaction) {
        self.txs.insert(tx.get_id(), tx);
    }

    /// Updates the mempool by removing transactions included in a given block.
    ///
    /// # Arguments
    ///
    /// * `block` - A reference to the block containing transactions to be removed from the mempool.
    pub fn update(&mut self, block: &Block) {
        for tx in block.get_txn_list() {
            self.txs.remove(&tx.get_id());
        }
    }

    /// Returns the number of transactions in the mempool.
    ///
    /// # Returns
    ///
    /// The number of transactions in the mempool.
    pub fn cant_txs(&self) -> usize {
        self.txs.len()
    }

    /// Returns a reference to the HashMap containing the transactions in the mempool.
    ///
    /// # Returns
    ///
    /// A reference to the HashMap containing the transactions in the mempool.
    pub fn get_txs(&self) -> &HashMap<Vec<u8>, Transaction> {
        &self.txs
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}
