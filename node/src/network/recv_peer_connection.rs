use super::{handshake::is_version_compatible, network_error::NetworkError};
use crate::{
    block_mod::{block_header::BlockHeader, blockchain::BlockChain},
    messages::{
        block::BlockMsg,
        get_data::GetData,
        get_headers::GetHeaders,
        header::MessageHeader,
        headers::Headers,
        message_constants::{
            GET_DATA_COMMAND, GET_HEADERS_COMMAND, NOT_FOUND_COMMAND, VERACK_COMMAND,
            VERSION_COMMAND,
        },
        version::Version,
    },
    settings_mod::settings::Settings,
};
use std::collections::HashMap;
use std::net::Ipv6Addr;
use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

fn handle_other_message(
    stream: &mut TcpStream,
    header: &MessageHeader,
) -> Result<(), NetworkError> {
    stream
        .read_exact(&mut vec![0u8; header.get_payload_size() as usize])
        .map_err(|_| NetworkError::PeerConnection)?;

    Ok(())
}

fn handle_get_header_message(
    start_string: Vec<u8>,
    block_headers: &Arc<Mutex<HashMap<Vec<u8>, BlockHeader>>>,
    get_headers: GetHeaders,
    client_node: &mut TcpStream,
) -> Result<(), NetworkError> {
    let locked_headers = block_headers
        .lock()
        .map_err(|_| NetworkError::PeerConnection)?;
    let mut headers: Vec<BlockHeader> = Vec::new();
    let mut i = 0;
    let mut actual_block: Option<&BlockHeader> =
        locked_headers.get(get_headers.get_last_block_header());
    let mut next_block_header = actual_block
        .ok_or(NetworkError::PeerConnection)?
        .get_next_block_header();

    while i < 2000 && next_block_header.is_some() {
        let next_block = locked_headers
            .get(
                next_block_header
                    .as_ref()
                    .ok_or(NetworkError::PeerConnection)?,
            )
            .ok_or(NetworkError::PeerConnection)?
            .clone();
        headers.push(next_block);
        actual_block = locked_headers.get(
            next_block_header
                .as_ref()
                .ok_or(NetworkError::PeerConnection)?,
        );
        next_block_header = actual_block
            .ok_or(NetworkError::PeerConnection)?
            .get_next_block_header();
        i += 1;
    }

    let headers_message = Headers::new(start_string, headers.clone());

    client_node
        .write_all(&headers_message.as_bytes())
        .map_err(|_| NetworkError::PeerConnection)?;

    Ok(())
}

fn handle_get_data_message(
    start_string: Vec<u8>,
    blockchain: &Arc<Mutex<BlockChain>>,
    get_data: GetData,
    client_node: &mut TcpStream,
) -> Result<(), NetworkError> {
    let locked_blockchain = blockchain
        .lock()
        .map_err(|_| NetworkError::PeerConnection)?;
    for inv in get_data.get_inv_list() {
        if let Some(block) = locked_blockchain.get_block(&inv.get_data()) {
            let block_message = BlockMsg::new(start_string.clone(), block.clone());
            client_node
                .write_all(&block_message.as_bytes())
                .map_err(|_| NetworkError::PeerConnection)?;
        } else {
            client_node
                .write_all(
                    &MessageHeader::new(start_string.clone(), NOT_FOUND_COMMAND.to_string())
                        .as_bytes(),
                )
                .map_err(|_| NetworkError::PeerConnection)?;
        };
    }
    Ok(())
}

fn handle_messages(
    peer: &mut TcpStream,
    settings: &Arc<Settings>,
    blockchain: &Arc<Mutex<BlockChain>>,
    headers: &Arc<Mutex<HashMap<Vec<u8>, BlockHeader>>>,
) -> Result<(), NetworkError> {
    let mut header = MessageHeader::from_bytes(peer).map_err(|_| NetworkError::PeerConnection)?;

    loop {
        let command_name: &str = header.get_command_name();
        match command_name {
            GET_HEADERS_COMMAND => {
                let get_headers = GetHeaders::from_bytes(header, peer)
                    .map_err(|_| NetworkError::PeerConnection)?;
                handle_get_header_message(settings.get_start_string(), headers, get_headers, peer)?;
            }
            GET_DATA_COMMAND => {
                let get_data =
                    GetData::from_bytes(header, peer).map_err(|_| NetworkError::PeerConnection)?;
                handle_get_data_message(settings.get_start_string(), blockchain, get_data, peer)?;
            }
            _ => {
                handle_other_message(peer, &header).map_err(|_| NetworkError::PeerConnection)?;
            }
        }
        header = MessageHeader::from_bytes(peer).map_err(|_| NetworkError::PeerConnection)?;
    }
}

pub fn recv_peer_connection(
    settings: &Arc<Settings>,
    blockchain: &Arc<Mutex<BlockChain>>,
    headers: &Arc<Mutex<HashMap<Vec<u8>, BlockHeader>>>,
) {
    let listener: TcpListener = match TcpListener::bind(settings.get_server_addr()) {
        Ok(listener) => listener,
        Err(_) => {
            println!("Node conection error bind ");
            return;
        }
    };

    while let Ok((mut peer, addr)) = listener.accept() {
        let shared_settings = settings.clone();
        let shared_blockchain = blockchain.clone();
        let shared_headers = headers.clone();

        thread::spawn(move || {
            let ip = match addr.ip() {
                IpAddr::V6(ipv6) => ipv6,
                IpAddr::V4(ipv4) => ipv4.to_ipv6_compatible(),
            };

            if let Err(err) = recv_handshake(&mut peer, &shared_settings, ip) {
                println!("{:?}", err);
            }

            if let Err(err) = handle_messages(
                &mut peer,
                &shared_settings,
                &shared_blockchain,
                &shared_headers,
            ) {
                println!("{:?}", err);
            };
        });
    }
}

fn recv_handshake(
    peer: &mut TcpStream,
    shared_settings: &Arc<Settings>,
    ip: Ipv6Addr,
) -> Result<(), NetworkError> {
    let mut header = match MessageHeader::from_bytes(peer) {
        Ok(header) => header,
        Err(_) => {
            return Err(NetworkError::PeerConnection);
        }
    };

    while header.get_command_name() != VERSION_COMMAND {
        //habría que ponerle un límite de tiempo para que corte
        if handle_other_message(peer, &header).is_err() {
            return Err(NetworkError::PeerConnection);
        }
        header = match MessageHeader::from_bytes(peer) {
            Ok(header) => header,
            Err(_) => {
                return Err(NetworkError::PeerConnection);
            }
        };
    }

    let peer_version = match Version::from_bytes(header, peer) {
        Ok(version) => version,
        Err(_) => {
            return Err(NetworkError::HandShake);
        }
    };

    if !is_version_compatible(&peer_version) {
        return Err(NetworkError::HandShake);
    }

    if peer
        .write_all(&Version::new(ip, shared_settings).as_bytes())
        .is_err()
    {
        return Err(NetworkError::HandShake);
    };

    header = match MessageHeader::from_bytes(peer) {
        Ok(header) => header,
        Err(_) => {
            return Err(NetworkError::HandShake);
        }
    };
    if header.get_command_name() != VERACK_COMMAND {
        return Err(NetworkError::HandShake);
    }

    let verack = MessageHeader::new(
        shared_settings.get_start_string(),
        VERACK_COMMAND.to_string(),
    );
    if peer.write_all(&verack.as_bytes()).is_err() {
        return Err(NetworkError::HandShake);
    };

    Ok(())
}
