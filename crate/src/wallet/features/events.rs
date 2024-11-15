use wasm_bindgen::JsValue;

use crate::{Reflection, SemverVersion, StandardFunction, WalletError, WalletResult};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StandardEvents(StandardFunction);

impl StandardEvents {
    pub fn new(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        let get_standard_event_fn = Reflection::new(value)?.get_function("on")?;

        Ok(Self(StandardFunction {
            version,
            callback: get_standard_event_fn,
        }))
    }

    pub(crate) async fn call_standard_event(&self) -> WalletResult<()> {
        let outcome = self.0.callback.call0(&JsValue::from_bool(false))?;

        let outcome = js_sys::Promise::resolve(&outcome);

        if let Some(error) = wasm_bindgen_futures::JsFuture::from(outcome).await.err() {
            let value: WalletError = error.into();
            return Err(WalletError::StandardEventsError(value.to_string()));
        }

        Ok(())
    }
}
