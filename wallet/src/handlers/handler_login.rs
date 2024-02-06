use super::{handler_constants::*, handler_content::clean_entry};
use crate::{accounts::Accounts, interface_error::InterfaceError};
use gtk::prelude::*;
use gtk::{Box, Builder, Button, CssProvider, Dialog, Entry, Label, Widget, Window};
use node::messages::read_from_bytes::decode_hex;
use std::sync::{Arc, Mutex};

/// Sets up the login button and its associated functionality.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK `Builder` object.
/// * `accounts` - An `Arc<Mutex<Accounts>>` representing the shared account data.
///
/// # Returns
///
/// Returns a `Result` indicating success (`Ok(())`) or an `InterfaceError` if any required objects are missing.
///
/// # Description
///
/// This function sets up the login button by retrieving the necessary GTK objects from the builder, connecting the click event, and defining the login logic. Upon clicking the login button, the function performs input validation, shows an authentication error dialog if the input is invalid, clears the entry fields, hides the login window, and displays the main window with the updated account information.
pub fn set_login_button(
    builder: &Builder,
    accounts: Arc<Mutex<Accounts>>,
) -> Result<(), InterfaceError> {
    let login_button: Button = builder
        .get_object("login_button")
        .ok_or(InterfaceError::MissingButton)?;
    let main_window: Window = builder
        .get_object("main_window")
        .ok_or(InterfaceError::MissingWindow)?;
    let login_window: Window = builder
        .get_object("login_window")
        .ok_or(InterfaceError::MissingWindow)?;

    let user_authentication_dialog: Dialog = builder
        .get_object("user_authentication_window")
        .ok_or(InterfaceError::MissingDialog)?;
    let title_label: Label = builder
        .get_object("title_error_label")
        .ok_or(InterfaceError::MissingLabel)?;
    let advice_label: Label = builder
        .get_object("advice_label")
        .ok_or(InterfaceError::MissingLabel)?;

    let overview_box: Widget = builder
        .get_object(OVERVIEW_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let content_box: Box = builder
        .get_object(CONTENT_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    content_box.add(&overview_box);

    let accounts_box: Box = builder
        .get_object(ACCOUNTS_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let actual_account_label: Label = builder
        .get_object("actual_account_label")
        .ok_or(InterfaceError::MissingLabel)?;

    // Create a CSS provider and load CSS data to define the color
    let css_provider = CssProvider::new();

    css_provider.load_from_path(STYLE_PATH)?;

    // Add the CSS provider to the style context
    let style_context = login_button.get_style_context();

    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    style_context.add_class(GREEN_BUTTON);

    let username_entry: Entry = builder
        .get_object(USERNAME_ENTRY)
        .ok_or(InterfaceError::MissingEntry)?;

    let public_key_entry: Entry = builder
        .get_object(PUBLIC_KEY_ENTRY)
        .ok_or(InterfaceError::MissingEntry)?;

    let private_key_entry: Entry = builder
        .get_object(PRIVATE_KEY_ENTRY)
        .ok_or(InterfaceError::MissingEntry)?;

    let public_key_to_copy: Label = builder
        .get_object(SHARED_PUBKEY)
        .ok_or(InterfaceError::MissingLabel)?;

    login_button.connect_clicked(move |_| {
        let username = username_entry.get_text();
        let public_key = public_key_entry.get_text();
        let private_key = private_key_entry.get_text();

        if !valid_username(username.as_str())
            || !valid_public_key(public_key.as_str())
            || !valid_private_key(private_key.as_str())
        {
            let mut auth_text = "Please complete the entries correctly".to_string();

            if !valid_username(username.as_str()) {
                auth_text += "\n \n - Username is invalid";
            }

            if !valid_public_key(public_key.as_str()) {
                auth_text += "\n \n - Public key is invalid";
            }

            if !valid_private_key(private_key.as_str()) {
                auth_text += "\n \n - Private key is invalid";
            }

            title_label.set_text("Login Authentication Error");
            advice_label.set_text(auth_text.as_str());
            user_authentication_dialog.show_all();
            return;
        }

        let new_account_button: Button = Button::new();
        new_account_button.set_label(&username);

        let username_account = username.to_string();
        let shared_accounts = accounts.clone();
        let shared_actual_account_label = actual_account_label.clone();
        let shared_public_key_to_copy = public_key_to_copy.clone();
        let shared_public_key = public_key.to_string();

        new_account_button.connect_clicked(move |_| {
            if let Ok(mut locked_accounts) = shared_accounts.lock() {
                locked_accounts.set_actual_account(username_account.clone());
                shared_actual_account_label.set_text(&username_account.clone());
                shared_public_key_to_copy.set_text(&shared_public_key);
            }
        });

        clean_entry(&username_entry);
        clean_entry(&public_key_entry);
        clean_entry(&private_key_entry);

        login_window.hide();

        public_key_to_copy.set_text(&public_key);

        if let Ok(mut accounts) = accounts.lock() {
            if let Ok(public_key_bytes) = decode_hex(&public_key) {
                if let Ok(private_key_bytes) = decode_hex(&private_key) {
                    actual_account_label.set_text(&username);
                    accounts_box.add(&new_account_button);
                    accounts.add_account(username.to_string(), public_key_bytes, private_key_bytes);
                    drop(accounts);
                    main_window.show_all();
                }
            }
        }
    });
    Ok(())
}

/// Sets up the functionality for the "Return" button.
///
/// This function connects the "Return" button to a click event handler. When clicked, it hides the login window,
/// cleans the username, public key, and private key entries, and shows the main window.
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
/// This function sets up the functionality for the "Return" button by performing the following steps:
///
/// 1. Retrieves the necessary UI elements from the builder, including the main window, login window,
///    "Return" button, and username, public key, and private key entries.
/// 2. Connects the "Return" button to a click event handler using the `connect_clicked` method.
/// 3. In the click event handler, hides the login window, cleans the username, public key, and private key entries,
///    and shows the main window.
/// 4. Returns `Ok(())` if the function executes successfully.
///
pub fn set_return_button(builder: &Builder) -> Result<(), InterfaceError> {
    let main_window: Window = builder
        .get_object(MAIN_WINDOW)
        .ok_or(InterfaceError::MissingWindow)?;
    let login_window: Window = builder
        .get_object(LOGIN_WINDOW)
        .ok_or(InterfaceError::MissingWindow)?;
    let return_button: Button = builder
        .get_object(RETURN_BUTTON)
        .ok_or(InterfaceError::MissingWindow)?;

    let username_entry: Entry = builder
        .get_object(USERNAME_ENTRY)
        .ok_or(InterfaceError::MissingEntry)?;
    let public_key_entry: Entry = builder
        .get_object(PUBLIC_KEY_ENTRY)
        .ok_or(InterfaceError::MissingEntry)?;
    let private_key_entry: Entry = builder
        .get_object(PRIVATE_KEY_ENTRY)
        .ok_or(InterfaceError::MissingEntry)?;

    return_button.connect_clicked(move |_| {
        login_window.hide();
        clean_entry(&username_entry);
        clean_entry(&public_key_entry);
        clean_entry(&private_key_entry);
        main_window.show_all();
    });

    Ok(())
}

/// Checks if the provided username is valid.
///
/// # Arguments
///
/// * `username` - A string slice representing the username to be validated.
///
/// # Returns
///
/// Returns a boolean value indicating whether the username is valid (`true`) or not (`false`).
///
/// # Description
///
/// This function validates the provided username by checking its length. It returns `true` if the length of the username is greater than 0 and less than 20, indicating that it meets the required criteria for a valid username.
///
fn valid_username(username: &str) -> bool {
    !username.is_empty() && username.len() < 20
}

/// Checks if the provided public key is valid.
///
/// # Arguments
///
/// * `public_key` - A string slice representing the public key to be validated.
///
/// # Returns
///
/// Returns a boolean value indicating whether the public key is valid (`true`) or not (`false`).
///
/// # Description
///
/// This function validates the provided public key by checking its length and ensuring that all characters are ASCII hexadecimal digits. It returns `true` if the length of the public key is 66 (indicating the expected length for a valid public key) and all characters in the public key are ASCII hexadecimal digits.
///
fn valid_public_key(public_key: &str) -> bool {
    public_key.len() == 66 && public_key.chars().all(|c| c.is_ascii_hexdigit())
}

/// Checks if the provided private key is valid.
///
/// # Arguments
///
/// * `private_key` - A string slice representing the private key to be validated.
///
/// # Returns
///
/// Returns a boolean value indicating whether the private key is valid (`true`) or not (`false`).
///
/// # Description
///
/// This function validates the provided private key by checking its length and ensuring that all characters are ASCII hexadecimal digits. It returns `true` if the length of the private key is 64 (indicating the expected length for a valid private key) and all characters in the private key are ASCII hexadecimal digits.
///
fn valid_private_key(private_key: &str) -> bool {
    private_key.len() == 64 && private_key.chars().all(|c| c.is_ascii_hexdigit())
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
pub fn set_login_ok_button(builder: &Builder) -> Result<(), InterfaceError> {
    let user_authentication_dialog: Dialog = builder
        .get_object("user_authentication_window")
        .ok_or(InterfaceError::MissingDialog)?;
    let ok_button: Button = builder
        .get_object("ok_button")
        .ok_or(InterfaceError::MissingButton)?;

    ok_button.connect_clicked(move |_| {
        user_authentication_dialog.hide();
    });

    Ok(())
}
