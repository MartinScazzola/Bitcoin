use gtk::{
    BoxExt, CellLayoutExt, CellRendererText, EditableSignals, Entry, EntryCompletion,
    EntryCompletionExt, EntryExt, ListStore, TreeModelExt, TreeModelFilter, TreeModelFilterExt,
    TreeModelSort, TreeView, TreeViewColumn, TreeViewColumnExt, TreeViewExt, TreeViewGridLines,
};
use std::{cell::RefCell, rc::Rc};

use super::views_constants::*;

pub fn create_transaction_tree_view(store: &ListStore) -> gtk::Box {
    let transaction_tree_view = TreeView::new();

    // Set the model for the TreeView
    transaction_tree_view.set_model(Some(store));
    transaction_tree_view.set_headers_clickable(true);
    transaction_tree_view.set_reorderable(false);
    transaction_tree_view.set_enable_search(false);
    transaction_tree_view.set_show_expanders(true);
    transaction_tree_view.set_grid_lines(TreeViewGridLines::Vertical);
    transaction_tree_view.set_grid_lines(TreeViewGridLines::Horizontal);

    // Create columns for the TreeView
    for (column_index, column_title) in [STATE, DATE, TYPE, LABEL, AMOUNT].iter().enumerate() {
        let renderer = CellRendererText::new();
        let column = TreeViewColumn::new();

        column.set_title(column_title);
        column.pack_start(&renderer, true);
        column.add_attribute(&renderer, TEXT, column_index as i32);
        column.set_expand(true);
        column.set_resizable(true);
        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        column.set_sort_column_id(column_index as i32);

        transaction_tree_view.append_column(&column);
    }

    // Create a TreeModelFilter as a filter for the TreeView
    let filter = TreeModelFilter::new(store, None);

    // Create a mutable reference to the Entry widget
    let entry = Rc::new(RefCell::new(Entry::new()));
    let entry_clone = entry.clone();

    // Set the visible function for the TreeModelFilter
    filter.set_visible_func(move |model, iter| {
        // Get the filter text entered by the user
        let filter_text = entry_clone.borrow().get_text().to_string();

        // Iterate over all columns and check if any column contains the filter text
        for column_index in 0..model.get_n_columns() {
            let value = model.get_value(iter, column_index);
            if let Ok(Some(column_text)) = value.get::<String>() {
                if column_text.contains(&filter_text) {
                    return true;
                }
            }
        }
        false
    });

    // Connect the filter to the TreeView
    transaction_tree_view.set_model(Some(&TreeModelSort::new(&filter)));

    entry.borrow().set_placeholder_text(Some(SEARCH_TEXT_HELP));
    // Connect the changed signal of the Entry widget
    entry.borrow().connect_changed(move |_| {
        filter.refilter();
    });

    // Create an Entry widget for entering the filter text
    let completion = EntryCompletion::new();
    completion.set_model(Some(store));
    completion.set_text_column(0);
    entry.borrow().set_completion(Some(&completion));
    let entry_widget = entry.borrow().clone();

    // Create a VBox to hold the TreeView and Entry widget
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&entry_widget, false, false, 0);
    vbox.pack_start(&transaction_tree_view, true, true, 0);

    vbox
}
