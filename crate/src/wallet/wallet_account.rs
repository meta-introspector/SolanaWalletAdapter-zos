use wasm_bindgen::JsValue;

use crate::{Reflection, WalletError, WalletIcon, WalletResult};

/// Interface of a **WalletAccount**, also referred to as an **Account**.
/// An account is a _read-only data object_ that is provided from the Wallet to the app,
/// authorizing the app to use it.
/// The app can use an account to display and query information from a chain.
/// The app can also act using an account by passing it to `features` field of the Wallet.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WalletAccount {
    /// Address of the account, corresponding with a public key.
    pub address: String,
    /// Public key of the account, corresponding with a secret key to use.
    pub public_key: [u8; 32],
    /// Chains supported by the account.
    /// This must be a subset of the {@link Wallet.chains | chains} of the Wallet.
    pub chains: Vec<String>,
    /// Feature names supported by the account.
    /// This must be a subset of the names of {@link Wallet.features | features} of the Wallet.
    pub features: Vec<String>,
    /// Optional user-friendly descriptive label or name for the account. This may be displayed by the app.
    pub label: Option<String>,
    /// Optional user-friendly icon for the account. This may be displayed by the app. */
    pub icon: Option<WalletIcon>,
    /// The Javascript Value Representation of a wallet,
    /// this mostly used internally in the wallet adapter
    pub js_value: JsValue,
}

impl WalletAccount {
    pub fn parse(reflection: Reflection) -> WalletResult<Self> {
        let address = reflection.string("address")?;
        let public_key = reflection.byte32array("publicKey")?;
        let chains = reflection.vec_string("chains")?;
        let features = reflection.vec_string("features")?;
        let icon = WalletIcon::from_jsvalue(&reflection)?;

        let label = match reflection.string("label") {
            Ok(value) => Some(value),
            Err(error) => {
                if error == WalletError::JsValueNotString {
                    Option::None
                } else {
                    return Err(error);
                }
            }
        };

        Ok(Self {
            address,
            public_key,
            chains,
            features,
            label,
            icon,
            js_value: reflection.take(),
        })
    }
}

impl PartialOrd for WalletAccount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let inner_self: InnerWalletAccount = self.into();
        let inner_other: InnerWalletAccount = other.into();

        Some(inner_self.cmp(&inner_other))
    }
}

impl Ord for WalletAccount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let inner_self: InnerWalletAccount = self.into();
        let inner_other: InnerWalletAccount = other.into();

        inner_self.cmp(&inner_other)
    }
}

impl core::cmp::Eq for WalletAccount {}

impl core::hash::Hash for WalletAccount {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let inner_self: InnerWalletAccount = self.into();

        inner_self.hash(state);
    }
}

// Reduce Eq, PartialEq, Ord, Hash work
#[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
struct InnerWalletAccount<'a> {
    pub address: &'a str,
    pub public_key: &'a [u8; 32],
    pub chains: &'a [String],
    pub features: &'a [String],
    pub label: Option<&'a String>,
    pub icon: Option<&'a WalletIcon>,
}

impl<'a> From<&'a WalletAccount> for InnerWalletAccount<'a> {
    fn from(value: &'a WalletAccount) -> Self {
        Self {
            address: value.address.as_str(),
            public_key: &value.public_key,
            chains: &value.chains.as_slice(),
            features: &value.features.as_slice(),
            label: value.label.as_ref(),
            icon: value.icon.as_ref(),
        }
    }
}
