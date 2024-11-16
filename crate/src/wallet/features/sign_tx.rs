use ed25519_dalek::Signature;
use js_sys::Function;
use wasm_bindgen::{JsCast, JsValue};

use core::hash::Hash;

use crate::{
    Cluster, Commitment, Reflection, SemverVersion, Utils, WalletAccount, WalletError, WalletResult,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SignTransaction {
    pub version: SemverVersion,
    pub legacy: bool,
    pub version_zero: bool,
    callback: Function,
}

impl SignTransaction {
    fn new(value: JsValue, version: SemverVersion, key: &str) -> WalletResult<Self> {
        let reflection = Reflection::new(value)?;
        let inner_value = reflection
            .reflect_inner(key)
            .or(Err(WalletError::MissingSignTransactionFunction))?;
        let callback =
            inner_value
                .dyn_into::<Function>()
                .or(Err(WalletError::JsValueNotFunction(
                    String::from("Namespace[`solana:") + key + "->" + key + "`]",
                )))?;

        let (legacy, version_zero) = Self::get_tx_version_support(&reflection)?;

        Ok(Self {
            version,
            callback,
            legacy,
            version_zero,
        })
    }

    pub fn new_sign_tx(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        Self::new(value, version, "signTransaction")
    }

    pub fn new_sign_and_send_tx(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        Self::new(value, version, "signAndSendTransaction")
    }

    fn get_tx_version_support(inner_value: &Reflection) -> WalletResult<(bool, bool)> {
        let tx_version_support_jsvalue = inner_value
            .reflect_inner("supportedTransactionVersions")
            .or(Err(WalletError::ExpectedValueNotFound(
                "supportedTransactionVersions".to_string(),
            )))?;
        let tx_version_support = tx_version_support_jsvalue
            .dyn_ref::<js_sys::Array>()
            .ok_or(WalletError::ExpectedArray(
                "supportedTransactionVersions".to_string(),
            ))?;

        let mut legacy = false;
        let mut version_zero = false;

        tx_version_support.iter().try_for_each(|value| {
            if value == JsValue::from_str("legacy") {
                legacy = true;
            } else if value == JsValue::from(0) {
                version_zero = true;
            } else {
                return Err(WalletError::UnsupportedTransactionVersion);
            }

            Ok(())
        })?;

        if legacy != true {
            return Err(WalletError::LegacyTransactionSupportRequired);
        }

        Ok((legacy, version_zero))
    }

    pub(crate) async fn call_sign_tx(
        &self,
        wallet_account: &WalletAccount,
        transaction_bytes: &[u8],
        cluster: Option<Cluster>,
    ) -> WalletResult<Vec<Vec<u8>>> {
        let tx_bytes_value: js_sys::Uint8Array = transaction_bytes.into();

        let mut tx_object = Reflection::new_object();
        tx_object.set_object(&"account".into(), &wallet_account.js_value)?;
        tx_object.set_object(&"transaction".into(), &tx_bytes_value)?;
        if let Some(cluster) = cluster {
            tx_object.set_object(&"chain".into(), &cluster.chain().into())?;
        }

        let outcome = self.callback.call1(&JsValue::null(), &tx_object.take())?;

        let outcome = js_sys::Promise::resolve(&outcome);

        let success = wasm_bindgen_futures::JsFuture::from(outcome).await?;
        Reflection::new(success)?.get_bytes_from_vec("signedTransaction")
    }

    pub async fn call_sign_and_send_transaction(
        &self,
        wallet_account: &WalletAccount,
        transaction_bytes: &[u8],
        cluster: Cluster,
        options: SendOptions,
    ) -> WalletResult<Signature> {
        let tx_bytes_value: js_sys::Uint8Array = transaction_bytes.into();

        let mut tx_object = Reflection::new_object();
        tx_object.set_object(&"account".into(), &wallet_account.js_value)?;
        tx_object.set_object(&"transaction".into(), &tx_bytes_value)?;
        tx_object.set_object(&"chain".into(), &cluster.chain().into())?;
        tx_object.set_object(&"options".into(), &options.to_object()?)?;

        let outcome = self.callback.call1(&JsValue::null(), &tx_object.take())?;

        let outcome = js_sys::Promise::resolve(&outcome);

        let success = wasm_bindgen_futures::JsFuture::from(outcome).await?;

        Reflection::new(success)?
            .get_bytes_from_vec("signature")?
            .get(0)
            .map(|value| Utils::signature(Utils::to64byte_array(value)?))
            .ok_or(WalletError::SendAndSignTransactionSignatureEmpty)?
    }
}

impl PartialOrd for SignTransaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.version.cmp(&other.version))
    }
}

impl Ord for SignTransaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl Hash for SignTransaction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct SendOptions {
    preflight_commitment: Commitment,
    skip_preflight: bool,
    max_retries: u8,
}

impl SendOptions {
    pub fn to_object(&self) -> WalletResult<JsValue> {
        let mut reflection = Reflection::new_object();
        reflection.set_object_str("preflightCommitment", &self.preflight_commitment.as_str())?;
        reflection.set_object(&"skipPreflight".into(), &JsValue::from(self.skip_preflight))?;
        reflection.set_object(&"maxRetries".into(), &JsValue::from(self.max_retries))?;

        Ok(reflection.take())
    }
}
