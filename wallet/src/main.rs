use glib::MainContext;
use glib::Priority;
use glib::Type;
use gtk::Builder;
use gtk::ListStore;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use wallet::accounts::Accounts;
use wallet::handlers::button_setting::set_buttons;
use wallet::handlers::handler_windows::set_windows;
use wallet::transactions::transaction_view::update_wallet_interface;
use wallet::update_progress::update_progress;
use wallet::update_progress::update_progress_bar_view;
use wallet::update_wallet::update_wallet;
use wallet::wallet_constants::NODE_IP;
use wallet::wallet_constants::NODE_PORT;
fn main() {
    let socket = SocketAddr::from((NODE_IP, NODE_PORT));
    let node: Arc<Mutex<TcpStream>> = match TcpStream::connect(socket) {
        Ok(conexion) => Arc::new(Mutex::new(conexion)),
        Err(_) => {
            println!("Node conection error");
            return;
        }
    };

    let accounts: Arc<Mutex<Accounts>> = Arc::new(Mutex::new(Accounts::new()));

    let (exit_sender, exit_recv): (Sender<bool>, Receiver<bool>) = mpsc::channel();
    let (update_sender, update_recv): (glib::Sender<bool>, glib::Receiver<bool>) =
        MainContext::channel(Priority::default());
    let (progress_sender, progress_recv): (glib::Sender<f64>, glib::Receiver<f64>) =
        MainContext::channel(Priority::default());

    let shared_accounts = accounts.clone();
    let shared_node = node.clone();

    let handle_interface = thread::spawn(move || {
        if let Err(err) = gtk::init() {
            eprintln!("Error inicializando GTK: {}", err);
            return;
        }

        let store = ListStore::new(&[
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
        ]);

        let glade_src = include_str!("../bitcoin_ui.glade");
        let builder = Builder::from_string(glade_src);

        if let Err(err) = set_windows(&builder) {
            println!("{:?}", err);
        };

        if let Err(err) = set_buttons(&builder, shared_accounts.clone(), shared_node, &store) {
            println!("{:?}", err);
        };

        match update_progress_bar_view(&builder, progress_recv) {
            Ok(()) => {}
            Err(err) => {
                println!("{:?}", err);
            }
        }

        if let Err(err) =
            update_wallet_interface(&builder, store, shared_accounts.clone(), update_recv)
        {
            println!("{:?}", err);
        };

        gtk::main();

        if exit_sender.send(true).is_err() {
            println!("Exit error");
        };
    });

    if let Err(err) = update_progress(&node, progress_sender) {
        println!("{:?}", err);
        return;
    }

    if let Err(err) = update_wallet(accounts, node, update_sender, exit_recv) {
        println!("{:?}", err);
    };

    if let Err(err) = handle_interface.join() {
        println!("{:?}", err);
    };
}
