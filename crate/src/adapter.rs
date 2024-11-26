use std::borrow::Borrow;

use ed25519_dalek::Signature;
use web_sys::{js_sys::Object, Document, Window};

use crate::{
    Cluster, SendOptions, SignInOutput, SignedMessageOutput, SigninInput, Wallet, WalletAccount,
    WalletError, WalletResult, WalletStorage,
};

/// Operations on a browser window.
/// `Window` and `Document` object must be present otherwise
/// an error is thrown.
#[derive(Debug)]
pub struct WalletAdapter {
    window: Window,
    document: Document,
    storage: WalletStorage,
    connected_wallet: Option<Wallet>,
    connected_account: Option<WalletAccount>,
}

impl WalletAdapter {
    /// Get the `Window` and `Document` object in the current browser window
    pub fn init() -> WalletResult<Self> {
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

        let new_self = Self {
            window,
            document,
            storage,
            connected_wallet: Option::default(),
            connected_account: Option::default(),
        };

        new_self.init_events()?;

        Ok(new_self)
    }

    pub async fn connect(&mut self, wallet_name: &str) -> WalletResult<WalletAccount> {
        let wallet = self.get_wallet(wallet_name)?;

        let wallet_account = wallet.features.connect.call_connect().await?;

        self.set_connected_account(wallet_account.clone());
        self.set_connected_wallet(wallet);

        Ok(wallet_account)
    }

    pub async fn disconnect(&mut self) -> WalletResult<()> {
        if let Some(wallet) = self.connected_wallet.take() {
            wallet.features.disconnect.call_disconnect().await?;
            self.set_disconnected();

            Ok(())
        } else {
            Err(WalletError::WalletNotFound)
        }
    }

    pub async fn sign_and_send_transaction(
        &self,
        transaction_bytes: &[u8],
        cluster: Cluster,
        options: SendOptions,
    ) -> WalletResult<Signature> {
        let account = self.connected_account()?;
        let wallet = self.connected_wallet()?;
        wallet
            .features
            .sign_and_send_tx
            .call_sign_and_send_transaction(account, transaction_bytes, cluster, options)
            .await
    }

    pub async fn sign_transaction(
        &self,
        transaction_bytes: &[u8],
        cluster: Option<Cluster>,
    ) -> WalletResult<Vec<Vec<u8>>> {
        let account = self.connected_account()?;
        let wallet = self.connected_wallet()?;
        wallet
            .features
            .sign_tx
            .call_sign_tx(account, transaction_bytes, cluster)
            .await
    }

    pub async fn sign_message<'a>(
        &self,
        message: &'a [u8],
    ) -> WalletResult<SignedMessageOutput<'a>> {
        let account = self.connected_account()?;
        let wallet = self.connected_wallet()?;

        wallet
            .features
            .sign_message
            .call_sign_message(account, message)
            .await
    }

    pub async fn sign_in(
        &self,
        signin_input: &SigninInput,
        public_key: [u8; 32],
    ) -> WalletResult<SignInOutput> {
        let wallet = self.connected_wallet()?;

        if let Some(fn_exists) = wallet.features.sign_in.as_ref() {
            fn_exists.call_signin(signin_input, public_key).await
        } else {
            Err(WalletError::MissingSignInFunction)
        }
    }

    pub fn set_connected_account(&mut self, account_name: WalletAccount) -> &mut Self {
        self.connected_account.replace(account_name);

        self
    }

    pub fn set_connected_wallet(&mut self, wallet: Wallet) -> &mut Self {
        self.connected_wallet.replace(wallet);

        self
    }

    pub fn set_disconnected(&mut self) -> &mut Self {
        self.connected_wallet.take();
        self.connected_account.take();

        self
    }

    pub fn is_connected(&self) -> bool {
        self.connected_account.is_some()
    }

    pub fn connected_account(&self) -> WalletResult<&WalletAccount> {
        self.connected_account
            .as_ref()
            .ok_or(WalletError::AccountNotFound)
    }

    pub fn connected_wallet(&self) -> WalletResult<&Wallet> {
        self.connected_wallet
            .as_ref()
            .ok_or(WalletError::WalletNotFound)
    }

    fn init_events(&self) -> WalletResult<()> {
        self.register_wallet_event(self.storage.clone_inner())?;
        self.dispatch_app_event(self.storage.clone_inner());

        Ok(())
    }

    /// Get an entry in the `Window` object
    pub fn get_entry(&self, property: &str) -> Option<Object> {
        self.window.get(property)
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn document(&self) -> &Document {
        &self.document
    }

    pub fn storage(&self) -> &WalletStorage {
        self.storage.borrow()
    }

    pub fn wallets(&self) -> Vec<Wallet> {
        self.storage.borrow().get_wallets()
    }

    pub fn get_wallet(&self, wallet_name: &str) -> WalletResult<Wallet> {
        self.storage
            .get_wallet(wallet_name)
            .ok_or(WalletError::WalletNotFound)
    }

    pub fn mainnet(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.mainnet())
    }

    pub fn devnet(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.devnet())
    }

    pub fn testnet(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.testnet())
    }

    pub fn localnet(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.localnet())
    }

    pub fn standard_connect(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.standard_connect())
    }

    pub fn standard_disconnect(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.standard_disconnect())
    }

    pub fn standard_events(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.standard_events())
    }

    pub fn solana_signin(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.solana_signin())
    }

    pub fn solana_sign_message(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.solana_sign_message())
    }

    pub fn solana_sign_and_send_transaction(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.solana_sign_and_send_transaction())
    }

    pub fn solana_sign_transaction(&self) -> WalletResult<bool> {
        Ok(self.connected_wallet()?.solana_sign_transaction())
    }
}
