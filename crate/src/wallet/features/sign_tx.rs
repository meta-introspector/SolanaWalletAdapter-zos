use js_sys::Function;
use wasm_bindgen::{JsCast, JsValue};

use core::hash::Hash;

use crate::{Cluster, Reflection, SemverVersion, WalletAccount, WalletError, WalletResult};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SignTransaction {
    pub version: SemverVersion,
    pub legacy: bool,
    pub version_zero: bool,
    callback: Function,
}

impl SignTransaction {
    pub fn new(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        let reflection = Reflection::new(value)?;
        let inner_value = reflection
            .reflect_inner(&"signTransaction")
            .or(Err(WalletError::MissingSignTransactionFunction))?;
        let callback =
            inner_value
                .dyn_into::<Function>()
                .or(Err(WalletError::JsValueNotFunction(
                    "Namespace[`solana:signTransaction -> signTransaction`]".to_string(),
                )))?;

        let (legacy, version_zero) = Self::get_tx_version_support(&reflection)?;

        Ok(Self {
            version,
            callback,
            legacy,
            version_zero,
        })
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
