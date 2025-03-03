use wallet_adapter::{Wallet, WalletAccount};
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::Link;

use crate::{
    app::{logo, Route},
    copied_address, get_select_value, trunk_cluster_name, AccountInfoState, ChangeWalletSvg,
    CloseSvg, ClusterNetState, ClusterStoreActions, ClusterStoreState, ClustersSvg, CopySvg,
    DisconnectSvg, GlobalAction, GlobalAppState, GradientWalletSvg, Loader, NetState,
    NetStateOptions, NotificationInfo, WalletSvg,
};

pub fn header(
    trigger: UseStateHandle<bool>,
    cluster_store_state: ClusterStoreState,
    net_state: NetState,
    global_state: GlobalAppState,
    account_info_state: AccountInfoState,
) -> Html {
    html! {
            <div class="flex flex-col w-full gap-4 justify-between items-center">
                <nav class="flex w-full justify-around items-center p-1 dark:shadow-lg shadow-sm border-b-[1px] dark:border-true-blue">
                    <div class="p-1 w-[15%]"> <img src={logo()} alt="LOGO"/></div>
                    <div class="flex items-center justify-around w-[80%] mx-2">
                        {NavItem(Route::Home, "Home")}
                        {NavItem(Route::Accounts, "Accounts")}
                        {NavItem(Route::Clusters, "Clusters")}
                        {NavItem(Route::Extras, "Extras")}
                        {NavClusterItem(trigger.clone(), cluster_store_state.clone(), net_state.clone(), global_state.clone())}
                    </div>
                    <div class="w-[25%] flex ml-2 text-white py-1 px-4 appearance-none items-center justify-center cursor-pointer">
                        if *global_state.loading.borrow(){
                            <div class="py-1 px-4 flex items-center justify-center hover:bg-true-yonder bg-true-blue rounded-full">
                                <span class="flex w-[20px] mr-5"> <WalletSvg /> </span>
                                <span class="flex mr-5"> <Loader/> </span>
                            </div>
                        } else {
                            <div class="flex hover:bg-true-yonder bg-true-blue text-white rounded-full py-1 px-4 appearance-none text-center cursor-pointer">
                                if let Ok(connected_account) = global_state.active_connection.borrow().connected_account() {
                                    {ActiveAccountDropDown(trigger.clone(), connected_account, global_state.clone(), account_info_state.clone())}
                                } else {
                                    <div
                                        onclick={
                                            let global_state = global_state.clone();
                                            let trigger = trigger.clone();

                                        Callback::from(move|_|{
                                            global_state.dispatch(GlobalAction::ConnectModalTrue(trigger.clone()));
                                        }) }
                                        class="flex w-full items-center justify-center cursor-pointer">
                                        <button class="text-sm cursor-pointer"> {"Select Wallet"}</button>
                                    </div>
                                }
                            </div>
                        }
                    </div>
                </nav>
                {ping_view(net_state.clone(), cluster_store_state.clone(), global_state.clone(), trigger.clone())}

                if *global_state.connect_modal.borrow() {
                    {ConnectWalletModalModal(trigger.clone(), global_state.clone())}
                }
            </div>
    }
}

fn ping_view(
    net_state: NetState,
    cluster_store_state: ClusterStoreState,
    global_state: GlobalAppState,
    trigger: UseStateHandle<bool>,
) -> Html {
    let active_cluster = cluster_store_state.active_cluster();

    let window = global_state.adapter.borrow().window().clone();
    let endpoint = active_cluster.endpoint().to_string();

    if *net_state.state.borrow() == ClusterNetState::Failure {
        html! {
            <div class="flex w-full justify-center h-[40px] bg-red-800 text-center items-center text-2xl">
                <div class="flex px-4 py-2 justify-center items-center">
                    <div class="flex w-full mr-2">
                        <span class="flex w-[30px] mr-1 text-white">
                            <ClustersSvg />
                        </span>
                        {active_cluster.name()} {" cluster is unreachable!"}
                    </div>
                    <button
                        onclick={
                            Callback::from(move|_| {
                                let endpoint = endpoint.clone();
                                let window = window.clone();

                                net_state.dispatch(NetStateOptions::Ping { endpoint, window, trigger: trigger.clone() });
                            })
                        }
                        class="flex bg-true-blue items-center justify-center text-sm text-white px-2 py-1 rounded-full hover:bg-cobalt-blue"
                    >{"REFRESH"}</button>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}

fn NavItem(route: Route, text: &'static str) -> Html {
    html! {
        <Link<Route> to={route} classes="w-[10%] hover:bg-transparent dark:text-blue-yonder dark:hover:text-white text-true-blue hover:text-black rounded-lg text-center p-1">
        { text }
        </Link<Route>>
    }
}

fn NavClusterItem(
    trigger: UseStateHandle<bool>,
    cluster_store_state: ClusterStoreState,
    net_state: NetState,
    global_state: GlobalAppState,
) -> Html {
    let trigger = trigger.clone();

    let net_state_inner = net_state.clone();

    html! {
        <div class="flex w-[15%]">
            <select
                onchange={
                    let cluster_store_state = cluster_store_state.clone();
                    Callback::from(move |event: web_sys::Event| {
                        let trigger_inner1 = trigger.clone();
                        let trigger_inner = trigger.clone();
                        net_state_inner.dispatch(NetStateOptions::Waiting(trigger_inner.clone()));

                        let cluster_store_state_inner = cluster_store_state.clone();

                        let cluster_name = get_select_value(event);
                        cluster_store_state_inner.dispatch(ClusterStoreActions::Set{name: cluster_name, trigger:trigger_inner, global_state:global_state.clone()});

                            let endpoint = cluster_store_state_inner
                            .active_cluster
                            .borrow()
                            .endpoint()
                            .to_string();
                        let window = global_state.adapter.borrow().window().clone();


                        net_state_inner.dispatch(NetStateOptions::Ping {
                            endpoint,
                            window,
                            trigger: trigger_inner1.clone(),
                        });
                    })
                }
                class="flex text-sm hover:bg-true-yonder bg-true-blue text-white rounded-full py-1 px-4 appearance-none text-center cursor-pointer">
                {cluster_store_state.clusters.borrow()
                    .iter()
                    .map(|adapter_cluster| {
                        let name = trunk_cluster_name(adapter_cluster.name());

                        let is_active_cluster = adapter_cluster.name().as_bytes()
                            == cluster_store_state.active_cluster().name().as_bytes();

                        html! {
                            <option value={name.clone()} selected={is_active_cluster}> {name.clone()} </option>
                        }
                    }).collect::<Vec<Html>>()}
            </select>
        </div>
    }
}

pub fn ActiveAccountDropDown(
    trigger: UseStateHandle<bool>,
    connected_account: &WalletAccount,
    global_state: GlobalAppState,
    account_info_state: AccountInfoState,
) -> Html {
    let connected_wallet = global_state
        .active_connection
        .borrow()
        .connected_wallet()
        .unwrap()
        .clone();

    let icon = connected_wallet.icon().cloned();

    let short_address = connected_account
        .shorten_address()
        .unwrap_or("ERROR".into())
        .to_string();
    let address = connected_account.address().to_string();

    let window = global_state.adapter.borrow().window().clone();

    html! {
        <div class="relative inline-block rounded-full">
            <div
                onclick={
                    let global_state = global_state.clone();
                    let trigger = trigger.clone();

                    Callback::from(move|_| {
                        let show_dropdown = *global_state.clone().wallet_dropdown.borrow();
                        if show_dropdown {
                            global_state.clone().dispatch(GlobalAction::DropdownFalse(trigger.clone()));
                        }else {
                            global_state.clone().dispatch(GlobalAction::DropdownTrue(trigger.clone()));
                        }
                    })
                }
                class="flex w-full text-center items-center justify-center">
                <span class="flex w-[20px] mr-2">
                    if let Some(icon) = icon {
                        <img class="rounded-lg" src={icon.to_string()}/>
                    }else {
                        <WalletSvg />
                    }
                </span>
                {short_address.clone()}
            </div>

            if *global_state.clone().wallet_dropdown.borrow() {
                <ul class="w-full min-w-[130px] text-white flex flex-col absolute z-1 text-md mt-2 bg-true-blue rounded-lg shadow-xl list-none">
                        <li class="flex w-full mb-2 mt-2 hover:bg-cobalt-blue cursor-pointer"
                            onclick={
                                let global_state = global_state.clone();
                                let trigger = trigger.clone();

                                Callback::from(move|_| {
                                    let address = address.clone();
                                    let global_state = global_state.clone();
                                    let global_state1 = global_state.clone();
                                    let trigger = trigger.clone();
                                    let window = window.clone();

                                    spawn_local(async move {
                                        global_state.clone().dispatch(GlobalAction::DropdownFalse(trigger.clone()));
                                    });

                                    spawn_local(async move {
                                        if let Err(error) = copied_address(&address, &window).await {
                                            global_state1.dispatch(GlobalAction::Message(NotificationInfo::new(error)))
                                        } else {
                                            global_state1.dispatch(GlobalAction::Message(NotificationInfo::new("Copied to clipboard")))
                                        }
                                    });
                                })
                            }
                            >
                            <div class="flex text-sm justify-left items-center ">
                                <span class="p-2 w-[30px]"> <CopySvg/> </span>
                                <span> {"Copy Address"} </span>
                            </div>
                        </li>

                    <li class="flex w-full mb-2 mt-2 hover:bg-cobalt-blue cursor-pointer"
                        onclick={
                            let global_state =global_state.clone();
                            let trigger =trigger.clone();
                            Callback::from(move|_| {
                                global_state.dispatch(GlobalAction::DropdownFalse(trigger.clone()));
                                global_state.dispatch(GlobalAction::ConnectModalTrue(trigger.clone()));
                            })
                        }>
                        <div class="flex text-sm justify-left items-center ">
                            <span class="p-2 w-[30px]"> <ChangeWalletSvg /> </span>
                            <span> {"Change Wallet"} </span>
                        </div>
                    </li>

                    <li class="flex w-full mb-2 mt-2 hover:bg-cobalt-blue cursor-pointer"
                        onclick={
                            Callback::from(move|_| {
                                let global_state =global_state.clone();
                                let account_info_state =account_info_state.clone();
                                let global_state1 =global_state.clone();
                                let trigger =trigger.clone();
                                let trigger1 =trigger.clone();

                                spawn_local(async move {
                                    global_state.dispatch(GlobalAction::Disconnect{trigger: trigger1.clone(), account_info_state});
                                });

                                spawn_local(async move {
                                    global_state1.clone().dispatch(GlobalAction::DropdownFalse(trigger.clone()));
                                });
                            }
                        )
                    }>
                        <div class="flex text-sm justify-left items-center ">
                            <span class="p-2 w-[30px]"> <DisconnectSvg/> </span>
                            <span>{"Disconnect"}</span>
                        </div>
                    </li>
                </ul>
            }
        </div>
    }
}

pub fn ConnectWalletModalModal(
    trigger: UseStateHandle<bool>,
    global_state: GlobalAppState,
) -> Html {
    let trigger_inner = trigger.clone();

    let is_wallets_empty = global_state.adapter.borrow().wallets().is_empty();

    if !is_wallets_empty {
        html! {
            <div class="flex flex-col w-full h-full bg-[#1a1a1a88] absolute items-center justify-center z-50">
                <div class="flex relative w-full max-w-[90%] lg:max-w-[40%] md:max-w-[55%] max-h-full">
                    <div
                        class="relative bg-white rounded-lg shadow-lg dark:bg-rich-black items-center justify-between flex flex-col w-full h-full min-h-[40vh]">
                        <div
                            class="flex items-center justify-center p-4 md:p-5 rounded-t w-full dark:border-gray-600 border-gray-200">
                            <div class="flex w-5/6 items-center justify-center">
                                <h3
                                    class="text-2xl flex items-center justify-center font-semibold text-blue-yonder dark:text-white">
                                    <span class="w-[30px] mr-2 flex"> <GradientWalletSvg /> </span>
                                    {"Connect A Wallet"}</h3>
                            </div>
                            <div class="flex w-1/6"><button
                                    onclick={
                                        let global_state = global_state.clone();
                                        Callback::from(move |_| {
                                            global_state.dispatch(GlobalAction::ConnectModalFalse(trigger.clone()));
                                        })
                                    }
                                    class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline- justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white"
                                    data-modal-hide="default-modal" type="button">
                                    <CloseSvg />
                                    <span class="sr-only">{"Close modal"}</span></button></div>
                        </div>
                        <ul class="flex space-y-4 mb-5 w-full justify-center flex-col items-center h-full">
                        {global_state.adapter.borrow().wallets()
                            .into_iter()
                            .map(|wallet: Wallet| {
                                let wallet_name = wallet.name();

                                let wallet_name_alt = wallet_name.to_string();
                                let wallet_name_text = wallet_name.to_string();
                                let optional_icon = wallet.icon().cloned();
                                let wallet = wallet.clone();
                                let trigger_inner = trigger_inner.clone();

                                html! {
                                    <li
                                    onclick={
                                        let global_state = global_state.clone();

                                        Callback::from(move|_|{
                                            let wallet = wallet.clone();
                                            let global_state = global_state.clone();
                                            let global_state_inner = global_state.clone();
                                            let trigger_inner = trigger_inner.clone();

                                            spawn_local(async move {
                                                global_state.dispatch(GlobalAction::ConnectModalFalse(trigger_inner.clone()));

                                                global_state_inner.dispatch(GlobalAction::Connect { wallet, trigger:trigger_inner.clone(), global_state: global_state.clone(), });
                                            });

                                        })
                                    }
                                    class="flex justify-center cursor-pointer w-full text-lg hover:bg-true-blue  text-true-blue hover:text-white dark:text-white px-4 py-2">
                                        <div class="max-w-[80%] flex justify-between w-full">
                                            <div class="flex items-center">
                                            if let Some(icon) = optional_icon {
                                                <img class="flex w-[25px] mr-2 items-center" src={icon.to_string()} alt={wallet_name_alt} />
                                            }else {
                                                <span class="flex w-[25px] mr-2 items-center"><WalletSvg/></span>
                                            }
                                            <span class="flex">{wallet_name_text}</span></div>
                                            <span class="flex">{"Detected"}</span>
                                        </div>
                                    </li>
                                    }
                                })
                                .collect::<Vec<Html>>()
                        }
                        </ul>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="flex flex-col w-full h-full bg-[#1a1a1a88] absolute items-center justify-center z-50">
                <div class="flex relative w-full max-w-[90%] lg:max-w-[40%] md:max-w-[55%] max-h-full">
                    <div
                        class="relative bg-white rounded-lg shadow-lg dark:bg-rich-black items-center justify-start p-2 flex flex-col w-full h-full min-h-[40vh]">
                        <div class="flex w-full mr-5">
                            <button
                                onclick={
                                    let global_state = global_state.clone();
                                    Callback::from(move |_| {
                                        global_state.dispatch(GlobalAction::ConnectModalFalse(trigger.clone()));

                                    })
                                }
                                class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline- justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white"
                                data-modal-hide="default-modal" type="button">
                                <CloseSvg/>
                                <span class="sr-only">{"Close modal"}</span></button></div>
                        <div class="flex text-2xl w-full p-5 flex-col items-center justify-around h-full">
                            <div class="flex w-full items-center justify-center">
                                <span class="flex w-[50px] mr-5 items-center">
                                    <WalletSvg/>
                                </span>
                                <span>{"No Solana Wallets Detected"}</span></div>
                            <div class="flex text-lg">{"Install a Solana Wallet Installed on your browser!"}</div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
