#[derive(Debug)]
pub enum InterfaceError {
    MissingButton,
    MissingLabel,
    MissingEntry,
    MissingComboBox,
    MissingBox,
    MissingFrame,
    MissingWindow,
    MissingAccount,
    MissingEntryText,
    MissingSpinButton,
    MissingDialog,
    MissingImage,
    InvalidWidgetType,
    LoadCssFile,
    TxCreate,
    LockAccounts,
    LockNode,
    Write,
    Read,
    Send,
    DecodeHex,
    InvalidResponse,
    MissingProgressBar,
    MissingSpinner,
    UpdateRecentTransactions,
}

impl From<glib::Error> for InterfaceError {
    fn from(_: glib::Error) -> InterfaceError {
        InterfaceError::LoadCssFile
    }
}
