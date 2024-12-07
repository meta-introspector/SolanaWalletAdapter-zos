use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::{SignInOutput, SigninInput, WalletAdapter};

#[component]
pub fn SignInComponent() -> View {
    let adapter = use_context::<Signal<WalletAdapter>>();

    let signin_output: Signal<Option<SignInOutput>> = create_signal(Option::None);
    let message = "DIOXUS LOGIN";
    let signin_supported = adapter.get_clone().solana_signin().unwrap();
    let connected_account = adapter
        .get_clone()
        .connected_account()
        .cloned()
        .as_ref()
        .unwrap()
        .clone();

    let address = connected_account.address.to_string();
    let public_key = connected_account.public_key;

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

                    view!{button(class="btn-inner",
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

                        spawn_local_scoped(async move {
                            let outcome = adapter.get_clone().sign_in(&signin_input, public_key).await.unwrap();
                            signin_output.set(Some(outcome));
                        });
                    }){"SIGN IN"}}
                }else {
                    view!(button(class="btn-inner-disabled",){"SIWS Unsupported"})
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
