use crate::{
    interface_error::InterfaceError, views::transaction_tree_view::create_transaction_tree_view,
};
use gtk::prelude::*;
use gtk::{Box, Builder, Button, ListStore, ScrolledWindow, Widget};

use super::{handler_constants::*, handler_content::replace_content};

/// Sets the functionality of the transactions button.
///
/// # Arguments
///
/// * `builder` - A reference to the builder object.
/// * `store` - A reference to the list store.
///
/// # Errors
///
/// Returns an `InterfaceError` if the button or box objects are missing.
pub fn set_transactions_button(builder: &Builder, store: &ListStore) -> Result<(), InterfaceError> {
    let transactions_button: Button = builder
        .get_object(TRANSACTIONS_BUTTON)
        .ok_or(InterfaceError::MissingButton)?;
    let transactions_tree_view: Widget = create_transaction_tree_view(store).upcast();
    let content_box: Box = builder
        .get_object(CONTENT_BOX)
        .ok_or(InterfaceError::MissingBox)?;
    let scrolled_window = ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
    scrolled_window.add(&transactions_tree_view);

    let scrolled_window_widget = scrolled_window.upcast();

    transactions_button.connect_clicked(move |_| {
        replace_content(&content_box, &scrolled_window_widget);
        scrolled_window_widget.show_all();
    });
    Ok(())
}
