use js_sys::{wasm_bindgen::JsValue, Reflect};
use thiserror::Error;

/// A Result<T, WalletError>
pub type WalletResult<T> = Result<T, WalletError>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Error)]
pub enum WalletError {
    /// An JavaScript Error corresponding to a [wasm_bindgen::JsValue] .
    /// It contains the error type represented by `name`,
    /// the error message `message`
    /// and the `stack` message which offers a trace of which functions were called.
    /// Learn about this error type from [Error - Mozilla Developer Network](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)
    #[error("An JavaScript Error corresponding to a `wasm_bindgen::JsValue`. It contains the error type represented by `name`, the error message `message` and the `stack` message which offers a trace of which functions were called.")]
    JsError {
        name: String,
        message: String,
        stack: String,
    },
    /// Unable to parse an `Err(JsValue)` to get the `name`, `message` or `stack`. One, some or all of these values might be missing
    #[error("Unable to parse an `Err(JsValue)` to get the `name`, `message` or `stack`. One, some or all of these values might be missing")]
    UnableToParseJsError,
    /// Attempted to convert a JsValue to a String where a String was expected
    #[error(" Attempted to convert a JsValue to a String where a String was expected")]
    JsValueNotString,
    /// Attempted to convert a JsError to a String
    #[error("Attempted to convert a JsError to a String")]
    JsErrorNotString,
    /// A value of `undefined` or `null` was encountered
    #[error("A value of `undefined` or `null` was encountered")]
    ValueNotFound,
    /// Unable to access browser window
    #[error("Unable to access browser window")]
    MissingAccessToBrowserWindow,
    /// Unable to access browser document
    #[error("Unable to access browser document")]
    MissingAccessToBrowserDocument,
    /// Only `processed`, `confirmed` and `finalized` commitments are supported by Solana clusters
    #[error("Only `processed`, `confirmed` and `finalized` commitments are supported by Solana clusters")]
    UnsupportedCommitment(String),
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
