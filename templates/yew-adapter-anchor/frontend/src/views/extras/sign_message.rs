use yew::{platform::spawn_local, prelude::*};

use crate::{GlobalAction, GlobalAppState, NotificationInfo, SignMessageSvg};

#[function_component]
pub fn SignMessage() -> Html {
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");

    let message = "Solana Foundation is awesome!";

    let mut solana_signmessage = false;

    if let Ok(wallet_account) = global_state.active_connection.borrow().connected_account() {
        solana_signmessage = wallet_account.solana_sign_message();
    }

    html! {
        <div class="flex dark:bg-[#160231] bg-white flex-col w-[300px] justify-around p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none">
            <div class="w-full flex flex-col items-center text-center text-true-blue justify-center mb-10">
                <div class="w-[80px] flex flex-col"> <SignMessageSvg /> </div>
                <div class="w-full text-sm"> {"Sign Message"}</div>
            </div>
            <div class="text-lg text-center"> {message} </div>

            <div class="flex items-center justify-center">
                if solana_signmessage {
                    <button class="bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full"
                        onclick={
                            Callback::from(move|_| {
                                let global_state = global_state.clone();

                                spawn_local(async move {
                                        if let Err(error) = global_state.adapter.borrow().sign_message(message.as_bytes()).await{
                                            global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!("SIGN MESSAGE ERROR: {error:?}"))));
                                        }else {
                                            global_state.dispatch(GlobalAction::Message(NotificationInfo::new("Sign Message Successful")));
                                        }
                                });
                            })
                        }
                        >
                        {"SIGN MESSAGE"}
                    </button>
                }else {
                    <div class="w-full items-center justify-center">
                        {"SIGN MESSAGE UNSUPPORTED"}
                    </div>
                }
            </div>
        </div>
    }
}
