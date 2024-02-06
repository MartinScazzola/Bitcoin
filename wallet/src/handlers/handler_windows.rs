use super::handler_constants::{LOGIN_WINDOW, MAIN_WINDOW};
use crate::interface_error::InterfaceError;
use gtk::prelude::*;
use gtk::{Builder, Window};

/// Sets up the main window and login window.
/// # Arguments
///
/// * `builder` - The Builder object used to access UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if there is an issue accessing UI elements.
///
/// # Description
///
/// This function sets up the main and login windows by calling the respective helper functions:
///
/// 1. `set_main_window`: Sets up the main window.
/// 2. `set_login_window`: Sets up the login window.
///

pub fn set_windows(builder: &Builder) -> Result<(), InterfaceError> {
    set_main_window(builder)?;
    set_login_window(builder)?;
    set_loading_window(builder)?;
    Ok(())
}

/// This function initializes and configures the main window of the interface.
///
/// # Arguments
///
/// * `builder` - The Builder object used to access UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if there is an issue accessing the main window.
///
/// # Description
///
/// This function sets up the main window by performing the following tasks:
///
/// 1. Retrieves the main window object from the builder.
/// 2. Connects the delete event of the main window to the gtk::main_quit function, ensuring the application terminates when the window is closed.
///
fn set_main_window(builder: &Builder) -> Result<(), InterfaceError> {
    let main_window: Window = builder
        .get_object(MAIN_WINDOW)
        .ok_or(InterfaceError::MissingWindow)?;

    main_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    Ok(())
}

/// Sets up the login window.
///
/// This function initializes and configures the login window of the interface.
///
/// # Arguments
///
/// * `builder` - The Builder object used to access UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if there is an issue accessing the login window.
///
/// # Description
///
/// This function sets up the login window by performing the following tasks:
///
/// 1. Retrieves the login window object from the builder.
/// 2. Connects the delete event of the login window to the gtk::main_quit function, ensuring the application terminates when the window is closed.
/// 3. Shows the login window.
///
fn set_login_window(builder: &Builder) -> Result<(), InterfaceError> {
    let login_window: Window = builder
        .get_object(LOGIN_WINDOW)
        .ok_or(InterfaceError::MissingWindow)?;

    login_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    //login_window.show_all();
    Ok(())
}

/// Sets up the loading window.
///
/// This function initializes and configures the loading window of the interface.
///
/// # Arguments
///
/// * `builder` - The Builder object used to access UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if there is an issue accessing the loading window.
///
/// # Description
///
/// This function sets up the loading window by performing the following tasks:
///
/// 1. Retrieves the loading window object from the builder.
/// 2. Connects the delete event of the loading window to the gtk::main_quit function, ensuring the application terminates when the window is closed.
/// 3. Shows the loading window.
///
/// # Note
///
/// This functionality is not be implemented right now but is expected to be set soon
///
pub fn set_loading_window(builder: &Builder) -> Result<(), InterfaceError> {
    let loading_window: Window = builder
        .get_object("loading_window")
        .ok_or(InterfaceError::MissingWindow)?;

    loading_window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    loading_window.show_all();
    Ok(())
}
