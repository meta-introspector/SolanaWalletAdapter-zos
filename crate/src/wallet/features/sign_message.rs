use ed25519_dalek::{PublicKey, Signature};
use js_sys::{Function, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};

use core::{hash::Hash, str};

use crate::{Reflection, SemverVersion, Utils, WalletAccount, WalletError, WalletResult};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SignMessage {
    version: SemverVersion,
    callback: Function,
}

impl SignMessage {
    pub fn new(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        let get_value = Reflection::new(value)?
            .reflect_inner(&"signMessage")
            .or(Err(WalletError::MissingSignMessageFunction))?;
        let get_fn = get_value
            .dyn_into::<Function>()
            .or(Err(WalletError::JsValueNotFunction(
                "Namespace[`solana:signMessage -> signMessage`]".to_string(),
            )))?;

        Ok(Self {
            version,
            callback: get_fn,
        })
    }

    pub(crate) async fn call_sign_message<'a>(
        &self,
        wallet_account: &WalletAccount,
        message: &'a [u8],
    ) -> WalletResult<SignedMessageOutput<'a>> {
        let message_value: js_sys::Uint8Array = message.into();

        let mut message_object = Reflection::new_object();
        message_object.set_object(&"account".into(), &wallet_account.js_value)?;
        message_object.set_object(&"message".into(), &message_value)?;

        // Call the callback with message and account
        let outcome = self
            .callback
            .call1(&JsValue::null(), message_object.get_inner())?;

        let outcome = js_sys::Promise::resolve(&outcome);
        let signed_message_result = wasm_bindgen_futures::JsFuture::from(outcome).await?;
        let signed_message_result = signed_message_result
            .dyn_ref::<js_sys::Array>()
            .ok_or(WalletError::JsValueNotArray(
                "solana:signedMessage -> SignedMessageOutput".to_string(),
            ))?
            .to_vec();

        if let Some(inner) = signed_message_result.get(0) {
            let reflect_outcome = Reflection::new(inner.clone())?;
            let signed_message = reflect_outcome.reflect_inner("signedMessage")?;
            let signature_value = reflect_outcome.reflect_inner("signature")?;

            let signed_message = signed_message
                .dyn_into::<Uint8Array>()
                .or(Err(WalletError::JsValueNotUnint8Array(
                    "solana:signedMessage -> SignedMessageOutput::signedMessage".to_string(),
                )))?
                .to_vec();

            if signed_message != message {
                return Err(WalletError::SignedMessageMismatch);
            }

            let signature = Utils::jsvalue_to_signature(
                signature_value,
                "solana::signMessage -> SignedMessageOutput::signature",
            )?;

            let public_key = Utils::public_key(wallet_account.public_key)?;

            Utils::verify_signature(public_key, &message, signature)?;

            Ok(SignedMessageOutput {
                message,
                public_key: wallet_account.public_key,
                signature: signature.to_bytes(),
            })
        } else {
            Err(WalletError::ReceivedAnEmptySignedMessagesArray)
        }
    }
}

impl PartialOrd for SignMessage {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.version.cmp(&other.version))
    }
}

impl Ord for SignMessage {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl Hash for SignMessage {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct SignedMessageOutput<'a> {
    message: &'a [u8],
    public_key: [u8; 32],
    signature: [u8; 64],
}

impl<'a> SignedMessageOutput<'a> {
    pub fn message(&self) -> &str {
        //Should never fail since verified message is always UTF-8 Format hence `.unwrap()` is used
        str::from_utf8(&self.message).unwrap()
    }

    pub fn public_key(&self) -> WalletResult<PublicKey> {
        Utils::public_key(self.public_key)
    }

    pub fn signature(&self) -> WalletResult<Signature> {
        Utils::signature(self.signature)
    }
}

impl<'a> Default for SignedMessageOutput<'a> {
    fn default() -> Self {
        Self {
            message: &[],
            public_key: [0u8; 32],
            signature: [0u8; 64],
        }
    }
}
