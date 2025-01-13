use wasm_bindgen::{prelude::Closure, JsValue};

use crate::{
    ConnectionInfoInner, Reflection, SemverVersion, StandardFunction, Utils, WalletError,
    WalletEvent, WalletEventSender, WalletResult,
};

/// `standard:events` struct containing the `version` and `callback`
/// within the [StandardFunction] field
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StandardEvents(StandardFunction);

impl StandardEvents {
    /// parse the callback for `standard:events` from the [JsValue]
    pub fn new(
        value: JsValue,
        version: SemverVersion,
        sender: WalletEventSender,
        connection_info: ConnectionInfoInner,
    ) -> WalletResult<Self> {
        let get_standard_event_fn = Reflection::new(value)?.get_function("on")?;

        let connection_exists = connection_info
            .as_ref()
            .borrow()
            .connected_account_raw()
            .is_some();

        let sender_inner = sender.clone();

        let on_account_change = Closure::wrap(Box::new(move |value: JsValue| {
            let sender2 = sender_inner.clone();

            let send_message = |wallet_event: WalletEvent| async move {
                sender2
                    .send(wallet_event)
                    .await
                    .expect("E01> Could not send message. Channel Closed");
            };

            let connection_info_inner = std::rc::Rc::clone(&connection_info);

            let public_key_changed_raw = Reflection::new(value)
                .expect(&WalletError::format_error(
                    "02",
                    "PublicKey is undefined or null",
                ))
                .reflect_inner_as_bytes("accounts", "01")
                .expect(&WalletError::format_error(
                    "01",
                    "`accounts` key not found in object",
                ));
            let public_key_bytes =
                Utils::to32byte_array(&public_key_changed_raw).expect(&WalletError::format_error(
                    "03",
                    "Invalid Bytes, expected 32 bytes as a requirement for Ed25519 PublicKey",
                ));

            Utils::public_key(public_key_bytes).expect(&WalletError::format_error(
                "04",
                "The bytes provided are invalid for conversion to an Ed25519 public key",
            ));

            wasm_bindgen_futures::spawn_local(async move {
                if !public_key_changed_raw.is_empty() {
                    if connection_exists {
                        send_message.clone()(WalletEvent::AccountChanged).await;
                    } else {
                        send_message(WalletEvent::Connected).await;
                    }
                } else {
                    connection_info_inner
                        .borrow_mut()
                        .disconnect()
                        .await
                        .expect("Received a disconnect event from the browser wallet but the `WalletAdapter::disconnect` did not execute successfully.");
                    send_message(WalletEvent::Disconnected).await;
                }
            });
        }) as Box<dyn FnMut(JsValue)>);

        let on_account_change_fn =
            Reflection::as_function_owned(on_account_change.into_js_value(), "05")?;

        get_standard_event_fn.call2(
            &JsValue::null(),
            &"change".into(),
            &on_account_change_fn.into(),
        )?;

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
