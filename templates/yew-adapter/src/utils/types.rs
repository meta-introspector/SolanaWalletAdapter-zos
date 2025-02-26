use std::fmt::Display;

use wallet_adapter::WalletResult;
use wasm_bindgen_futures::JsFuture;
use web_sys::Window;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ClusterNetState {
    #[default]
    Success,
    Waiting,
    Failure,
}

impl Display for ClusterNetState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct NotificationInfo {
    key: u32,
    secs: u32,
    message: String,
}

impl NotificationInfo {
    pub fn new(message: impl core::fmt::Display) -> Self {
        let key = fastrand::u32(..);

        Self {
            key,
            secs: 2,
            message: message.to_string(),
        }
    }

    /// Sets default seconds to 5
    pub fn error(message: impl core::fmt::Display) -> Self {
        Self::new(message).set_secs(15)
    }

    pub fn set_secs(mut self, secs: u32) -> Self {
        self.secs = secs;

        self
    }

    pub fn key(&self) -> u32 {
        self.key
    }

    pub fn secs(&self) -> u32 {
        self.secs
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

pub async fn copied_address(address: &str, window: &Window) -> WalletResult<()> {
    let pending: JsFuture = window.navigator().clipboard().write_text(address).into();

    pending.await?;

    Ok(())
}

// #[derive(Debug, Default, PartialEq, Clone)]
// pub struct AccountState {
//     pub balance: String,
//     pub token_accounts: Vec<TokenAccountResponse>,
//     pub transactions: Vec<SignaturesResponse>,
// }

// impl AccountState {
//     pub fn token_accounts_is_empty(&self) -> bool {
//         self.token_accounts.is_empty()
//     }

//     pub fn transactions_is_empty(&self) -> bool {
//         self.token_accounts.is_empty()
//     }

//     pub fn token_accounts(&self) -> &[TokenAccountResponse] {
//         self.token_accounts.as_slice()
//     }

//     pub fn transactions(&self) -> &[SignaturesResponse] {
//         self.transactions.as_slice()
//     }
// }
