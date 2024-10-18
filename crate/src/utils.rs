use wasm_bindgen::JsValue;

use crate::WalletError;

/// A 32 byte array representing a Public Key
pub type PublicKey = [u8; 32];

/// A 64 byte array represnting a Signature
pub type Signature = [u8; 64];

/// The Version of the Wallet Standard currently implemented.
/// This may be used by the app to determine compatibility and feature detect.
pub const WALLET_STANDARD_VERSION: &str = "1.0.0";

pub struct Utils;

impl Utils {
    pub fn jsvalue_to_error<T: core::fmt::Debug>(
        value: Result<T, JsValue>,
    ) -> Result<(), WalletError> {
        if let Err(error) = value {
            Err(error.into())
        } else {
            Ok(())
        }
    }
}
