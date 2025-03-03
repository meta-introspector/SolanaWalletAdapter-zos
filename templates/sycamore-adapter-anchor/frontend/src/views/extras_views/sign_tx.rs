use solana_sdk::{pubkey::Pubkey, transaction::Transaction, instruction::Instruction};
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::{ConnectionInfo, Utils, WalletAdapter};
use partial_idl_parser::AnchorIdlPartialData;

use crate::{app::GlobalMessage, sign_tx_svg, ClusterStore, NotificationInfo, get_blockhash, IDL_RAW_DATA};

#[component]
pub fn SignTx() -> View {
    let adapter = use_context::<Signal<WalletAdapter>>();
    let active_connection = use_context::<Signal<ConnectionInfo>>();
    let global_message = use_context::<Signal<GlobalMessage>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();
    let endpoint = cluster_storage.get_clone().active_cluster().endpoint().to_string();

    let mut public_key = [0u8; 32];

    if let Ok(wallet_account) = active_connection.get_clone().connected_account() {
        public_key = wallet_account.public_key();
    }

    let parsed_idl = AnchorIdlPartialData::parse(IDL_RAW_DATA).unwrap();
    let discriminant = parsed_idl
        .get_discriminant("initialize")
        .unwrap_or_default();
    let program_id = parsed_idl.program_id().to_string();
    let shortened_program_address = Utils::shorten_base58(parsed_idl.program_id()).unwrap_or("Error: Invalid Base58 program ID".into()).to_string();


    view! {
        div (class="flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none"){
            div (class="w-full flex flex-col items-center text-center text-true-blue justify-center mb-10"){
                div(class="w-[80px] flex flex-col"){ img(src=sign_tx_svg())}
                div(class="w-full text-sm"){ "Sign Transaction"}
            }
            div (class="text-lg text-center"){ 
                "Greetings from " (shortened_program_address) " program!"
            }

        div (class="flex items-center justify-center"){
                button(class="bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full",
                    on:click=move |_| {
                        let endpoint = endpoint.clone();
                        let program_id = program_id.clone();
                        
                        spawn_local_scoped(async move {
                            match get_blockhash(&endpoint).await {
                                Err(error) => {
                                    global_message.update(|store|store.push_back(NotificationInfo::error(format!("Unable to get the blockhash. This transactions is likely to fail. Error: {error:?}!"))));
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
                                    let cluster = cluster_storage.get_clone().active_cluster().cluster();

                                    match adapter.get_clone().sign_transaction(&tx_bytes, Some(cluster)).await{
                                        Err(error) => global_message.update(|store| store.push_back(
                                                NotificationInfo::error(format!("SIGN MESSAGE ERROR: {error:?}"))
                                            )
                                        ),
                                        Ok(output) => {
                                            if let Err(error) = bincode::deserialize::<Transaction>(&output[0]){
                                                global_message.update(|store|store.push_back(
                                                    NotificationInfo::error(format!("SIGN TX ERROR: {error:?}"))
                                                    )
                                                );
                                            }else {
                                                global_message.update(|store|store.push_back(
                                                    NotificationInfo::new("Sign Transaction Successful")
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                            
                        });
                    }){
                    "SIGN TRANSACTION"
                }
            }
        }
    }
}
