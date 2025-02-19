use solana_sdk::{pubkey::Pubkey, system_instruction, transaction::Transaction};
use sycamore::{futures::spawn_local_scoped, prelude::*};
use wallet_adapter::{ConnectionInfo, Utils, WalletAdapter};

use crate::{app::GlobalMessage, sign_tx_svg, ClusterStore, NotificationInfo};

#[component]
pub fn SignTx() -> View {
    let adapter = use_context::<Signal<WalletAdapter>>();
    let active_connection = use_context::<Signal<ConnectionInfo>>();
    let global_message = use_context::<Signal<GlobalMessage>>();
    let cluster_storage = use_context::<Signal<ClusterStore>>();

    let lamports = 500_000_000u64;

    let mut public_key = [0u8; 32];

    if let Ok(wallet_account) = active_connection.get_clone().connected_account() {
        public_key = wallet_account.public_key();
    }

    view! {
        div (class="flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none"){
            div (class="w-full flex flex-col items-center text-center text-true-blue justify-center mb-10"){
                div(class="w-[80px] flex flex-col"){ img(src=sign_tx_svg())}
                div(class="w-full text-sm"){ "Sign Transaction"}
            }
            div (class="text-lg text-center"){ "Sign transfer of " (lamports.to_string()) " lamports!" }

        div (class="flex items-center justify-center"){
                button(class="bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full",
                    on:click=move |_| {
                        spawn_local_scoped(async move {
                            let pubkey = Pubkey::new_from_array(public_key);
                            let recipient_pubkey = Pubkey::new_from_array(Utils::public_key_rand());

                            let instr = system_instruction::transfer(&pubkey, &recipient_pubkey, lamports);
                            let tx = Transaction::new_with_payer(&[instr], Some(&pubkey));
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
                        });
                    }){
                    "SIGN TRANSACTION"
                }
            }
        }
    }
}
