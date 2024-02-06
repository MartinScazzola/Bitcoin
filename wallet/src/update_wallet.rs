use crate::{
    accounts::Accounts, interface_error::InterfaceError,
    transactions::create_transactions::pk_script_from_public_key,
};
use node::{
    messages::read_from_bytes::{fill_command, read_string_from_bytes},
    wallet_utils::{
        get_transactions::GetTransactions,
        transactions::Transactions,
        wallet_utils_constants::{EXIT_COMMAND, TRANSACTIONS_COMMAND},
    },
};
use std::{
    io::Write,
    net::TcpStream,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread,
    time::Duration,
};

/// Updates the wallet by retrieving and processing transactions from the node.
///
/// This function continuously loops and updates the wallet by retrieving and processing
/// transactions from the node. It takes the shared `Accounts` object, the shared `TcpStream`
/// representing the connection to the node, and a sender for transaction update signals as input.
/// Within the loop, it locks the `Accounts` object to access the current user's information. If
/// there is an actual account, it retrieves the necessary information, such as the public key and
/// the last update timestamp. It then locks the `TcpStream` to communicate with the node and
/// requests transactions using the `GetTransactions` command. The retrieved transactions are
/// processed and updated in the user's account. Finally, a transaction update signal is sent using
/// the provided sender. The loop continues to execute after a brief sleep of 10 seconds.
///
/// # Arguments
///
/// * `accounts` - The shared `Accounts` object.
/// * `node` - The shared `TcpStream` representing the connection to the node.
/// * `txs_sender` - The sender for transaction update signals.
///
/// # Returns
///
/// Returns `Ok(())` if the wallet is successfully updated, or an `InterfaceError` if there is an
/// error while retrieving transactions, processing them, or sending the transaction update signal.
pub fn update_wallet(
    accounts: Arc<Mutex<Accounts>>,
    node: Arc<Mutex<TcpStream>>,
    txs_sender: glib::Sender<bool>,
    exit_recv: Receiver<bool>,
) -> Result<(), InterfaceError> {
    loop {
        if exit_recv.try_recv().is_ok() {
            if let Ok(mut locked_node) = node.lock() {
                if locked_node
                    .write_all(fill_command(EXIT_COMMAND).as_bytes())
                    .is_err()
                {
                    println!("Exit error");
                };
            }
            return Ok(());
        }

        let mut locked_accounts = accounts.lock().map_err(|_| InterfaceError::LockAccounts)?;

        if let Some(user_info) = locked_accounts.get_actual_account() {
            let mut locked_node = node.lock().map_err(|_| InterfaceError::LockNode)?;
            let pk_script = pk_script_from_public_key(&user_info.get_public_key());
            let get_transactions = GetTransactions::new(
                pk_script,
                user_info.get_public_key(),
                user_info.get_last_update(),
            );
            locked_node
                .write_all(&get_transactions.as_bytes())
                .map_err(|_| InterfaceError::Write)?;
            let command_name =
                read_string_from_bytes(&mut *locked_node, 12).map_err(|_| InterfaceError::Read)?;

            if command_name != TRANSACTIONS_COMMAND {
                return Err(InterfaceError::InvalidResponse);
            }

            let transactions =
                Transactions::from_bytes(&mut *locked_node).map_err(|_| InterfaceError::Read)?;
            drop(locked_node);

            locked_accounts.update(&transactions);

            txs_sender.send(true).map_err(|_| InterfaceError::Send)?;
        }
        drop(locked_accounts);
        thread::sleep(Duration::from_secs(5));
    }
}
