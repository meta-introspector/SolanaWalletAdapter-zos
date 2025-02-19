use sycamore::prelude::*;
use wallet_adapter::ConnectionInfo;

use crate::views::{ConnectWalletFirst, SignInWithSolana, SignMessage, SignTx};

#[component]
pub fn Extras() -> View {
    let active_connection = use_context::<Signal<ConnectionInfo>>();

    if active_connection.get_clone().connected_account().is_ok() {
        view! {
            div (class="flex justify-center mt-10 mb-5 gap-8 w-full flex-wrap items-stretch"){
                SignInWithSolana{}
                SignMessage{}
                SignTx{}
            }
        }
    } else {
        view! {ConnectWalletFirst {}}
    }
}
