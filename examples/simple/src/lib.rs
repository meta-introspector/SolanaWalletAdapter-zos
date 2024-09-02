use log::{info, trace, Level};
use solana_wallet_adapter::{PhantomWallet, WindowOps};
use std::panic;
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    match console_log::init_with_level(Level::Trace) {
        Ok(_) => (),
        Err(e) => trace!("{:?}", e),
    }

    let window_ops = WindowOps::new();
    let mut phantom_wallet = PhantomWallet::get_phantom(&window_ops).unwrap();
    info!("WALLET KEY:{:?}", &phantom_wallet);

    wasm_bindgen_futures::spawn_local(async move {
        let foo = phantom_wallet.connect().await;
        info!("PUBLIC KEY:{:?}", &foo);

        phantom_wallet.disconnect().await.unwrap();

        let sign_outcome = phantom_wallet
            .sign_message(&"custom message 405")
            .await
            .unwrap();

        info!("SIGNED OUTCOME: {:?}", &sign_outcome);
    });

    Ok(())
}
