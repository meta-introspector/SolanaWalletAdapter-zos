use sycamore::prelude::*;
use wallet_adapter::Cluster;

use crate::{
    app::GlobalMessage,
    check_svg, close_svg, clusters_svg, id_svg, link_svg, trash_svg,
    utils::{get_cluster_svg, get_input_value, get_select_value, trunk_cluster_name},
    AdapterCluster, ClusterStore, NotificationInfo,
};

#[derive(Debug, Clone, Copy, Default)]
struct AddClusterModalBool(bool);


#[component]
pub fn Clusters() -> View {
    provide_context(create_signal(AddClusterModalBool::default()));

    let show_add_cluster_modal = use_context::<Signal<AddClusterModalBool>>();


    view! {
       div(class="flex w-full flex-col justify-start p-10 items-center"){
        div(class="flex flex-col w-full items-center justify-center text-4xl"){
            span(class="flex w-[100px]"){ img(src=clusters_svg()) } "Clusters"
            div (class="text-xl"){
                "Manage your Solana endpoints"
            }
            button (on:click=move|_|{
                    show_add_cluster_modal.set(AddClusterModalBool(true));
                },
                class="bg-true-blue text-sm text-white px-5 py-2 mt-5 rounded-full hover:bg-cobalt-blue"){
                "ADD CLUSTER"
            }
            div (class="flex flex-wrap w-full items-stretch justify-center gap-4 mt-20"){
                (ClusterInfo())
            }
        }
       }

       (AddClusterModal())
    }
}

#[component]
fn ClusterInfo() -> View {
    let cluster_storage = use_context::<Signal<ClusterStore>>();

    let cluster_info = cluster_storage
        .get_clone()
        .get_clusters()
        .iter()
        .map(|adapter_cluster| {
            
            let cluster = adapter_cluster.cluster();
            let cluster_name = adapter_cluster.name().to_string();
            let endpoint = adapter_cluster.endpoint().to_string();
            let chain = adapter_cluster.cluster().chain().to_string();

            let cluster_name1 = cluster_name.clone();
            let cluster_name2 = cluster_name.clone();

            let active_cluster = cluster_name.as_bytes() == cluster_storage
                        .get_clone()
                        .active_cluster()
                        .name()
                        .as_bytes();

            view! {
                div (class="flex flex-col text-xl p-5 w-[250px] bg-true-blue rounded-xl"){
                    div (class="flex w-full"){
                        span (class="w-[25px] mr-2"){
                            img(src=get_cluster_svg(cluster))
                        }
                        (trunk_cluster_name(&cluster_name1))
                    }
                    div (class="flex flex-col w-full"){
                        div (class="flex w-full items-start flex-col mt-2.5 mb-5"){
                            div (class="bg-blue-100 text-blue-800 text-sm font-semibold px-2.5 py-0.5 rounded-full dark:bg-blue-200 dark:text-blue-800"){
                                (chain)
                            }
                            div (class="text-sm mt-2"){ (endpoint) }
                        }

                        div (class="flex w-full items-center justify-between"){
                            (if !active_cluster {
                                view!{
                                    div (class="text-3xl font-bold text-gray-900 dark:text-white"){
                                        (Switch(&cluster_name2))
                                    }
                                    div (class=" hover:bg-blue-800 rounded-xl dark:hover:bg-blue-700"){
                                        (Delete(cluster_name.clone()))
                                    }
                                }
                            }else {
                                view!{ span (class="w-5"){ img(src=check_svg()) } }
                            })
                        }
                    }
                }
            }
        })
        .collect::<Vec<View>>();

    view! {
        (cluster_info)
    }
}

fn Switch(cluster_name: &str) -> View {
    let global_message = use_context::<Signal<GlobalMessage>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();
    let cluster_name = cluster_name.to_string();

    view! {
        label (
            on:click=move|_|{
                let cluster_name = cluster_name.clone();

                let find_cluster = cluster_storage.get_clone().get_cluster(&cluster_name).cloned();

                if let Some(found_cluster) = find_cluster{
                    cluster_storage.update(|store| {store.set_active_cluster(found_cluster);});
                    global_message.update(|store| store.push_back(NotificationInfo::new(String::new() + &cluster_name + " cluster now active!")));

                }else {
                    global_message.update(|store| store.push_back(NotificationInfo::new(String::from("Could not find `") + &cluster_name + "` cluster!")));
                }

            },
            title="Switch",
            class="inline-flex items-center cursor-pointer shadow-2xl"){
            input (class="sr-only peer", r#type="checkbox", value=""){}
            div (class="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 dark:peer-checked:bg-blue-600"){}
        }
    }
}

fn Delete(cluster_name: String) -> View {
    let global_message = use_context::<Signal<GlobalMessage>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();

    view! {
        div(on:click=move|_|{
            let cluster_name = cluster_name.clone();

                cluster_storage.update(|store| if  store.remove_cluster(cluster_name.as_str()).is_some(){
                   global_message.update(|store| store.push_back(NotificationInfo::new(String::new() + cluster_name.as_str() + " cluster has been removed!")));
                }else {
                    global_message.update(|store| store.push_back(NotificationInfo::new(String::from("Could not find `") + cluster_name.as_str() + "` cluster!")));
                });

            },
            title="Delete", class="cursor-pointer w-8"){ img(src=trash_svg())
        }
    }
}

fn AddClusterModal() -> View {
    let global_message = use_context::<Signal<GlobalMessage>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();
    let show_add_cluster_modal = use_context::<Signal<AddClusterModalBool>>();

    #[derive(Debug, Clone, Default)]
    struct AddCluster {
        name: String,
        endpoint: String,
        network: Cluster,
    }

    let add_cluster_state = create_signal(AddCluster::default());

    let add_cluster = create_memo(move || add_cluster_state.get_clone());
   

    let should_show_button = move || {
        !add_cluster.get_clone().name.is_empty() && !add_cluster.get_clone().endpoint.is_empty()
    };

    let clusters_view = cluster_storage
        .get_clone()
        .get_clusters()
        .iter()
        .map(|cluster| {
            let identifier = cluster.identifier().clone();
            let cluster_name = cluster.name().to_string();
            let cluster_name_key = cluster_name.clone();

            view! {
                option(
                    "key"=cluster_name_key,
                    value=identifier,
                ){(view!{(cluster_name)})}
            }
        })
        .collect::<Vec<View>>();

    let add_to_store = move || {
        let adapter_cluster = AdapterCluster::new()
            .add_name(add_cluster.get_clone().name.as_str())
            .add_endpoint(add_cluster.get_clone().endpoint.as_str())
            .add_cluster(add_cluster.get_clone().network);

        cluster_storage.update(|store|
            if let Err(error) =  store.add_cluster(adapter_cluster) {

            global_message.update(|store| store.push_back(NotificationInfo::new(format!("Error Adding Cluster: `{error}`!"))));

        }else {
            global_message.update(|store| store.push_back(NotificationInfo::new(format!("Added `{}` cluster!", add_cluster.get_clone().name.as_str()))));
        });

        show_add_cluster_modal.set(AddClusterModalBool(false));
    };

    if show_add_cluster_modal.get().0 {
        view! {
            div (class="fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center"){
                div (class="flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-gray-200 dark:bg-[#10141f] rounded-3xl"){
                    div (class="flex w-full justify-end items-center p-5"){
                        button (on:click=move |_| {
                            show_add_cluster_modal.set(AddClusterModalBool(false));
                        },
                            class="wallet-adapter-modal-button-close w-[25px] items-center justify-center"){
                            img(src=close_svg())
                        }
                    }
                    div (class="flex w-4/5 rounded-xl min-h-[40vh] p-5 mb-10 items-start justify-center flex-col"){
                        label (class="block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                            r#for="cluster-name"){
                            "What would you like to call your cluster?"
                        }
                        div (class="flex w-full mb-10"){
                            span (class="w-[40px] inline-flex items-center px-3 text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600"){
                                img(src=id_svg())
                            }
                            input (
                                on:input=move|event: web_sys::Event| {
                                    let value = get_input_value(event);
                                    add_cluster_state.update(|inner| inner.name = value);
                                },
                                class="rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id="cluster-name",
                                placeholder="Rising Sun",
                                r#type="text",
                                required=true,
                            )
                        }
                        label (class="block mb-2 text-sm font-medium text-gray-900 dark:text-white",
                            r#for="endpoint"){
                            "While URL & Custom port will you reach your cluster?"
                        }
                        div (class="flex w-full"){
                            span (class="w-[40px] inline-flex items-center px-3 text-lg text-gray-900 bg-gray-200 border rounded-e-0 border-gray-300 border-e-0 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600"){
                                img(src=link_svg())
                            }
                            input (
                                on:input=move |event: web_sys::Event| {
                                    let value = get_input_value(event);

                                    if validate_url(&value) {
                                        add_cluster_state.update(|inner| inner.endpoint = value);
                                    }
                                },
                                class="rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id="endpoint",
                                placeholder="URL Endpoint ,eg. http://localhost:8899",
                                r#type="url",
                                required=true,
                            )
                        }
                        label (class="block mb-2 text-sm mt-5 font-medium text-gray-900 dark:text-white",
                            r#for="network"){
                            "Network"
                        }
                        div (class= "flex w-full"){
                            span (class="w-[40px] inline-flex items-center px-3 bg-gray-200 border border-gray-300 rounded-s-md dark:bg-gray-600 dark:text-gray-400 dark:border-gray-600"){
                                img(src=clusters_svg())
                            }
                            select(
                                on:change=move |event: web_sys::Event| {
                                    let value = get_select_value(event);

                                    let network: Cluster = value.as_str().try_into().expect(
                                        "This is a fatal error, you provided an invalid cluster"
                                    );
                                    add_cluster_state.update(|inner| inner.network = network);
                                },
                                class="rounded-none rounded-e-lg bg-gray-50 border text-gray-900 focus:ring-blue-500 focus:border-blue-500 block flex-1 min-w-0 w-full text-sm border-gray-300 p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                                id="network",
                                name="network",
                                required=true){
                                    (clusters_view)
                                }
                        }
                        div (class="flex w-full items-center justify-center p-5 mt-5"){
                            (if should_show_button() {                                
                                view!{
                                    button (on:click=move |_| {
                                            add_to_store()
                                        },
                                        class="bg-true-blue text-sm text-white px-5 py-2 rounded-full hover:bg-cobalt-blue"){
                                        "ADD CLUSTER"
                                    }
                                }
                            } else {view!{}})
                        }
                    }
                }
            }
        }
    } else {
        view! {}
    }
}

fn validate_url(value: &str) -> bool {
    let scheme_exists = value.starts_with("http://") || value.starts_with("https://");

    scheme_exists && value.len() > 8
}
