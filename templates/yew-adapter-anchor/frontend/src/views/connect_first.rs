use yew::prelude::*;

use crate::WalletSvg;

#[function_component]
pub fn ConnectWalletFirst() -> Html {
    html! {
        <div class="flex w-full text-2xl justify-center items-center">
            <span class="flex w-[30px]"> <WalletSvg/> </span>
            {"Connect a Wallet first!"}
        </div>
    }
}
