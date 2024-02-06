use std::{
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use node::{
    block_mod::{
        block::Block, block_header::BlockHeader, blockchain::BlockChain, mempool::Mempool,
        utxo::UnspentTx,
    },
    block_saver::wait_new_blocks,
    network::{
        block_download::{block_download, filter_headers, store_blocks_in_file},
        broadcasting::broadcasting,
        handshake::handshake,
        headers_download::headers_download,
        recv_peer_connection::recv_peer_connection,
    },
    settings_mod::{settings::Settings, settings_error::SettingError},
    wallet_utils::wallet_connect::wallet_connect,
};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("{:?}", SettingError::FileNotFound);
        return;
    }

    let settings = match Settings::from_file(&args[1]) {
        Ok(settings) => settings,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let listener: TcpListener = match TcpListener::bind(settings.get_wallet_connection_address()) {
        Ok(listener) => listener,
        Err(_) => {
            return;
        }
    };

    let mut streams: Vec<TcpStream> = match handshake(&settings) {
        Ok(streams) => streams,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let headers: Vec<BlockHeader> = match headers_download(&settings, &mut streams) {
        Ok(headers) => headers,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let filtred_headers = match filter_headers(&settings, &headers) {
        Ok(inv) => inv,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };

    let (block_sender, block_recv): (Sender<Block>, Receiver<Block>) = mpsc::channel();

    let blockchain: Arc<Mutex<BlockChain>> = Arc::new(Mutex::new(BlockChain::new()));
    let utxo: Arc<Mutex<UnspentTx>> = Arc::new(Mutex::new(UnspentTx::new()));
    let mempool: Arc<Mutex<Mempool>> = Arc::new(Mutex::new(Mempool::new()));

    let streams: Vec<Arc<Mutex<TcpStream>>> = streams
        .into_iter()
        .map(|streams| Arc::new(Mutex::new(streams)))
        .collect();

    let settings = Arc::new(settings);

    let handle_recv_block_download = wait_new_blocks(blockchain.clone(), utxo.clone(), block_recv);

    let handle_wallet_connect = wallet_connect(
        listener,
        blockchain.clone(),
        utxo.clone(),
        mempool.clone(),
        settings.clone(),
        &streams,
        filtred_headers.len(),
    );

    match block_download(settings.clone(), &streams, filtred_headers, block_sender) {
        Ok(blocks) => blocks,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };
    if handle_recv_block_download.join().is_err() {
        return;
    }

    if let Err(err) = store_blocks_in_file(settings.get_blocks_path(), &blockchain) {
        println!("{:?}", err);
        return;
    };

    let headers_hashmap: HashMap<Vec<u8>, BlockHeader> = headers
        .into_iter()
        .map(|block_header| (block_header.get_header(), block_header)) // Apply the desired transformation
        .collect();

    let headers = Arc::new(Mutex::new(headers_hashmap));
    let handle_broadcasting =
        broadcasting(&settings, streams, &blockchain, utxo, mempool, &headers);

    recv_peer_connection(&settings, &blockchain, &headers);

    if handle_wallet_connect.join().is_err() {
        println!("Join thread wallet connect error");
    }

    if handle_broadcasting.join().is_err() {
        println!("Join thread wallet connect error");
    }
}
