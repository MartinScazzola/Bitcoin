use gtk::prelude::*;
use gtk::{Builder, Button, Window};

use crate::interface_error::InterfaceError;

use super::handler_constants::*;

/// Sets up the functionality for the "Send Transaction" button.
///
/// This function connects the "Send Transaction" button to a click event handler. When clicked, it
/// retrieves the target list, fee value, and private key from the UI elements and attempts to create
/// a transaction. If the transaction creation is successful, it creates a new transaction view and
/// updates the transaction box accordingly.
///
/// # Arguments
///
/// * `builder` - The Builder object for accessing UI elements.
/// * `node` - An Arc-wrapped Mutex-wrapped TcpStream for communication with the node.
/// * `accounts` - An Arc-wrapped Mutex for accessing account information.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if any UI elements are missing.
///
/// # Description
///
/// This function sets up the functionality for the "Send Transaction" button by performing the following steps:
///
/// 1. Retrieves the necessary UI elements from the builder, including the "Send Transaction" button,
///    transaction box, fee spin button, and account information.
/// 2. Connects the "Send Transaction" button to a click event handler using the `connect_clicked` method.
/// 3. In the click event handler, retrieves the target list, fee value, and private key from the UI elements.
/// 4. Acquires a lock on the accounts Mutex to access the account information.
/// 5. If the account information is available, attempts to create a transaction using the `create_transaction` function.
/// 6. If the transaction creation is successful, creates a new transaction view and updates the transaction box.
/// 7. Handles any errors that may occur during the transaction creation process, such as insufficient funds.
/// 8. Drops the lock on the accounts Mutex.
/// 9. Returns `Ok(())` if the function executes successfully.
///
pub fn set_new_account_button(builder: &Builder) -> Result<(), InterfaceError> {
    let main_window: Window = builder
        .get_object(MAIN_WINDOW)
        .ok_or(InterfaceError::MissingWindow)?;
    let login_window: Window = builder
        .get_object(LOGIN_WINDOW)
        .ok_or(InterfaceError::MissingWindow)?;
    let new_account_button: Button = builder
        .get_object(NEW_ACCOUNT_BUTTON)
        .ok_or(InterfaceError::MissingWindow)?;
    let return_button: Button = builder
        .get_object(RETURN_BUTTON)
        .ok_or(InterfaceError::MissingWindow)?;

    new_account_button.connect_clicked(move |_| {
        main_window.hide();
        return_button.show();
        login_window.show_all();
    });

    Ok(())
}
