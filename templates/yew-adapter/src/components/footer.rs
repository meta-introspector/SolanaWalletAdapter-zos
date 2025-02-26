use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
    <footer class="flex p-3 items-center justify-center w-full ">
        <div>
            <p class="flex flex-wrap md:flex-row lg:flex-row items-center justify-center">
                <span>{"Generated Using "}</span>
                <a href="https://crates.io/crates/cargo-generate" class="ml-2 mr-2 underline">{"cargo-generate"}</a>
                {"and"}
                <a href="https://github.com/JamiiDao/SolanaWalletAdapter/tree/master/templates" class="ml-2 underline">{"Rust Wallet Adapter Yew Template"}</a>
            </p>
        </div>
    </footer>}
}
