#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum ProviderMethods {
    Connect,
    Disconnect,
    SignAndSendTransaction,
    SignAllTransactions,
    SignTransaction,
    SignMessage,
}
