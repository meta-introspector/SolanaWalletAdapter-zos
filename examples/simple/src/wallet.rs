use log::info;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::{global, Function, Reflect};

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
// pub struct Feature {
//     #[serde(rename)]
//     standard_connect: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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

pub type WalletResult<T> = Result<T, WalletError>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum WalletError {
    JsError {
        name: String,
        message: String,
        stack: String,
    },
    UnableToParseJsError,
    JsValueNotString,
    JsErrorNotString,
    ValueNotFound,
    UnableToGetWalletName,
}

impl WalletError {
    pub fn not_found(&self) -> bool {
        self == &WalletError::ValueNotFound
    }
}

impl From<JsValue> for WalletError {
    fn from(value: JsValue) -> Self {
        let reflect = |key: &str| -> Result<String, Self> {
            Reflect::get(&value, &key.into())
                .map_err(|_: JsValue| WalletError::UnableToParseJsError)?
                .as_string()
                .map_or(Err(WalletError::JsErrorNotString), |inner| Ok(inner))
        };

        let name = match reflect("name") {
            Ok(inner) => inner,
            Err(error) => return error,
        };

        let stack = match reflect("stack") {
            Ok(inner) => inner,
            Err(error) => return error,
        };
        let message = match reflect("message") {
            Ok(inner) => inner,
            Err(error) => return error,
        };

        Self::JsError {
            message,
            name,
            stack,
        }
    }
}
