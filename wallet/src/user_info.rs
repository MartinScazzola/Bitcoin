use node::{
    block_mod::tx_out::TxOut,
    wallet_utils::{transactions::Transactions, wallet_tx::WalletTx},
};

use crate::transactions::create_transactions::pk_script_from_public_key;

#[derive(Debug)]
/// Represents the information related to a user's wallet.
pub struct UserInfo {
    public_key: Vec<u8>,
    private_key: Vec<u8>,
    confirmed_txs_send: Vec<WalletTx>,
    confirmed_txs_recv: Vec<WalletTx>,
    unconfirmed_txs_send: Vec<WalletTx>,
    unconfirmed_txs_recv: Vec<WalletTx>,
    utxo: Vec<(Vec<u8>, u32, TxOut)>,
    last_update_time: u32,
}

impl UserInfo {
    pub fn new(public_key: Vec<u8>, private_key: Vec<u8>) -> UserInfo {
        UserInfo {
            public_key,
            private_key,
            utxo: vec![],
            confirmed_txs_send: vec![],
            confirmed_txs_recv: vec![],
            unconfirmed_txs_send: vec![],
            unconfirmed_txs_recv: vec![],
            last_update_time: 0,
        }
    }

    pub fn get_public_key(&self) -> Vec<u8> {
        self.public_key.clone()
    }

    pub fn get_private_key(&self) -> &Vec<u8> {
        &self.private_key
    }

    pub fn get_pk_script(&self) -> Vec<u8> {
        pk_script_from_public_key(&self.public_key)
    }

    pub fn get_confirmed_txs_send(&self) -> &Vec<WalletTx> {
        &self.confirmed_txs_send
    }

    pub fn get_confirmed_txs_recv(&self) -> &Vec<WalletTx> {
        &self.confirmed_txs_recv
    }

    pub fn get_unconfirmed_txs_send(&self) -> &Vec<WalletTx> {
        &self.unconfirmed_txs_send
    }

    pub fn get_unconfirmed_txs_recv(&self) -> &Vec<WalletTx> {
        &self.unconfirmed_txs_recv
    }

    pub fn get_last_update(&self) -> u32 {
        self.last_update_time
    }

    /// Returns the total available balance based on the UTXO (Unspent Transaction Output) of the `UserInfo` struct.
    ///
    /// This function calculates the sum of all the values of the UTXOs in the `UserInfo` struct, representing the total available balance.
    ///
    /// # Returns
    ///
    /// The total available balance as an `i64` value.
    pub fn get_avaiable(&self) -> i64 {
        let mut avaiable: i64 = 0;
        for tx in self.utxo.iter() {
            avaiable += tx.2.get_value();
        }
        avaiable
    }

    /// Filters the UTXO (Unspent Transaction Output) of the `UserInfo` struct based on the provided list of new outgoing transactions.
    ///
    /// This function removes the UTXOs that are spent by the given list of new outgoing transactions from the `UserInfo` struct's UTXO collection.
    ///
    /// # Arguments
    ///
    /// * `new_tx_send` - A reference to a vector of `WalletTx` representing the new outgoing transactions.
    fn filter_utxo(&mut self, new_tx_send: &[WalletTx]) {
        for tx in new_tx_send.iter() {
            for txin in tx.get_tx().get_tx_in_list() {
                let prev_id = txin.get_prev_output().get_tx_id();
                let prev_index = txin.get_prev_output().get_index();

                let mut aux = 0;
                for i in 0..self.utxo.len() {
                    if self.utxo[i - aux].0 == *prev_id && self.utxo[i - aux].1 == prev_index {
                        self.utxo.remove(i - aux);
                        aux += 1;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, txs: &Transactions) {
        let new_tx_send = txs.get_confirmed_txs_send();
        self.confirmed_txs_send.extend(new_tx_send.clone());
        self.confirmed_txs_recv.extend(txs.get_confirmed_txs_recv());
        self.unconfirmed_txs_send = txs.get_unconfirmed_txs_send();
        self.unconfirmed_txs_recv = txs.get_unconfirmed_txs_recv();
        self.utxo.extend(txs.get_utxo());
        self.filter_utxo(&new_tx_send);
        self.last_update_time = txs.get_last_update();
    }

    pub fn get_last_update_time(&self) -> u32 {
        self.last_update_time
    }

    pub fn get_utxo(&self) -> Vec<(Vec<u8>, u32, TxOut)> {
        self.utxo.clone()
    }
}
