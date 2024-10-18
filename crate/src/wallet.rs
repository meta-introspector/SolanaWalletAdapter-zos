use wasm_bindgen::JsValue;
use web_sys::js_sys::Reflect;

use crate::{WalletError, WalletResult};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wallet {
    accounts: Vec<String>,
    chains: Vec<String>,
    // features: Feature,
    icon: Option<String>,
    name: String,
    version: String,
}

impl Wallet {
    pub fn from_jsvalue(value: JsValue) -> WalletResult<Self> {
        let reflection = Reflection::new(value);

        let (name_key, wallet_name) = reflection.string("name")?;
        assert_eq!(name_key.as_str(), "name");

        let (wallet_key, wallet_version) = reflection.string("version")?;
        assert_eq!(wallet_key.as_str(), "version");

        let mut wallet = Self::default();
        wallet.name = wallet_name;
        wallet.version = wallet_version;

        Ok(wallet)
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
