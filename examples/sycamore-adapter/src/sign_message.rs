use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::{SignedMessageOutput, WalletAdapter};

#[component]
pub fn SignMessageComponent() -> View {
    let adapter = use_context::<Signal<WalletAdapter>>();

    let signed_message_output: Signal<Option<SignedMessageOutput>> = create_signal(Option::None);
    let message = "Using Sycamore Framework";
    let sign_message_supported = adapter.get_clone().solana_sign_message().unwrap();

    view! {
        div(class="inner-section") {
            (if signed_message_output.get_clone().is_none() {

                view!{div{
                    div(class="inner-header"){ "MESSAGE TO SIGN"}
                    div(class="inner-body"){ "MESSAGE: " (message)}

                    (if sign_message_supported {
                        view!{button(class="btn-inner",
                            on:click={
                                move |_| {
                                spawn_local_scoped(async move {
                                    let output = adapter.get_clone().sign_message(message.as_bytes()).await.unwrap();
                                    signed_message_output.set(Some(output));
                                });
                            }},
                            ){"SIGN MESSAGE"}
                        }
                    }else {
                        view!{button(class="btn-inner-disabled"){"SIGN MESSAGE UNSUPPORTED"}}
                    })
                }}
            }else {
                view!{
                    div(class="inner-header"){ "MESSAGE SIGNED"}
                    div(class="inner-body"){ "ADDRESS: " (signed_message_output.get_clone().unwrap().address().unwrap())}
                    div(class="inner-body"){ "MESSAGE: " (signed_message_output.get_clone().unwrap().message().to_string())}
                    div(class="inner-body"){ "SIGNATURE: " (signed_message_output.get_clone().unwrap().base58_signature().unwrap())}
                }
            })
        }
    }
}
