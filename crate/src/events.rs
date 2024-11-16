use std::rc::Rc;

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CustomEvent, CustomEventInit};

use crate::{
    StorageType, Utils, Wallet, WalletAdapter, WalletError, WalletResult,
    WINDOW_APP_READY_EVENT_TYPE,
};

impl WalletAdapter {
    pub fn dispatch_app_event(&self, storage: StorageType) {
        let app_ready_init = CustomEventInit::new();
        app_ready_init.set_bubbles(false);
        app_ready_init.set_cancelable(false);
        app_ready_init.set_composed(false);
        app_ready_init.set_detail(&Self::register_object(storage));

        let app_ready_ev =
            CustomEvent::new_with_event_init_dict(WINDOW_APP_READY_EVENT_TYPE, &app_ready_init)
                .unwrap();

        self.window().dispatch_event(&app_ready_ev).unwrap();
    }

    pub fn register_wallet_event(&self, storage: StorageType) -> WalletResult<()> {
        let inner_storage = Rc::clone(&storage);

        let listener_closure = Closure::wrap(Box::new(move |custom_event: CustomEvent| {
            let detail = custom_event
                .detail()
                .dyn_into::<Function>()
                .map_err(|error| {
                    let outcome: WalletError = error.into();
                    outcome
                })
                .unwrap();

            Utils::jsvalue_to_error(detail.call1(
                &JsValue::null(),
                &Self::register_object(inner_storage.clone()),
            ))
            .unwrap()
        }) as Box<dyn Fn(_)>);

        let listener_fn = listener_closure
            .as_ref()
            .dyn_ref::<Function>()
            .ok_or(WalletError::CastClosureToFunction)?;

        self.window().add_event_listener_with_callback(
            crate::WINDOW_REGISTER_WALLET_EVENT_TYPE,
            listener_fn,
        )?;

        listener_closure.forget();

        Ok(())
    }

    pub fn register_object(storage: StorageType) -> Object {
        // The `register` function that logs and returns a closure like in your JS code
        let register =
            Closure::wrap(
                Box::new(move |value: JsValue| match Wallet::from_jsvalue(value) {
                    Ok(wallet) => {
                        let inner_outcome = storage.clone();

                        inner_outcome.borrow_mut().insert(
                            blake3::hash(wallet.name().to_lowercase().as_bytes()),
                            wallet,
                        );
                    }
                    Err(error) => {
                        web_sys::console::error_2(
                            &"REGISTER EVENT ERROR".into(),
                            &error.to_string().into(),
                        );
                    }
                }) as Box<dyn Fn(_)>,
            );

        // Create an object and set the `register` property
        let register_object = Object::new();

        if let Err(error) = Reflect::set(
            &register_object,
            &JsValue::from("register"),
            &register.into_js_value(),
        ) {
            web_sys::console::error_2(&"REGISTER EVENT ERROR".into(), &error);
        }

        register_object
    }
}
