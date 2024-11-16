use wasm_bindgen::JsValue;

use crate::{
    Reflection, WalletError, WalletIcon, WalletResult,
    SOLANA_SIGN_AND_SEND_TRANSACTION_IDENTIFIER, SOLANA_SIGN_IN_IDENTIFIER,
    SOLANA_SIGN_MESSAGE_IDENTIFIER, SOLANA_SIGN_TRANSACTION_IDENTIFIER,
    STANDARD_CONNECT_IDENTIFIER, STANDARD_DISCONNECT_IDENTIFIER, STANDARD_EVENTS_IDENTIFIER,
};

use super::{
    ChainSupport, Cluster, FeatureSupport,
};

/// Interface of a **WalletAccount**, also referred to as an **Account**.
/// An account is a _read-only data object_ that is provided from the Wallet to the app,
/// authorizing the app to use it.
/// The app can use an account to display and query information from a chain.
/// The app can also act using an account by passing it to `features` field of the Wallet.
#[derive(Clone, Default, PartialEq)]
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
    pub(crate) js_value: JsValue,
    // Convinience field, instead of going through the `features` field
    supported_features: FeatureSupport,
    // Convinience field, instead of iteration through the `chains` field
    supported_chains: ChainSupport,
}

impl core::fmt::Debug for WalletAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletAccount")
            .field("address", &self.address)
            .field("public_key", &self.public_key)
            .field("chains", &self.chains)
            .field("features", &self.features)
            .field("label", &self.label)
            .field("icon", &self.icon)
            .finish()
    }
}

impl WalletAccount {
    pub fn parse(reflection: Reflection) -> WalletResult<Self> {
        let address = reflection.string("address")?;
        let public_key = reflection.byte32array("publicKey")?;
        let chains = reflection.vec_string("chains")?;
        let features = reflection.vec_string("features")?;

        let mut supported_chains = ChainSupport::default();

        chains.iter().try_for_each(|chain| {
            if chain.as_str() == Cluster::MainNet.chain() {
                supported_chains.mainnet = true;
            } else if chain.as_str() == Cluster::DevNet.chain() {
                supported_chains.devnet = true;
            } else if chain.as_str() == Cluster::TestNet.chain() {
                supported_chains.testnet = true;
            } else if chain.as_str() == Cluster::LocalNet.chain() {
                supported_chains.localnet = true;
            } else {
                return Err(WalletError::UnsupportedChain(chain.to_owned()));
            }

            Ok(())
        })?;

        let mut supported_features = FeatureSupport::default();

        features.iter().try_for_each(|feature| {
            if feature.as_str() == STANDARD_CONNECT_IDENTIFIER {
                supported_features.connect = true;
            } else if feature.as_str() == STANDARD_DISCONNECT_IDENTIFIER {
                supported_features.disconnect = true;
            } else if feature.as_str() == STANDARD_EVENTS_IDENTIFIER {
                supported_features.events = true;
            } else if feature.as_str() == SOLANA_SIGN_IN_IDENTIFIER {
                supported_features.sign_in = true;
            } else if feature.as_str() == SOLANA_SIGN_AND_SEND_TRANSACTION_IDENTIFIER {
                supported_features.sign_and_send_tx = true;
            } else if feature.as_str() == SOLANA_SIGN_TRANSACTION_IDENTIFIER {
                supported_features.sign_tx = true;
            } else if feature.as_str() == SOLANA_SIGN_MESSAGE_IDENTIFIER {
                supported_features.sign_message = true;
            } else {
                return Err(WalletError::UnsupportedWalletFeature(feature.to_owned()));
            }

            Ok(())
        })?;

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
            supported_chains,
            supported_features,
            js_value: reflection.take(),
        })
    }

    pub fn mainnet(&self) -> bool {
        self.supported_chains.mainnet
    }

    pub fn devnet(&self) -> bool {
        self.supported_chains.devnet
    }

    pub fn testnet(&self) -> bool {
        self.supported_chains.testnet
    }

    pub fn localnet(&self) -> bool {
        self.supported_chains.localnet
    }

    pub fn icon(&self) -> Option<&WalletIcon> {
        self.icon.as_ref()
    }

    pub fn standard_connect(&self) -> bool {
        self.supported_features.connect
    }

    pub fn standard_disconnect(&self) -> bool {
        self.supported_features.disconnect
    }

    pub fn standard_events(&self) -> bool {
        self.supported_features.events
    }

    pub fn solana_signin(&self) -> bool {
        self.supported_features.sign_in
    }

    pub fn solana_sign_message(&self) -> bool {
        self.supported_features.sign_message
    }

    pub fn solana_sign_and_send_transaction(&self) -> bool {
        self.supported_features.sign_and_send_tx
    }

    pub fn solana_sign_transaction(&self) -> bool {
        self.supported_features.sign_tx
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
            features: &value.features,
            label: value.label.as_ref(),
            icon: value.icon.as_ref(),
        }
    }
}
