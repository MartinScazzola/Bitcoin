use gtk::prelude::*;
use gtk::{Box, Builder, Button, Clipboard, Label};

use crate::interface_error::InterfaceError;

use super::{
    handler_constants::*, handler_content::replace_content, handler_styles::set_button_style,
};

/// Sets up the receive button in the user interface.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK GUI builder.
///
/// # Returns
///
/// Returns `Result<(), InterfaceError>` indicating the success or failure of setting up the receive button. An `Ok` value is returned upon successful setup, while an `Err` value of `InterfaceError` type is returned in case of any errors.
///
/// # Description
///
/// This function sets up the receive button in the user interface. It retrieves the necessary GTK objects from the builder, such as the receive button, receive frame, content box, and copy button. When the receive button is clicked, it sets the button style for the copy button, shows the copy button, replaces the content of the content box with the receive frame, and shows all the widgets within the receive frame.
///
pub fn set_receive_button(builder: &Builder) -> Result<(), InterfaceError> {
    let receive_button: Button = builder
        .get_object(RECEIVE_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let receive_box = builder
        .get_object(RECEIVE_BOX)
        .ok_or(InterfaceError::MissingFrame)?;
    let content_box: Box = builder
        .get_object(CONTENT_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let copy_button: Button = builder
        .get_object(COPY_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;

    receive_button.connect_clicked(move |_| {
        if let Err(err) = set_button_style(
            &copy_button,
            COPY_BUTTON_STYLE1,
            COPY_BUTTON_STYLE2,
            COPY_BUTTON_STYLE3,
        ) {
            println!("{:?}", err);
        };
        copy_button.show_all();
        replace_content(&content_box, &receive_box);
        receive_box.show_all();
    });
    Ok(())
}

/// The copy button allows the user to copy the public key text to the clipboard.
/// When clicked, the public key text is copied to the clipboard, and the button style is updated to indicate a successful copy.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK builder used to access UI elements.
///
/// # Returns
///
/// Returns `Ok(())` if the copy button setup is successful, or an `InterfaceError` if any required UI element is missing.
///
/// # Description
///
/// This function sets up the copy button by connecting its `clicked` signal to a closure that performs the copy functionality.
/// It retrieves the copy button and the label containing the public key text from the builder. When the copy button is clicked,
/// the closure is executed. It gets the default clipboard and sets the public key text as the clipboard content. Then, it calls
/// the `set_button_style` function to update the button style to indicate a successful copy. If any error occurs during this
/// process, it prints the error message to the console.
pub fn set_copy_button(builder: &Builder) -> Result<(), InterfaceError> {
    let copy_button: Button = builder
        .get_object(COPY_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let public_key_to_copy: Label = builder
        .get_object(SHARED_PUBKEY)
        .ok_or(InterfaceError::MissingLabel)?;

    copy_button.connect_clicked(move |copy_button| {
        if let Some(clipboard) = Clipboard::get_default(&copy_button.get_display()) {
            clipboard.set_text(public_key_to_copy.get_text().as_str());
        }
        if let Err(err) = set_button_style(
            copy_button,
            COPY_BUTTON_STYLE2,
            COPY_BUTTON_STYLE1,
            COPY_BUTTON_STYLE4,
        ) {
            println!("{:?}", err);
        };
    });

    Ok(())
}
