use sycamore::prelude::*;
use wallet_adapter::SignedMessageOutput;

use crate::Controller;

#[component]
pub fn SignMessage(controller: Controller) -> View {
    let signed_message_output: Signal<Option<SignedMessageOutput>> = create_signal(Option::None);
    let message = "Using Sycamore Framework";
    let sign_message_supported = controller.connected_wallet.solana_sign_message();

    let account = create_signal(controller.connected_account.clone());
    let wallet = create_signal(controller.connected_wallet.clone());

    view! {
        div {
            (if signed_message_output.get_clone().is_none() {

                view!{div(class="inner-section"){
                    div(class="inner-header"){ "MESSAGE TO SIGN"}
                    div(class="inner-body"){ "MESSAGE: " (message)}

                    (if sign_message_supported {
                        view!{button(id="btn-primary",
                            on:click={
                                move |_| {
                                wasm_bindgen_futures::spawn_local(async move {
                                    let output = wallet.get_clone().sign_message(message.as_bytes(), &account.get_clone()).await.unwrap();
                                    signed_message_output.set(Some(output));
                                });
                            }},
                            ){"SIGN MESSAGE"}
                        }
                    }else {
                        view!{button(id="btn-primary-disabled"){"SIGN MESSAGE UNSUPPORTED"}}
                    })
                }}
            }else {
                view!{
                    div(class="inner-section"){
                        div(class="inner-header"){ "MESSAGE SIGNED"}
                        div(class="inner-body"){ "ADDRESS: " (signed_message_output.get_clone().unwrap().address().unwrap())}
                        div(class="inner-body"){ "MESSAGE: " (signed_message_output.get_clone().unwrap().message().to_string())}
                        div(class="inner-body"){ "SIGNATURE: " (signed_message_output.get_clone().unwrap().base58_signature().unwrap())}
                    }
                }
            })
        }
    }
}
