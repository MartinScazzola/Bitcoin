use std::{
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::
    block_mod::{block::Block, blockchain::BlockChain, utxo::UnspentTx};

pub fn wait_new_blocks(
    blockchain: Arc<Mutex<BlockChain>>,
    utxo: Arc<Mutex<UnspentTx>>,
    rx: Receiver<Block>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        while let Ok(block) = rx.recv() {
            let mut locked_blockchain = match blockchain.lock() {
                Ok(locked_blockchain) => locked_blockchain,
                Err(_) => {
                    println!("Lock blockchain error");
                    return;
                }
            };

            let mut locked_utxo = match utxo.lock() {
                Ok(locked_utxo) => locked_utxo,
                Err(_) => {
                    println!("Lock utxo error");
                    return;
                }
            };

            locked_utxo.update(&block);
            locked_blockchain.add(block);
            println!(
                "Cantidad actual de bloques: {}",
                locked_blockchain.get_cant_act_blocks()
            );
            drop(locked_blockchain);
            drop(locked_utxo);
        }
    })
}
