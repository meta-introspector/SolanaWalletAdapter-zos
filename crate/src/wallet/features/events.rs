use std::hash::Hash;

use js_sys::Function;
use wasm_bindgen::JsValue;

use crate::{Reflection, SemverVersion, WalletError, WalletResult};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StandardEvents {
    version: SemverVersion,
    callback: Function,
}
impl StandardEvents {
    pub fn new(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        let get_standard_event_fn = Reflection::new(value)?.get_function("on")?;

        Ok(Self {
            version,
            callback: get_standard_event_fn,
        })
    }

    pub(crate) async fn call_standard_event(&self) -> WalletResult<()> {
        let outcome = self.callback.call0(&JsValue::from_bool(false))?;

        let outcome = js_sys::Promise::resolve(&outcome);

        if let Some(error) = wasm_bindgen_futures::JsFuture::from(outcome).await.err() {
            let value: WalletError = error.into();
            return Err(WalletError::StandardEventsError(value.to_string()));
        }

        Ok(())
    }
}

impl PartialOrd for StandardEvents {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.version.cmp(&other.version))
    }
}

impl Ord for StandardEvents {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl Hash for StandardEvents {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.version.hash(state);
    }
}
