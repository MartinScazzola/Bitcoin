use super::{handler_constants::*, handler_content::replace_content};
use crate::interface_error::InterfaceError;
use gtk::prelude::*;
use gtk::{Box, Builder, Button};

/// Sets up the overview button in the user interface.
///
/// # Arguments
///
/// * `builder` - A reference to the GTK GUI builder.
///
/// # Returns
///
/// Returns `Result<(), InterfaceError>` indicating the success or failure of setting up the overview button. An `Ok` value is returned upon successful setup, while an `Err` value of `InterfaceError` type is returned in case of any errors.
///
/// # Description
///
/// This function sets up the overview button in the user interface. It retrieves the overview button, overview box, and content box from the GTK GUI builder. When the overview button is clicked, it replaces the content of the content box with the overview box and shows all the widgets within the overview box.
///
pub fn set_overview_button(builder: &Builder) -> Result<(), InterfaceError> {
    let overview_button: Button = builder
        .get_object(OVERVIEW_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let overview_box = builder
        .get_object(OVERVIEW_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let content_box: Box = builder
        .get_object(CONTENT_BOX)
        .ok_or(InterfaceError::MissingBox)?;

    overview_button.connect_clicked(move |_| {
        replace_content(&content_box, &overview_box);
        overview_box.show_all();
    });
    Ok(())
}
