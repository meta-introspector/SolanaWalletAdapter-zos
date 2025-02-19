use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::ConnectionInfo;

use crate::{
    app::GlobalMessage, close_svg, copy_svg, receive_svg, utils::copied_address, NotificationInfo,
};

use super::ReceiveModal;

#[component]
pub fn ReceiveSol() -> View {
    let show_receive_modal = use_context::<Signal<ReceiveModal>>();

    let active_connection = use_context::<Signal<ConnectionInfo>>();
    let global_message = use_context::<Signal<GlobalMessage>>();

    let mut address = String::default();
    let mut shortened_address = String::default();

    if let Ok(wallet_account) = active_connection.get_clone().connected_account() {
        address = wallet_account.address().to_string();
        shortened_address = wallet_account
            .shorten_address()
            .unwrap_or_default()
            .to_string();
    }

    let qrcode = crate::address_qrcode(&address);
    let address_inner = address.clone();

    if show_receive_modal.get().0 {
        view! {
            div (class="fixed overflow-y-hidden min-h-screen z-10 flex flex-col w-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white"){
                div (class="flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 lg:w-[90%] max-w-screen-sm justify-start items-center bg-white dark:bg-[#0b0414] rounded-3xl"){
                    div (class="flex w-full justify-end items-center p-5"){
                        button (on:click=move|_|{ show_receive_modal.set(ReceiveModal::default()) },
                            class="wallet-adapter-modal-button-close w-[30px] items-center justify-center"){
                            span(class="flex w-[20px]"){ img(src=close_svg()) }
                        }
                    }
                    div (class="overflow-y-scroll w-full mb-5 items-center justify-center flex flex-col"){
                        div (class="flex w-full items-center justify-center text-2xl"){ span(class="w-[30px] mr-2"){ img(src=receive_svg()) } "Receive SOL" }
                        div (class="mb-2 mt-5 rounded-full bg-true-blue hover:bg-cobalt-blue cursor-pointer",
                            on:click=move|_| {
                                let address_inner = address_inner.clone();

                                spawn_local_scoped(async move {
                                    if let Err(error) = copied_address(&address_inner).await {
                                        global_message.update(|store| store.push_back(NotificationInfo::error(format!("COPY ERROR: {:?}", error))));
                                    } else {
                                        global_message.update(|store| store.push_back(NotificationInfo::new("Copied to clipboard")));
                                    }
                                });
                            }){
                            div (class="flex justify-left items-center px-2 py-1 text-white rounded-full "){
                                span (class="flex p-2 w-[30px]"){ img(src=copy_svg()) }
                                span { (shortened_address) }
                            }
                        }
                        div (class="w-[200px] rounded-xl flex mt-5 mb-5 bg-white"){ img(src=qrcode) }
                    }
                }
            }
        }
    } else {
        view! {}
    }
}
