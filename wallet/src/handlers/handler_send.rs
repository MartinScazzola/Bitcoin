use gtk::{prelude::*, Dialog};
use gtk::{Box, Builder, Button, Fixed, SpinButton, Widget};
use node::wallet_utils::broadcast_txn::BroadcastTxn;
use std::io::Write;

use crate::accounts::Accounts;
use crate::transactions::create_transaction_error::TransactionCreateError;
use crate::transactions::create_transactions::create_transaction;
use crate::{interface_error::InterfaceError, views::transaction_view::create_transaction_view};

use super::{handler_constants::*, handler_content::replace_content};

use std::sync::{Arc, Mutex};

use std::net::TcpStream;

/// Sets the functionality of the send button.
///
/// # Arguments
///
/// * `builder` - A reference to the builder object.
///
/// # Errors
///
/// Returns an `InterfaceError` if the button, frame, or box objects are missing.
pub fn set_send_button(builder: &Builder) -> Result<(), InterfaceError> {
    let overview_button: Button = builder
        .get_object(SEND_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let send_frame = builder
        .get_object(SEND_FRAME)
        .ok_or(InterfaceError::MissingFrame)?;
    let transaction_box: Box = builder
        .get_object(TRANSACTION_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let content_box: Box = builder
        .get_object(CONTENT_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let new_transaction: Fixed = create_transaction_view(transaction_box.clone())?;
    transaction_box.add(&new_transaction);

    overview_button.connect_clicked(move |_| {
        replace_content(&content_box, &send_frame);
        send_frame.show_all();
    });
    Ok(())
}

/// Sets up the add recipient button and its associated functionality.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK `Builder` object.
///
/// # Returns
///
/// Returns a `Result` indicating success (`Ok(())`) or an `InterfaceError` if any required objects are missing.
///
/// # Description
///
/// This function sets up the add recipient button by retrieving the necessary GTK objects from the builder, connecting the click event, and defining the logic to add a new recipient. Upon clicking the add recipient button, the function creates a new transaction view and adds it to the transaction box. The new transaction view is then shown.
///
pub fn set_add_recipient_button(builder: &Builder) -> Result<(), InterfaceError> {
    let add_recipient_button: Button = builder
        .get_object(ADD_RECIPIENT_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let transaction_box: Box = builder
        .get_object(TRANSACTION_BOX)
        .ok_or(InterfaceError::MissingBox)?;

    add_recipient_button.connect_clicked(move |_| {
        if let Ok(new_transaction) = create_transaction_view(transaction_box.clone()) {
            transaction_box.add(&new_transaction);
            new_transaction.show_all();
        };
    });
    Ok(())
}

/// Sets up the functionality for the "Clear All" button.
///
///
/// # Arguments
///
/// * `builder` - A reference to the GTK builder.
///
/// # Returns
///
/// * `Result<(), InterfaceError>` - A result indicating success or failure. Returns `Ok(())`
///   if the button setup was successful, or an `InterfaceError` if any required objects are missing.
///
///  # Description
///
/// The "Clear All" button, when clicked, clears all transaction-related widgets from the
/// transaction box and adds a new transaction view widget.
///
pub fn set_clear_all_button(builder: &Builder) -> Result<(), InterfaceError> {
    let clear_all_button: Button = builder
        .get_object(CLEAR_ALL_BUTTON)
        .ok_or(InterfaceError::MissingLabel)?;
    let transaction_box: Box = builder
        .get_object(TRANSACTION_BOX)
        .ok_or(InterfaceError::MissingBox)?;

    clear_all_button.connect_clicked(move |_| {
        if let Ok(new_transaction) = create_transaction_view(transaction_box.clone()) {
            clear_and_add_widget(&transaction_box, new_transaction.upcast_ref());
        }
    });
    Ok(())
}

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
pub fn set_send_transaction_button(
    builder: &Builder,
    node: Arc<Mutex<TcpStream>>,
    accounts: Arc<Mutex<Accounts>>,
) -> Result<(), InterfaceError> {
    let send_transaction_button: Button = builder
        .get_object(SEND_TX_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let transaction_box: Box = builder
        .get_object(TX_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let spin_button_fee: SpinButton = builder
        .get_object(FEE_SPIN_BUTTON)
        .ok_or(InterfaceError::MissingSpinButton)?;
    let insuficient_funds_dialog: Dialog = builder
        .get_object(INSUFFICIENT_FUNDS_DIALOG)
        .ok_or(InterfaceError::MissingDialog)?;

    send_transaction_button.connect_clicked(move |_| {
        let target_list = get_target_list(&transaction_box);
        if let Ok(locked_accounts) = accounts.lock() {
            if let Some(user_info) = locked_accounts.get_actual_account() {
                let private_key = user_info.get_private_key();
                let fee = spin_button_fee.get_value() * 100000000.0;
                match create_transaction(target_list, user_info.get_utxo(), private_key, fee as i64)
                {
                    Ok(transaction) => {
                        if let Ok(mut locked_node) = node.lock() {
                            let broadcast_txn = BroadcastTxn::new(transaction);

                            if locked_node.write_all(&broadcast_txn.as_bytes()).is_err() {
                                return;
                            }

                            if let Ok(new_transaction) =
                                create_transaction_view(transaction_box.clone())
                            {
                                clear_and_add_widget(
                                    &transaction_box,
                                    new_transaction.upcast_ref(),
                                );
                            }
                            drop(locked_node);
                        }
                    }
                    Err(TransactionCreateError::InsufficientFounds) => {
                        insuficient_funds_dialog.show()
                    }
                    _ => {}
                }
            }
            drop(locked_accounts);
        }
    });

    Ok(())
}

pub fn set_insuficient_ok_button(builder: &Builder) -> Result<(), InterfaceError> {
    let insufficient_funds_dialog: Dialog = builder
        .get_object(INSUFFICIENT_FUNDS_DIALOG)
        .ok_or(InterfaceError::MissingDialog)?;
    let ok_button: Button = builder
        .get_object(INSUFFICIENT_FUNDS_OK_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    ok_button.connect_clicked(move |_| {
        insufficient_funds_dialog.hide();
    });

    Ok(())
}

/// Clears the contents of a GTK box and adds a new widget to it.
///
/// # Arguments
///
/// * `gtk_box` - A reference to the GTK `Box` object.
/// * `widget` - A reference to the GTK `Widget` to be added.
///
/// # Description
///
/// This function removes all existing children from the provided GTK box and adds a new widget to it. The widget is then shown.
///
fn clear_and_add_widget(gtk_box: &Box, widget: &Widget) {
    gtk_box.foreach(|child| {
        gtk_box.remove(child);
    });
    gtk_box.add(widget);
    widget.show_all();
}

/// Retrieves the target list from the transaction box.
///
/// This function iterates over the children of the transaction box and extracts the target values
/// (byte arrays) and amounts (as integers) from the corresponding UI elements.
///
/// # Arguments
///
/// * `transaction_box` - The Box object representing the transaction box container.
///
/// # Returns
///
/// Returns a vector of tuples containing the target values and amounts.
///
/// # Description
///
/// This function retrieves the target list from the transaction box by performing the following steps:
///
/// 1. Initializes an empty vector to store the target values and amounts.
/// 2. Iterates over each transaction UI element within the transaction box.
/// 3. For each transaction, extracts the target value and amount by iterating over its child elements.
/// 4. Checks if the child element is a SpinButton or Entry widget and retrieves the corresponding value.
/// 5. Stores the target value and amount as a tuple in the vector.
/// 6. Returns the vector of target values and amounts.
///
fn get_target_list(transaction_box: &Box) -> Vec<(Vec<u8>, i64)> {
    let mut target_list = vec![];

    for tx in transaction_box.get_children() {
        let mut target: (Vec<u8>, i64) = (vec![], 0);
        if let Some(tx_fixed) = tx.downcast_ref::<Fixed>() {
            for fixed_child in tx_fixed.get_children() {
                if let Some(spin_button) = fixed_child.downcast_ref::<gtk::SpinButton>() {
                    target.1 = (spin_button.get_value() * 100000000.0) as i64;
                } else if let Some(entry) = fixed_child.downcast_ref::<gtk::Entry>() {
                    target.0 = entry.get_text().as_bytes().to_vec();
                }
            }
        }
        target_list.push(target);
    }

    target_list
}
