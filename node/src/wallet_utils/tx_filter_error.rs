#[derive(Debug)]
pub enum TxFilterError {
    UnfoundBlock,
    DateTimeError,
    LockBlockchain,
    LockMempool,
    LockUtxo,
}
