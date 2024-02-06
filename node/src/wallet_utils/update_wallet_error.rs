#[derive(Debug)]
pub enum UpdateWalletError {
    Read,
    Write,
    SendProof,
    BroadcastTx,
    LockMempool,
    LockBlockchain,
    GetTxn,
    AcceptConnection,
    Progress,
}
