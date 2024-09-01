use js_sys::{Function, Promise, Reflect};
use log::{info, trace, Level};
use std::panic;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Object;

use crate::{WalletAdapterError, WalletAdapterResult, WindowOps};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    match console_log::init_with_level(Level::Trace) {
        Ok(_) => (),
        Err(e) => trace!("{:?}", e),
    }

    let window_ops = WindowOps::new();
    let phantom_wallet = PhantomWallet::get_phantom(&window_ops).unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        let foo = phantom_wallet.connect().await;
        info!("PUBLIC KEY:{:?}", &foo);
    });

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PhantomWallet {
    is_phantom: bool,
    // window.phantom.solana.connect()
    connect: Function,
}

impl PhantomWallet {
    pub fn get_phantom(window_ops: &WindowOps) -> WalletAdapterResult<Self> {
        let entry = window_ops.get_entry("phantom");
        let get_phantom = Reflect::get(&entry.unwrap(), &"solana".into())?;
        let is_phantom = Reflect::get(&get_phantom, &"isPhantom".into())?
            .as_bool()
            .is_some();

        let connect = Reflect::get(&get_phantom, &"connect".into())?;

        if !connect.is_function() {
            info!("Expected call to window.phantom.solana to be a function");
            return Err(WalletAdapterError::PhantonSolanaConnectNotFunction);
        }

        // TODO: Redirect to phantom website if phantom is not detected
        // window.open('https://phantom.app/', '_blank');

        Ok(PhantomWallet {
            is_phantom,
            connect: connect.into(),
        })
    }

    pub async fn connect(&self) -> WalletAdapterResult<String> {
        let to_promise = Promise::resolve(&self.connect.call0(&JsValue::null())?);

        match JsFuture::from(to_promise).await {
            Ok(outcome) => {
                let public_key = Reflect::get(&outcome, &"publicKey".into())?;

                let public_key_as_object = Object::from(public_key);

                Ok(public_key_as_object.to_string().into())
            }
            Err(error) => {
                let code = Reflect::get(&error, &"code".into())?;

                let parsed_error = WalletAdapterError::parse_error_code(&code);

                match parsed_error {
                    WalletAdapterError::UnrecognizedError => return Err(error.into()),
                    _ => return Err(parsed_error),
                }
            }
        }
    }
}
