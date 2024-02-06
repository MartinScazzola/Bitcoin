use chrono::DateTime;
use chrono::Utc;

use super::headers_download::handle_other_message;

use crate::block_mod::blockchain::BlockChain;
use crate::network::network_constants::MSG_BLOCK_DATA_TYPE;
use crate::{
    block_mod::{block::Block, block_header::BlockHeader},
    messages::{
        get_data::GetData, header::MessageHeader, inventory::Inventory,
        message_constants::BLOCK_COMMAND,
    },
    network::{network_constants::DATE_FORMAT, network_error::NetworkError},
    settings_mod::settings::Settings,
};
use std::fs::OpenOptions;
use std::{
    io::Write,
    net::TcpStream,
    sync::mpsc::Sender,
    sync::{Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

pub fn filter_headers(
    settings: &Settings,
    headers: &[BlockHeader],
) -> Result<Vec<Inventory>, NetworkError> {
    let date_time = DateTime::parse_from_str(settings.get_date_limit(), DATE_FORMAT)
        .map_err(|_| NetworkError::BlockDownload)?
        .with_timezone(&Utc)
        .timestamp();

    let headers: Vec<&BlockHeader> = headers
        .iter()
        .filter(|block_header| block_header.get_time() > date_time as u32)
        .collect();

    let inventories: Vec<Inventory> = headers
        .iter()
        .map(|block_header| Inventory::new(MSG_BLOCK_DATA_TYPE, block_header.get_header()))
        .collect();

    Ok(inventories)
}

fn manage_error(shared_inv: &Arc<Mutex<Vec<Inventory>>>, inv_thread: Vec<Inventory>) {
    if let Ok(mut guard) = shared_inv.lock() {
        guard.extend(inv_thread);
    }
}

/// Downloads blocks from a network using multiple TCP streams and filters them based on date and merkle tree validation.
///
/// # Arguments
/// * `settings` - A reference to the network settings.
/// * `streams` - A mutable vector of TCP streams used for communication.
/// * `headers` - A vector of block headers to download.
/// * `utxo_set` - A mutable reference to the unspent transaction output set.
///
/// # Returns
/// A Result containing a HashMap of block headers to their corresponding blocks if successful,
/// otherwise a NetworkError indicating the reason for failure.
///
/// # Errors
/// This function can return a NetworkError if there are no available streams for block download,
/// if there's an error parsing the date, or if there are issues with network communication.
pub fn block_download(
    settings: Arc<Settings>,
    streams: &Vec<Arc<Mutex<TcpStream>>>,
    inventories: Vec<Inventory>,
    tx: Sender<Block>,
) -> Result<(), NetworkError> {
    println!(
        "Se inicia la descarga de bloques, hay que descargar {}\n",
        inventories.len()
    );

    let inventories = Arc::new(Mutex::new(inventories));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    for stream in streams {
        let shared_stream = stream.clone();
        let shared_settings = settings.clone();
        let shared_tx = tx.clone();
        let shared_inv = inventories.clone();

        let handle = thread::spawn(move || {
            let mut locked_stream = match shared_stream.lock() {
                Ok(locked_stream) => locked_stream,
                Err(_) => return,
            };

            loop {
                if let Ok(mut guard) = shared_inv.lock() {
                    if guard.is_empty() {
                        break;
                    };
                    let inv_thread: Vec<Inventory> = take_n(&mut guard, 50);
                    drop(guard);

                    let cant_inv = inv_thread.len();

                    let get_data =
                        GetData::new(shared_settings.get_start_string(), inv_thread.clone());

                    if locked_stream.write_all(&get_data.as_bytes()).is_err() {
                        return manage_error(&shared_inv, inv_thread);
                    }

                    for _i in 0..cant_inv {
                        let mut header = match MessageHeader::from_bytes(&mut *locked_stream) {
                            Ok(header) => header,
                            Err(_) => {
                                return manage_error(&shared_inv, inv_thread);
                            }
                        };

                        while header.get_command_name() != BLOCK_COMMAND {
                            if handle_other_message(
                                &mut locked_stream,
                                header,
                                shared_settings.get_start_string(),
                            )
                            .is_err()
                            {
                                return manage_error(&shared_inv, inv_thread);
                            }

                            header = match MessageHeader::from_bytes(&mut *locked_stream) {
                                Ok(header) => header,
                                Err(_) => {
                                    return manage_error(&shared_inv, inv_thread);
                                }
                            }
                        }

                        let block = match Block::from_bytes(&mut *locked_stream) {
                            Ok(block) => block,
                            Err(_) => {
                                return manage_error(&shared_inv, inv_thread);
                            }
                        };

                        if block.proof_of_inclusion() && shared_tx.send(block).is_err() {
                            println!("Send error");
                        };
                    }
                };
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().map_err(|_| NetworkError::BlockDownload)?;
    }

    Ok(())
}

pub fn take_n(inventories: &mut MutexGuard<Vec<Inventory>>, n: usize) -> Vec<Inventory> {
    let mut new_list = Vec::new();
    for _i in 0..n {
        match inventories.pop() {
            Some(inv) => new_list.push(inv),
            None => {
                break;
            }
        };
    }
    new_list
}

pub fn take_n_streams(
    streams: &mut Vec<Arc<Mutex<TcpStream>>>,
    n: usize,
) -> Vec<Arc<Mutex<TcpStream>>> {
    let mut new_list = Vec::new();
    for _i in 0..n {
        match streams.pop() {
            Some(stream) => new_list.push(stream),
            None => {
                break;
            }
        };
    }
    new_list
}

pub fn store_blocks_in_file(
    path: &str,
    blockchain: &Arc<Mutex<BlockChain>>,
) -> Result<(), NetworkError> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .map_err(|_| NetworkError::BlockDownload)?;

    let mut blocks = vec![];

    let locked_blockchain = blockchain.lock().map_err(|_| NetworkError::BlockDownload)?;

    let mut act_block_header = &locked_blockchain.get_last_block_header();
    while let Some(block) = locked_blockchain.get_block(act_block_header) {
        blocks.push(block);
        act_block_header = block.get_header().get_previuos_block_header();
    }

    blocks.reverse();
    
    for block in blocks {
        file.write(&block.as_bytes())
        .map_err(|_| NetworkError::BlockDownload)?;
    }
    
    Ok(())
}
