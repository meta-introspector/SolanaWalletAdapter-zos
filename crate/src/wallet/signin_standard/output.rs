use crate::WalletAccount;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct SignInOutput {
    pub account: WalletAccount,
    pub message: String,
    pub signature: [u8; 64],
    pub public_key: [u8; 32],
}

impl SignInOutput {
    pub fn signature(&self) -> String {
        bs58::encode(&self.signature).into_string()
    }

    pub fn public_key(&self) -> String {
        bs58::encode(&self.public_key).into_string()
    }

    pub fn address(&self) -> &str {
        self.account.address.as_str()
    }
}
