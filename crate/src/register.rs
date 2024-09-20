// use core::fmt;
// use log::{info, trace, Level};
// use std::{cell::RefCell, collections::HashMap, panic, rc::Rc};
// use wasm_bindgen::{prelude::*, JsValue};
// use web_sys::{
//     js_sys::{self, global, wasm_bindgen, Function, Object, Reflect},
//     window, CustomEvent, CustomEventInit, Element, Event, Window,
// };

// use crate::WindowOps;

// pub const WALLET_VERSION: &str = "1.0.0";

pub const WINDOW_APP_READY_EVENT_TYPE: &str = "wallet-standard:app-ready";

pub const WINDOW_REGISTER_WALLET_EVENT_TYPE: &str = "wallet-standard:register-wallet";

// pub fn app_ready_event(window_ops: &WindowOps) {
//     let register_wallet_closure = Closure::wrap(Box::new(move |event: Event| {
//         let custom_event = event.dyn_ref::<CustomEvent>().unwrap();
//         let detail = custom_event.detail();
//         let detail_obj = detail.dyn_ref::<js_sys::Object>().unwrap();
//         let wallet_value = js_sys::Reflect::get(&detail_obj, &JsValue::from_str("wallet"))
//             .expect("Unable to fetch wallet");

//         info!("REGISTER> WALLET OBJECT: {:?}", wallet_value);
//     }) as Box<dyn FnMut(Event)>);

//     let global = global();
//     let register_wallet_function = register_wallet_closure.as_ref().unchecked_ref::<Function>();
//     js_sys::Reflect::set(
//         &window_ops.document().body().unwrap(),
//         &WINDOW_REGISTER_WALLET_EVENT_TYPE.into(),
//         &register_wallet_function.into(),
//     )
//     .expect("Failed to set `wallet register function` on global object");

//     //  window_ops.document().add_event_listener_with_callback(&WINDOW_REGISTER_WALLET_EVENT_TYPE, listener)
// }

// //WindowAppReadyEventAPI
// fn register(wallet: Wallet) {}

// // WindowRegisterWalletEventCallback
// fn window_register_wallet_event_callback() {
//     //api:
//     register(Wallet::default())
// }

// //WindowAppReadyEvent
// fn window_app_ready_event() {
//     unstopabble_custom_event(
//         WINDOW_APP_READY_EVENT_TYPE.into(),
//         register(Wallet::default()),
//     )
// }

// //WindowRegisterWalletEvent
// fn window_register_wallet_event() {
//     unstopabble_custom_event(
//         WINDOW_REGISTER_WALLET_EVENT_TYPE.into(),
//         window_register_wallet_event_callback(),
//     );
// }

// fn unstopabble_custom_event<T: fmt::Debug>(r#type: String, detail: T) {
//     //type
//     // detail
// }

// #[derive(Debug)]
// pub struct Record {
//     pub identifier: String,
//     pub value: String, //TODO: Be generic
// }

// #[derive(Debug, Default)]
// pub struct Wallet {
//     pub version: String,
//     pub name: String,
//     pub icon: String,
//     pub chains: String,
//     pub features: HashMap<String, Record>,
//     pub accounts: Vec<WalletAccount>,
// }

// #[derive(Debug)]
// pub struct WalletAccount {
//     pub address: String,
//     pub public_key: String,
//     pub chains: Vec<String>,
//     pub features: Vec<String>,
//     pub label: Option<String>,
//     pub icon: Option<String>,
// }
