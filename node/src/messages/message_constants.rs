pub const VERSION_COMMAND: &str = "version";
pub const VERACK_COMMAND: &str = "verack";
pub const ADDR_COMMAND: &str = "addr";
pub const GET_ADDR_COMMAND: &str = "getaddr";
pub const GET_HEADERS_COMMAND: &str = "getheaders";
pub const HEADERS_COMMAND: &str = "headers";
pub const PONG_COMMAND: &str = "pong";
pub const PING_COMMAND: &str = "ping";
pub const GET_DATA_COMMAND: &str = "getdata";
pub const BLOCK_COMMAND: &str = "block";
pub const SEND_HEADERS_COMMAND: &str = "sendheaders";
pub const INV_COMMAND: &str = "inv";
pub const TX_COMMAND: &str = "tx";
pub const FILTER_LOAD_COMMAND: &str = "filterload";
pub const MERKLE_BLOCK_COMMAND: &str = "merkleblock";
pub const NOT_FOUND_COMMAND: &str = "notfound";

pub const HEADER_BYTES_SIZE: usize = 24;

pub const PAYLOAD_EMPTY_MSG: u32 = 0;
pub const CHECKSUM_EMPTY_MSG: [u8; 4] = [93, 246, 224, 226];

pub const BYTE_SIZE: u64 = 252;
pub const TWO_BYTE_SIZE: u8 = 253;
pub const FOUR_BYTE_SIZE: u8 = 254;
pub const EIGHT_BYTE_SIZE: u8 = 255;
