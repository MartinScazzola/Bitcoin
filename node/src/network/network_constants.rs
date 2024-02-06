//hand shake
pub const DURATION_TIMEOUT_MILLIS: u64 = 1000;
pub const VERSION_ACEPTED: i32 = 70016;
pub const SERVICES_ACEPTED: u64 = 1033;
pub const DIG_COMMAND: &str = "dig";
pub const SHORT_ARG: &str = "+short";

//headers download
pub const GENESIS_VERSION: i32 = 1;
pub const GENESIS_PREVIOUS_BLOCK_HEADER_HASH: [u8; 32] = [0u8; 32];
pub const GENESIS_MERKLE_ROOT_HASH: [u8; 32] = [
    59, 163, 237, 253, 122, 123, 18, 178, 122, 199, 44, 62, 103, 118, 143, 97, 127, 200, 27, 195,
    136, 138, 81, 50, 58, 159, 184, 170, 75, 30, 94, 74,
];
pub const GENESIS_TIME: u32 = 1296688602;
pub const GENESIS_NBITS: u32 = 486604799;
pub const GENESIS_NONCE: u32 = 414098458;
pub const STOPPING_HASH: [u8; 32] = [0; 32];

//block download
pub const MAX_BLOCKS_GET_DATA: usize = 50000;
pub const MAX_HEADERS_COUNT: u64 = 2000;
pub const MSG_BLOCK_DATA_TYPE: u32 = 2;
pub const MSG_TX_DATA_TYPE: u32 = 1;

pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%:z";
