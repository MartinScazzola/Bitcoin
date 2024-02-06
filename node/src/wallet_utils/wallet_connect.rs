use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::{
    block_mod::{blockchain::BlockChain, mempool::Mempool, utxo::UnspentTx},
    settings_mod::settings::Settings,
};

use super::update_wallet::update_wallet;

pub fn wallet_connect(
    listener: TcpListener,
    blockchain: Arc<Mutex<BlockChain>>,
    utxo: Arc<Mutex<UnspentTx>>,
    mempool: Arc<Mutex<Mempool>>,
    settings: Arc<Settings>,
    streams: &[Arc<Mutex<TcpStream>>],
    cant_total_blocks: usize,
) -> JoinHandle<()> {
    let shared_streams = streams.to_owned();
    thread::spawn(move || loop {
        let (wallet, _addr) = match listener.accept() {
            Ok(conection) => conection,
            Err(err) => {
                println!("{:?}", err);
                continue;
            }
        };

        if let Err(err) = update_wallet(
            wallet,
            &blockchain,
            &utxo,
            &mempool,
            &settings,
            &shared_streams,
            cant_total_blocks,
        ) {
            println!("{:?}", err);
        };
    })
}
