use std::borrow::Cow;

use wasm_bindgen::JsValue;
use web_sys::js_sys::Reflect;

use crate::{WalletError, WalletResult};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Wallet {
    accounts: Vec<String>,
    chains: Vec<String>,
    // features: Feature,
    icon: Option<WalletIcon>,
    name: String,
    version: SemverVersion,
}

impl Wallet {
    pub fn from_jsvalue(value: JsValue) -> WalletResult<Self> {
        let reflection = Reflection::new(value);

        let (name_key, wallet_name) = reflection.string("name")?;
        assert_eq!(name_key.as_str(), "name");

        let (wallet_key, wallet_version) = reflection.string("version")?;
        assert_eq!(wallet_key.as_str(), "version");

        let (icon_key, icon) = match reflection.string("icon") {
            Ok((icon_key, icon)) => (icon_key, Option::Some(WalletIcon(Cow::Owned(icon)))),
            Err(error) => {
                if error == WalletError::JsValueNotString {
                    (String::from("icon"), Option::None)
                } else {
                    return Err(error);
                }
            }
        };

        assert_eq!(icon_key.as_str(), "icon");

        let mut wallet = Self::default();
        wallet.name = wallet_name;
        wallet.version = SemverVersion::parse(&wallet_version)?;
        wallet.icon = icon;

        Ok(wallet)
    }

    pub fn accounts(&self) -> &[String] {
        &self.accounts
    }

    pub fn chains(&self) -> &[String] {
        &self.chains
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

/// A data URI containing a base64-encoded SVG, WebP, PNG, or GIF image.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WalletIcon(
    /// Format `data:image/${'svg+xml' | 'webp' | 'png' | 'gif'};base64,${string}`
    pub Cow<'static, str>,
);

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemverVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl SemverVersion {
    pub fn parse(version: &str) -> WalletResult<Self> {
        let chunks = version.split(".").collect::<Vec<&str>>();

        if chunks.len() != 3 {
            return Err(WalletError::InvalidWalletVersion(version.to_string()));
        }

        let version_chunks = chunks
            .iter()
            .map(|chunk| {
                chunk
                    .parse::<u8>()
                    .map_err(|_| WalletError::InvalidSemVerNumber(chunk.to_string()))
            })
            .collect::<WalletResult<Vec<u8>>>()?;

        Ok(Self {
            major: version_chunks[0],
            minor: version_chunks[1],
            patch: version_chunks[2],
        })
    }

    pub fn get_version(&self) -> &Self {
        self
    }

    pub fn stringify_version(&self) -> Cow<str> {
        Cow::Borrowed("")
            + Cow::Owned(self.major.to_string())
            + "."
            + Cow::Owned(self.minor.to_string())
            + "."
            + Cow::Owned(self.minor.to_string())
    }
}

impl core::fmt::Debug for SemverVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SemverVersion({}.{}.{})",
            self.major, self.minor, self.patch
        )
    }
}

impl core::fmt::Display for SemverVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
