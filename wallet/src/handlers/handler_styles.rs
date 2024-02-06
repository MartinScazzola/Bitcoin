use super::handler_constants::STYLE_PATH;
use crate::interface_error::InterfaceError;
use gtk::StyleContextExt;
use gtk::{Button, ButtonExt, CssProviderExt, WidgetExt};

/// Sets the style and label for a button.
///
/// This function applies a new CSS class and updates the label of a button.
///
/// # Arguments
///
/// * `button` - The Button object to modify.
/// * `new_class` - The name of the new CSS class to apply to the button.
/// * `old_class` - The name of the old CSS class to remove from the button.
/// * `label` - The new label for the button.
///
/// # Returns
///
/// Returns `Ok(())` if the function executes successfully, or an `InterfaceError` if there is an issue loading the CSS provider.
///
/// # Description
///
/// This function sets the style and label for a button by performing the following steps:
///
/// 1. Creates a new `CssProvider` object and loads the CSS file from the specified path.
/// 2. Retrieves the style context of the button.
/// 3. Removes the old CSS class from the button's style context.
/// 4. Adds the new CSS class to the button's style context.
/// 5. Sets the label of the button to the specified value.
/// 6. Returns `Ok(())` if the function executes successfully.
///
pub fn set_button_style(
    button: &Button,
    new_class: &str,
    old_class: &str,
    label: &str,
) -> Result<(), InterfaceError> {
    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_path(STYLE_PATH)?;

    let style_context = button.get_style_context();

    style_context.remove_class(old_class);

    style_context.add_provider(&css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

    style_context.add_class(new_class);

    button.set_label(label);

    Ok(())
}
