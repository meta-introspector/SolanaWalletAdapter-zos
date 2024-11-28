use ed25519_dalek::Signature;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Reflect;

use crate::{
    Cluster, Features, Reflection, SemverVersion, WalletAccount, WalletError, WalletIcon,
    WalletResult,
};

use super::{
    ChainSupport, FeatureSupport, SendOptions, SignInOutput, SignedMessageOutput, SigninInput,
};

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wallet {
    name: String,
    version: SemverVersion,
    icon: Option<WalletIcon>,
    accounts: Vec<WalletAccount>,
    chains: Vec<Cluster>,
    pub(crate) features: Features,
    // Convinience field, instead of going through the `features` field
    supported_features: FeatureSupport,
    // Convinience field, instead of iteration through the `chains` field
    supported_chains: ChainSupport,
}

impl Wallet {
    pub async fn connect(&self) -> WalletResult<WalletAccount> {
        self.features.connect.call_connect().await
    }

    pub async fn disconnect(&self) -> WalletResult<()> {
        self.features.disconnect.call_disconnect().await
    }

    pub async fn sign_in(
        &self,
        signin_input: &SigninInput,
        public_key: [u8; 32],
    ) -> WalletResult<SignInOutput> {
        if let Some(fn_exists) = self.features.sign_in.as_ref() {
            fn_exists.call_signin(signin_input, public_key).await
        } else {
            Err(WalletError::MissingSignInFunction)
        }
    }

    pub async fn sign_message<'a>(
        &self,
        message: &'a [u8],
        account: &WalletAccount,
    ) -> WalletResult<SignedMessageOutput<'a>> {
        self.features
            .sign_message
            .call_sign_message(account, message)
            .await
    }

    pub async fn sign_transaction(
        &self,
        transaction_bytes: &[u8],
        cluster: Option<Cluster>,
        account: &WalletAccount,
    ) -> WalletResult<Vec<Vec<u8>>> {
        self.features
            .sign_tx
            .call_sign_tx(account, transaction_bytes, cluster)
            .await
    }

    pub async fn sign_and_send_transaction(
        &self,
        transaction_bytes: &[u8],
        cluster: Cluster,
        options: SendOptions,
        account: &WalletAccount,
    ) -> WalletResult<Signature> {
        self.features
            .sign_and_send_tx
            .call_sign_and_send_transaction(account, transaction_bytes, cluster, options)
            .await
    }

    pub async fn events(&self) -> WalletResult<()> {
        self.features
            .events
            .as_ref()
            .ok_or(WalletError::MissingStandardEventsFunction)?
            .call_standard_event()
            .await
    }

    pub fn from_jsvalue(value: JsValue) -> WalletResult<Self> {
        let reflection = Reflection::new(value)?;

        let mut supported_chains = ChainSupport::default();

        let chains_raw = reflection.vec_string_and_filter("chains", "solana:")?;
        let chains = chains_raw
            .into_iter()
            .map(|chain_raw| {
                let cluster = chain_raw.as_str().try_into();
                if let Ok(cluster_inner) = &cluster {
                    if cluster_inner == &Cluster::MainNet {
                        supported_chains.mainnet = true;
                    } else if cluster_inner == &Cluster::DevNet {
                        supported_chains.devnet = true;
                    } else if cluster_inner == &Cluster::TestNet {
                        supported_chains.testnet = true;
                    } else if cluster_inner == &Cluster::LocalNet {
                        supported_chains.localnet = true;
                    } else {
                    }
                }

                cluster
            })
            .collect::<WalletResult<Vec<Cluster>>>()?;

        let name = reflection.string("name")?;
        let version = SemverVersion::parse(&reflection.string("version")?)?;
        let icon = WalletIcon::from_jsvalue(&reflection)?;
        let accounts = Self::get_accounts(&reflection, "accounts")?;
        let (features, supported_features) = Features::parse(&reflection)?;

        Ok(Wallet {
            name,
            version,
            icon,
            accounts,
            chains,
            features,
            supported_features,
            supported_chains,
        })
    }

    fn get_accounts(reflection: &Reflection, key: &str) -> WalletResult<Vec<WalletAccount>> {
        let accounts_raw = Reflect::get(reflection.get_inner(), &key.into())?;
        Reflection::check_is_undefined(&accounts_raw)?;

        if !accounts_raw.is_array() {
            return Err(WalletError::ExpectedArray(
                "Reflection for `accounts` key".to_string(),
            ));
        }

        let accounts_array: js_sys::Array = accounts_raw.unchecked_into();

        accounts_array
            .iter()
            .map(|account| WalletAccount::parse(Reflection::new(account)?))
            .collect::<WalletResult<Vec<WalletAccount>>>()
    }

    pub fn features(&self) -> &Features {
        &self.features
    }

    pub fn accounts(&self) -> &[WalletAccount] {
        &self.accounts
    }

    pub fn chains(&self) -> &[Cluster] {
        &self.chains
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

    pub fn icon(&self) -> Option<&WalletIcon> {
        self.icon.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &SemverVersion {
        &self.version
    }
}

impl core::fmt::Debug for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chains = self
            .chains
            .iter()
            .map(|cluster| cluster.chain())
            .collect::<Vec<&str>>();

        f.debug_struct("Wallet")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("icon", &self.icon)
            .field("accounts", &self.accounts)
            .field("chains", &chains)
            .field("features", &self.features)
            .finish()
    }
}
