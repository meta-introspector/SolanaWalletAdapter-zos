use sycamore::prelude::*;

#[component]
pub fn Footer() -> View {
    view! {
        footer(class="footer footer-center p-4 bg-base-300 text-base-content"){
            aside{
                p { "Generated using "
                    a(class="link hover:text-white", href="https://cargo-generate.github.io/cargo-generate/"){ "cargo-generate"}
                    " with template "
                    a(class="link hover:text-white",
                        href="https://github.com/JamiiDao/Solana-Rust-Wallet-Adapter-Templates/tree/master/yew", target="_blank",
                        rel="noopener noreferrer"){"Yew Template" }
                }
            }
        }
    }
}
