use wallet_adapter::Cluster;
use yew::prelude::*;

use crate::{
    get_cluster_svg, get_input_value, get_select_value, header, trunk_cluster_name, AccountInfoData, CheckSvg, CloseSvg, ClusterStoreActions, ClusterStoreState, ClustersSvg, Footer, GlobalAppState, IdSvg, LinkSvg, NetState,  TrashSvg
};

#[function_component]
pub fn Clusters() -> Html {
    let cluster_store_state =
        use_context::<ClusterStoreState>().expect("no global ctx `ClusterStoreState` found");
    let net_state = use_context::<NetState>().expect("no global ctx `NetState` found");
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");

    let re_render = use_state(|| bool::default());
    let show_add_modal = use_state(|| bool::default());
    let new_cluster_info = use_state(||{
        let mut default = NewClusterInfo::default();
        default.network = "devnet".to_string();

        default
    });
    let account_info_state = use_reducer(|| AccountInfoData::default());


    html! {
        <div class= "flex flex-col w-full min-h-full justify-between items-center">
            {header(re_render.clone(), cluster_store_state.clone(), net_state.clone(), global_state.clone(), account_info_state.clone())}
                <div class="flex w-full flex-col justify-start p-10 items-center">
                    <div class="flex flex-col w-full items-center justify-center text-4xl">
                        <span class="flex w-[100px]"> <ClustersSvg/> </span> {"Clusters"}
                        <div class="text-xl"> {"Manage your Solana endpoints"} </div>
                        <button
                            onclick={
                                let show_add_modal = show_add_modal.clone();
                                Callback::from(move|_|{
                                    show_add_modal.set(true);
                                })
                            }
                            class="bg-true-blue text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue">
                            {"ADD CLUSTER"}
                        </button>
                        {ClusterInfo(cluster_store_state.clone(), global_state.clone(), re_render.clone())}
                    </div>
                </div>
            <Footer/>
            {AddClusterModal(new_cluster_info, cluster_store_state, global_state, re_render.clone(), show_add_modal.clone())}
        </div>
    }
}



fn ClusterInfo(
    cluster_store_state: ClusterStoreState,
    global_state: GlobalAppState,
    trigger: UseStateHandle<bool>,
) -> Html {
    let cluster_info = cluster_store_state
        .get_clusters()
        .iter()
        .map(|adapter_cluster| {
            
            let cluster = adapter_cluster.cluster();
            let cluster_name = adapter_cluster.name().to_string();
            let endpoint = adapter_cluster.endpoint().to_string();
            let chain = adapter_cluster.cluster().chain().to_string();

            let cluster_name1 = cluster_name.clone();
            let cluster_name2 = cluster_name.clone();

            let active_cluster = cluster_name.as_bytes() == cluster_store_state
                        .active_cluster()
                        .name()
                        .as_bytes();

            html! {
                <div class="flex flex-col text-xl p-5 w-[250px] bg-true-blue rounded-xl">
                    <div class="flex w-full">
                        <span class="w-[25px] mr-2">
                            {get_cluster_svg(cluster)}
                        </span>
                        {trunk_cluster_name(&cluster_name1)}
                    </div>
                    <div class="flex flex-col w-full">
                        <div class="flex w-full items-start flex-col mt-2.5 mb-5">
                            <div class="bg-blue-100 text-blue-800 text-sm font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800">
                                {chain}
                            </div>
                            <div class="text-sm mt-2"> {endpoint}  </div>
                        </div>

                        <div class="flex w-full items-center justify-between">
                            if !active_cluster {
                                <div class="text-3xl font-bold text-gray-900 dark:text-white">
                                    <label
                                        onclick={
                                            let cluster_name = cluster_name2.clone();
                                            let trigger = trigger.clone();
                                            let global_state = global_state.clone();
                                            let cluster_store_state = cluster_store_state.clone();

                                            Callback::from(move |_| {
                                                let trigger_inner = trigger.clone();
                                                let cluster_name = cluster_name.clone();
                                                let cluster_store_state_inner = cluster_store_state.clone();
                                                
                                                cluster_store_state_inner.dispatch(ClusterStoreActions::Set{name: cluster_name, trigger:trigger_inner, global_state:global_state.clone()});
                                            })
                                        }
                                        title="Switch"
                                        class="inline-flex items-center cursor-pointer shadow-2xl">
                                        <input class="sr-only peer" type="checkbox" value=""/>
                                        <div class="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"></div>
                                    </label>
                                
                                </div>
                                <div class=" hover:bg-blue-800 rounded-xl dark:hover:bg-blue-700">
                                    {Delete(cluster_name, cluster_store_state.clone(), global_state.clone(), trigger.clone())}
                                </div>
                            }else {
                                <span class="w-5"> <CheckSvg /> </span>
                            }
                        </div>
                    </div>
                </div>
            }
        })
        .collect::<Vec<Html>>();

    html! {
        <div class="flex flex-wrap w-full items-stretch justify-center gap-4 mt-20">
            {cluster_info}
        </div>
    }
}


fn Delete(cluster_name: String, cluster_store_state: ClusterStoreState, global_state: GlobalAppState,     trigger: UseStateHandle<bool>,) -> Html {
    html! {
        <div
            onclick={
                move|_|{
                    let cluster_name = cluster_name.clone();

                    cluster_store_state.dispatch(ClusterStoreActions::Remove{cluster_name, global_state: global_state.clone(), trigger: trigger.clone()});
                }
            }
            title="Delete"
            class="cursor-pointer w-8">
            <TrashSvg/>
        </div>
    }
}

#[derive(Debug, Default, Clone)]
struct NewClusterInfo {
    name: String,
    endpoint: String,
    network: String,
}
fn AddClusterModal(
    new_cluster_info: UseStateHandle<NewClusterInfo>, 
    cluster_store_state: ClusterStoreState, 
    global_state: GlobalAppState, 
    trigger: UseStateHandle<bool>, 
    show_add_modal: UseStateHandle<bool>,
) -> Html {
    let clusters_view = cluster_store_state
        .get_networks()
        .iter()
        .map(|cluster| {
            let is_devnet = *cluster == Cluster::DevNet;

            html! {
                <option selected={is_devnet} key={cluster.to_string()} value={cluster.to_string()}>{cluster.to_string()}</option>
            }
        })
        .collect::<Vec<Html>>();

    if *show_add_modal {
        html! {
            <div class="fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center">
                <div class="flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-gray-200 dark:bg-[#10141f] rounded-3xl">
                    <div class="flex w-full justify-end items-center p-5">
                        <button
                            onclick={
                                let show_add_modal = show_add_modal.clone();
                                Callback::from(move |_| {
                                    show_add_modal.set(false);
                                })
                            }
                            class="wallet-adapter-modal-button-close w-[25px] items-center justify-center cursor-pointer">
                            <CloseSvg />
                        </button>
                    </div>
                    <div class="flex w-4/5 rounded-xl min-h-[40vh] p-5 mb-10 items-start justify-center flex-col">
                        <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white"
                            for="cluster-name">
                            {"What would you like to call your cluster?"}
                        </label>
                        <div class="flex w-full mb-10">
                            <span class="w-[40px] inline-flex items-center px-3 text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600"
                            ><IdSvg/></span>
                            <input
                            onchange={
                                let new_cluster_info = new_cluster_info.clone();
                                    Callback::from(

                                        move |event: Event| {
                                            let value = get_input_value(event);
                                            let mut temp = (*new_cluster_info).clone();
                                            temp.name = value;
                                            new_cluster_info.set(temp);
                                        }
                                    )
                                }
                                class="rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                id="cluster-name"
                                placeholder="Rising Sun"
                                type="text"
                                required=true />
                        </div>
                        <label class="block mb-2 text-sm font-medium text-gray-900 dark:text-white" for="endpoint">
                            {"While URL & Custom port will you reach your cluster?"}
                        </label>
                        <div class="flex w-full">
                            <span class="w-[40px] inline-flex items-center px-3 text-lg text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600">
                                <LinkSvg/>
                            </span>
                            <input
                                onchange={
                                    let new_cluster_info = new_cluster_info.clone();
                                    
                                    Callback::from(

                                        move |event: Event| {
                                            let value = get_input_value(event);
                                            let mut temp = (*new_cluster_info).clone();
                                            temp.endpoint = value;
                                            new_cluster_info.set(temp);
                                        }
                                    )
                                }
                                class="rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                id="endpoint"
                                placeholder="URL Endpoint ,eg. http://localhost:8899"
                                type="url"
                                required=true
                            />
                        </div>
                        <label class="block mb-2 text-sm mt-5 font-medium text-gray-900 dark:text-white" for="network"> {"Network"} </label>
                        <div class= "flex w-full">
                            <span class="w-[40px] inline-flex items-center px-3 bg-gray-200 border border-gray-300 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600">
                                <ClustersSvg />
                            </span>
                            <select
                                onchange={
                                    let new_cluster_info = new_cluster_info.clone();

                                    Callback::from(
                                        move |event: web_sys::Event| {
                                            let value = get_select_value(event);
                                            let value: Cluster = value.as_str().try_into().unwrap_or_default();

                                            let mut temp = (*new_cluster_info).clone();
                                            temp.network = value.to_string();

                                            new_cluster_info.set(temp);
                                        }
                                    )
                                }
                                class="rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                                id="network"
                                name="network"
                                required=true>
                                {clusters_view}
                            </select>
                        </div>
                        <div class="flex w-full items-center justify-center p-5 mt-5">
                            <button
                                onclick={
                                    let new_cluster_info = new_cluster_info.clone();
                                    let cluster_store_state = cluster_store_state.clone();
                                    let global_state = global_state.clone();

                                    Callback::from(move |_| {
                                        cluster_store_state.dispatch(ClusterStoreActions::Add { 
                                            name: new_cluster_info.name.clone(), 
                                            endpoint: new_cluster_info.endpoint.clone(), 
                                            network: new_cluster_info.network.clone(), 
                                            global_state: global_state.clone(),
                                            trigger: trigger.clone()
                                        });

                                        show_add_modal.set(false);
                                    })
                                }
                                class="bg-true-blue text-sm text-white px-5 py-2 rounded-full hover:bg-cobalt-blue pointer">
                                    {"ADD CLUSTER"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
