use log::Level;
use wallet_adapter::WalletAdapter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(connect_ev_node: &str) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).unwrap();

    log::info!("TARGET NODE: {}", connect_ev_node);

    WalletAdapter::init().unwrap().execute();
}
