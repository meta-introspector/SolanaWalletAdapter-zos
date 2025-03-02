use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::{
    web_sys::{self},
    ConnectionInfo, WalletAdapter,
};

use crate::{
    app::{logo, GlobalMessage},
    change_wallet_svg, close_svg, clusters_svg, copy_svg, disconnect_svg, gradient_wallet_svg,
    types::ClusterNetState,
    utils::{copied_address, get_select_value, trunk_cluster_name},
    wallet_svg, AdapterCluster, ClusterStore, FetchReq, Loader, NotificationInfo,
};

#[derive(Debug, Clone, Copy, Default)]
struct ShowModal(bool);

#[derive(Debug, Clone, Copy, Default)]
pub struct ShowConnecting(pub bool);

#[component]
pub fn Header() -> View {
    provide_context(create_signal(ShowModal::default()));
    provide_context(create_signal(ShowConnecting::default()));

    let cluster_net_state = use_context::<Signal<ClusterNetState>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();

    view! {
        div (class="flex flex-col w-full gap-4 justify-between items-center"){
            nav (class="flex w-full justify-around items-center p-1 dark:shadow-lg shadow-sm border-b-[1px] dark:border-true-blue"){
                div(class="p-1 w-[15%]"){ img(src=logo, alt="LOGO"){} }
                div (class="flex items-center justify-around w-[80%] mx-2"){
                    (NavItem("", "Home"))
                    (NavItem("accounts", "Accounts"))
                    (NavItem("clusters", "Clusters"))
                    (NavItem("extras", "Extras"))
                    (NavClusterItem())
                }
                (NavWalletItem())
            }
            (ping_cluster())
            (if cluster_net_state.get() == ClusterNetState::Failure {
                view! {
                    div (class="flex w-full justify-center h-[40px] bg-red-800 text-center items-center text-2xl"){
                        div(class="flex px-4 py-2 justify-center items-center"){
                            div(class="flex w-full mr-2"){
                                span (class="flex w-[30px] mr-1 text-white"){
                                    img(src=clusters_svg())
                                }
                                (cluster_storage.get_clone().active_cluster().name().to_string()) " cluster is unreachable!"
                            }
                            button (
                                on:click=move|_| {
                                    let active_cluster = cluster_storage.get_clone().active_cluster().clone();
                                    cluster_storage.update(|store| {store.set_active_cluster(active_cluster);});
                                },
                                class="flex bg-true-blue items-center justify-center text-sm text-white px-2 py-1 rounded-full hover:bg-cobalt-blue"
                            ){
                                "REFRESH"
                            }
                        }
                    }
                }
            }else {view! {}})
        }

        (ConnectWalletModalModal())
    }
}

#[component]
pub fn ConnectWalletModalModal() -> View {
    let global_message = use_context::<Signal<GlobalMessage>>();

    let show_modal = use_context::<Signal<ShowModal>>();
    let show_connecting = use_context::<Signal<ShowConnecting>>();

    let adapter = use_context::<Signal<WalletAdapter>>();

    let wallet_list = adapter
        .get_clone()
        .wallets()
        .into_iter()
        .map(|wallet| {
            let wallet_name = wallet.name();

            let wallet_name_alt = wallet_name.to_string();
            let wallet_name_text = wallet_name.to_string();
            let icon = wallet.icon().cloned();
            view! {
                    li (
                    on:click=move|_|{
                        let wallet = wallet.clone();

                        spawn_local_scoped(async move {
                            let mut adapter_inner = adapter.get_clone().clone();
                            if let Err(error) = adapter_inner.connect(wallet.clone()).await{
                                global_message.update(|store| store.push_back(NotificationInfo::error(error)));
                            }
                            adapter.set(adapter_inner);

                            show_modal.set(ShowModal(false));
                            show_connecting.set(ShowConnecting(false));
                        });
                    },
                    class="flex justify-center cursor-pointer w-full text-lg hover:bg-true-blue  text-true-blue hover:text-white dark:text-white px-4 py-2"
                ){
                    (view!{
                        div(class="max-w-[80%] flex justify-between w-full"){
                            div (class="flex items-center"){
                                (if let Some(icon) = icon {
                                    view!{img (class="flex w-[25px] mr-2 items-center", src=icon.to_string(), alt=wallet_name_alt)}
                                }else {
                                    view!{span (class="flex w-[25px] mr-2 items-center"){ img(src=wallet_svg()) }}
                                })
                                span (class="flex"){ (wallet_name_text)  }
                            }
                            span (class="flex"){"Detected"}
                        }
                    })
                }
            }
        })
        .collect::<Vec<View>>();

    if show_modal.get().0 {
        let wallets_exist = adapter.get_clone().wallets().is_empty();
        view! {
            div (class="flex flex-col w-full h-full bg-[#1a1a1a88] absolute items-center justify-center z-50"){

                div (class="flex relative w-full max-w-[90%] lg:max-w-[40%] md:max-w-[55%] max-h-full"){
                    (if !wallets_exist{
                        view!{
                            div (class="relative bg-white rounded-lg shadow-lg dark:bg-rich-black items-center justify-between flex flex-col w-full h-full min-h-[40vh]"){
                            div (class="flex items-center justify-center p-4 md:p-5 rounded-t w-full dark:border-gray-600 border-gray-200"){
                                div (class="flex w-5/6 items-center justify-center"){
                                    h3 (class="text-2xl flex items-center justify-center font-semibold text-blue-yonder dark:text-white"){
                                        span(class="w-[30px] mr-2 flex"){ img(src=gradient_wallet_svg()) } "Connect A Wallet"
                                    }
                                }
                                div (
                                    on:click=move|_| {show_modal.set(ShowModal(false));},
                                    class="flex w-1/6"){
                                    button (class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white",
                                        r#type= "button"){
                                        img(src=close_svg)
                                        span (class="sr-only"){"Close modal" }
                                    }
                                }
                            }
                            ul (class="flex space-y-4 mb-5 w-full justify-center flex-col items-center h-full"){
                                (wallet_list)
                            }
                        }
                    }
                    }else {
                        view!{
                            div (class="relative bg-white rounded-lg shadow-lg dark:bg-rich-black items-center justify-start p-2 flex flex-col w-full h-full min-h-[40vh]"){
                                div (class="flex w-full mr-5"){
                                    button (
                                        on:click=move|_|{show_modal.set(ShowModal(false));},
                                        class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline- justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white",
                                        "data-modal-hide"="default-modal",
                                        r#type="button"){
                                        img(src=close_svg)
                                        span (class="sr-only"){"Close modal" }
                                    }
                                }
                                div(class="flex text-2xl w-full p-5 flex-col items-center justify-around h-full"){
                                    div(class="flex w-full items-center justify-center"){
                                        span(class="flex w-[50px] mr-5 items-center"){ img(src=gradient_wallet_svg())}
                                        span{"No Solana Wallets Detected"}
                                    }
                                    div (class="flex text-lg"){"Install a Solana Wallet Installed on your browser!"}
                                }
                            }
                        }
                    })
                }
            }
        }
    } else {
        view! {}
    }
}

fn NavItem(route: &'static str, text: &'static str) -> View {
    view! {
        li (class="list-none"){
            a(href=(String::from("/") + route),
            class="w-[10%] hover:bg-transparent dark:text-blue-yonder dark:hover:text-white text-true-blue hover:text-black rounded-lg text-center p-1")
            { (text) }
        }
    }
}

#[component]
fn NavClusterItem() -> View {
    let cluster_net_state = use_context::<Signal<ClusterNetState>>();
    let global_message = use_context::<Signal<GlobalMessage>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();

    let cluster_options = cluster_storage
        .get_clone()
        .get_clusters()
        .iter()
        .map(|adapter_cluster: &AdapterCluster| {
            let name = trunk_cluster_name(adapter_cluster.name());

            let is_active_cluster = adapter_cluster.name().as_bytes()
                == cluster_storage
                    .get_clone()
                    .active_cluster()
                    .name()
                    .as_bytes();
            let name_value = name.clone();
            view! {
                option(
                    value=name_value,
                    selected=is_active_cluster
                ){(view!{(name.clone())})}
            }
        })
        .collect::<Vec<View>>();

    view! {
        div (class="flex w-[15%]"){
            select(
                on:change=move |event: web_sys::Event| {
                    let value = get_select_value(event);
                    cluster_net_state.set(ClusterNetState::Waiting);


                    let cluster = cluster_storage.get_clone().get_cluster(&value).cloned().unwrap_or_default();
                    let cluster_identifier = String::new() + cluster.name() + " cluster now active!";
                    cluster_storage.update(|store| {store.set_active_cluster(cluster);});

                    global_message.update(|store| store.push_back(NotificationInfo::new(cluster_identifier)));
                },
                class="flex text-sm hover:bg-true-yonder bg-true-blue text-white rounded-full py-1 px-4 appearance-none text-center cursor-pointer"){
                (cluster_options)
            }
        }
    }
}

#[component]
fn NavWalletItem() -> View {
    let active_connection = use_context::<Signal<ConnectionInfo>>();

    let show_modal = use_context::<Signal<ShowModal>>();

    view! {
        div (class="w-[25%] flex ml-2 text-white py-1 px-4 appearance-none items-center justify-center cursor-pointer"){
            (if show_modal.get().0 {
                view!{
                    div(class="py-1 px-4 flex items-center justify-center hover:bg-true-yonder bg-true-blue rounded-full"){
                        span(class="flex w-[20px] mr-5"){ img(src=wallet_svg()) }
                        span(class="flex mr-5"){ Loader() }
                    }
                }
            } else {
                view!{
                    div(class="flex hover:bg-true-yonder bg-true-blue text-white rounded-full py-1 px-4 appearance-none text-center cursor-pointer"){
                        (if let Ok(connected_account) = active_connection.get_clone().connected_account() {
                            let shortened_address = connected_account.shorten_address().unwrap().to_string();

                            view! { (ActiveAccountDropDown(shortened_address.to_string())) }
                        } else {
                            view! {
                                div (on:click=move|_|{show_modal.set(ShowModal(true));},
                                    class="flex w-full items-center justify-center cursor-pointer"){
                                    button (
                                        class="text-sm cursor-pointer",
                                    ){
                                        "Select Wallet"
                                    }
                                }
                            }
                        })
                    }
                }
            })
        }
    }
}

#[component]
pub fn ActiveAccountDropDown(shortened_address: String) -> View {
    let show_dropdown = create_signal(false);

    let show_modal = use_context::<Signal<ShowModal>>();

    let adapter = use_context::<Signal<WalletAdapter>>();
    let active_connection = use_context::<Signal<ConnectionInfo>>();
    let global_message = use_context::<Signal<GlobalMessage>>();

    let connected_wallet = active_connection
        .get_clone()
        .connected_wallet()
        .unwrap()
        .clone();

    let icon = connected_wallet.icon().cloned();

    let shortened_address_inner = shortened_address.clone();

    view! {
        div (class="relative inline-block rounded-full"){
            div (
                on:click=move|_| {
                    if show_dropdown.get() {
                        show_dropdown.set(false);
                    }else {
                        show_dropdown.set(true);
                    }
                },
                class="flex w-full text-center items-center justify-center"){
                span(class="flex w-[20px] mr-2"){
                    (if let Some(icon) = icon {
                        view!{img(class="rounded-lg", src=icon.to_string())}
                    }else {
                        view!{img(src=wallet_svg())}
                    })
                }
                (shortened_address)
            }

            (if show_dropdown.get() {
                let clone_address = shortened_address_inner.clone();
                let shortened_address_copy = clone_address.clone();
                view!{
                    ul (class="w-full min-w-[130px] text-white flex flex-col absolute z-1 text-md mt-2 bg-true-blue rounded-lg shadow-xl list-none"){
                            li(class="flex w-full mb-2 mt-2 hover:bg-cobalt-blue cursor-pointer",
                                on:click=move|_| {
                                    let clone_address = clone_address.clone();

                                    show_dropdown.set(false);
                                    spawn_local_scoped(async move {
                                        if let Err(error) = copied_address(&clone_address).await {
                                            global_message.update(|message_store| {
                                                message_store.push_back(NotificationInfo::new(error));
                                            });
                                        } else {
                                            global_message.update(|message_store| {
                                                message_store.push_back(NotificationInfo::new("Copied to clipboard"));
                                            });
                                        }
                                    });
                                }){
                                div (class="flex text-sm justify-left items-center "){
                                    span (class="p-2 w-[30px]"){ img(src=copy_svg()) }
                                    span { (shortened_address_copy) }
                                }
                            }

                        li(class="flex w-full mb-2 mt-2 hover:bg-cobalt-blue cursor-pointer",
                            on:click=move|_| {
                                show_dropdown.set(false);
                                show_modal.set(ShowModal(true));
                            }){
                            div (class="flex text-sm justify-left items-center "){
                                span (class="p-2 w-[30px]"){ img(src=change_wallet_svg()) }
                                span {"Change Wallet"}
                            }
                        }

                        li(class="flex w-full mb-2 mt-2 hover:bg-cobalt-blue cursor-pointer",
                            on:click=move|_| {
                                spawn_local_scoped(async move {
                                    let mut adapter_inner = adapter.get_clone().clone();
                                    adapter_inner.disconnect().await;
                                    adapter.set(adapter_inner);

                                    show_modal.set(ShowModal(false));
                                });
                            }){
                            div (class="flex text-sm justify-left items-center "){
                                span (class="p-2 w-[30px]"){ img(src=disconnect_svg()) }
                                span {"Disconnect"}
                            }
                        }
                    }
                }
            }else {view!{}})
        }
    }
}

fn ping_cluster() {
    let cluster_net_state = use_context::<Signal<ClusterNetState>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();

    create_memo(move || {
        cluster_storage.get_clone();

        spawn_local_scoped(async move {
            cluster_net_state.set(ClusterNetState::Waiting);

            let net_state =
                FetchReq::ping(cluster_storage.get_clone().active_cluster().endpoint()).await;

            cluster_net_state.set(net_state);
        });
    });
}
