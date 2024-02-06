//! This module contains the messages used for communication between peers in the Bitcoin protocol.
//!
//! # Modules
//!
//! - [`addr`](addr) - Is the response to the "GetAddress" message, and it contains IP addresses of other peers.
//! - [`compact_size`](compact_size) - Provides utilities for working with values of variable length byte size.
//! - [`get_data`](get_data) - Implements the `getdata` message for requesting different types of data from peers.
//! - [`get_headers`](get_headers) - Implements the `getheaders` message for requesting block headers from peers.
//! - [`header`](header) - Defines the structure and operations related to block headers.
//! - [`headers`](headers) - Implements the `headers` message for sending block headers to peers.
//! - [`inventory`](inventory) - Defines the structure and operations related to inventory items.
//! - [`ip`](ip) - Provides IP address handling utilities.
//! - [`message_constants`](message_constants) - Defines constants related to Bitcoin protocol messages.
//! - [`message_error`](message_error) - Implements error handling for Bitcoin protocol messages.
//! - [`ping`](ping) - Implements the `ping` message for network connection testing.
//! - [`pong`](pong) - Implements the `pong` message as a response to `ping` messages.
//! - [`read_from_bytes`](read_from_bytes) - Provides utilities for reading data from byte buffers.
//! - [`script`](script) - Defines the structure and operations related to Bitcoin script.
//! - [`version`](version) - Implements the `version` message for protocol version negotiation.

pub mod addr;
pub mod block;
pub mod compact_size;
pub mod filterload;
pub mod get_data;
pub mod get_headers;
pub mod header;
pub mod headers;
pub mod inv;
pub mod inventory;
pub mod ip;
pub mod message_constants;
pub mod message_error;
pub mod not_found;
pub mod ping;
pub mod pong;
pub mod queue;
pub mod read_from_bytes;
pub mod script;
pub mod tx;
pub mod version;
