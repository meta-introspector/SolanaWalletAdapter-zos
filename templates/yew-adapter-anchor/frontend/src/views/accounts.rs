use wallet_adapter::Cluster;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    format_address_url, format_timestamp, format_tx_url, get_cluster_svg, header,
    link_target_blank, trunk_cluster_name, AccountInfoAction, AccountInfoData, AccountInfoState,
    AdapterCluster, Airdrop, AirdropSvg, AtaSvg, AvatarSvg, BalanceSvg, CheckSvg, ClusterNetState,
    ClusterStoreState, ConnectWalletFirst, ErrorSvg, Footer, GlobalAction, GlobalAppState, Loader,
    MintSvg, NetState, ReceiveSol, ReceiveSvg, SendSol, SendSvg, SignatureSvg, TimestampSvg,
    WalletSvg,
};

#[function_component]
pub fn Accounts() -> Html {
    let cluster_store_state =
        use_context::<ClusterStoreState>().expect("no global ctx `ClusterStoreState` found");
    let net_state = use_context::<NetState>().expect("no global ctx `NetState` found");
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");
    let account_info_state = use_reducer(|| AccountInfoData::default());

    let re_render = use_state(|| bool::default());

    let show_send_modal = use_state(|| bool::default());
    let show_receive_modal = use_state(|| bool::default());
    let show_airdrop_modal = use_state(|| bool::default());

    html! {
        <div class= "flex flex-col w-full min-h-full justify-between items-center">
            {header(re_render.clone(), cluster_store_state.clone(), net_state.clone(), global_state.clone(), account_info_state.clone())}

                if global_state.active_connection.borrow().connected_account().is_ok() {
                    if *net_state.state.borrow() == ClusterNetState::Success {
                        {ClusterSuccess (
                            global_state.clone(),
                            cluster_store_state.clone(),
                            account_info_state.clone(),
                            show_send_modal.clone(),
                            show_receive_modal.clone(),
                            show_airdrop_modal.clone(),
                            re_render.clone()
                        )}
                    }else if *net_state.state.borrow() == ClusterNetState::Waiting {
                        {"Loading account info..."}
                    } else {
                        {"CLUSTER NETWORK UNREACHABLE"}
                    }
                } else {
                    <ConnectWalletFirst />
                }
            <Footer/>


            if *show_send_modal {
                {SendSol(show_send_modal.clone(), global_state.clone(), cluster_store_state.clone(), re_render.clone())}
            }
            if *show_receive_modal {
                {ReceiveSol(show_receive_modal.clone(), global_state.clone())}
            }
            if *show_airdrop_modal {
                {Airdrop(show_airdrop_modal.clone(), global_state.clone(), cluster_store_state.clone(), re_render.clone())}
            }
        </div>
    }
}

fn ClusterSuccess(
    global_state: GlobalAppState,
    cluster_store_state: ClusterStoreState,
    account_info_state: AccountInfoState,
    show_send_modal: UseStateHandle<bool>,
    show_receive_modal: UseStateHandle<bool>,
    show_airdrop_modal: UseStateHandle<bool>,
    trigger: UseStateHandle<bool>,
) -> Html {
    let account_info_state_inner = account_info_state.clone();
    let check_balance = move || {
        let balance = account_info_state_inner
            .balance
            .borrow()
            .as_str()
            .parse::<f64>()
            .unwrap_or_default();

        balance == f64::default()
    };
    let wallet_account_inner = global_state
        .active_connection
        .borrow()
        .connected_account()
        .cloned()
        .unwrap_or_default();

    let shortened_address = wallet_account_inner
        .shorten_address()
        .unwrap_or_default()
        .to_string();

    let address = wallet_account_inner.address().to_string();

    let token_view = account_info_state
        .token_accounts
        .borrow()
        .iter()
        .map(|token_account| {
            TokenAccountCard(
                token_account.mint(),
                token_account.ata_address(),
                token_account.balance(),
                token_account.state(),
                &cluster_store_state.active_cluster(),
            )
        })
        .collect::<Vec<Html>>();

    let tx_view = account_info_state
        .transactions
        .borrow()
        .iter()
        .map(|tx| {
            TxCard(
                tx.signature.clone(),
                tx.block_time,
                tx.confirmation_status.clone(),
                tx.err.is_none(),
                address.clone(),
                &cluster_store_state.active_cluster(),
            )
        })
        .collect::<Vec<Html>>();

    let balance = account_info_state.balance.borrow().clone();
    let address = global_state
        .active_connection
        .borrow()
        .connected_account()
        .unwrap()
        .address()
        .to_string();
    let endpoint = cluster_store_state.active_cluster().endpoint().to_string();

    html! {
        <div class="flex w-full h-full mt-4 mb-10 flex-col items-center">
            <div class="shadow-sm p-5 w-full flex flex-col items-center mb-10 justify-center">
                <div class="text-center w-full">
                    if !*global_state.loading.borrow(){
                        <span class="text-3xl"> {balance} {" SOL"} </span>
                    }else {
                        <span class="mr-2"> <Loader/> </span>  <span class="text-sm"> {"Loading Balance..."}  </span>
                    }
                </div>
                <div class="flex w-full items-center justify-center">
                    <span class="flex w-[20px] mr-1"> <WalletSvg/> </span>
                    {link_target_blank(format_address_url(&address, &cluster_store_state.active_cluster.borrow()), shortened_address.clone())}
                </div>
                <div class="w-full flex gap-4 flex-wrap items-center justify-center">
                    if !check_balance() {
                        <button
                            onclick={
                                Callback::from(move|_|{show_send_modal.set(true)})
                            }
                            class="flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue">
                            <span class="w-[25px] flex mr-1"> <SendSvg/> </span> {"Send"}
                        </button>
                    }
                    <button
                        onclick={
                            Callback::from(move|_|{show_receive_modal.set(true)})
                        }
                        class="flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue">
                        <span class="w-[25px] flex mr-1"> <ReceiveSvg />  </span> {"Receive"}
                    </button>
                    if cluster_store_state.active_cluster().cluster() != Cluster::MainNet{
                        <button
                            onclick={
                                Callback::from(move|_|{show_airdrop_modal.set(true)})
                            }
                            class="flex bg-true-blue items-center justify-center text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue">
                            <span class="w-[25px] flex mr-1"> <AirdropSvg/> </span> {"Airdrop"}
                        </button>
                    }
                    <button
                        onclick={
                            let trigger = trigger.clone();
                            let global_state = global_state.clone();
                            let window = global_state.adapter.borrow().window().clone();

                            let account_info_state = account_info_state.clone();
                            let global_state = global_state.clone();

                            Callback::from(move|_|{
                                let endpoint = endpoint.clone();
                                let address = address.clone();
                                let trigger = trigger.clone();
                                let window = window.clone();
                                let account_info_state = account_info_state.clone();
                                let global_state = global_state.clone();

                                let action_info = AccountInfoAction {
                                    window,
                                    address,
                                    endpoint,
                                };

                                global_state.dispatch(GlobalAction::LoadingTrue(trigger.clone()));

                                spawn_local(async move {
                                    account_info_state.dispatch((trigger.clone(),action_info,global_state.clone()));
                                });
                            },)
                        }
                    class="flex items-center text-sm text-white px-5 py-2 mt-5 rounded-full bg-true-blue hover:bg-cobalt-blue">
                        <span class="w-[25px] flex mr-1"> <WalletSvg/> </span>
                        {"Refresh"}
                    </button>
                </div>
            </div>
            <div class="flex flex-col flex-wrap w-full mt-5 text-2xl items-center justify-center">
                <div class="flex items-center text-true-blue dark:text-white justify-center">
                    <span class="w-[30px] mr-1"> <AtaSvg/> </span>

                    if !*global_state.loading.borrow() {
                        if account_info_state.token_accounts.borrow().is_empty(){
                        <span class="text-sm"> {"No Token Accounts Found"} </span>
                        }else {
                            {"Token Accounts"}
                        }
                    }
                    else {
                        <span class="mr-2"> <Loader/> </span> {"Loading Token Accounts..."}
                    }
                </div>

                {token_view}
            </div>
            <div class="flex flex-col flex-wrap w-full mt-5 text-2xl items-center justify-center">
                <div class="flex mb-5 items-center text-true-blue dark:text-white justify-center">
                    <span class="w-[30px] flex mr-1"> <SignatureSvg/> </span>
                    if !*global_state.loading.borrow() {
                        if account_info_state.transactions.borrow().is_empty(){
                            <span class="text-sm"> {"No Transactions Found" } </span>
                        }else {
                            {"Transactions"}
                        }
                    }else {
                        <span class="mr-2"> <Loader/> </span> {"Loading Transactions..." }
                    }
                </div>
                <div class="flex w-full gap-4 flex-wrap items-center justify-center"> {tx_view}</div>
            </div>
        </div>
    }
}

fn TokenAccountCard(
    mint: String,
    ata_address: String,
    token_balance: String,
    state: String,
    active_cluster: &AdapterCluster,
) -> Html {
    let cluster = active_cluster.cluster();
    let cluster_image = get_cluster_svg(cluster);

    let cluster_name = trunk_cluster_name(active_cluster.name());

    let shortened_mint_address = wallet_adapter::Utils::shorten_base58(&mint)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Mint Address"));
    let shortened_ata_address = wallet_adapter::Utils::shorten_base58(&ata_address)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Owner Address"));

    html! {
        <div class="flex flex-col items-start p-4 w-[250px] m-5 rounded-lg bg-true-blue">
            <div class="flex w-full items-center">
                <span class="w-[28px] pr-2"> {cluster_image} </span>
                <h5 class="flex text-2xl"> {cluster_name} </h5>
            </div>
            <div class="flex flex-col w-full">
                <div class="flex w-full items-start flex-col mt-2.5">
                    <div class="w-full justify-between  flex">
                        <div class="bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800">
                            {active_cluster.cluster().chain().to_string()}
                        </div>
                        <div class="bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800">
                            {state}
                        </div>
                    </div>

                    <div class="text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between">
                        <div class="flex items-center">
                            <div class="w-1/5"> <MintSvg /> </div>
                            <div class="w-4/5 flex text-sm pl-2">
                                {link_target_blank(format_address_url(&mint, &active_cluster), shortened_mint_address.clone()) }
                            </div>
                        </div>
                    </div>

                    <div class="text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between">
                        <div class="flex items-center">
                            <div class="w-1/5"> <AtaSvg/> </div>
                            <div class="w-4/5 flex text-sm pl-2">
                                {link_target_blank(format_address_url(&ata_address, &active_cluster), shortened_ata_address.clone())}
                            </div>
                        </div>
                    </div>

                    <div class="text-black text-lg dark:text-white mt-2 w-full flex flex-col items-start justify-between">
                        <div class="flex items-center">
                            <div class="w-2/5"> <BalanceSvg /> </div>
                            <div class="w-3/5 flex text-[12px] p-1"> {token_balance} </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn TxCard(
    tx: String,
    timestamp: Option<i64>,
    state: Option<String>,
    succeeded: bool,
    address: String,
    active_cluster: &AdapterCluster,
) -> Html {
    let cluster = active_cluster.cluster();
    let cluster_name = active_cluster.name().to_string();
    let cluster_image = get_cluster_svg(cluster);

    let cluster_name = trunk_cluster_name(&cluster_name);

    let shortened_address = wallet_adapter::Utils::shorten_base58(&address)
        .map(|address| address.to_string())
        .unwrap_or(String::from("Invalid Address"));

    let shortened_tx = wallet_adapter::Utils::shorten_base58(&tx)
        .map(|tx| tx.to_string())
        .unwrap_or(String::from("Invalid Address"));

    let succeeded = if succeeded {
        html! {<CheckSvg/>}
    } else {
        html! {<ErrorSvg/>}
    };

    html! {
        <div class="flex rounded-lg flex-col items-start p-5 w-[250px] bg-true-blue">
            <div class="flex items-center">
                <span class="w-[28px] pr-2"> {cluster_image}</span>
                <h5 class="text-2xl font-semibold tracking-tight"> {cluster_name} </h5>
            </div>
            <div class="flex flex-col w-full">
                <div class="flex w-full items-start flex-col mt-2.5">
                    <div class="w-full justify-between items-start  flex">
                        <div class="flex bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800">
                            {cluster.chain().to_string()}
                        </div>
                        if let Some(state_inner) = state {
                            <div class="flex bg-blue-100 text-blue-800 text-xs font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800">
                                {state_inner.to_uppercase()}
                            </div>
                        }
                    </div>

                    <div class="text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between">
                        <div class="flex items-center">
                            <span class="w-[20px]"> <AvatarSvg/> </span>
                            <span class="flex text-sm pl-2">
                                {link_target_blank(format_address_url(&address, &active_cluster), shortened_address.clone())}
                            </span>
                        </div>
                    </div>

                    <div class="text-black text-lg dark:text-white mt-2 w-full flex items-start justify-between">
                        <div class="flex items-center">
                            <span class="w-[20px]"> <SignatureSvg /> </span>
                            <span class="flex text-sm pl-2"> {link_target_blank(format_tx_url(&tx, &active_cluster), shortened_tx.clone())} </span>
                            <div class="flex items-center">
                                <span class="ml-2 w-[15px]"> {succeeded} </span>
                            </div>
                        </div>
                    </div>

                    if let Some(timestamp) = timestamp {
                        <div class="text-black text-lg dark:text-white mt-2 w-full flex flex-col items-start justify-between">
                            <div class="flex items-center">
                                <span class="w-[30px]"> <TimestampSvg/> </span>
                                <span class="flex text-[12px] p-1"> {format_timestamp(timestamp)} </span>
                            </div>
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
