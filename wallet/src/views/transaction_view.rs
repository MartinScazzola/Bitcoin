use super::views_constants::*;
use crate::interface_error::InterfaceError;
use gtk::StyleContextExt;
use gtk::{
    Adjustment, Align, Box, Button, ButtonExt, ContainerExt, CssProviderExt, Entry, EntryExt,
    Fixed, FixedExt, IconSize, Label, Orientation, Separator, SpinButton, WidgetExt,
};

pub fn create_transaction_view(transaction_box: Box) -> Result<Fixed, InterfaceError> {
    let transaction_fixed: Fixed = Fixed::new();

    transaction_fixed.set_halign(Align::Center);

    let pay_to_label: Label = Label::new(Some(PAY_TO_LABEL));
    pay_to_label.set_size_request(80, 40);

    let amount_label: Label = Label::new(Some(AMOUNT_LABEL));
    amount_label.set_size_request(80, 40);

    let btc_label: Label = Label::new(Some(BTC_LABEL));
    btc_label.set_size_request(80, 40);

    // Create a CSS provider and load CSS data to define the color
    let css_provider = gtk::CssProvider::new();
    let css_data = CSS_ORANGE_LABEL_CLASS;
    css_provider.load_from_data(css_data.as_bytes())?;

    // Add the CSS provider to the style context
    let style_context = pay_to_label.get_style_context();
    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    pay_to_label.get_style_context().add_class(CSS_ORANGE_LABEL);

    let style_context = amount_label.get_style_context();
    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    amount_label.get_style_context().add_class(CSS_ORANGE_LABEL);

    let style_context = btc_label.get_style_context();
    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    btc_label.get_style_context().add_class(CSS_ORANGE_LABEL);

    transaction_fixed.put(&pay_to_label, 0, 0);
    transaction_fixed.put(&amount_label, 0, 40);
    transaction_fixed.put(&btc_label, 290, 38);

    let pay_to_entry = Entry::new();
    pay_to_entry.set_size_request(650, 34);
    pay_to_entry.set_placeholder_text(Some(CREATE_TRANSACTION_PAY_TO_ENTRY));

    transaction_fixed.put(&pay_to_entry, 75, 0);

    let adjustment = Adjustment::new(0.0, 0.0, 100.0, 0.001, 10.0, 0.0);
    let spin_button = SpinButton::new(Some(&adjustment), 0.0, 7);
    spin_button.set_size_request(220, 34);
    transaction_fixed.put(&spin_button, 75, 40);

    let clear_output_button = Button::from_icon_name(Some(ERROR_ICON), IconSize::Button);

    let shared_transaction_fixed = transaction_fixed.clone();
    clear_output_button.connect_clicked(move |_| {
        transaction_box.remove(&shared_transaction_fixed);
    });

    transaction_fixed.put(&clear_output_button, 685, 40);

    let separator = Separator::new(Orientation::Horizontal);
    separator.set_size_request(725, 25);
    transaction_fixed.put(&separator, 0, 80);

    Ok(transaction_fixed)
}
