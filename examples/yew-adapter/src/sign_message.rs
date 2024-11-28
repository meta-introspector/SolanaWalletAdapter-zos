use wallet_adapter::SignedMessageOutput;
use yew::prelude::*;

use crate::AdapterActions;

#[function_component]
pub fn SignMessageComponent(controller: &AdapterActions) -> Html {
    let signed_message_output: UseStateHandle<Option<SignedMessageOutput>> =
        use_state(|| Option::None);
    let message = "Using Dioxus Framework";
    let sign_message_supported = controller.connected_wallet.solana_sign_message();
    let connected_wallet = controller.connected_wallet.clone();
    let connected_account = controller.connected_account.clone();

    html! {
        if signed_message_output.is_none() {
                <div class="inner-section">
                    <div class="inner-header"> {"MESSAGE TO SIGN"} </div>
                    <div class="inner-body"> {"MESSAGE: "} {message} </div>

                    if sign_message_supported {
                        <button id="btn-primary"
                            onclick={
                                Callback::from(move |_| {
                                    let connected_wallet = connected_wallet.clone();
                                    let connected_account = connected_account.clone();
                                    let signed_message_output = signed_message_output.clone();

                                    wasm_bindgen_futures::spawn_local(async move {
                                        let output = connected_wallet.sign_message(message.as_bytes(), &connected_account).await.unwrap();
                                        signed_message_output.set(Some(output));
                                    });
                                })
                                }
                        > {"SIGN MESSAGE"}
                        </button>
                    }else {
                        <button id="btn-primary-disabled"> {"SIGN MESSAGE UNSUPPORTED"}</button>
                    }
                </div>
        }else {
                <div class="inner-section">
                    <div class="inner-header"> {"MESSAGE SIGNED"}</div>
                    <div class="inner-body"> {"ADDRESS: "} {signed_message_output.clone().as_ref().unwrap().address().unwrap()}</div>
                    <div class="inner-body"> {"MESSAGE: "} {signed_message_output.clone().as_ref().unwrap().message()}</div>
                    <div class="inner-body"> {"SIGNATURE: "} {signed_message_output.clone().as_ref().unwrap().base58_signature().unwrap()}</div>
                </div>
            }
    }
}
