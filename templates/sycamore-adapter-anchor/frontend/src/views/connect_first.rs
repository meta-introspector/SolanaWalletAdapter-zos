use sycamore::prelude::*;

use crate::wallet_svg;

#[component]
pub fn ConnectWalletFirst() -> View {
    view! {
        div (class="flex w-full text-2xl justify-center items-center"){
            span (class="flex w-[30px]"){img(src=wallet_svg())}
            "Connect a Wallet first!"
        }
    }
}
