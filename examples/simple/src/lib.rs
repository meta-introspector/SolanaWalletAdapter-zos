use log::Level;
use wallet_adapter::WalletAdapter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main(connect_ev_node: &str) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Trace).unwrap();

    log::info!("TARGET NODE: {}", connect_ev_node);

    wasm_bindgen_futures::spawn_local(async move {
        let mut adapter = WalletAdapter::init().unwrap();
        adapter.connect("solflare").await.unwrap();

        log::info!("CONNECTED WALLET: {:?}", &adapter.connected_wallet());
        log::info!("CONNECTED ACCOUNT: {:?}", &adapter.connected_account());

        // adapter.disconnect().await.unwrap();

        let signed_msg = adapter.sign_message(b"FOO").await.unwrap();
        log::info!("SIGNED MSG: {:?}", &signed_msg);
    });
}
