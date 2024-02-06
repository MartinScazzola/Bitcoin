use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    block_mod::{blockchain::BlockChain, mempool::Mempool, utxo::UnspentTx},
    messages::{read_from_bytes::read_string_from_bytes, tx::Tx},
    network::broadcasting::broadcast_new_txn,
    proof_of_inclusion_mod::proof_of_inclusion::send_proof,
    settings_mod::settings::Settings,
    wallet_utils::{
        broadcast_txn::BroadcastTxn, get_proof::GetProof, get_transactions::GetTransactions,
        transactions::Transactions, tx_filter::get_wallet_txns,
    },
};

use super::{
    progress::Progress,
    update_wallet_error::UpdateWalletError,
    wallet_utils_constants::{
        BROADCAST_TX_COMMAND, EXIT_COMMAND, GET_PROGRESS_COMMAND, GET_PROOF_COMMAND, GET_TX_COMMAND,
    },
};

/// Updates the wallet by processing incoming commands from the connected `TcpStream`.
///
/// # Arguments
///
/// * `wallet` - The `TcpStream` representing the connection to the wallet.
/// * `blockchain` - A reference to the `BlockChain` wrapped in an `Arc<Mutex>`.
/// * `utxo` - A reference to the `UnspentTx` wrapped in an `Arc<Mutex>`.
/// * `mempool` - A reference to the `Mempool` wrapped in an `Arc<Mutex>`.
/// * `settings` - A reference to the `Settings` wrapped in an `Arc`.
/// * `streams` - A vector of `TcpStream` wrapped in `Arc<Mutex>`, representing connections to other nodes.
///
/// # Errors
///
/// Returns an `UpdateWalletError` if there is an error reading from or writing to the `TcpStream`,
/// parsing the incoming command, sending the proof, or broadcasting the transaction.
pub fn update_wallet(
    mut wallet: TcpStream,
    blockchain: &Arc<Mutex<BlockChain>>,
    utxo: &Arc<Mutex<UnspentTx>>,
    mempool: &Arc<Mutex<Mempool>>,
    settings: &Arc<Settings>,
    streams: &Vec<Arc<Mutex<TcpStream>>>,
    cant_total_blocks: usize,
) -> Result<(), UpdateWalletError> {
    loop {
        let command_name =
            read_string_from_bytes(&mut wallet, 12).map_err(|_| UpdateWalletError::Read)?;
        match command_name.as_str() {
            GET_TX_COMMAND => {
                let get_transactions =
                    GetTransactions::from_bytes(command_name.to_string(), &mut wallet)
                        .map_err(|_| UpdateWalletError::Read)?;
                let transactions: Transactions =
                    get_wallet_txns(blockchain, utxo, mempool, get_transactions)
                        .map_err(|_| UpdateWalletError::GetTxn)?;
                wallet
                    .write_all(&transactions.as_bytes())
                    .map_err(|_| UpdateWalletError::Write)?;
            }
            GET_PROOF_COMMAND => {
                let get_proof = GetProof::from_bytes(command_name.to_string(), &mut wallet)
                    .map_err(|_| UpdateWalletError::Read)?;
                send_proof(
                    get_proof.get_block_header(),
                    get_proof.get_tx_id(),
                    blockchain,
                    &mut wallet,
                )
                .map_err(|_| UpdateWalletError::SendProof)?;
            }
            BROADCAST_TX_COMMAND => {
                let broadcast_txn = BroadcastTxn::from_bytes(command_name.to_string(), &mut wallet)
                    .map_err(|_| UpdateWalletError::Read)?;
                let tx_msg = Tx::new(settings.get_start_string(), broadcast_txn.get_txn());

                broadcast_new_txn(tx_msg, streams).map_err(|_| UpdateWalletError::BroadcastTx)?;
            }
            GET_PROGRESS_COMMAND => {
                let locked_blockchain = blockchain
                    .lock()
                    .map_err(|_| UpdateWalletError::LockBlockchain)?;

                let act_blocks = locked_blockchain.get_cant_act_blocks() as u64;

                let progress = Progress::new(act_blocks, cant_total_blocks as u64);

                wallet
                    .write_all(&progress.as_bytes())
                    .map_err(|_| UpdateWalletError::Write)?;
            }
            EXIT_COMMAND => {
                return Ok(());
            }
            _ => {}
        }
    }
}
