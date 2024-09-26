use log::{info, trace, Level};
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    panic,
    rc::Rc,
    sync::{LazyLock, Mutex},
};
use wallet_adapter::WindowOps;
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::{
    js_sys::{self, global, Function, Object, Reflect},
    CustomEvent, CustomEventInit, Document, Element, Event, EventTarget, Storage, Window,
};

const WINDOW: LazyLock<Window> = LazyLock::new(|| web_sys::window().unwrap());
const DOCUMENT: LazyLock<Document> = LazyLock::new(|| WINDOW.document().unwrap());

const DATA_STORE: LazyLock<Storage> = LazyLock::new(|| {
    WINDOW.local_storage().unwrap().unwrap() //TODO Tell user local storage doesn't exist
});

// /// Register Wallet Event
pub const WINDOW_REGISTER_WALLET_EVENT_TYPE: &str = "wallet-standard:register-wallet";

///App Ready Event
pub const WINDOW_APP_READY_EVENT_TYPE: &str = "wallet-standard:app-ready";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Wallet {
    accounts: Vec<String>,
    chains: Vec<String>,
    // features: HashMap<String, String>,
    icon: Option<String>,
    name: String,
    version: String,
}

fn features(_value: &JsValue) {
    /*
        solana:signAndSendTransaction
    solana:signIn
    solana:signMessage
    solana:signTransaction
    standard:connect
    standard:disconnect
    standard:events
    */

    // let features = Reflect::get(value, &"features".into()).unwrap();

    // let featurs_as_object = features.as_ref().dyn_ref::<Object>().unwrap();

    // for key in Object::keys(&featurs_as_object) {
    //     info!("ITER: {:?}", &key);
    // }

    // let signin = Reflect::get(&features, &"solana:signIn".into()).unwrap();
    // let signin_version = Reflect::get(&signin, &"version".into()).unwrap();
    // let signin_fn = Reflect::get(&signin, &"signIn".into()).unwrap();

    // info!(
    //     "SIGNIN: version-{:?}, func-{:?}",
    //     &signin_version, signin_fn
    // );
}

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).unwrap();

    local_storage();

    register_wallet_event();

    dispatch_app_event();
}

fn dispatch_app_event() {
    let app_ready_init = CustomEventInit::new();
    app_ready_init.set_bubbles(false);
    app_ready_init.set_cancelable(false);
    app_ready_init.set_composed(false);
    app_ready_init.set_detail(&register_object());

    let app_ready_ev =
        CustomEvent::new_with_event_init_dict(WINDOW_APP_READY_EVENT_TYPE, &app_ready_init)
            .unwrap();

    WINDOW.dispatch_event(&app_ready_ev).unwrap();
}

fn register_wallet_event() {
    let listener_closure = Closure::wrap(Box::new(move |custom_event: CustomEvent| {
        let detail = custom_event.detail().dyn_into::<Function>().unwrap();

        // Call the JavaScript function passed as `detail`, passing the `register_object`
        detail.call1(&JsValue::null(), &register_object()).unwrap();
    }) as Box<dyn Fn(_)>);

    let listener_fn = listener_closure.as_ref().dyn_ref::<Function>().unwrap();

    WINDOW
        .add_event_listener_with_callback("wallet-standard:register-wallet", listener_fn)
        .unwrap();

    listener_closure.forget();
}

fn register_object() -> Object {
    // The `register` function that logs and returns a closure like in your JS code
    let register = Closure::wrap(Box::new(move |value: JsValue| {
        let wallet = serde_wasm_bindgen::from_value::<Wallet>(value.clone()).unwrap();

        DATA_STORE.set_item(&wallet.name, &wallet.version).unwrap();

        features(&value);
    }) as Box<dyn Fn(_)>);

    // Create an object and set the `register` property
    let register_object = Object::new();
    Reflect::set(
        &register_object,
        &JsValue::from("register"),
        &register.into_js_value(), // Use the Rust closure as the register function
    )
    .unwrap();

    register_object
}

fn local_storage() {
    let items = serde_wasm_bindgen::from_value::<HashMap<String, String>>(
        DATA_STORE.dyn_ref::<JsValue>().unwrap().clone(),
    )
    .unwrap();

    let wallets = items
        .iter()
        .map(|(name, version)| {
            info!("WALLET:> {name}-{version}",);

            let node = DOCUMENT.create_element("li").unwrap();

            node.set_text_content(Some(name));

            node
        })
        .collect::<Vec<Element>>();

    let modal_node = Rc::new(DOCUMENT.get_element_by_id("modal").unwrap());
    let connect_node = Rc::new(DOCUMENT.get_element_by_id("connect").unwrap());

    let storage_ev = Closure::wrap(Box::new(move |_: Event| {
        if wallets.is_empty() {
            return;
        } else {
            let header = DOCUMENT.get_element_by_id("connected-wallets").unwrap();
            header.set_text_content(Some("Wallets Detected"));

            wallets.iter().for_each(|wallet_element| {
                modal_node.append_child(&wallet_element).unwrap();
            })
        }
    }) as Box<dyn Fn(_)>);

    let storage_ev_fn = storage_ev.as_ref().dyn_ref::<Function>().unwrap();

    connect_node
        .add_event_listener_with_callback("click", storage_ev_fn)
        .unwrap();

    storage_ev.forget();
}
