use std::{cell::RefCell, rc::Rc};

use async_channel::{Receiver, Sender};
use js_sys::{Function, Object, Reflect};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::{CustomEvent, CustomEventInit, Window};

use crate::{
    ConnectionInfo, Logger, StorageType, Utils, Wallet, WalletAccount, WalletAdapter, WalletError,
    WalletResult, WalletStorage, WINDOW_APP_READY_EVENT_TYPE,
};

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
    pub fn init(
        &self,
        adapter: &mut WalletAdapter,
        receiver: WalletEventReceiver,
    ) -> WalletResult<()> {
        let storage = adapter.storage();
        let connection_info = adapter.connection_info_inner();
        self.register_wallet_event(storage.clone_inner())?;
        self.dispatch_app_event(storage.clone_inner());

        spawn_local(async move {
            Logger::value(
                &(String::new()
                    + "Initializing the Receiver with bounded capacity of "
                    + &receiver.capacity().unwrap_or(0).to_string()
                    + "in a background thread...."),
            );

            if let Ok(wallet_event) = receiver.recv().await {
                let wallet = connection_info
                    .as_ref()
                    .borrow_mut()
                    .set_account(WalletAccount::default());
            } else {
                Logger::value("Error: the channel is empty and closed.");
            }
        });

        Ok(())
    }

    /// An App Ready event registered to the browser window
    pub fn dispatch_app_event(&self, storage: StorageType) {
        let app_ready_init = CustomEventInit::new();
        app_ready_init.set_bubbles(false);
        app_ready_init.set_cancelable(false);
        app_ready_init.set_composed(false);
        app_ready_init.set_detail(&Self::register_object(storage));

        let app_ready_ev =
            CustomEvent::new_with_event_init_dict(WINDOW_APP_READY_EVENT_TYPE, &app_ready_init)
                .unwrap();

        self.window.dispatch_event(&app_ready_ev).unwrap();
    }

    /// The register wallet event registered to the browser window
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

        self.window.add_event_listener_with_callback(
            crate::WINDOW_REGISTER_WALLET_EVENT_TYPE,
            listener_fn,
        )?;

        listener_closure.forget();

        Ok(())
    }

    pub(crate) fn register_object(storage: StorageType) -> Object {
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

/// Events emitted by connected browser extensions
/// when an account is connected, disconnected or changed.
/// Wallets implementing the wallet standard emit these events
/// from the `standard:events` events namespace specifically,
/// `wallet.features[standard:events].on`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum WalletEvent {
    /// An account has been connected and an event `change` emitted.
    Connected,
    /// An account has been disconnected and an event `change` emitted.
    Disconnected,
    /// An account has been connected and an event `change` emitted.
    /// The wallet adapter then updates the connected [WalletAccount].
    AccountChanged,
    /// Events can be asynchronous and happen within scopes of an event listner,
    /// these errors can be important in different scenarios. This enum field
    /// will help with that
    BackgroundTaskError(WalletError),
}

/// The `Sender` part of an [async_channel::bounded] channel
pub type WalletEventSender = Sender<WalletEvent>;

/// The `Receiver` part of an [async_channel::bounded] channel
pub type WalletEventReceiver = Receiver<WalletEvent>;
