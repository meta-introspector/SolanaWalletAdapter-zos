use partial_idl_parser::AnchorIdlPartialData;
use solana_sdk::{pubkey::Pubkey, instruction::Instruction, transaction::Transaction};
use wallet_adapter::Utils;
use yew::{platform::spawn_local, prelude::*};

use crate::{ClusterStoreState, GlobalAction,IDL_RAW_DATA, GlobalAppState, NotificationInfo, SignTxSvg, get_blockhash};

#[function_component]
pub fn SignTx() -> Html {
    let cluster_store_state =
        use_context::<ClusterStoreState>().expect("no global ctx `ClusterStoreState` found");
    let global_state =
        use_context::<GlobalAppState>().expect("no global ctx `GlobalAppState` found");

    let mut public_key = [0u8; 32];

    if let Ok(wallet_account) = global_state.active_connection.borrow().connected_account() {
        public_key = wallet_account.public_key();
    }

    let endpoint = cluster_store_state.active_cluster().endpoint().to_string();
    let window = global_state.adapter.borrow().window().clone();

    let parsed_idl = AnchorIdlPartialData::parse(IDL_RAW_DATA).unwrap();
    let discriminant = parsed_idl
        .get_discriminant("initialize")
        .unwrap_or_default();
    let program_id = parsed_idl.program_id().to_string();
    let shortened_program_address = Utils::shorten_base58(parsed_idl.program_id()).unwrap_or("Error: Invalid Base58 program ID".into()).to_string();

    html! {
        <div class="flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none">
            <div class="w-full flex flex-col items-center text-center text-true-blue justify-center mb-10">
                <div class="w-[80px] flex flex-col"> <SignTxSvg/> </div>
                <div class="w-full text-sm">{ "Sign Transaction"} </div>
            </div>
            <div class="text-lg text-center"> { "Greetings from "} {shortened_program_address} {" program!"} </div>

            <div class="flex items-center justify-center">
                <button class="bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full"
                    onclick={
                        Callback::from(move |_| {
                            let endpoint = endpoint.clone();
                            let window = window.clone();
                            let global_state = global_state.clone();
                            let cluster_store_state = cluster_store_state.clone();
                            let program_id = program_id.clone();

                            spawn_local(async move {
                                match get_blockhash(&endpoint, &window).await {
                                    Err(error) => {
                                        global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!("Unable to get the blockhash. This transactions is likely to fail. Error: {error:?}!"))));
                                    },
                                    Ok(blockhash) => {
                                        let pubkey = Pubkey::new_from_array(public_key);
    
                                        let program_id = Pubkey::from_str_const(&program_id);
    
                                        let ix = Instruction {
                                            program_id,
                                            accounts: vec![],
                                            data: discriminant.to_vec(),
                                        };
    
                                        let mut tx = Transaction::new_with_payer(&[ix], Some(&pubkey));
                                        tx.message.recent_blockhash = blockhash;
                                        let tx_bytes = bincode::serialize(&tx).unwrap();
                                        let cluster = cluster_store_state.active_cluster().cluster();
                                   
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
