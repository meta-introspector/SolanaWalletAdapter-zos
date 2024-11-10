use ed25519_dalek::{PublicKey, Signature, Verifier};
use js_sys::{Array, Function, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};

use crate::{WalletError, WalletResult};

/// A 32 byte array representing a Public Key
pub type PublicKeyBytes = [u8; 32];

/// A 64 byte array represnting a Signature
pub type SignatureBytes = [u8; 64];

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

    pub fn public_key(public_key_bytes: [u8; 32]) -> WalletResult<PublicKey> {
        PublicKey::from_bytes(&public_key_bytes).or(Err(WalletError::InvalidEd25519PublicKeyBytes))
    }

    pub fn signature(signature_bytes: [u8; 64]) -> WalletResult<Signature> {
        Signature::from_bytes(&signature_bytes).or(Err(WalletError::InvalidEd25519SignatureBytes))
    }

    pub fn verify_signature(
        public_key: PublicKey,
        message: &[u8],
        signature: Signature,
    ) -> WalletResult<()> {
        public_key
            .verify(message, &signature)
            .or(Err(WalletError::InvalidSignature))
    }

    pub fn jsvalue_to_signature(value: JsValue, error_identifier: &str) -> WalletResult<Signature> {
        let signature_bytes: [u8; 64] = value
            .dyn_into::<js_sys::Uint8Array>()
            .or(Err(WalletError::JsValueNotUnint8Array(
                error_identifier.to_string(),
            )))?
            .to_vec()
            .try_into()
            .or(Err(WalletError::InvalidEd25519PublicKeyBytes))?;

        Self::signature(signature_bytes)
    }
}

#[derive(Debug)]
pub struct Reflection(JsValue);

impl Reflection {
    pub fn new(value: JsValue) -> WalletResult<Self> {
        Reflection::check_is_undefined(&value)?;

        Ok(Self(value))
    }

    pub fn new_from_str(value: &JsValue, key: &str) -> WalletResult<Self> {
        let inner = Reflect::get(&value, &key.into())?;

        Reflection::new(inner)
    }

    pub fn new_object() -> Self {
        Self(Object::new().into())
    }

    pub fn take(self) -> JsValue {
        self.0
    }

    pub fn set_object_str(&mut self, key: &str, value: &str) -> WalletResult<&Self> {
        self.set_object(&key.into(), &value.into())
    }

    pub fn set_object_string_optional(
        &mut self,
        key: &str,
        value: Option<&String>,
    ) -> WalletResult<&Self> {
        if let Some(inner_value) = value {
            self.set_object(&key.into(), &inner_value.into())
        } else {
            Ok(self)
        }
    }

    pub fn set_object_string_optional_undefined(
        &mut self,
        key: &str,
        value: Option<&String>,
    ) -> WalletResult<&Self> {
        if let Some(inner_value) = value {
            self.set_object(&key.into(), &inner_value.into())
        } else {
            self.set_object(&key.into(), &JsValue::undefined())
        }
    }

    pub fn set_object(&mut self, key: &JsValue, value: &JsValue) -> WalletResult<&Self> {
        if !self.0.is_object() {
            return Err(WalletError::JsValueNotObject);
        }

        let target = self.0.dyn_ref::<Object>().unwrap();

        Reflect::set(&target, &key, &value)?;

        self.0 = target.into();

        Ok(self)
    }

    pub fn reflect_inner(&self, key: &str) -> WalletResult<JsValue> {
        let inner = Reflect::get(&self.0, &key.into())?;

        Reflection::check_is_undefined(&inner)?;

        Ok(inner)
    }

    pub fn mutate_inner(mut self, key: &str) -> WalletResult<Self> {
        let inner = Reflect::get(&self.0, &key.into())?;

        Reflection::check_is_undefined(&inner)?;

        self.0 = inner;

        Ok(self)
    }

    pub fn string(&self, key: &str) -> WalletResult<String> {
        let name = Reflect::get(&self.0, &key.into())?;

        let parsed = name.as_string().ok_or(WalletError::JsValueNotString)?;

        Ok(parsed)
    }

    pub fn byte32array(&self, key: &str) -> WalletResult<[u8; 32]> {
        let js_value = Reflect::get(&self.0, &key.into())?;

        let to_js_array: js_sys::Uint8Array = js_value.unchecked_into();

        let byte32array: [u8; 32] = to_js_array
            .to_vec()
            .try_into()
            .or(Err(WalletError::Expected32ByteLength))?;

        Ok(byte32array)
    }

    pub fn get_array(&self) -> WalletResult<Array> {
        Ok(self.0.clone().dyn_into::<js_sys::Array>()?)
    }

    pub fn get_string(value: &JsValue) -> WalletResult<String> {
        value.as_string().ok_or(WalletError::JsValueNotString)
    }

    pub fn vec_string(&self, key: &str) -> WalletResult<Vec<String>> {
        let to_js_array = self.get_js_array(key)?;

        to_js_array
            .iter()
            .map(|value| Self::get_string(&value))
            .collect::<WalletResult<Vec<String>>>()
    }

    pub fn get_js_array(&self, key: &str) -> WalletResult<Array> {
        let js_value = Reflect::get(&self.0, &key.into())?;

        if !js_value.is_array() {
            return Err(WalletError::ExpectedArray(key.to_string()));
        }

        Ok(js_value.unchecked_into())
    }

    pub fn vec_string_and_filter(&self, key: &str, filter: &str) -> WalletResult<Vec<String>> {
        let js_value = Reflect::get(&self.0, &key.into())?;

        if !js_value.is_array() {
            return Err(WalletError::ExpectedArray(key.to_string()));
        }

        let to_js_array: js_sys::Array = js_value.unchecked_into();

        to_js_array
            .iter()
            .map(|value| value.as_string().ok_or(WalletError::JsValueNotString))
            .map(|value| {
                let value = value?;

                if value.starts_with(filter) {
                    Ok(value)
                } else {
                    Err(WalletError::UnsupportedChain(value.to_string()))
                }
            })
            .collect::<WalletResult<Vec<String>>>()
    }

    pub fn object_to_vec_string(&self, key: &str) -> WalletResult<Vec<String>> {
        let features_value = Reflect::get(&self.0, &key.into())?;

        let features_object = features_value
            .dyn_ref::<Object>()
            .ok_or(WalletError::ExpectedObject(key.to_string()))?;

        Object::keys(features_object)
            .iter()
            .map(|value| value.as_string().ok_or(WalletError::JsValueNotString))
            .collect::<WalletResult<Vec<String>>>()
    }

    pub fn check_is_undefined(value: &JsValue) -> WalletResult<()> {
        if value.is_undefined() || value.is_null() {
            return Err(WalletError::ValueNotFound);
        } else {
            Ok(())
        }
    }

    pub fn get_function(&self, key: &str) -> WalletResult<Function> {
        let js_value = Reflect::get(&self.0, &key.into())?;

        js_value
            .dyn_into::<Function>()
            .or(Err(WalletError::JsValueNotFunction(key.to_string())))
    }

    pub fn as_function(&self) -> WalletResult<&Function> {
        self.0
            .dyn_ref::<Function>()
            .ok_or(WalletError::CastJsValueAsFunction)
    }

    pub fn as_owned_function(self) -> WalletResult<Function> {
        self.0
            .dyn_into::<Function>()
            .or(Err(WalletError::CastJsValueAsFunction))
    }

    pub fn keys(&self) -> WalletResult<Vec<String>> {
        Object::keys(&self.0.clone().into())
            .iter()
            .map(|value| value.as_string().ok_or(WalletError::JsValueNotString))
            .collect::<WalletResult<Vec<String>>>()
    }

    pub fn get_inner(&self) -> &JsValue {
        &self.0
    }
}
