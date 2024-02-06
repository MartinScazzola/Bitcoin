use crate::accounts::Accounts;
use crate::interface_error::InterfaceError;
use gtk::{Builder, ListStore};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use super::handler_accounts::set_new_account_button;
use super::handler_login::{set_login_button, set_login_ok_button, set_return_button};
use super::handler_overview::set_overview_button;
use super::handler_proof::{
    set_make_proof_button, set_poi_button, set_poi_error_ok_button, set_poi_success_ok_button,
};
use super::handler_receive::{set_copy_button, set_receive_button};
use super::handler_send::{
    set_add_recipient_button, set_clear_all_button, set_insuficient_ok_button, set_send_button,
    set_send_transaction_button,
};
use super::handler_transactions::set_transactions_button;

/// Sets various buttons on the interface.
///
/// # Arguments
///
/// * `builder` - A reference to the builder object.
/// * `accounts` - A shared mutable reference to the accounts.
/// * `node` - A shared mutable reference to the TCP stream node.
/// * `store` - A reference to the list store.
///
/// # Errors
///
/// Returns an `InterfaceError` if an error occurs while setting the buttons.
pub fn set_buttons(
    builder: &Builder,
    accounts: Arc<Mutex<Accounts>>,
    node: Arc<Mutex<TcpStream>>,
    store: &ListStore,
) -> Result<(), InterfaceError> {
    set_login_button(builder, accounts.clone())?;
    set_overview_button(builder)?;
    set_send_button(builder)?;
    set_receive_button(builder)?;
    set_transactions_button(builder, store)?;
    set_add_recipient_button(builder)?;
    set_clear_all_button(builder)?;
    set_copy_button(builder)?;
    set_poi_button(builder)?;
    set_send_transaction_button(builder, node.clone(), accounts)?;
    set_make_proof_button(builder, node)?;
    set_new_account_button(builder)?;
    set_return_button(builder)?;
    set_login_ok_button(builder)?;
    set_poi_success_ok_button(builder)?;
    set_poi_error_ok_button(builder)?;
    set_insuficient_ok_button(builder)?;
    Ok(())
}
