use gloo_timers::callback::Timeout;
use wallet_adapter::WalletAdapter;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{
    Accounts, ClusterStore, ClusterStoreState, Clusters, Dashboard, Extras, GlobalAction,
    GlobalAppInfo, GlobalAppState, NetState, NetStateInfo, NotificationBellSvg, NotificationInfo,
};

const LOGO_PATH: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/logo.png"));

pub fn logo() -> String {
    String::from("data:image/png;base64,") + data_encoding::BASE64.encode(LOGO_PATH).as_str()
}

#[function_component(App)]
pub fn app() -> Html {
    let adapter = WalletAdapter::init().unwrap();

    let events = adapter.events().clone();

    let init_state = GlobalAppInfo::new(adapter);

    let global_state = use_reducer(|| init_state);
    let cluster_store_state = use_reducer(|| ClusterStore::new());
    let net_state = use_reducer(|| NetStateInfo::default());

    let global_state_inner = global_state.clone();

    spawn_local(async move {
        while let Ok(event) = events.recv().await {
            global_state_inner.dispatch(GlobalAction::Message(NotificationInfo::new(event)));
        }
    });

    html! {
        <ContextProvider<GlobalAppState> context={global_state.clone()}>
            <ContextProvider<ClusterStoreState> context={cluster_store_state.clone()}>
                <ContextProvider<NetState> context={net_state.clone()}>

                <BrowserRouter>
                <div class="w-full flex min-h-screen font-[sans-serif] dark:bg-rich-black bg-white text-black dark:text-white">
                    if global_state.messages.borrow().is_empty() { <span></span> }else {<Notification />}
                    <Switch<Route> render={switch} />
                </div>
                </BrowserRouter>
                </ContextProvider<NetState>>
            </ContextProvider<ClusterStoreState>>
        </ContextProvider<GlobalAppState>>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/")]
    Home,
    #[at("/accounts")]
    Accounts,
    #[at("/clusters")]
    Clusters,
    #[at("/extras")]
    Extras,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Dashboard/>},
        Route::Accounts => html! {<Accounts/>},
        Route::Clusters => html! {<Clusters/>},
        Route::Extras => html! {<Extras/>},
        Route::NotFound => html! { <h1>{ "Page Not Found" }</h1> },
    }
}

#[function_component]
fn Notification() -> Html {
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");

    let global_state_inner = global_state.clone();
    let global_state_1 = global_state.clone();

    let timer_callback = |secs: u32, key: u32| {
        // Start a timeout for each notification
        spawn_local(async move {
            let timeout = Timeout::new(secs * 1000, move || {
                global_state_inner.dispatch(GlobalAction::RemoveMessage(key));
            });
            timeout.forget();
        });
    };

    let notification_views = global_state_1.messages.borrow().iter().map(|notification_info| {
        timer_callback.clone()(notification_info.secs(), notification_info.key());

        let message = notification_info.message().to_string();
        let key = notification_info.key();

        html!{
            <div
                onclick={
                    let app_context_inner2 = global_state_1.clone();
                    Callback::from(move|_|{
                        let app_context_inner2 = app_context_inner2.clone();
                        spawn_local(async move {app_context_inner2.dispatch(GlobalAction::RemoveMessage(key));});
                    })
                }
            class="flex border dark:border-none items-center translate-y-4 animate-fade-in w-full max-w-xs p-2 space-x-2 text-gray-600 bg-white divide-x divide-gray-200 rounded-lg shadow-sm dark:text-gray-400 dark:divide-gray-700 dark:bg-gray-800">
                <div class="flex w-[25px]"> <NotificationBellSvg/> </div>
                <div class="ps-4 text-xs font-normal"> {message} </div>
            </div>
        }
    }).collect::<Vec<Html>>();

    html! {
        <div class="cursor-pointer fixed z-[1000] top-4 right-4 flex flex-col space-y-2 min-w-[300px] shadow-sm">
            {notification_views}
        </div>
    }
}
