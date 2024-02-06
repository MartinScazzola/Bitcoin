use super::{tx_filter_error::TxFilterError, wallet_utils_constants::DATE_FORMAT};
use crate::{
    block_mod::{blockchain::BlockChain, mempool::Mempool, tx_out::TxOut, utxo::UnspentTx},
    wallet_utils::{
        get_transactions::GetTransactions, transactions::Transactions, wallet_tx::WalletTx,
    },
};
use chrono::{Local, NaiveDateTime};
use std::sync::{Arc, Mutex};

/// Filters confirmed transactions from the blockchain based on the provided criteria.
///
/// # Arguments
///
/// * `blockchain` - An `Arc<Mutex<BlockChain>>` representing the blockchain to filter transactions from.
/// * `pk_script` - A reference to a vector of bytes representing the public key script to filter transactions.
/// * `public_key` - A vector of bytes representing the public key.
/// * `last_update` - The last update time to use as a filter.
///
/// # Returns
///
/// A tuple containing two vectors of `WalletTx`: `confirmed_txs_send` and `confirmed_txs_recv`.
/// `confirmed_txs_send` contains filtered transactions where the provided public key matches the signature script.
/// `confirmed_txs_recv` contains filtered transactions where the provided public key script matches any of the transaction outputs.
pub fn filter_confirmed_transactions(
    blockchain: &Arc<Mutex<BlockChain>>,
    pk_script: &Vec<u8>,
    public_key: Vec<u8>,
    last_update: u32,
) -> Result<(Vec<WalletTx>, Vec<WalletTx>), TxFilterError> {
    let mut confirmed_txs_send: Vec<WalletTx> = vec![];
    let mut confirmed_txs_recv: Vec<WalletTx> = vec![];
    let locked_blockchain = blockchain
        .lock()
        .map_err(|_| TxFilterError::LockBlockchain)?;
    let mut last_block_header = &locked_blockchain.get_last_block_header();

    while let Some(block) = locked_blockchain.get_block(last_block_header) {
        let datetime = NaiveDateTime::from_timestamp_opt(block.get_header().get_time() as i64, 0)
            .ok_or(TxFilterError::DateTimeError)?;
        let date = datetime.format(DATE_FORMAT).to_string();

        if block.get_header().get_time() <= last_update {
            break;
        }

        for transaction in block.get_txn_list() {
            if let Some(first_txin) = transaction.get_tx_in_list().get(0) {
                let signature = first_txin.get_signature_script();

                if signature.len() >= 33 && signature[(signature.len() - 33)..] == public_key {
                    confirmed_txs_send.push(WalletTx::new(transaction.clone(), date.clone()));
                    continue;
                }
            }

            let tx_outs: Vec<Vec<u8>> = transaction
                .get_tx_out_list()
                .iter()
                .map(|tx_out| tx_out.get_pk_script())
                .collect();
            if tx_outs.contains(pk_script) {
                confirmed_txs_recv.push(WalletTx::new(transaction.clone(), date.clone()));
            }
        }

        last_block_header = block.get_previuos_block_header();
    }
    drop(locked_blockchain);
    Ok((confirmed_txs_send, confirmed_txs_recv))
}

/// Filters unconfirmed transactions from the mempool based on the provided criteria.
///
/// # Arguments
///
/// * `mempool` - An `Arc<Mutex<Mempool>>` representing the mempool to filter transactions from.
/// * `pk_script` - A reference to a vector of bytes representing the public key script to filter transactions.
/// * `public_key` - A vector of bytes representing the public key.
///
/// # Returns
///
/// A tuple containing two vectors of `WalletTx`: `unconfirmed_txs_send` and `unconfirmed_txs_recv`.
/// `unconfirmed_txs_send` contains filtered transactions from the mempool where the provided public key matches the signature script.
/// `unconfirmed_txs_recv` contains filtered transactions from the mempool where the provided public key script matches any of the transaction outputs.
pub fn filter_unconfirmed_transactions(
    mempool: &Arc<Mutex<Mempool>>,
    pk_script: &Vec<u8>,
    public_key: Vec<u8>,
) -> Result<(Vec<WalletTx>, Vec<WalletTx>), TxFilterError> {
    let mut unconfirmed_txs_send: Vec<WalletTx> = vec![];
    let mut unconfirmed_txs_recv: Vec<WalletTx> = vec![];
    let date = Local::now().naive_local().format(DATE_FORMAT).to_string();

    let locked_mempool = mempool.lock().map_err(|_| TxFilterError::LockMempool)?;

    for transaction in locked_mempool.get_txs().iter() {
        if let Some(first_txin) = transaction.1.get_tx_in_list().get(0) {
            let signature = first_txin.get_signature_script();
            if signature.len() >= 33 && signature[(signature.len() - 33)..] == public_key {
                unconfirmed_txs_send.push(WalletTx::new(transaction.1.clone(), date.clone()));
                continue;
            }
        }
        let txouts: Vec<Vec<u8>> = transaction
            .1
            .get_tx_out_list()
            .iter()
            .map(|txout| txout.get_pk_script())
            .collect();
        if txouts.contains(pk_script) {
            unconfirmed_txs_recv.push(WalletTx::new(transaction.1.clone(), date.clone()));
        }
    }

    drop(locked_mempool);

    Ok((unconfirmed_txs_send, unconfirmed_txs_recv))
}

/// Filters unspent transaction outputs (UTXOs) from the provided UTxO hash map based on the given criteria.
///
/// # Arguments
///
/// * `utxo_hash` - An `Arc<Mutex<UnspentTx>>` representing the UTxO hash map to filter UTXOs from.
/// * `confirmed_txs` - A reference to a vector of `WalletTx` representing the confirmed transactions to filter UTXOs for.
/// * `pk_script` - A reference to a vector of bytes representing the public key script to filter UTXOs.
///
/// # Returns
///
/// A vector of tuples `(Vec<u8>, u32, TxOut>)` representing the filtered UTXOs. Each tuple contains the transaction ID, output index, and the corresponding `TxOut`.
pub fn filter_utxo(
    utxo_hash: &Arc<Mutex<UnspentTx>>,
    confirmed_txs: &[WalletTx],
    pk_script: &Vec<u8>,
) -> Result<Vec<(Vec<u8>, u32, TxOut)>, TxFilterError> {
    let mut utxo_txs: Vec<(Vec<u8>, u32, TxOut)> = vec![];
    let locked_utxo_hash = utxo_hash.lock().map_err(|_| TxFilterError::LockUtxo)?;

    for transaction in confirmed_txs.iter() {
        if let Some(outputs) = locked_utxo_hash
            .get_utxo()
            .get(&transaction.get_tx().get_id())
        {
            let current_txouts = outputs.iter().filter_map(|(&index, tx_out)| {
                if tx_out.get_pk_script() == *pk_script {
                    Some((transaction.get_tx().get_id(), index, tx_out.clone()))
                } else {
                    None
                }
            });
            utxo_txs.extend(current_txouts);
        }
    }
    drop(locked_utxo_hash);

    Ok(utxo_txs)
}

/// Retrieves wallet transactions based on the specified criteria.
///
/// # Arguments
///
/// * `blockchain` - An `Arc<Mutex<BlockChain>>` representing the blockchain.
/// * `utxo_hash` - An `Arc<Mutex<UnspentTx>>` representing the UTXO hash map.
/// * `mempool` - An `Arc<Mutex<Mempool>>` representing the mempool.
/// * `get_transactions` - A `GetTransactions` object specifying the criteria for retrieving wallet transactions.
///
/// # Returns
///
/// A `Transactions` object containing the wallet transactions that match the specified criteria.
pub fn get_wallet_txns(
    blockchain: &Arc<Mutex<BlockChain>>,
    utxo_hash: &Arc<Mutex<UnspentTx>>,
    mempool: &Arc<Mutex<Mempool>>,
    get_transactions: GetTransactions,
) -> Result<Transactions, TxFilterError> {
    let pk_script = get_transactions.get_pk_script();
    let public_key = get_transactions.get_public_key();
    let mut last_update = get_transactions.get_last_update();

    let (confirmed_txs_send, confirmed_txs_recv): (Vec<WalletTx>, Vec<WalletTx>) =
        filter_confirmed_transactions(blockchain, pk_script, public_key.clone(), last_update)?;
    let (unconfirmed_txs_send, unconfirmed_txs_recv): (Vec<WalletTx>, Vec<WalletTx>) =
        filter_unconfirmed_transactions(mempool, pk_script, public_key.clone())?;

    let utxo_txs: Vec<(Vec<u8>, u32, TxOut)> = filter_utxo(
        utxo_hash,
        &vec![confirmed_txs_send.clone(), confirmed_txs_recv.clone()].concat(),
        pk_script,
    )?;

    let locked_blockchain = blockchain
        .lock()
        .map_err(|_| TxFilterError::LockBlockchain)?;
    let last_block = locked_blockchain
        .get_block(&locked_blockchain.get_last_block_header())
        .ok_or(TxFilterError::UnfoundBlock)?;
    last_update = last_block.get_header().get_time();

    Ok(Transactions::new(
        confirmed_txs_send,
        confirmed_txs_recv,
        unconfirmed_txs_send,
        unconfirmed_txs_recv,
        utxo_txs,
        last_update,
    ))
}
