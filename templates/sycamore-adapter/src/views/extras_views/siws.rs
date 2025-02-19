use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::{ConnectionInfo, SigninInput, WalletAdapter};

use crate::{
    app::GlobalMessage, avatar_svg, error_svg, siws_svg, timestamp_svg, token_mint_svg,
    NotificationInfo,
};

#[component]
pub fn SignInWithSolana() -> View {
    let adapter = use_context::<Signal<WalletAdapter>>();
    let active_connection = use_context::<Signal<ConnectionInfo>>();
    let global_message = use_context::<Signal<GlobalMessage>>();

    let community = "JamiiDAO";
    let user_id = "X48K48";

    let mut address = String::default();
    let mut public_key = [0u8; 32];
    let mut solana_signin = false;

    if let Ok(wallet_account) = active_connection.get_clone().connected_account() {
        address = wallet_account.address().to_string();
        public_key = wallet_account.public_key();
        solana_signin = wallet_account.solana_signin();
    }

    // Check if wallet supported SIWS
    let (signin_input, nonce, public_key) = if solana_signin {
        let mut signin_input = SigninInput::new();
        signin_input.set_nonce();
        let nonce = signin_input.nonce().unwrap().clone();

        let message = String::new()
            + "Community: "
            + community
            + "USER ID: "
            + user_id
            + "SESSION: "
            + nonce.as_str();

        signin_input
            .set_domain(adapter.get_clone().window())
            .unwrap()
            .set_statement(&message)
            .set_chain_id(wallet_adapter::Cluster::DevNet)
            // NOTE: Some wallets require this field or the wallet adapter
            // will return an error `MessageResponseMismatch` which is as
            // a result of the sent message not corresponding with the signed message
            .set_address(&address)
            .unwrap();

        (signin_input, nonce, public_key)
    } else {
        (SigninInput::default(), String::default(), [0u8; 32])
    };

    view! {
        div (class="flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none"){
            div (class="flex w-full flex-col"){
                div(class="flex w-full text-true-blue items-center justify-center text-6xl"){
                    span(class="flex w-[40px]"){img(src=(siws_svg()))} "SIWS"
                }
                div(class="flex text-true-blue w-full justify-center text-sm"){"Sign In With Solana"}
                div(class="flex mt-5 flex-col w-full justify-center text-lg"){
                    div(class="flex w-full text-lg"){span(class="flex w-[20px] mr-2"){ img(src=token_mint_svg()) } "Community: " (community)}
                    div(class="flex w-full text-lg"){span(class="flex w-[20px] mr-2"){ img(src=avatar_svg()) } "User ID: " (user_id) }
                    div(class="flex  text-lg mt-5 text-true-blue dark:text-blue-yonder  w-full"){
                        (if solana_signin {
                            view!{div(class="flex items-center w-full"){
                                span(class="flex w-[25px]"){img(src=timestamp_svg())}
                                (truncate_nonce(&nonce))
                            }}
                       }else {
                            view!{span (class="flex text-sm w-[20px] mr-1"){ img(src=error_svg())  }
                            "WALLET DOES NOT SUPPORT SIWS"}
                       })
                    }

                    (if solana_signin {
                        view!{div (class="flex w-full justify-center items-center"){
                            button (on:click=move|_|{
                                        let signin_input = signin_input.clone();
                                        spawn_local_scoped(async move {
                                            if let Err(error) = adapter.get_clone().sign_in(&signin_input, public_key)
                                            .await{
                                                global_message.update(|store|store.push_back(NotificationInfo::error(format!("SIWS ERROR: {error:?}"))));
                                            }else {
                                                global_message.update(|store|store.push_back(NotificationInfo::new("SIWS Successful")));
                                            }
                                        });
                                },
                                class="bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full"){
                                "SIGN IN"
                            }
                        }}
                    }else {view!{}})
                }

            }
        }
    }
}

fn truncate_nonce(value: &str) -> String {
    let value = String::from("SESSION: ") + value;

    if value.len() <= 20 {
        value.to_string()
    } else {
        value.chars().take(20).collect::<String>() + "..."
    }
}
