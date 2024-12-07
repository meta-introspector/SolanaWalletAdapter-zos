use sycamore::prelude::*;
use wallet_adapter::WalletAdapter;

use crate::{
    Footer, SignAndSendTxComponent, SignInComponent, SignMessageComponent, SignTxComponent,
};

#[component]
pub fn HomePage() -> View {
    let adapter = use_context::<Signal<WalletAdapter>>();

    view! {
        div(class="flex-grow mx-4 lg:mx-auto"){
            div{
                (if adapter.get_clone().is_connected(){
                    view!{
                        (SignInComponent())
                        (SignMessageComponent())
                        (SignTxComponent())
                        (SignAndSendTxComponent())
                    }
                }else{
                    view!(
                        div(class="min-height40vh centered") {"CONNECT A WALLET FIRST"}
                    )
                })

            }
            div(style="position: fixed; z-index: 9999; inset: 16px; pointer-events: none;"){}
        }
        (Footer())
    }
}
