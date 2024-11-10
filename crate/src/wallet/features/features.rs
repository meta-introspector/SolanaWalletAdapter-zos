use wasm_bindgen::{JsCast, JsValue};
use web_sys::js_sys::Reflect;

use crate::{
    Reflection, SemverVersion, SignInOutput, SigninInput, WalletAccount, WalletError, WalletResult,
    STANDARD_CONNECT_IDENTIFIER, STANDARD_EVENTS_IDENTIFIER,
};

use super::{Connect, Disconnect, SignIn, SignMessage, SignedMessageOutput, StandardEvents};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureInfo {
    pub version: SemverVersion,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureInfoWithTx {
    pub version: SemverVersion,
    pub legacy: bool,
    pub version_zero: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Features {
    /// standard:connect
    connect: Connect,
    /// standard:disconnect
    disconnect: Disconnect,
    /// standard:events
    events: Option<StandardEvents>,
    /// solana:signAndSendTransaction
    sign_and_send_tx: Option<FeatureInfoWithTx>,
    /// solana:signTransaction
    sign_tx: Option<FeatureInfoWithTx>,
    /// solana:signMessage
    sign_message: SignMessage,
    /// solana:signIn
    sign_in: Option<SignIn>,
    /// Non-standard features
    extensions: Vec<String>,
}

impl Features {
    pub fn parse(reflection: &Reflection) -> WalletResult<Self> {
        let features_keys = reflection.object_to_vec_string("features")?;
        let features_object = Reflection::new_from_str(reflection.get_inner(), "features")?;

        let mut features = Features::default();

        features_keys.into_iter().try_for_each(|feature| {
            let inner_object = features_object.reflect_inner(&feature)?;

            if feature.starts_with("standard:") || feature.starts_with("solana:") {
                let version = SemverVersion::from_jsvalue(&inner_object)?;

                let get_tx_version_support =
                    |value: &JsValue, version: SemverVersion| -> WalletResult<FeatureInfoWithTx> {
                        let tx_version_support_jsvalue =
                            Reflect::get(value, &"supportedTransactionVersions".into()).or(Err(
                                WalletError::ExpectedValueNotFound(
                                    "supportedTransactionVersions".to_string(),
                                ),
                            ))?;
                        let tx_version_support = tx_version_support_jsvalue
                            .dyn_ref::<js_sys::Array>()
                            .ok_or(WalletError::ExpectedArray(
                                "supportedTransactionVersions".to_string(),
                            ))?;

                        let mut tx_info = FeatureInfoWithTx {
                            version,
                            legacy: false,
                            version_zero: false,
                        };

                        tx_version_support.iter().try_for_each(|value| {
                            if value == JsValue::from_str("legacy") {
                                tx_info.legacy = true;
                            } else if value == JsValue::from(0) {
                                tx_info.version_zero = true;
                            } else {
                                return Err(WalletError::UnsupportedTransactionVersion);
                            }

                            Ok(())
                        })?;

                        if tx_info.legacy != true {
                            return Err(WalletError::LegacyTransactionSupportRequired);
                        }

                        Ok(tx_info)
                    };

                if feature == STANDARD_CONNECT_IDENTIFIER {
                    features.connect = Connect::new(inner_object, version)?;
                } else if feature == "standard:disconnect" {
                    features.disconnect = Disconnect::new(inner_object, version)?;
                } else if feature == STANDARD_EVENTS_IDENTIFIER {
                    features
                        .events
                        .replace(StandardEvents::new(inner_object, version)?);
                } else if feature == "solana:signAndSendTransaction" {
                    features
                        .sign_and_send_tx
                        .replace(get_tx_version_support(&inner_object, version)?);
                } else if feature == "solana:signTransaction" {
                    features
                        .sign_tx
                        .replace(get_tx_version_support(&inner_object, version)?);
                } else if feature == "solana:signMessage" {
                    features.sign_message = SignMessage::new(inner_object, version)?;
                } else if feature == "solana:signIn" {
                    features
                        .sign_in
                        .replace(SignIn::new(inner_object, version)?);
                } else {
                    return Err(WalletError::UnsupportedStandardFeature(feature));
                }
            } else {
                features.extensions.push(feature);
            }

            Ok(())
        })?;

        Ok(features)
    }

    pub async fn connect(&self) -> WalletResult<Vec<WalletAccount>> {
        self.connect.call_connect().await
    }

    pub async fn disconnect(&self) -> WalletResult<()> {
        self.disconnect.call_disconnect().await
    }

    pub async fn events(&self) -> WalletResult<()> {
        self.events
            .as_ref()
            .ok_or(WalletError::MissingStandardEventsFunction)?
            .call_standard_event()
            .await
    }

    pub fn sign_and_send_transaction(&self) -> Option<&FeatureInfoWithTx> {
        self.sign_and_send_tx.as_ref()
    }

    pub fn sign_transaction(&self) -> Option<&FeatureInfoWithTx> {
        self.sign_tx.as_ref()
    }

    pub async fn sign_message<'a>(
        &self,
        account: &WalletAccount,
        message: &'a [u8],
    ) -> WalletResult<SignedMessageOutput<'a>> {
        self.sign_message.call_sign_message(account, message).await
    }

    pub async fn sign_in(
        &self,
        signin_input: &SigninInput,
        public_key: [u8; 32],
    ) -> WalletResult<SignInOutput> {
        if let Some(fn_exists) = self.sign_in.as_ref() {
            fn_exists.call_signin(signin_input, public_key).await
        } else {
            Err(WalletError::MissingSignInFunction)
        }
    }

    pub fn extensions(&self) -> &[String] {
        &self.extensions
    }
}
