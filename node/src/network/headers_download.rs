use super::{network_constants::*, network_error::NetworkError};

use crate::{
    block_mod::block_header::BlockHeader,
    messages::{
        get_headers::GetHeaders,
        header::MessageHeader,
        headers::Headers,
        message_constants::{HEADERS_COMMAND, PING_COMMAND},
        ping::Ping,
        pong::Pong,
    },
    settings_mod::settings::Settings,
};

use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    net::TcpStream,
};

/// Handles non-specific messages received from the peer.
///
/// # Arguments
///
/// * `stream` - The TCP stream to the peer.
/// * `header` - The message header.
/// * `settings` - The network settings.
///
/// # Returns
///
/// Result indicating success or failure.
pub fn handle_other_message(
    stream: &mut TcpStream,
    header: MessageHeader,
    start_string: Vec<u8>,
) -> Result<(), NetworkError> {
    if header.get_command_name() == PING_COMMAND {
        let ping = Ping::from_bytes(header, stream).map_err(|_| NetworkError::HeaderDownload)?;
        let pong = Pong::new(start_string, ping.get_nonce());
        stream
            .write_all(&pong.as_bytes())
            .map_err(|_| NetworkError::HeaderDownload)?;
    } else {
        stream
            .read_exact(&mut vec![0u8; header.get_payload_size() as usize])
            .map_err(|_| NetworkError::HeaderDownload)?;
    }
    Ok(())
}

/// Validates a list of block headers and adds them to the header list if they pass the proof of work.
///
/// # Arguments
///
/// * `headers` - The list of block headers to validate.
/// * `header_list` - The list to store the valid block headers.
/// * `file` - The file to write the valid block headers to.
///
/// # Returns
///
/// Result indicating whether all headers passed the validation or not.
fn validate_headers(
    headers: Vec<BlockHeader>,
    header_list: &mut Vec<BlockHeader>,
    file: &mut File,
) -> Result<(), NetworkError> {
    for h in headers {
        if h.proof_of_work() {
            file.write_all(&h.as_bytes())
                .map_err(|_| NetworkError::HeaderDownload)?;
            header_list
                .last_mut()
                .ok_or(NetworkError::HeaderDownload)?
                .set_next_block_header(h.get_header());
            header_list.push(h);
        } else {
            return Err(NetworkError::HeaderDownload);
        }
    }
    Ok(())
}

/// Checks if a file is empty.
///
/// # Arguments
///
/// * `file_path` - The path to the file.
///
/// # Returns
///
/// Returns true if the file is empty, false otherwise.
fn is_file_empty(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        return metadata.len() == 0;
    }
    // Failed to retrieve metadata, handle the error as needed
    false
}

/// Loads block headers from a file.
///
/// # Arguments
///
/// * `file` - The file to load the headers from.
///
/// # Returns
///
/// Returns a vector of block headers if successful, or a `NetworkError` if an error occurs.
pub fn load_headers(
    settings: &Settings,
    file: &mut File,
) -> Result<Vec<BlockHeader>, NetworkError> {
    let mut block_headers: Vec<BlockHeader> = Vec::new();

    if is_file_empty(settings.get_headers_path()) {
        let genesis = BlockHeader::new(
            GENESIS_VERSION,
            GENESIS_PREVIOUS_BLOCK_HEADER_HASH.to_vec(),
            GENESIS_MERKLE_ROOT_HASH.to_vec(),
            GENESIS_TIME,
            GENESIS_NBITS,
            GENESIS_NONCE,
        );
        file.write_all(&genesis.as_bytes())
            .map_err(|_| NetworkError::HeaderDownload)?;
        block_headers.push(genesis);
        return Ok(block_headers);
    }

    loop {
        let block_header = match BlockHeader::from_bytes(file) {
            Ok(header) => header,
            Err(_) => return Ok(block_headers),
        };

        if !block_headers.is_empty() {
            block_headers
                .last_mut()
                .ok_or(NetworkError::HeaderDownload)?
                .set_next_block_header(block_header.get_header());
        }

        block_headers.push(block_header);
    }
}

/// Downloads block headers from peers.
///
/// # Arguments
///
/// * `settings` - The network settings.
/// * `streams` - The TCP streams to communicate with peers.
///
/// # Returns
///
/// Returns a vector of block headers if successful, or a `NetworkError` if an error occurs.
pub fn headers_download(
    settings: &Settings,
    streams: &mut [TcpStream],
) -> Result<Vec<BlockHeader>, NetworkError> {
    let stream = streams.last_mut().ok_or(NetworkError::HeaderDownload)?;

    // Open the file in read-write mode
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(settings.get_headers_path())
        .map_err(|_| NetworkError::HeaderDownload)?;

    let mut header_list: Vec<BlockHeader> = load_headers(settings, &mut file)?;

    println!(
        "Se inicia la descarga de headers con {} previamente descargados",
        header_list.len()
    );

    loop {
        println!("{}", header_list.len());

        let last_header = header_list
            .last()
            .ok_or(NetworkError::HeaderDownload)?
            .get_header();

        let get_headers = GetHeaders::new(
            settings.get_start_string(),
            settings.get_protocol_version(),
            last_header.clone(),
            STOPPING_HASH.to_vec(),
        );

        stream
            .write_all(&get_headers.as_bytes())
            .map_err(|_| NetworkError::HeaderDownload)?;

        let mut header =
            MessageHeader::from_bytes(stream).map_err(|_| NetworkError::HeaderDownload)?;

        while header.get_command_name() != HEADERS_COMMAND {
            handle_other_message(stream, header.clone(), settings.get_start_string())?;
            header = MessageHeader::from_bytes(stream).map_err(|_| NetworkError::HeaderDownload)?;
        }

        let headers =
            Headers::from_bytes(header, stream).map_err(|_| NetworkError::HeaderDownload)?;

        validate_headers(headers.get_headers(), &mut header_list, &mut file)?;

        if headers.get_count() != MAX_HEADERS_COUNT {
            break;
        }
    }

    println!("Cantidad total headers {}", header_list.len());
    println!("El ultimo header descargado es: \n");
    if let Some(last) = header_list.last() {
        println!("{}", last)
    }

    Ok(header_list)
}

#[cfg(test)]
mod test_header_download {
    use crate::network::headers_download::BlockHeader;
    use crate::network::network_constants::*;
    use crate::network::network_error::NetworkError;
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    #[test]
    fn test_save_genesis_in_file() -> Result<(), NetworkError> {
        let file_path = "data/headers.bin";

        // Open the file in read-write mode
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(file_path)
            .expect("Failed to open the file");

        let genesis = BlockHeader::new(
            GENESIS_VERSION,
            GENESIS_PREVIOUS_BLOCK_HEADER_HASH.to_vec(),
            GENESIS_MERKLE_ROOT_HASH.to_vec(),
            GENESIS_TIME,
            GENESIS_NBITS,
            GENESIS_NONCE,
        );

        file.write_all(&genesis.as_bytes())
            .map_err(|_| NetworkError::HeaderDownload)?;

        file.seek(std::io::SeekFrom::Start(0))
            .map_err(|_| NetworkError::HeaderDownload)?;

        let genesis_recv = BlockHeader::from_bytes(&mut file)?;

        assert_eq!(genesis, genesis_recv);

        Ok(())
    }
}
