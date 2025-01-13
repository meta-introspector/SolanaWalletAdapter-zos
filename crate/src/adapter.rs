use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    rc::Rc,
};

use async_channel::bounded;
use ed25519_dalek::Signature;
use web_sys::{js_sys::Object, Document, Window};

use crate::{
    events::InitEvents, Cluster, SendOptions, SignInOutput, SignedMessageOutput, SigninInput,
    Wallet, WalletAccount, WalletError, WalletEvent, WalletEventReceiver, WalletResult,
    WalletStorage,
};

/// Containsthe connected wallet and account.
/// Containing them in the same structs allows passing of this type
/// by containing it in types like [Rc] and [RefCell] when moving the type
/// out of it's scope like in background tasks or async functions *`async move`).
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionInfo {
    wallet: Option<Wallet>,
    account: Option<WalletAccount>,
}

impl ConnectionInfo {
    /// Create a default [ConnectionInfo]
    pub fn new() -> Self {
        ConnectionInfo::default()
    }

    /// Set the connected wallet
    pub fn set_wallet(&mut self, wallet: Wallet) -> &mut Self {
        self.wallet.replace(wallet);

        self
    }

    /// Set the connected account
    pub fn set_account(&mut self, account: WalletAccount) -> &mut Self {
        self.account.replace(account);

        self
    }

    /// Send a disconnect request to the browser wallet
    pub async fn disconnect(&mut self) -> WalletResult<()> {
        if let Some(wallet) = self.wallet.take() {
            wallet.disconnect().await?;
            self.set_disconnected();

            Ok(())
        } else {
            Err(WalletError::WalletNotFound)
        }
    }

    /// Send a connect request to the browser wallet
    pub async fn connect(wallet: Wallet) -> WalletResult<Self> {
        let wallet_account = wallet.features.connect.call_connect().await?;

        let mut connection_info = Self::new();
        connection_info
            .set_account(wallet_account)
            .set_wallet(wallet);

        Ok(connection_info)
    }

    /// Set the disconnected account
    pub fn set_disconnected(&mut self) -> &mut Self {
        self.wallet.take();
        self.account.take();

        self
    }

    /// Get the connected [wallet](Wallet)
    pub fn connected_wallet(&self) -> WalletResult<&Wallet> {
        self.wallet.as_ref().ok_or(WalletError::WalletNotFound)
    }

    /// Get the connected [account](WalletAccount)
    pub fn connected_account(&self) -> WalletResult<&WalletAccount> {
        self.account.as_ref().ok_or(WalletError::WalletNotFound)
    }

    /// Get the connected [wallet](Wallet) but return an [Option]
    /// to show the wallet exists instead of a [WalletResult]
    pub fn connected_wallet_raw(&self) -> Option<&Wallet> {
        self.wallet.as_ref()
    }

    /// Get the connected [account](WalletAccount)
    /// but return an [Option] to show the account exists instead of a [WalletResult]
    pub fn connected_account_raw(&self) -> Option<&WalletAccount> {
        self.account.as_ref()
    }
}

pub(crate) type ConnectionInfoInner = Rc<RefCell<ConnectionInfo>>;

/// Operations on a browser window.
/// `Window` and `Document` object must be present otherwise
/// an error is thrown.
#[derive(Debug, Clone)]
pub struct WalletAdapter {
    window: Window,
    document: Document,
    storage: WalletStorage,
    connection_info: ConnectionInfoInner,
    wallet_events: WalletEventReceiver,
}

impl WalletAdapter {
    /// Get the `Window` and `Document` object in the current browser window,
    /// initialize the `AppReady` and `Register` events of the wallet standard
    /// and creates a bounded channel with capacity default of 10 messages before capcity is filled.
    /// Use [WalletAdapter::init_with_channel_capacity] to initialize with a desired channel capacity.
    pub fn init() -> WalletResult<Self> {
        Self::init_with_channel_capacity(5)
    }

    /// Same as [WalletAdapter::init] but a `capacity` value
    /// can be passed to create an channel with a desired capacity
    pub fn init_with_channel_capacity(capacity: usize) -> WalletResult<Self> {
        let storage = WalletStorage::default();

        let window = if let Some(window) = web_sys::window() {
            window
        } else {
            return Err(WalletError::MissingAccessToBrowserWindow);
        };

        let document = if let Some(document) = window.document() {
            document
        } else {
            return Err(WalletError::MissingAccessToBrowserDocument);
        };

        let (sender, receiver) = bounded::<WalletEvent>(capacity);

        let mut new_self = Self {
            window: window.clone(),
            document,
            storage,
            connection_info: Rc::new(RefCell::new(ConnectionInfo::default())),
            wallet_events: receiver,
        };

        InitEvents::new(&window).init(&mut new_self, sender)?;

        Ok(new_self)
    }

    /// Listen for [WalletEvent] to be notified when a wallet
    /// receives `connected`, `disconnected` and `accountChanged` events triggered
    /// when the `change` event is dispatched by a connected browser extension
    pub fn events(&self) -> WalletEventReceiver {
        self.wallet_events.clone()
    }

    /// Send a connect request to the browser wallet
    pub async fn connect(&mut self, wallet: Wallet) -> WalletResult<ConnectionInfo> {
        ConnectionInfo::connect(wallet).await
    }

    /// Lookup a wallet entry by name from the registered wallets
    /// and then send a connect request to the browser extension wallet
    pub async fn connect_by_name(&mut self, wallet_name: &str) -> WalletResult<ConnectionInfo> {
        let wallet = self.get_wallet(wallet_name)?;

        ConnectionInfo::connect(wallet).await
    }

    /// Send a disconnect request to the browser wallet
    pub async fn disconnect(&mut self) -> WalletResult<()> {
        self.connection_info.borrow_mut().disconnect().await
    }

    /// Send a sign in request to the browser wallet to Sign In With Solana
    pub async fn sign_in(
        &self,
        signin_input: &SigninInput,
        public_key: [u8; 32],
    ) -> WalletResult<SignInOutput> {
        self.connection_info()
            .connected_wallet()?
            .sign_in(signin_input, public_key)
            .await
    }

    /// Send a sign and send transaction request to the browser wallet
    pub async fn sign_and_send_transaction(
        &self,
        transaction_bytes: &[u8],
        cluster: Cluster,
        options: SendOptions,
    ) -> WalletResult<Signature> {
        let connection_info = self.connection_info();

        connection_info
            .connected_wallet()?
            .sign_and_send_transaction(
                transaction_bytes,
                cluster,
                options,
                connection_info.connected_account()?,
            )
            .await
    }

    /// Send a connect request to the browser wallet
    pub async fn sign_transaction(
        &self,
        transaction_bytes: &[u8],
        cluster: Option<Cluster>,
    ) -> WalletResult<Vec<Vec<u8>>> {
        let connection_info = self.connection_info();

        connection_info
            .connected_wallet()?
            .sign_transaction(
                transaction_bytes,
                cluster,
                connection_info.connected_account()?,
            )
            .await
    }

    /// Send a sign message request to the browser wallet
    pub async fn sign_message<'a>(
        &self,
        message: &'a [u8],
    ) -> WalletResult<SignedMessageOutput<'a>> {
        let connection_info = self.connection_info();

        connection_info
            .connected_wallet()?
            .sign_message(message, connection_info.connected_account()?)
            .await
    }

    /// Check if an [account](WalletAccount) is connected
    pub fn is_connected(&self) -> bool {
        self.connection_info.as_ref().borrow().account.is_some()
    }

    /// Get the connected [ConnectionInfo] containing the
    /// [account](WalletAccount) and [wallet](Wallet)
    pub fn connection_info(&self) -> Ref<'_, ConnectionInfo> {
        self.connection_info.as_ref().borrow()
    }

    pub(crate) fn connection_info_inner(&self) -> ConnectionInfoInner {
        self.connection_info.clone()
    }

    /// Get an entry in the `Window` object
    pub fn get_entry(&self, property: &str) -> Option<Object> {
        self.window.get(property)
    }

    /// Get the browser window
    pub fn window(&self) -> &Window {
        &self.window
    }

    /// Get the browser document
    pub fn document(&self) -> &Document {
        &self.document
    }

    /// Get the storage where the adapter stores the registered wallets
    pub fn storage(&self) -> &WalletStorage {
        self.storage.borrow()
    }

    /// Get the clusters supported by the connected wallet
    pub fn clusters(&self) -> WalletResult<Vec<Cluster>> {
        let mut clusters = Vec::<Cluster>::default();

        if self.mainnet()? {
            clusters.push(Cluster::MainNet);
        }
        if self.devnet()? {
            clusters.push(Cluster::DevNet);
        }
        if self.localnet()? {
            clusters.push(Cluster::LocalNet);
        }
        if self.testnet()? {
            clusters.push(Cluster::TestNet);
        }

        Ok(clusters)
    }

    /// Get the registered wallets
    pub fn wallets(&self) -> Vec<Wallet> {
        self.storage.borrow().get_wallets()
    }

    /// Get a certain wallet by its name
    pub fn get_wallet(&self, wallet_name: &str) -> WalletResult<Wallet> {
        self.storage
            .get_wallet(wallet_name)
            .ok_or(WalletError::WalletNotFound)
    }

    /// Check if the connected wallet supports mainnet cluster
    pub fn mainnet(&self) -> WalletResult<bool> {
        Ok(self.connection_info().connected_wallet()?.mainnet())
    }

    /// Check if the connected wallet supports devnet cluster
    pub fn devnet(&self) -> WalletResult<bool> {
        Ok(self.connection_info().connected_wallet()?.devnet())
    }

    /// Check if the connected wallet supports testnet cluster
    pub fn testnet(&self) -> WalletResult<bool> {
        Ok(self.connection_info().connected_wallet()?.testnet())
    }

    /// Check if the connected wallet supports localnet cluster
    pub fn localnet(&self) -> WalletResult<bool> {
        Ok(self.connection_info().connected_wallet()?.localnet())
    }

    /// Check if the connected wallet supports `standard:connect` feature
    pub fn standard_connect(&self) -> WalletResult<bool> {
        Ok(self
            .connection_info()
            .connected_wallet()?
            .standard_connect())
    }

    /// Check if the connected wallet supports `standard:disconnect` feature
    pub fn standard_disconnect(&self) -> WalletResult<bool> {
        Ok(self
            .connection_info()
            .connected_wallet()?
            .standard_disconnect())
    }

    /// Check if the connected wallet supports `standard:events` feature
    pub fn standard_events(&self) -> WalletResult<bool> {
        Ok(self.connection_info().connected_wallet()?.standard_events())
    }

    /// Check if the connected wallet supports `solana:signIn` feature
    pub fn solana_signin(&self) -> WalletResult<bool> {
        Ok(self.connection_info().connected_wallet()?.solana_signin())
    }

    /// Check if the connected wallet supports `solana:signMessage` feature
    pub fn solana_sign_message(&self) -> WalletResult<bool> {
        Ok(self
            .connection_info()
            .connected_wallet()?
            .solana_sign_message())
    }

    /// Check if the connected wallet supports `solana:signAndSendTransaction` feature
    pub fn solana_sign_and_send_transaction(&self) -> WalletResult<bool> {
        Ok(self
            .connection_info()
            .connected_wallet()?
            .solana_sign_and_send_transaction())
    }

    /// Check if the connected wallet supports `solana:signTransaction` feature
    pub fn solana_sign_transaction(&self) -> WalletResult<bool> {
        Ok(self
            .connection_info()
            .connected_wallet()?
            .solana_sign_transaction())
    }
}
