//! This module contains the components related to network functionality and communication of a Bitcoin node.
//!
//! # Modules
//!
//! - [`block_download`](block_download) - Implements block download functionality for syncing with the Bitcoin network.
//! - [`broadcasting`](broadcasting) - Listens for incoming new headers to update the blockchain.
//! - [`handshake`](handshake) - Implements the handshake protocol for establishing connections with peers.
//! - [`headers_download`](headers_download) - Implements headers download functionality for syncing block headers with the Bitcoin network.
//! - [`network_constants`](network_constants) - Defines constants related to the Bitcoin network.
//! - [`network_error`](network_error) - Implements error handling for network-related operations.

pub mod block_download;
pub mod broadcasting;
pub mod handshake;
pub mod headers_download;
pub mod network_constants;
pub mod network_error;
pub mod recv_peer_connection;
