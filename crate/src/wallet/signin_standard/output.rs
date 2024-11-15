use crate::WalletAccount;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SignInOutput {
    pub account: WalletAccount,
    pub message: String,
    pub signature: [u8; 64],
    pub public_key: [u8; 32],
}
