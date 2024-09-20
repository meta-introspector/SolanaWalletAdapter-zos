use gloo_utils::format::JsValueSerdeExt;
use log::{info, trace, Level};
use serde::{Deserialize, Deserializer, Serialize};
use std::{cell::RefCell, collections::HashMap, panic, rc::Rc};
use wallet_adapter::WindowOps;
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::{
    js_sys::{self, global, Function, Object, Reflect},
    window, CustomEvent, CustomEventInit, Element, Event, EventTarget, Window,
};

// /// Register Wallet Event
pub const WINDOW_REGISTER_WALLET_EVENT_TYPE: &str = "wallet-standard:register-wallet";

///App Ready Event
pub const WINDOW_APP_READY_EVENT_TYPE: &str = "wallet-standard:app-ready";

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Wallet {
    accounts: Vec<String>,
    chains: Vec<String>,
    // features: HashMap<String, String>,
    icon: Option<String>,
    name: Option<String>,
    version: String,
}

fn features(value: &JsValue) {
    /*
        solana:signAndSendTransaction
    solana:signIn
    solana:signMessage
    solana:signTransaction
    standard:connect
    standard:disconnect
    standard:events
    */

    let features = Reflect::get(value, &"features".into()).unwrap();

    let featurs_as_object = features.as_ref().dyn_ref::<Object>().unwrap();

    for key in Object::keys(&featurs_as_object) {
        info!("ITER: {:?}", &key);
    }

    let signin = Reflect::get(&features, &"solana:signIn".into()).unwrap();
    let signin_version = Reflect::get(&signin, &"version".into()).unwrap();
    let signin_fn = Reflect::get(&signin, &"signIn".into()).unwrap();

    info!(
        "SIGNIN: version-{:?}, func-{:?}",
        &signin_version, signin_fn
    );
}

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).unwrap();

    let window = window().unwrap();

    let listener_closure = Closure::wrap(Box::new(move |custom_event: CustomEvent| {
        let detail = custom_event.detail().dyn_into::<Function>().unwrap();

        // The `register` function that logs and returns a closure like in your JS code
        let register = Closure::wrap(Box::new(move |value: JsValue| {
            let wallet = serde_wasm_bindgen::from_value::<Wallet>(value.clone()).unwrap();

            info!("VALUES:> {:?}", wallet);

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

        // Call the JavaScript function passed as `detail`, passing the `register_object`
        detail.call1(&JsValue::null(), &register_object).unwrap();
    }) as Box<dyn Fn(_)>);

    let listener_fn = listener_closure.as_ref().dyn_ref::<Function>().unwrap();

    window
        .add_event_listener_with_callback("wallet-standard:register-wallet", listener_fn)
        .unwrap();

    listener_closure.forget();
}
