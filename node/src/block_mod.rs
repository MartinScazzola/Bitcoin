//! This crate contains the implementation of a Bitcoin block with its relative structures
//!
//! # Modules
//!
//! - [`block`](block) - Defines the structure and operations related to blocks.
//! - [`block_header`](block_header) - Provides utilities for working with block headers.
//! - [`blockchain`](blockchain) - Implements the main blockchain functionality.
//! - [`coinbase`](coinbase) - Defines the structure of a coinbase transaction.
//! - [`merkle_tree`](merkle_tree) - Implements a Merkle tree for verifying block transactions.
//! - [`outpoint`](outpoint) - Defines the structure of a transaction outpoint.
//! - [`transaction`](transaction) - Defines the structure of a transaction.
//! - [`tx_in`](tx_in) - Defines the structure of a transaction input.
//! - [`tx_in_coinbase`](tx_in_coinbase) - Defines the structure of a coinbase transaction input.
//! - [`tx_out`](tx_out) - Defines the structure of a transaction output.
//! - [`utxo`](utxo) - Implements the unspent transaction output (UTXO) model.

pub mod block;
pub mod block_header;
pub mod blockchain;
pub mod coinbase;
pub mod mempool;
pub mod outpoint;
pub mod script;
pub mod transaction;
pub mod tx_in;
pub mod tx_in_coinbase;
pub mod tx_out;
pub mod utxo;
