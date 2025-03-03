use wallet_adapter::SigninInput;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    AvatarSvg, ErrorSvg, GlobalAction, GlobalAppState, MintSvg, NotificationInfo, SiwsSvg,
    TimestampSvg,
};

#[function_component]
pub fn SignInWithSolana() -> Html {
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");

    let community = "JamiiDAO";
    let user_id = "X48K48";

    let mut address = String::default();
    let mut public_key = [0u8; 32];
    let mut solana_signin = false;

    if let Ok(wallet_account) = global_state.active_connection.borrow().connected_account() {
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
            .set_domain(global_state.adapter.borrow().window())
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

    html! {
        <div class="flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none">
            <div class="flex w-full flex-col">
                <div class="flex w-full text-true-blue items-center justify-center text-6xl">
                    <span class="flex w-[40px]"> <SiwsSvg/> </span> {"SIWS"}
                </div>
                <div class="flex text-true-blue w-full justify-center text-sm">{"Sign In With Solana"}</div>
                <div class="flex mt-5 flex-col w-full justify-center text-lg">
                    <div class="flex w-full text-lg"> <span class="flex w-[20px] mr-2"> <MintSvg/></span> {"Community: "} {community}</div>
                    <div class="flex w-full text-lg"> <span class="flex w-[20px] mr-2"> <AvatarSvg/> </span> {"User ID: "} {user_id} </div>
                    <div class="flex  text-lg mt-5 text-true-blue dark:text-blue-yonder  w-full">
                        if solana_signin {
                            <div class="flex items-center w-full">
                                <span class="flex w-[25px]"> <TimestampSvg/> </span>
                                {truncate_nonce(&nonce)}
                            </div>
                       }else {
                            <span class="flex text-sm w-[20px] mr-1"> <ErrorSvg/> </span>
                            {"WALLET DOES NOT SUPPORT SIWS"}
                       }
                    </div>

                    if solana_signin {
                        <div class="flex w-full justify-center items-center">
                            <button
                                onclick={
                                    let global_state = global_state.clone();
                                    Callback::from(move|_|{
                                        let global_state = global_state.clone();

                                        let signin_input = signin_input.clone();
                                        spawn_local(async move {
                                            if let Err(error) = global_state.adapter.borrow().sign_in(&signin_input, public_key)
                                            .await{
                                                global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!("SIWS ERROR: {error:?}"))));
                                            }else {
                                                global_state.dispatch(GlobalAction::Message(NotificationInfo::new("SIWS Successful")));
                                            }
                                        });
                                    })
                                }
                                class="bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full">
                                {"SIGN IN"}
                            </button>
                        </div>
                    }
                </div>

            </div>
        </div>
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
