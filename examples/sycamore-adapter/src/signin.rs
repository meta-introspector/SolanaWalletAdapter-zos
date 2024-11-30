use sycamore::prelude::*;
use wallet_adapter::{SignInOutput, SigninInput};

use crate::Controller;

#[component]
pub fn SignIn(controller: Controller) -> View {
    let signin_output: Signal<Option<SignInOutput>> = create_signal(Option::None);
    let message = "DIOXUS LOGIN";
    let signin_supported = controller.connected_wallet.solana_signin();
    let address = controller.connected_account.address.clone();
    let public_key = controller.connected_account.public_key;
    let connected_wallet = create_signal(controller.connected_wallet.clone());

    view! {
        (if signin_output.get_clone().is_none() {
            let address_inner = address.clone();
            let address_input = address.clone();
            view!{div(class="inner-section"){
                div(class="inner-header"){ "SIGN IN DETAILS"}
                div(class="inner-body"){ "ADDRESS: " (address_inner)}
                div(class="inner-body"){ "MESSAGE: " (message)}
                (if signin_supported {
                    let address_input = address_input.clone();

                    view!{button(class="btn-primary",
                        on:click=move |_| {
                        let mut signin_input = SigninInput::new();
                        signin_input
                            .set_domain(&window()).unwrap()
                            .set_statement(message)
                            .set_chain_id(wallet_adapter::Cluster::DevNet)
                            // NOTE: Some wallets require this field or the wallet adapter
                            // will return an error `MessageResponseMismatch` which is as
                            // a result of the sent message not corresponding with the signed message
                            .set_address(&address_input.clone())
                            .unwrap();


                        wasm_bindgen_futures::spawn_local(async move {
                            let outcome = connected_wallet.get_clone().sign_in(&signin_input, public_key).await.unwrap();
                            signin_output.set(Some(outcome));
                        });
                    }){"SIGN IN"}}
                }else {
                    view!(button(class="btn-primary-disabled",){"SIWS Unsupported"})
                })
            }}
        }else {
            let output = signin_output.clone().get_clone().unwrap().clone();
            let address = output.address().to_string();
            let signature = output.signature();
            let message = output.message;

            view!{div(class="inner-section"){
                div(class="inner-header"){ "SIGNED DETAILS"}
                div(class="inner-body"){"ACCOUNT  :" (address)}
                div(class="inner-body"){"MESSAGE  : " (message)}
                p(class="inner-body"){ "SIGNATURE: " (signature)}
            }}
        })
    }
}
