use yew::prelude::*;

use crate::{header, AccountInfoData, ClusterStoreState, Footer, GlobalAppState, NetState};

#[function_component]
pub fn Dashboard() -> Html {
    let cluster_store_state =
        use_context::<ClusterStoreState>().expect("no global ctx `ClusterStoreState` found");
    let net_state = use_context::<NetState>().expect("no global ctx `NetState` found");
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");

    let re_render = use_state(|| bool::default());
    let account_info_state = use_reducer(|| AccountInfoData::default());

    html! {
        <div class= "flex flex-col w-full min-h-full justify-between items-center">
            {header(re_render.clone(), cluster_store_state.clone(), net_state.clone(), global_state.clone(), account_info_state.clone())}
            <div class="flex flex-col justify-around items-center w-full m-h-[100%] h-full p-5">
                <h1 class="text-black dark:text-gray-300 text-6xl">{"gm"}</h1>
                <h2 class="dark:text-gray-300 text-2xl">{"Say hi to your new Solana dApp."}</h2>
                <div class="text-center flex flex-col justify-between items-center">
                    <h3 class="text-black dark:text-gray-300 text-md">{"Here are some helpful links to get you started."}</h3>
                    <div>
                        <a class="underline text-black dark:text-gray-300 text-md" href="https://crates.io/crates/wallet-adapter">{"Rust Wallet Adapter (crates.io)"}</a>
                    </div>
                    <div>
                        <a class="underline text-black dark:text-gray-300 text-md" href="https://github.com/JamiiDao/SolanaWalletAdapter">{"Rust Wallet Adapter (Github)"}</a>
                    </div>
                    <div>
                        <a class="underline text-black dark:text-gray-300 text-md" href="https://github.com/JamiiDao/SolanaWalletAdapter/tree/master/examples">{"Rust Wallet Adapter Examples"}</a>
                    </div>
                    <div><
                        a class="underline text-black dark:text-gray-300 text-md" href="https://github.com/JamiiDao/SolanaWalletAdapter/tree/master/templates">{"Rust Wallet Adapter Templates"}</a>
                    </div>
                    <div>
                        <a class="underline text-black dark:text-gray-300 text-md" href="https://docs.solana.com/">{"Solana Docs"}</a>
                    </div>
                    <div>
                        <a class="underline text-black dark:text-gray-300 text-md" href="https://faucet.solana.com/">{"Solana Faucet"}</a>
                    </div>
                    <div>
                        <a class="underline text-black dark:text-gray-300 text-md" href="https://solanacookbook.com/">{"Solana Cookbook"}</a>
                    </div>
                    <div>
                        <a class="underline text-black dark:text-gray-300 text-md" href="https://solana.stackexchange.com/">{"Solana Stack Overflow"}</a>
                    </div>
                    <div>
                        <a class="underline text-black dark:text-gray-300 text-md" href="https://github.com/solana-developers/">{"Solana Developers GitHub"}</a>
                    </div>
                </div>
            </div>
            <Footer/>
        </div>
    }
}
