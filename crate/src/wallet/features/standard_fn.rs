use core::hash::Hash;

use js_sys::Function;
use wasm_bindgen::{JsCast, JsValue};

use crate::{Reflection, SemverVersion, WalletError, WalletResult};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StandardFunction {
    pub(crate) version: SemverVersion,
    pub(crate) callback: Function,
}

impl StandardFunction {
    pub fn new(
        value: JsValue,
        version: SemverVersion,
        key: &str,
        namespace: &str,
    ) -> WalletResult<Self> {
        let fn_value = Reflection::new(value)?
            .reflect_inner(&key)
            .or(Err(WalletError::MissingConnectFunction))?;
        let get_fn = fn_value
            .dyn_into::<Function>()
            .or(Err(WalletError::JsValueNotFunction(
                String::from("Namespace[`") + namespace + ":" + key + "-> " + key + "`]",
            )))?;

        Ok(Self {
            version,
            callback: get_fn,
        })
    }
}

impl PartialOrd for StandardFunction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.version.cmp(&other.version))
    }
}

impl Ord for StandardFunction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl Hash for StandardFunction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
    }
}
