use gtk::{CssProvider, Fixed, FixedExt, Image, Label, StyleContextExt, WidgetExt};

use crate::{
    handlers::handler_constants::*,
    interface_error::InterfaceError,
    transactions::create_transactios_constants::{CONFIRMED, RECEIVED},
};

use gtk::CssProviderExt;

pub fn create_recent_transaction_view(
    state: &str,
    date: &str,
    tx_type: &str,
    label: Result<std::string::String, InterfaceError>,
    amount: f64,
) -> Result<Fixed, InterfaceError> {
    let recent_transaction_fixed: Fixed = Fixed::new();

    let state_label: Label = Label::new(Some(state));
    state_label.set_size_request(90, 25);

    let date_label: Label = Label::new(Some(date));
    date_label.set_size_request(130, 25);

    let tx_type_label: Label = Label::new(Some(tx_type));
    tx_type_label.set_size_request(90, 25);

    let tx_label = label?;
    let label_label: Label = Label::new(Some(tx_label.as_str()));
    label_label.set_size_request(330, 25);

    let amount_string = amount.to_string();
    let minus = if tx_type == RECEIVED { "" } else { "-" };

    let amount_str = format!("{}{}", minus, amount_string);

    let amount_label: Label = Label::new(Some(amount_str.as_str()));
    amount_label.set_size_request(120, 25);

    let image = if state == CONFIRMED {
        Image::from_file("images/confirmed.png")
    } else {
        Image::from_file("images/unconfirmed.png")
    };

    // Create a CSS provider and load CSS data to define the color
    let css_provider = CssProvider::new();

    css_provider.load_from_path(STYLE_PATH)?;

    // Date Label
    let date_context = date_label.get_style_context();
    date_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    date_context.add_class("date-label");

    // State Label
    let state_context = state_label.get_style_context();
    state_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    state_context.add_class("ubuntu-mono-label");

    // Tx_type Label
    let tx_type_context = tx_type_label.get_style_context();
    tx_type_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    tx_type_context.add_class("ubuntu-mono-label");

    // Label Label
    let label_context = label_label.get_style_context();
    label_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    label_context.add_class("ubuntu-mono-label");

    // Amount Label
    let amount_context = amount_label.get_style_context();
    amount_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    amount_context.add_class("ubuntu-mono-label");

    recent_transaction_fixed.put(&image, 0, 10);
    recent_transaction_fixed.put(&date_label, 60, 0);
    recent_transaction_fixed.put(&tx_type_label, 60, 25);
    recent_transaction_fixed.put(&state_label, 150, 25);
    recent_transaction_fixed.put(&amount_label, 275, 25);
    recent_transaction_fixed.put(&label_label, 60, 50);

    Ok(recent_transaction_fixed)
}
