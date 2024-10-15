use log::info;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Reflect;

use crate::{WalletError, WalletResult};

pub const WALLET_VERSION: &str = "1.0.0";

pub const WINDOW_APP_READY_EVENT_TYPE: &str = "wallet-standard:app-ready";

pub const WINDOW_REGISTER_WALLET_EVENT_TYPE: &str = "wallet-standard:register-wallet";

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Wallet {
    accounts: Vec<String>,
    chains: Vec<String>,
    // features: Feature,
    icon: Option<String>,
    name: String,
    version: String,
}

impl Wallet {
    pub fn from_jsvalue(value: JsValue) {
        let reflection = Reflection::new(value);
        let name = reflection.string("name");

        info!("{:?}", &name);
    }

    pub fn accounts(&self) -> &[String] {
        &self.accounts
    }

    pub fn chains(&self) -> &[String] {
        &self.chains
    }

    pub fn icon(&self) -> Option<&String> {
        self.icon.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }
}

#[derive(Debug)]
pub struct Reflection(JsValue);

impl Reflection {
    pub fn new(value: JsValue) -> Self {
        Self(value)
    }

    pub fn string(&self, key: &str) -> WalletResult<(String, String)> {
        let name = Reflect::get(&self.0, &key.into())?;

        Reflection::check_is_undefined(&name)?;

        let parsed = name.as_string().ok_or(WalletError::JsValueNotString)?;

        Ok((key.to_string(), parsed))
    }

    pub fn check_is_undefined(value: &JsValue) -> WalletResult<()> {
        if value.is_undefined() || value.is_null() {
            return Err(WalletError::ValueNotFound);
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct VersionedValue {
    version: String,
}

/// A data URI containing a base64-encoded SVG, WebP, PNG, or GIF image.
pub struct WalletIcon(
    /// Format `data:image/${'svg+xml' | 'webp' | 'png' | 'gif'};base64,${string}`
    &'static str,
);
