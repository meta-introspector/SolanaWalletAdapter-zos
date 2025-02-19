use solana_sdk::native_token::LAMPORTS_PER_SOL;
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::ConnectionInfo;

use crate::{
    airdrop_svg, app::GlobalMessage, close_svg, fetch_parser::request_airdrop,
    utils::get_input_value, ClusterStore, Loader, NotificationInfo,
};

use super::AirdropModal;

#[component]
pub fn Airdrop() -> View {
    let active_connection = use_context::<Signal<ConnectionInfo>>();
    let global_message = use_context::<Signal<GlobalMessage>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();

    let show_airdrop_modal = use_context::<Signal<AirdropModal>>();

    let loading = create_signal(false);
    let lamports = create_signal(0u64);

    let mut address = String::default();

    let endpoint = cluster_storage
        .get_clone()
        .active_cluster()
        .endpoint()
        .to_string();

    if let Ok(wallet_account) = active_connection.get_clone().connected_account() {
        address = wallet_account.address().to_string();
    }

    if show_airdrop_modal.get().0 {
        view! {
            div (class="fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white"){
                div(class="flex flex-col bg-rich-black w-[60%] min-h-[40vh] rounded-lg p-5 shadow-2xl") {
                    div (class="flex w-full justify-end items-center p-5"){
                        button (on:click=move |_| {
                                show_airdrop_modal.set(AirdropModal::default());
                            },
                            class="wallet-adapter-modal-button-close w-[30px] items-center justify-center")
                        {
                            span(class="flex w-[20px]"){ img(src=close_svg()) }
                        }
                    }
                    div (class="overflow-y-scroll max-h-[90%] w-full mb-5 items-center justify-center flex flex-col"){
                        div (class="flex w-full items-center justify-center text-2xl"){
                            span(class="w-[50px] mr-2"){img(src=airdrop_svg())} "Request Airdrop"
                        }
                        div (class="mt-2 rounded-3x"){
                            div (class="flex items-center rounded-xl p-1 bg-transparent"){
                                div (class="shrink-0 select-none text-base text-gray-500 sm:text-sm/6"){"SOL" }
                                input (on:input=move |event: web_sys::Event| {
                                    let element =  get_input_value(event);

                                    let data = element.parse::<u64>().unwrap_or(1);
                                    lamports.set(data*LAMPORTS_PER_SOL);
                                    },
                                    class="focus:outline-none bg-transparent border-b-2 border-white block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6",
                                    id="airdrop", min="0", name="airdrop", placeholder="2", r#type="number", value="2",
                                )
                            }
                        }
                        div (class="flex-w-full items-center justify-center mt-4"){
                            button (disabled=(loading.get()),
                                on:click=move|_|{
                                    let endpoint = endpoint.clone();
                                    let address = address.clone();
                                    spawn_local_scoped(async move {
                                        loading.set(true);

                                        if request_airdrop(lamports.get(), &address, &endpoint).await.is_err() {
                                            global_message.update(|store|store.push_back(
                                                NotificationInfo::error("REQUEST AIRDROP ERROR: You might have reached your daily limit.")
                                            ));
                                        }else {
                                            global_message.update(|store|store.push_back(
                                                NotificationInfo::new("REQUESTED AIRDROP")
                                            ));
                                        }

                                        show_airdrop_modal.set(AirdropModal::default());
                                    });
                                },
                                class="flex text-sm bg-true-blue hover:bg-cobalt-blue text-white px-4 py-2 items-center justify-center rounded-full"){
                                (if loading.get() {
                                    view!{(Loader()) "Requesting Airdrop..."}
                                }else {
                                    view!{"REQUEST AIRDROP"}
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
