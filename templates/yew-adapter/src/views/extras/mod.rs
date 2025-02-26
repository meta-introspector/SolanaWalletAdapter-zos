use yew::prelude::*;

use crate::{
    header, AccountInfoData, ClusterStoreState, ConnectWalletFirst, Footer, GlobalAppState,
    NetState,
};

mod siws;
pub use siws::*;

mod sign_message;
use sign_message::*;

mod sign_tx;
use sign_tx::*;

#[function_component]
pub fn Extras() -> Html {
    let cluster_store_state =
        use_context::<ClusterStoreState>().expect("no global ctx `ClusterStoreState` found");
    let net_state = use_context::<NetState>().expect("no global ctx `NetState` found");
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");
    let account_info_state = use_reducer(|| AccountInfoData::default());

    let re_render = use_state(|| bool::default());

    html! {
        <div class= "flex flex-col w-full min-h-full justify-between items-center">
            {header(re_render.clone(), cluster_store_state.clone(), net_state.clone(), global_state.clone(), account_info_state.clone())}

            if global_state
                .active_connection
                .borrow()
                .connected_account()
                .is_ok()
            {
                <div class="flex justify-center mt-10 mb-5 gap-8 w-full flex-wrap items-stretch">
                    <SignInWithSolana />
                    <SignMessage />
                    <SignTx />
                </div>
            } else {
                <ConnectWalletFirst />
            }
            <Footer/>
        </div>
    }
}
