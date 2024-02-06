//! This crate contains the main functionality for
//! # Modules
//!
//! - [`block_mod`](block_mod) - Implements the block with with its respective transactions, block header, and Merkle tree.
//! - [`messages`](messages) - Defines various messages for the communication between peers.
//! - [`network`](network) - Implements networking functionalities.
//! - [`settings_mod`](settings_mod) - Handles configuration settings of the node.

pub mod block_mod;
pub mod block_saver;
pub mod messages;
pub mod network;
pub mod proof_of_inclusion_mod;
pub mod settings_mod;
pub mod wallet_utils;
