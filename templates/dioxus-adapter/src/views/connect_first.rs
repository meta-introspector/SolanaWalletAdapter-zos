use dioxus::prelude::*;

#[component]
pub fn ConnectWalletFirst() -> Element {
    rsx! {
        div {class:"flex w-full text-2xl justify-center items-center",
            span { class:"flex w-[30px]",}
            "Connect a Wallet first!"
        }
    }
}
