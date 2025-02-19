use sycamore::{futures::spawn_local, prelude::*};
use wallet_adapter::{ConnectionInfo, WalletAdapter};

use crate::{
    app::GlobalMessage, avatar_svg, close_svg, fetch_parser::send_sol_req, send_svg,
    utils::get_input_value, ClusterStore, Loader, NotificationInfo,
};

use super::SendModal;

#[component]
pub fn SendSol() -> View {
    let show_send_modal = use_context::<Signal<SendModal>>();

    let active_connection = use_context::<Signal<ConnectionInfo>>();

    let adapter = use_context::<Signal<WalletAdapter>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();
    let global_message = use_context::<Signal<GlobalMessage>>();

    let loading = create_signal(false);
    let address: Signal<Option<String>> = create_signal(Option::default());
    let lamports = create_signal(0u64);

    let mut public_key_bytes = [0u8; 32];

    let endpoint = cluster_storage
        .get_clone()
        .active_cluster()
        .endpoint()
        .to_string();
    let cluster = cluster_storage.get_clone().active_cluster().cluster();

    if let Ok(wallet_account) = active_connection.get_clone().connected_account() {
        public_key_bytes = wallet_account.public_key();
    }

    if show_send_modal.get().0 {
        view! {
            div (class="fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white"){
                div (class="flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-white dark:bg-[#0b0414] rounded-3xl"){
                    div (class="flex w-full justify-end items-center p-5"){
                        button (
                            on:click=move|_|{show_send_modal.set(SendModal::default())},
                            class="wallet-adapter-modal-button-close w-[30px] items-center justify-center",
                        ){ img(src=close_svg())}
                    }
                    div (class="overflow-y-scroll max-h-[90%] w-full mb-5 items-center justify-center flex flex-col"){
                        div (class="flex text-true-blue dark:text-white w-full items-center justify-center"){
                            span(class="w-[30px] flex mb-10 mr-2"){ img(src=send_svg()) }
                            span(class="flex mb-10 mr-2 text-3xl"){ "Send Lamports"}
                        }
                        div (class="flex flex-col w-3/5 mt-2 rounded-3x"){
                            div (class="flex w-full items-center rounded-xl p-1 bg-transparent"){
                                div (class="shrink-0 select-none text-base text-true-blue dark:text-white sm:text-sm/6 mb-10"){ "SOL" }
                                input (
                                    on:input=move |event: web_sys::Event| {
                                        if let Ok(data) = get_input_value(event).parse::<u64>(){
                                            lamports.set(data);
                                        }
                                    },
                                    class= "focus:outline-none mb-10 bg-transparent border-b-2 border-true-blue block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6",
                                    id="lamports",
                                    min="0",
                                    name="lamports",
                                    placeholder="50000000",
                                    r#type="number",
                                )
                            }
                            div (class="flex items-center rounded-xl p-1 bg-transparent"){
                                div (class="shrink-0 select-none text-base text-gray-500 sm:text-sm/6"){ span (class="flex w-[20px]"){ img(src=avatar_svg())  } }
                                input (
                                    on:input=move |event: web_sys::Event| {
                                        address.set(Some(get_input_value(event)));
                                    },
                                    class="w-full focus:outline-none bg-transparent border-b-2 border-true-blue block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6",
                                    id="address",
                                    min="0",
                                    name="address",
                                    r#type="text",
                                    placeholder="Enter Recipient Address",
                                )
                            }
                        }
                    div (class="flex w-full items-center justify-center mt-4"){
                            button (
                                disabled=loading.get() && address.get_clone().is_none(),
                                on:click=move|_|{
                                    let endpoint = endpoint.clone();
                                    spawn_local(async move {
                                        loading.set(true);

                                        if let Err(error) = send_sol_req(
                                            &address.get_clone().as_ref().cloned().unwrap_or_default(),
                                            lamports.get(),
                                            &endpoint,
                                            &adapter.get_clone(),
                                            public_key_bytes,
                                            cluster,
                                        ).await {
                                            global_message.update(|store| store.push_back(
                                                NotificationInfo::error(format!("SEND SOL ERROR: {:?}", error))
                                            ));
                                        }

                                        loading.set(false);
                                        show_send_modal.set(SendModal::default());

                                        global_message.update(|store| store.push_back(NotificationInfo::new("Sent")));

                                    });
                                },
                                class="flex bg-true-blue hover:bg-cobalt-blue text-sm mb-10  dark:stext-white text-black px-4 py-1 items-center justify-center rounded-full"){
                                (if loading.get() {
                                    view!{ (Loader()) span (class="text-white"){ "Sending SOL..." } }
                                }else {
                                    view! {"SEND SOL"}
                                })
                            }
                        }
                    }
                }
            }
        }
    } else {
        view! {}
    }
}
