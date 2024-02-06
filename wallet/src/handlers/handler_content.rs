use gtk::prelude::*;
use gtk::{Box, Entry};

use super::handler_constants::*;

/// Replaces the content of a GTK container with a new widget.
///
/// # Arguments
///
/// * `container` - A reference to the GTK container (`Box`) whose content will be replaced.
/// * `widget` - A reference to the new GTK widget that will replace the existing content.
///
/// # Description
///
/// This function replaces the current content of a GTK container with a new widget. It removes the last widget from the container (if any) and adds the new widget in its place. The new widget is then expanded horizontally and vertically within the container.
///
pub fn replace_content(container: &Box, widget: &gtk::Widget) {
    let children = container.get_children();
    if let Some(last_widget) = children.last() {
        container.remove(last_widget);
        container.add(widget);
        widget.set_hexpand(true);
        widget.set_vexpand(true);
    }
}

/// Clears the text in the provided GTK entry widget.
///
/// # Arguments
///
/// * `entry` - A reference to the GTK `Entry` widget.
///
/// # Description
///
/// This function clears the text in the provided GTK `Entry` widget by setting its text content to an empty string.
///
pub fn clean_entry(entry: &Entry) {
    entry.set_text(EMPTY);
}
