use std::collections::VecDeque;

use gloo_timers::callback::Timeout;
use sycamore::{
    futures::{spawn_local, spawn_local_scoped},
    prelude::*,
};
use sycamore_router::{HistoryIntegration, Route, Router};
use wallet_adapter::{ConnectionInfo, WalletAdapter};

use crate::{
    notification_svg,
    types::{AccountState, ClusterNetState},
    views::{
        Accounts, AirdropModal, Clusters, Dashboard, Extras, Loading, ReceiveModal, SendModal,
    },
    AdapterCluster, ClusterStore, Footer, Header, NotificationInfo,
};

const LOGO_PATH: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.png"));

pub fn logo() -> String {
    String::from("data:image/png;base64,") + data_encoding::BASE64.encode(LOGO_PATH).as_str()
}

pub fn run() {
    sycamore::render(App);
}

pub type GlobalMessage = VecDeque<NotificationInfo>;

#[component]
fn App() -> View {
    let adapter = WalletAdapter::init_custom(window(), document()).unwrap();

    let mut cluster_storage = ClusterStore::default();

    let clusters = vec![
        AdapterCluster::devnet(),
        AdapterCluster::mainnet(),
        AdapterCluster::testnet(),
        AdapterCluster::localnet(),
    ];

    if cluster_storage.add_clusters(clusters).is_err() {}

    provide_context(create_signal(adapter));
    provide_context(create_signal(cluster_storage));
    provide_context(create_signal(GlobalMessage::default()));
    provide_context(create_signal(AccountState::default()));
    provide_context(create_signal(ClusterNetState::default()));
    provide_context(create_signal(ConnectionInfo::default()));
    provide_context(create_signal(Loading::default()));

    provide_context(create_signal(SendModal(bool::default())));
    provide_context(create_signal(AirdropModal(bool::default())));
    provide_context(create_signal(ReceiveModal(bool::default())));

    let adapter = use_context::<Signal<WalletAdapter>>();
    let account_state = use_context::<Signal<AccountState>>();
    let active_connection = use_context::<Signal<ConnectionInfo>>();
    let global_message = use_context::<Signal<GlobalMessage>>();

    spawn_local(async move {
        while let Ok(wallet_event) = adapter.get_clone().events().recv().await {
            account_state.set(AccountState::default());

            let connection_info = (adapter.get_clone().connection_info().await).clone();
            active_connection.set(connection_info);

            global_message.update(|store| store.push_back(NotificationInfo::new(wallet_event)));
        }
    });

    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| {

                view! {
                    div(class="w-full flex min-h-screen font-[sans-serif] dark:bg-rich-black bg-white text-black dark:text-white") {
                        (Notification())
                        div (class= "flex flex-col w-full min-h-full justify-between items-center"){
                            (Header())
                            (match route.get_clone() {
                                AppRoutes::Home =>  Dashboard(),
                                AppRoutes::Accounts => Accounts(),
                                AppRoutes::Clusters => Clusters(),
                                AppRoutes::Extras => Extras(),
                                _ => view! {
                                    h1 { "Not Found" }
                                },
                            })
                            (Footer())
                        }
                    }
                }
            }
        )
    }
}

#[derive(Route, Clone)]
enum AppRoutes {
    #[to("/")]
    Home,
    #[to("/accounts")]
    Accounts,
    #[to("/clusters")]
    Clusters,
    #[to("/extras")]
    Extras,
    #[not_found]
    NotFound,
}

#[component]
fn Notification() -> View {
    let global_message = use_context::<Signal<GlobalMessage>>();

    if global_message.get_clone().is_empty() {
        return view! {};
    }

    let message_index = move |key: u32| {
        global_message
            .get_clone()
            .into_iter()
            .enumerate()
            .find(|(_, value)| value.key() == key)
            .map(|(index, _value)| index)
    };

    let timer_callback = |secs: u32, key: u32| {
        // Start a timeout for each notification
        spawn_local_scoped(async move {
            let timeout = Timeout::new(secs * 1000, move || {
                message_index(key).map(|index| global_message.update(|store| store.remove(index)));
            });
            timeout.forget();
        });
    };

    let mut key = Some(0u32);

    let notification_views = global_message.get_clone().into_iter().map(|notification_info| {
        key.replace(notification_info.key());
        timer_callback(notification_info.secs(), notification_info.key());

        let message = notification_info.message().to_string();

        view!{
            div(
                on:click=move|_|{
                if let Some(key_inner) = key {
                    message_index(key_inner).map(|index| global_message.update(|store|store.remove(index)));
                }
                key.take();
            },
            class="flex border dark:border-none items-center translate-y-4 animate-fade-in w-full max-w-xs p-2 space-x-2 text-gray-600 bg-white divide-x divide-gray-200 rounded-lg shadow-sm dark:text-gray-400 dark:divide-gray-700 dark:bg-gray-800"
        ){
            div (class="flex w-[25px]"){img(src=notification_svg())}
            div(class="ps-4 text-xs font-normal"){(message)}
        }}
    }).collect::<Vec<View>>();

    view! {
        div (class="cursor-pointer fixed z-[1000] top-4 right-4 flex flex-col space-y-2 min-w-[300px] shadow-sm"){
            (notification_views)
        }
    }
}
