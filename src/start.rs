use js_sys::{Function, Promise, Reflect, Uint8Array};
use log::{info, trace, Level};
use std::panic;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PhantomWallet {
    is_phantom: bool,
    // window.phantom.solana.connect()
    connect: Function,
    disconnect: Function,
    sign_message: Function,
}

impl PhantomWallet {
    pub fn get_phantom(window_ops: &WindowOps) -> WalletAdapterResult<Self> {
        let entry = window_ops.get_entry("phantom");
        let get_solana = Reflect::get(&entry.unwrap(), &"solana".into())?;
        let is_phantom = Reflect::get(&get_solana, &"isPhantom".into())?
            .as_bool()
            .is_some();

        let connect = Reflect::get(&get_solana, &"connect".into())?;

        if !connect.is_function() {
            info!("Expected call to window.phantom.solana.connect to be a function");
            return Err(WalletAdapterError::ExpectedAFunction("connect".into()));
        }

        let disconnect = Reflect::get(&get_solana, &"disconnect".into())?;

        if !disconnect.is_function() {
            info!("Expected call to window.phantom.solana.disconnect to be a function");
            return Err(WalletAdapterError::ExpectedAFunction("disconnect".into()));
        }

        let sign_message = Reflect::get(&get_solana, &"signMessage".into())?;

        if !sign_message.is_function() {
            info!("Expected call to window.phantom.solana.signMessage to be a function");
            return Err(WalletAdapterError::ExpectedAFunction("signMessage".into()));
        }

        // TODO: Redirect to phantom website if phantom is not detected
        // window.open('https://phantom.app/', '_blank');

        Ok(PhantomWallet {
            is_phantom,
            connect: connect.into(),
            disconnect: disconnect.into(),
            sign_message: sign_message.into(),
        })
    }

    pub async fn connect(&self) -> WalletAdapterResult<String> {
        let to_promise = Promise::resolve(&self.connect.call0(&JsValue::null())?);

        match JsFuture::from(to_promise).await {
            Ok(outcome) => {
                let is_connected = Reflect::get(&outcome, &"isConnected".into())?;

                info!("IS CONNECTED: {:?}", is_connected);

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

    pub async fn disconnect(&self) -> WalletAdapterResult<()> {
        let to_promise = Promise::resolve(&self.disconnect.call0(&JsValue::null())?);

        match JsFuture::from(to_promise).await {
            Ok(outcome) => {
                info!("DISCONNECT: {:?}", &outcome);
            }
            Err(error) => {
                info!("DISCONNECT: {:?}", &error);
            }
        }

        Ok(())
    }

    pub async fn sign_message(&self, message: &str) -> WalletAdapterResult<([u8; 64], String)> {
        let message_js_array = Uint8Array::from(message.as_bytes());
        let to_promise = Promise::resolve(
            &self
                .sign_message
                .call1(&JsValue::null(), &message_js_array.into())?,
        );
        match JsFuture::from(to_promise).await {
            Ok(response) => {
                let signature: Uint8Array = Reflect::get(&response, &"signature".into())?.into();
                let public_key_obj = Reflect::get(&response, &"publicKey".into())?;

                // Call the `toString` method on a javascript object
                let public_key = Reflect::get(&public_key_obj, &JsValue::from_str("toString"))?
                    .dyn_into::<js_sys::Function>()?
                    .call0(&public_key_obj)?
                    .as_string()
                    .ok_or_else(|| JsValue::from_str("Expected publicKey to be a string"))?; //TODO check if this can be handled via error enum

                let mut signature_buffer = [0u8; 64];

                if signature.length() != 64 {
                    return Err(WalletAdapterError::InvalidSignatureBytes);
                }

                signature.copy_to(&mut signature_buffer);

                Ok((signature_buffer, public_key))
            }
            Err(error) => {
                info!("error: {:?}", &error);

                Err(WalletAdapterError::parse_error_code(&error))
            }
        }
    }
}
