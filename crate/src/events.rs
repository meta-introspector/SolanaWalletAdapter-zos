use std::rc::Rc;

use async_channel::{Receiver, Sender};
use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CustomEvent, CustomEventInit, Window};

use crate::{
    ConnectionInfoInner, StorageType, Utils, Wallet, WalletAdapter, WalletError, WalletResult,
    WINDOW_APP_READY_EVENT_TYPE,
};

/// The `Sender` part of an [async_channel::bounded] channel
pub type WalletEventSender = Sender<WalletEvent>;

/// The `Receiver` part of an [async_channel::bounded] channel
pub type WalletEventReceiver = Receiver<WalletEvent>;

/// Used to initialize the `Register` and `AppReady` events to the browser window
#[derive(Debug, PartialEq, Eq)]
pub struct InitEvents<'a> {
    window: &'a Window,
}

impl<'a> InitEvents<'a> {
    /// Instantiate [Self](self)
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    /// Register events by providing a [WalletStorage] that is used to store
    /// all registered wallets
    pub fn init(&self, adapter: &mut WalletAdapter, sender: WalletEventSender) -> WalletResult<()> {
        let storage = adapter.storage();
        let connection_info = adapter.connection_info_inner();
        self.register_wallet_event(
            storage.clone_inner(),
            connection_info.clone(),
            sender.clone(),
        )?;
        self.dispatch_app_event(
            storage.clone_inner(),
            connection_info.clone(),
            sender.clone(),
        );

        Ok(())
    }

    /// An App Ready event registered to the browser window
    pub fn dispatch_app_event(
        &self,
        storage: StorageType,
        connection_info: ConnectionInfoInner,
        sender: WalletEventSender,
    ) {
        let app_ready_init = CustomEventInit::new();
        app_ready_init.set_bubbles(false);
        app_ready_init.set_cancelable(false);
        app_ready_init.set_composed(false);
        app_ready_init.set_detail(&Self::register_object(storage, connection_info, sender));

        let app_ready_ev =
            CustomEvent::new_with_event_init_dict(WINDOW_APP_READY_EVENT_TYPE, &app_ready_init)
                .unwrap();

        self.window.dispatch_event(&app_ready_ev).unwrap();
    }

    /// The register wallet event registered to the browser window
    pub fn register_wallet_event(
        &self,
        storage: StorageType,
        connection_info: ConnectionInfoInner,
        sender: WalletEventSender,
    ) -> WalletResult<()> {
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
                &Self::register_object(
                    inner_storage.clone(),
                    connection_info.clone(),
                    sender.clone(),
                ),
            ))
            .unwrap()
        }) as Box<dyn Fn(_)>);

        let listener_fn = listener_closure
            .as_ref()
            .dyn_ref::<Function>()
            .ok_or(WalletError::CastClosureToFunction)?;

        self.window.add_event_listener_with_callback(
            crate::WINDOW_REGISTER_WALLET_EVENT_TYPE,
            listener_fn,
        )?;

        listener_closure.forget();

        Ok(())
    }

    pub(crate) fn register_object(
        storage: StorageType,
        connection_info: ConnectionInfoInner,
        sender: WalletEventSender,
    ) -> Object {
        // The `register` function that logs and returns a closure like in your JS code
        let register = Closure::wrap(Box::new(move |value: JsValue| {
            match Wallet::from_jsvalue(value, sender.clone(), connection_info.clone()) {
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
            }
        }) as Box<dyn Fn(_)>);

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

/// Events emitted by connected browser extensions
/// when an account is connected, disconnected or changed.
/// Wallets implementing the wallet standard emit these events
/// from the `standard:events` events namespace specifically,
/// `wallet.features[standard:events].on`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum WalletEvent {
    /// An account has been connected and an event `change` emitted.
    Connected,
    /// An account has been disconnected and an event `change` emitted.
    Disconnected,
    /// An account has been connected and an event `change` emitted.
    /// The wallet adapter then updates the connected [WalletAccount].
    AccountChanged,
}
