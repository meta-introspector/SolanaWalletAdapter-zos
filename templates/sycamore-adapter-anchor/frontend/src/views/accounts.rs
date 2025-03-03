use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::{Cluster, ConnectionInfo};

use crate::{
    airdrop_svg,
    app::GlobalMessage,
    avatar_svg, balance_svg, check_svg, error_svg,
    fetch_parser::format_timestamp,
    link_target_blank, receive_svg, send_svg, signature_svg, timestamp_svg, token_ata_svg,
    token_mint_svg,
    types::{AccountState, ClusterNetState},
    utils::{format_address_url, format_tx_url, get_cluster_svg, trunk_cluster_name},
    views::{Airdrop, ConnectWalletFirst, ReceiveSol, SendSol},
    wallet_svg, ClusterStore, Loader, NotificationInfo,
};

#[component]
pub fn Accounts() -> View {
    let active_connection = use_context::<Signal<ConnectionInfo>>();
    let cluster_net_state = use_context::<Signal<ClusterNetState>>();

    create_memo(move || {
        cluster_net_state.get_clone();

        if let Ok(wallet_account) = active_connection.get_clone().connected_account() {
            if cluster_net_state.get() == ClusterNetState::Success {
                let address = wallet_account.address().to_string();

                spawn_local_scoped(async move {
                    let fetch_account_info = FetchAccountInfo {
                success_msg: Option::None,
                error_msg: Some(
                    "Unable to load the balance, token accounts and transactions for this account"
                        .to_string(),
                ),
                address,
            };

                    FetchAccountState(fetch_account_info);
                });
            }
        }
    });

    if active_connection.get_clone().connected_account().is_ok() {
        if cluster_net_state.get_clone() == ClusterNetState::Success {
            view! {
                (ClusterSuccess (active_connection.get_clone().clone()))
            }
        } else if cluster_net_state.get_clone() == ClusterNetState::Waiting {
            view! {"Loading account info..."}
        } else {
            view! {"CLUSTER NETWORK UNREACHABLE"}
        }
    } else {
        view! {ConnectWalletFirst {}}
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct SendModal(pub bool);

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct AirdropModal(pub bool);

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct ReceiveModal(pub bool);

#[component]
fn ClusterSuccess(connection_info: ConnectionInfo) -> View {
    let account_state = use_context::<Signal<AccountState>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();
    let loading = use_context::<Signal<Loading>>();
    let global_message = use_context::<Signal<GlobalMessage>>();

    let check_balance = move || {
        let balance = account_state
            .get_clone()
            .balance
            .as_str()
            .parse::<f64>()
            .unwrap_or_default();

        balance == f64::default()
    };

    let endpoint = cluster_storage
        .get_clone()
        .active_cluster()
        .endpoint()
        .to_string();

    let wallet_account_inner = connection_info
        .connected_account()
        .cloned()
        .unwrap_or_default();

    let shortened_address = wallet_account_inner
        .shorten_address()
        .unwrap_or_default()
        .to_string();

    let address = wallet_account_inner.address().to_string();

    let token_view = account_state
        .get_clone()
        .token_accounts()
        .iter()
        .map(|token_account| {
            TokenAccountCard(
                token_account.mint(),
                token_account.ata_address(),
                token_account.balance(),
                token_account.state(),
                cluster_storage,
            )
        })
        .collect::<Vec<View>>();

    let tx_view = account_state
        .get_clone()
        .transactions()
        .iter()
        .map(|tx| {
            TxCard(
                tx.signature.clone(),
                tx.block_time,
                tx.confirmation_status.clone(),
                tx.err.is_none(),
                address.clone(),
                cluster_storage,
            )
        })
        .collect::<Vec<View>>();

    let balance = account_state.get_clone().balance.clone();
    let clone_address = address.clone();
    let clone_address2 = address.clone();

    let show_receive_modal = use_context::<Signal<ReceiveModal>>();
    let show_airdrop_modal = use_context::<Signal<AirdropModal>>();
    let show_send_modal = use_context::<Signal<SendModal>>();

    view! {
    div (class="flex w-full h-full mt-4 mb-10 flex-col items-center"){
        div (class="shadow-sm p-5 w-full flex flex-col items-center mb-10 justify-center"){
            div(class="text-center w-full"){
                (if !loading.get().0{
                    view!{ span(class="text-3xl"){ (balance) " SOL" } }
                }else {
                    view!{ span(class="mr-2"){ (Loader()) }  span(class="text-sm"){ "Loading Balance..." }  }
                })
            }
            div (class="flex w-full items-center justify-center"){
                span (class="flex w-[20px] mr-1"){ img(src=wallet_svg()) }
                (link_target_blank(format_address_url(&clone_address, cluster_storage.get_clone().active_cluster()), shortened_address.clone()))
            }
            div (class="w-full flex gap-4 flex-wrap items-center justify-center"){
                (if !check_balance() {
                    view!{
                        button (
                            on:click=move|_|{show_send_modal.set(SendModal(true))},
                            class="flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue"){
                            span(class="w-[25px] flex mr-1"){ img(src=send_svg()) } "Send"
                        }
                    }
                }else {view!{}})
                button (
                    on:click=move|_|{show_receive_modal.set(ReceiveModal(true))},
                    class="flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue"){
                    span(class="w-[25px] flex mr-1"){ img(src=receive_svg()) } "Receive"
                }
                (if cluster_storage.get_clone().active_cluster().cluster() != Cluster::MainNet{
                    view!{button (
                        on:click=move|_|{show_airdrop_modal.set(AirdropModal(true))},
                        class="flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue"){
                        span(class="w-[25px] flex mr-1"){ img(src=airdrop_svg()) } "Airdrop"
                    }}
                }else {view!{}})
                button(
                    on:click=move|_|{
                        let endpoint = endpoint.clone();
                        let clone_address2 = clone_address2.clone();
                        account_state.set(AccountState::default());

                        spawn_local_scoped(async move{
                            let success_msg = Some("REFRESHED ACCOUNTS".to_string());
                            let error_msg = Some("REFRESH ERROR".to_string());
                            global_message.update(|store| store.push_back(NotificationInfo::new("REFRESHING ACCOUNTS")));

                            match crate::accounts_runner(&clone_address2, &endpoint).await {
                                Ok(value) => {
                                    account_state.set(value);
                                    if let Some(success_msg) = success_msg {
                                        global_message.update(|store| store.push_back(NotificationInfo::new(success_msg)));
                                    }
                                }
                                Err(error) => {
                                    if let Some(error_msg) = error_msg {
                                        global_message.update(|store| {
                                            store.push_back(NotificationInfo::error(format!("{error_msg}: {:?}", error)))
                                        });
                                    }
                                }
                            }
                        });
                    },
                    class="flex items-center text-sm text-white px-5 py-2 mt-5 rounded-full bg-true-blue hover:bg-cobalt-blue"){
                    span(class="w-[25px] flex mr-1"){ img(src=wallet_svg()) }
                        "Refresh"
                    }
                }
            }
        }
        div (class="flex flex-col flex-wrap w-full mt-5 text-2xl items-center justify-center"){
            div (class="flex items-center text-true-blue dark:text-white justify-center"){
                span(class="w-[30px] mr-1"){ img(src=token_ata_svg()) }

                (if !loading.get().0 {
                    if account_state.get_clone().token_accounts_is_empty(){
                       view!{ span(class="text-sm"){ "No Token Accounts Found" } }
                    }else {
                        view!{"Token Accounts"}
                    }
                }
                else {
                    view!{ span(class="mr-2"){ (Loader()) } "Loading Token Accounts..." }
                })
            }

            (token_view)
        }
        div (class="flex flex-col flex-wrap w-full mt-5 text-2xl items-center justify-center"){
            div (class="flex mb-5 items-center text-true-blue dark:text-white justify-center"){
                span(class="w-[30px] flex mr-1"){ img(src=signature_svg()) }
                (if !loading.get().0 {
                    if account_state.get_clone().transactions_is_empty(){
                        view!{ span (class="text-sm"){ "No Transactions Found" } }
                    }else {
                        view!{ "Transactions" }
                    }
                }else {
                    view!{ span(class="mr-2"){ (Loader()) } "Loading Transactions..." }
                })
            }
            div (class="flex w-full gap-4 flex-wrap items-center justify-center"){
                (tx_view)
            }
        }


    (SendSol())
    (ReceiveSol())
    (if cluster_storage.get_clone().active_cluster().cluster() != Cluster::MainNet{
        view!{(Airdrop())}
    }else{view!{}})
    }
}

fn TokenAccountCard(
    mint: String,
    ata_address: String,
    token_balance: String,
    state: String,
    cluster_store: Signal<ClusterStore>,
) -> View {
    let cluster = cluster_store.get_clone().active_cluster().cluster();
    let cluster_image = get_cluster_svg(cluster);

    let cluster_name = trunk_cluster_name(cluster_store.get_clone().active_cluster().name());

    let shortened_mint_address = wallet_adapter::Utils::shorten_base58(&mint)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Mint Address"));
    let shortened_ata_address = wallet_adapter::Utils::shorten_base58(&ata_address)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Owner Address"));

    view! {
        div (class="flex flex-col items-start p-4 w-[250px] m-5 rounded-lg bg-true-blue"){
            div (class="flex w-full items-center"){
                span(class="w-[28px] pr-2"){ img(src=cluster_image) }
                h5 (class="flex text-2xl"){ (cluster_name) }
            }
            div (class="flex flex-col w-full"){
                div (class="flex w-full items-start flex-col mt-2.5"){
                    div (class="w-full justify-between  flex"){
                        div (class="bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800"){
                            (cluster_store.get_clone().active_cluster().cluster().chain().to_string())
                        }
                        div (class="bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800"){
                            (state)
                        }
                    }

                    div (class="text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between"){
                        div (class="flex items-center"){
                            div(class="w-1/5"){ img(src=token_mint_svg()) }
                            div(class="w-4/5 flex text-sm pl-2"){ (link_target_blank(format_address_url(&mint, cluster_store.get_clone().active_cluster()), shortened_mint_address.clone()) ) }
                        }
                    }

                    div (class="text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between"){
                        div (class="flex items-center"){
                            div(class="w-1/5"){ img(src=token_ata_svg()) }
                            div(class="w-4/5 flex text-sm pl-2"){ (link_target_blank(format_address_url(&ata_address, cluster_store.get_clone().active_cluster()), shortened_ata_address.clone())) }
                        }
                    }

                    div (class="text-black text-lg dark:text-white mt-2 w-full flex flex-col items-start justify-between"){
                        div (class="flex items-center"){
                             div(class="w-2/5"){ img(src=balance_svg()) }
                             div(class="w-3/5 flex text-[12px] p-1"){ (token_balance) }
                        }
                    }
                }
            }
        }
    }
}

fn TxCard(
    tx: String,
    timestamp: Option<i64>,
    state: Option<String>,
    succeeded: bool,
    address: String,
    cluster_store: Signal<ClusterStore>,
) -> View {
    let cluster_image = get_cluster_svg(cluster_store.get_clone().active_cluster().cluster());

    let cluster_name = trunk_cluster_name(cluster_store.get_clone().active_cluster().name());

    let shortened_address = wallet_adapter::Utils::shorten_base58(&address)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Address"));

    let shortened_tx = wallet_adapter::Utils::shorten_base58(&tx)
        .map(|tx| tx.to_string())
        .unwrap_or(String::from("Invalid Address"));

    let succeeded = if succeeded { check_svg() } else { error_svg() };

    view! {
        div (class="flex rounded-lg flex-col items-start p-5 w-[250px] bg-true-blue"){
            div (class="flex items-center"){
                span(class="w-[28px] pr-2"){ img(src=cluster_image)}
                h5 (class="text-2xl font-semibold tracking-tight"){
                    (cluster_name)
                }
            }
            div (class="flex flex-col w-full"){
                div (class="flex w-full items-start flex-col mt-2.5"){
                    div (class="w-full justify-between items-start  flex"){
                        div (class="flex bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800"){
                            (cluster_store.get_clone().active_cluster().cluster().chain().to_string())
                        }
                        (if let Some(state_inner) = state {
                            view!{
                                div (class="flex bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800"){
                                (state_inner.to_uppercase())
                                }
                            }
                        }else {view!{}})
                    }

                    div (class="text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between"){
                        div (class="flex items-center"){
                            span (class="w-[20px]"){ img(src=avatar_svg()) }
                            span (class="flex text-sm pl-2"){
                                (link_target_blank(format_address_url(&address, cluster_store.get_clone().active_cluster()), shortened_address.clone()))
                            }
                        }
                    }

                    div (class="text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between"){
                        div (class="flex items-center"){
                            span (class="w-[20px]"){ img(src=signature_svg()) }
                            span (class="flex text-sm pl-2"){ (link_target_blank(format_tx_url(&tx, cluster_store.get_clone().active_cluster()), shortened_tx.clone())) }
                            div (class="flex items-center"){
                                span (class="ml-2 w-[15px]"){ img(src=succeeded) }
                            }
                        }
                    }

                    (if let Some(timestamp) = timestamp {
                        view!{
                            div (class="text-black text-lg dark:text-white mt-2 w-full flex flex-col items-start justify-between"){
                                div (class="flex items-center"){
                                    span (class="w-[30px]"){ img(src=timestamp_svg()) }
                                    span (class="flex text-[12px] p-1"){ (format_timestamp(timestamp)) }
                                }
                            }
                        }
                    }else {view!{}})
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct FetchAccountInfo {
    success_msg: Option<String>,
    error_msg: Option<String>,
    address: String,
}

#[component]
pub async fn FetchAccountState(fetch_account_info: FetchAccountInfo) -> View {
    let loading = use_context::<Signal<Loading>>();
    let account_state = use_context::<Signal<AccountState>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();
    let global_message = use_context::<Signal<GlobalMessage>>();

    account_state.set(AccountState::default());

    loading.set(Loading(true));

    let endpoint = cluster_storage
        .get_clone()
        .active_cluster()
        .endpoint()
        .to_string();

    match crate::accounts_runner(&fetch_account_info.address, &endpoint).await {
        Ok(value) => {
            account_state.set(value);
            if let Some(success_msg) = fetch_account_info.success_msg {
                global_message.update(|store| store.push_back(NotificationInfo::new(success_msg)));
            }
        }
        Err(error) => {
            if let Some(error_msg) = fetch_account_info.error_msg {
                global_message.update(|store| {
                    store.push_back(NotificationInfo::error(format!("{error_msg}: {:?}", error)))
                });
            }
        }
    }

    loading.set(Loading::default());

    view! {}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Loading(pub bool);
