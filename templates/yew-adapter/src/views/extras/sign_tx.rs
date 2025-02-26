use std::borrow::Borrow;

use solana_sdk::{pubkey::Pubkey, system_instruction, transaction::Transaction};
use wallet_adapter::Utils;
use yew::{platform::spawn_local, prelude::*};

use crate::{ClusterStoreState, GlobalAction, GlobalAppState, NotificationInfo, SignTxSvg};

#[function_component]
pub fn SignTx() -> Html {
    let cluster_store_state =
        use_context::<ClusterStoreState>().expect("no global ctx `ClusterStoreState` found");
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");

    let lamports = 500_000_000u64;

    let mut public_key = [0u8; 32];

    if let Ok(wallet_account) = global_state.active_connection.borrow().connected_account() {
        public_key = wallet_account.public_key();
    }

    html! {
        <div class="flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none">
            <div class="w-full flex flex-col items-center text-center text-true-blue justify-center mb-10">
                <div class="w-[80px] flex flex-col"> <SignTxSvg/> </div>
                <div class="w-full text-sm">{ "Sign Transaction"} </div>
            </div>
            <div class="text-lg text-center"> { "Sign transfer of "} {lamports.to_string()} {" lamports!"} </div>

            <div class="flex items-center justify-center">
                <button class="bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full"
                    onclick={
                        Callback::from(move |_| {
                            let pubkey = Pubkey::new_from_array(public_key);
                            let recipient_pubkey = Pubkey::new_from_array(Utils::public_key_rand());

                            let instr = system_instruction::transfer(&pubkey, &recipient_pubkey, lamports);
                            let tx = Transaction::new_with_payer(&[instr], Some(&pubkey));
                            let tx_bytes = bincode::serialize(&tx).unwrap();
                            let cluster = cluster_store_state.active_cluster().borrow().cluster();


                            let global_state = global_state.clone();

                            spawn_local(async move {
                                match global_state.adapter.borrow().sign_transaction(&tx_bytes, Some(cluster)).await{
                                    Err(error) => global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!("SIGN MESSAGE ERROR: {error:?}")))
                                    ),
                                    Ok(output) => {
                                        if let Err(error) = bincode::deserialize::<Transaction>(&output[0]){
                                            global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!("SIGN TX ERROR: {error:?}"))));
                                        }else {
                                            global_state.dispatch(GlobalAction::Message(NotificationInfo::new("Sign Transaction Successful")));
                                        }
                                    }
                                }
                            })
                        })
                    }
                    > {"SIGN TRANSACTION"}
                </button>
            </div>
        </div>
    }
}
