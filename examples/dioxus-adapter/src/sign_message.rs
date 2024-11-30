use dioxus::prelude::*;
use wallet_adapter::{SignedMessageOutput, WalletAdapter};

pub fn SignMessage(adapter: Signal<WalletAdapter>) -> Element {
    let mut signed_message_output: Signal<Option<SignedMessageOutput>> =
        use_signal(|| Option::None);
    let message = "Using Dioxus Framework";
    let sign_message_supported = adapter
        .read()
        .connected_wallet()
        .unwrap()
        .solana_sign_message();
    rsx! {
        div {class:"inner-section",
            if signed_message_output.read().is_none() {
                div {class:"inner-header", "MESSAGE TO SIGN"}
                div {class:"inner-body", "MESSAGE: {message}"}

                if sign_message_supported {
                    button{
                        id:"btn-primary",
                        onclick: move |_| {
                            spawn(async move {
                                let output = adapter.read().sign_message(message.as_bytes()).await.unwrap();
                                signed_message_output.write().replace(output);
                            });
                        },
                        "SIGN MESSAGE"
                    }
                }else {
                    button{
                        id:"btn-primary-disabled",
                        "SIGN MESSAGE UNSUPPORTED"
                    }
                }
            }else {
                div {class:"inner-header", "MESSAGE SIGNED"}
                div {class:"inner-body", "ADDRESS: {signed_message_output.read().unwrap().address().unwrap()}"}
                div {class:"inner-body", "MESSAGE: {signed_message_output.read().unwrap().message()}"}
                div {class:"inner-body", "SIGNATURE: {signed_message_output.read().unwrap().base58_signature().unwrap()}"}
            }
        }
    }
}
