use dioxus::prelude::*;
use wallet_adapter::{SignInOutput, SigninInput};

use crate::DioxusWalletAdapter;

pub fn SignInComponent() -> Element {
    let adapter: Signal<DioxusWalletAdapter> = use_context();

    let mut signin_output: Signal<Option<SignInOutput>> = use_signal(|| Option::None);
    let message = "DIOXUS LOGIN";
    let signin_supported = adapter.read().connection.solana_signin().unwrap();

    rsx! {
        if signin_output.read().is_none() {
            div {class:"inner-section",
                div {class:"inner-header", "SIGN IN DETAILS"},
                div {class:"inner-body", "ADDRESS: {adapter.read().connection.connected_account().unwrap().address}"}
                div {class:"inner-body", "MESSAGE: {message}"}
                if signin_supported {
                    button {class:"btn-inner",
                        onclick: move |_| {
                        let mut signin_input = SigninInput::new();
                        signin_input
                            .set_domain(adapter.read().connection.window())
                            .unwrap()
                            .set_statement(message)
                            .set_chain_id(wallet_adapter::Cluster::DevNet)
                            // NOTE: Some wallets require this field or the wallet adapter
                            // will return an error `MessageResponseMismatch` which is as
                            // a result of the sent message not corresponding with the signed message
                            .set_address(&adapter.read().connection.connected_account().unwrap().address)
                            .unwrap();

                        let public_key = adapter.read().connection.connected_account().unwrap().public_key;

                        spawn(async move {
                            let outcome = adapter.read().connection.sign_in(&signin_input, public_key).await.unwrap();
                            signin_output.set(Some(outcome));
                        });
                    },

                    "SIGN IN"
                    }
                }else {
                    button {
                        class:"btn-primary-disabled",
                        "SIWS Unsupported"
                    }
                }
            }
        }else {
            div {class:"inner-section",
                div {class:"inner-header", "SIGNED DETAILS"}
                div {class:"inner-body","ACCOUNT  : {signin_output.read().as_ref().unwrap().address()}"}
                div {class:"inner-body","MESSAGE  : {signin_output.read().as_ref().unwrap().message.as_str()}"}
                p {class:"inner-body", "SIGNATURE: {signin_output.read().as_ref().unwrap().signature()}"}
            }
        }
    }
}
