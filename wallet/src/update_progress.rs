use std::{
    io::Write,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use glib::{clone, Continue};
use gtk::{prelude::BuilderExtManual, Builder, ProgressBar, ProgressBarExt, WidgetExt, Window};
use node::wallet_utils::{progress::Progress, wallet_utils_constants::GET_PROGRESS_COMMAND};

use crate::{handlers::handler_constants::*, interface_error::InterfaceError};

pub fn update_progress(
    node: &Arc<Mutex<TcpStream>>,
    progress_sender: glib::Sender<f64>,
) -> Result<(), InterfaceError> {
    loop {
        let mut locked_node = node.lock().map_err(|_| InterfaceError::LockNode)?;
        locked_node
            .write_all(GET_PROGRESS_COMMAND.as_bytes())
            .map_err(|_| InterfaceError::Write)?;

        let progress = Progress::from_bytes(&mut *locked_node).map_err(|_| InterfaceError::Read)?;

        if progress_sender.send(progress.get_progress()).is_err() {
            return Err(InterfaceError::Send);
        };

        if progress.get_progress() == 1.0 {
            break;
        }
    }

    Ok(())
}

pub fn update_progress_bar_view(
    builder: &Builder,
    progress_recv: glib::Receiver<f64>,
) -> Result<(), InterfaceError> {
    let progress_bar: ProgressBar = builder
        .get_object(PROGRESS_BAR)
        .ok_or(InterfaceError::MissingProgressBar)?;
    let login_window: Window = builder
        .get_object(LOGIN_WINDOW)
        .ok_or(InterfaceError::MissingWindow)?;
    let loading_window: Window = builder
        .get_object(LOADING_WINDOW)
        .ok_or(InterfaceError::MissingWindow)?;

    progress_recv.attach(
        None,
        clone!(@weak progress_bar => @default-return Continue(false),
            move |progress| {
                progress_bar.set_fraction(progress);
                if progress == 1.0 {
                    loading_window.hide();
                    login_window.show();
                }
                Continue(true)
            }
        ),
    );

    Ok(())
}
