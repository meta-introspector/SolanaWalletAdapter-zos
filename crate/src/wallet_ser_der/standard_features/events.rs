use js_sys::Function;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

use crate::{Reflection, SemverVersion, StandardFunction, WalletError, WalletResult};

/// `standard:events` struct containing the `version` and `callback`
/// within the [StandardFunction] field
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StandardEvents(StandardFunction);

impl StandardEvents {
    /// parse the callback for `standard:events` from the [JsValue]
    pub fn new(value: JsValue, version: SemverVersion) -> WalletResult<Self> {
        let get_standard_event_fn = Reflection::new(value)?.get_function("on")?;

        let on_account_change = Closure::wrap(Box::new(move |value: JsValue| {
            web_sys::console::log_2(&"CALLED ON CONNECTED FEATURE".into(), &value);
        }) as Box<dyn Fn(_)>);
        let on_account_change_fn = on_account_change.as_ref().dyn_ref::<Function>().unwrap();

        get_standard_event_fn
            .call2(
                &JsValue::null(),
                &"change".into(),
                &on_account_change_fn.into(),
            )
            .unwrap();
        on_account_change.forget();

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
