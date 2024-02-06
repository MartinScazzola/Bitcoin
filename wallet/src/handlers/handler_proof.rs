use gtk::prelude::*;
use gtk::{Box, Builder, Button, Dialog, Entry, Label};
use node::messages::read_from_bytes::{decode_hex, read_string_from_bytes};
use node::wallet_utils::get_proof::GetProof;
use node::wallet_utils::merkle_block::MerkleBlock;
use std::io::Write;

use crate::interface_error::InterfaceError;
use crate::proof_of_inclusion::get_proof_of_inclusion::get_proof_of_inclusion;

use super::{handler_constants::*, handler_content::replace_content};

use std::sync::{Arc, Mutex};

use std::net::TcpStream;

/// Sets up the functionality for the Point of Interest (POI) button.
///
/// The POI button allows the user to navigate to the Point of Interest section of the application.
/// When clicked, the content box is replaced with the POI box, which displays the relevant information.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK builder used to access UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the POI button setup is successful, or an `InterfaceError` if any required UI element is missing.
///
/// # Description
///
/// This function sets up the POI button by connecting its `clicked` signal to a closure that performs the navigation functionality.
/// It retrieves the POI button, POI box, and content box from the builder. When the POI button is clicked, the closure is executed.
/// It calls the `replace_content` function to replace the content in the content box with the POI box. Finally, it shows the POI box.
///
pub fn set_poi_button(builder: &Builder) -> Result<(), InterfaceError> {
    let poi_button: Button = builder
        .get_object(POI_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let poi_box = builder
        .get_object(POI_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let content_box: Box = builder
        .get_object(CONTENT_BOX)
        .ok_or(InterfaceError::MissingBox)?;

    poi_button.connect_clicked(move |_| {
        replace_content(&content_box, &poi_box);
        poi_box.show_all();
    });
    Ok(())
}

/// Sets up the "Make Proof" button functionality.
///
/// This function connects the "Make Proof" button to the corresponding action when clicked.
/// It retrieves the necessary UI elements from the builder and handles user input validation.
/// It then sends a request to the specified node to obtain a proof of inclusion for a given block header and transaction ID.
/// Finally, it displays the result in a dialog window.
///
/// # Arguments
///
/// * `builder` - The builder object containing the UI definition.
/// * `node` - The TCP stream representing the connection to the node.
///
/// # Returns
///
/// Returns `Ok(())` if the setup is successful, or an `InterfaceError` if any UI element is missing.
///
/// # Description
///
/// This function sets up the "Make Proof" button by performing the following steps:
///
/// 1. Retrieves the "Make Proof" button, block header entry, and transaction ID entry from the builder.
/// 2. Retrieves the necessary dialog elements for error display.
/// 3. Connects the "Make Proof" button's click event to the corresponding action.
/// 4. Validates the block header and transaction ID input provided by the user.
/// 5. Sends a request to the specified node to obtain a proof of inclusion.
/// 6. Displays the result in a dialog window.
///
pub fn set_make_proof_button(
    builder: &Builder,
    node: Arc<Mutex<TcpStream>>,
) -> Result<(), InterfaceError> {
    let make_proof_button: Button = builder
        .get_object(MAKE_PROOF_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let block_header_entry: Entry = builder
        .get_object(BLOCK_HEADER_ENTRY)
        .ok_or(InterfaceError::MissingEntry)?;
    let transaction_id_entry: Entry = builder
        .get_object(TRANSACTION_ID_ENTRY)
        .ok_or(InterfaceError::MissingEntry)?;

    let user_authentication_dialog: Dialog = builder
        .get_object("user_authentication_window")
        .ok_or(InterfaceError::MissingDialog)?;
    let title_label: Label = builder
        .get_object("title_error_label")
        .ok_or(InterfaceError::MissingLabel)?;
    let advice_label: Label = builder
        .get_object("advice_label")
        .ok_or(InterfaceError::MissingLabel)?;

    let poi_error_dialog: Dialog = builder
        .get_object("proof_of_inclusion_error_window")
        .ok_or(InterfaceError::MissingDialog)?;
    let poi_success_dialog: Dialog = builder
        .get_object("proof_of_inclusion_success_window")
        .ok_or(InterfaceError::MissingDialog)?;

    make_proof_button.connect_clicked(move |_| {
        let block_header_text = block_header_entry.get_text();
        let transaction_id_text = transaction_id_entry.get_text();

        if !valid_block_header(block_header_text.as_str())
            || !valid_transaction_id(transaction_id_text.as_str())
        {
            let mut auth_text = "Please complete the entries correctly".to_string();

            if !valid_block_header(block_header_text.as_str()) {
                auth_text += "\n \n - Block header is invalid";
            }

            if !valid_transaction_id(transaction_id_text.as_str()) {
                auth_text += "\n \n - Transaction ID is invalid";
            }

            title_label.set_text("Proof of Inclusion Authentication Error");
            advice_label.set_text(auth_text.as_str());
            user_authentication_dialog.set_size_request(600, 200);
            user_authentication_dialog.show_all();
            return;
        }
        let block_header: Vec<u8> = match decode_hex(&block_header_entry.get_text()) {
            Ok(header) => header,
            Err(_) => return,
        };

        let tx_id: Vec<u8> = match decode_hex(&transaction_id_entry.get_text()) {
            Ok(tx_id) => tx_id,
            Err(_) => return,
        };
        let get_proof = GetProof::new(block_header, tx_id);

        let mut locked_node = match node.lock() {
            Ok(locked_node) => locked_node,
            Err(_) => return,
        };

        if locked_node.write_all(&get_proof.as_bytes()).is_err() {
            return;
        };

        let command_name = match read_string_from_bytes(&mut *locked_node, 12) {
            Ok(command_name) => command_name,
            Err(_) => return,
        };

        if command_name != MERKLE_BLOCK {
            poi_error_dialog.show();
            drop(locked_node);
            return;
        }

        let merkle_block = match MerkleBlock::from_bytes(command_name, &mut *locked_node) {
            Ok(merkle_block) => merkle_block,
            Err(_) => return,
        };

        let proof_of_inclusion = match get_proof_of_inclusion(merkle_block) {
            Ok(proof) => proof,
            Err(_) => return,
        };

        if proof_of_inclusion {
            poi_success_dialog.show();
        } else {
            poi_error_dialog.show();
        }
        drop(locked_node);
    });

    Ok(())
}

/// Validates a block header.
///
/// Checks if the given block header is a valid hexadecimal string with a length of 64 characters.
///
/// # Arguments
///
/// * `block_header` - The block header to validate as a hexadecimal string.
///
/// # Returns
///
/// Returns `true` if the block header is valid, or `false` otherwise.
///
/// # Description
///
/// This function validates a block header by checking if it meets the following criteria:
///
/// * The block header has a length of 64 characters.
/// * All characters in the block header are valid hexadecimal digits.
///
fn valid_block_header(block_header: &str) -> bool {
    block_header.len() == 64 && block_header.chars().all(|c| c.is_ascii_hexdigit())
}

/// Validates a transaction ID.
///
/// Checks if the given transaction ID is a valid hexadecimal string with a length of 64 characters.
///
/// # Arguments
///
/// * `transaction_id` - The transaction ID to validate as a hexadecimal string.
///
/// # Returns
///
/// Returns `true` if the transaction ID is valid, or `false` otherwise.
///
/// # Description
///
/// This function validates a transaction ID by checking if it meets the following criteria:
///
/// * The transaction ID has a length of 64 characters.
/// * All characters in the transaction ID are valid hexadecimal digits.
///
fn valid_transaction_id(transaction_id: &str) -> bool {
    transaction_id.len() == 64 && transaction_id.chars().all(|c| c.is_ascii_hexdigit())
}

/// Sets up the functionality for the "OK" buttons in the user authentication and proof of inclusion dialogs.
///
/// This function connects the "OK" buttons to click event handlers. When clicked, it hides the respective dialog.
///
/// # Arguments
///
/// * `builder` - The Builder object for accessing UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if any UI elements are missing.
///
/// # Description
///
/// This function sets up the functionality for the "OK" buttons in the user authentication and proof of inclusion dialogs
/// by performing the following steps:
///
/// 1. Retrieves the necessary UI elements from the builder, including the user authentication dialog, proof of inclusion dialog,
///    "OK" button in the user authentication dialog, and "OK" button in the proof of inclusion dialog.
/// 2. Connects the "OK" buttons to click event handlers using the `connect_clicked` method.
/// 3. In the click event handlers, hides the respective dialog.
/// 4. Returns `Ok(())` if the function executes successfully.
///
pub fn set_poi_success_ok_button(builder: &Builder) -> Result<(), InterfaceError> {
    let poi_dialog: Dialog = builder
        .get_object("proof_of_inclusion_success_window")
        .ok_or(InterfaceError::MissingDialog)?;
    let poi_ok_button: Button = builder
        .get_object("poi_success_ok_button")
        .ok_or(InterfaceError::MissingButton)?;

    poi_ok_button.connect_clicked(move |_| {
        poi_dialog.hide();
    });

    Ok(())
}

pub fn set_poi_error_ok_button(builder: &Builder) -> Result<(), InterfaceError> {
    let poi_dialog: Dialog = builder
        .get_object("proof_of_inclusion_error_window")
        .ok_or(InterfaceError::MissingDialog)?;
    let poi_ok_button: Button = builder
        .get_object("poi_error_ok_button")
        .ok_or(InterfaceError::MissingButton)?;

    poi_ok_button.connect_clicked(move |_| {
        poi_dialog.hide();
    });

    Ok(())
}
