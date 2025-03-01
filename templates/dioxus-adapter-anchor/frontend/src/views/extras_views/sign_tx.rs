use dioxus::prelude::*;
use partial_idl_parser::AnchorIdlPartialData;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, transaction::Transaction};
use wallet_adapter::Utils;

use crate::{
    fetch_parser::get_blockhash, NotificationInfo, SignTxSvg, ACTIVE_CONNECTION, CLUSTER_STORAGE,
    GLOBAL_MESSAGE, IDL_RAW_DATA, WALLET_ADAPTER,
};

#[component]
pub fn SignTx() -> Element {
    let mut public_key = [0u8; 32];

    if let Ok(wallet_account) = ACTIVE_CONNECTION.read().connected_account() {
        public_key = wallet_account.public_key();
    }

    let parsed_idl = AnchorIdlPartialData::parse(IDL_RAW_DATA).unwrap();
    let discriminant = parsed_idl
        .get_discriminant("initialize")
        .unwrap_or_default();
    let program_id = parsed_idl.program_id().to_string();

    rsx! {
        div { class:"flex dark:bg-[#160231] bg-white flex-col w-[300px] p-5 rounded-lg dark:shadow-2xl shadow-sm border dark:border-none",
            div {class:"w-full flex flex-col items-center text-center text-true-blue justify-center mb-10",
                div{class:"w-[80px] flex flex-col", {SignTxSvg()}}
                div{class:"w-full text-sm", "Sign Transaction"}
            }
            div { class:"text-lg text-center",
            " Greetings from " {Utils::shorten_base58(parsed_idl.program_id()).unwrap_or("Error: Invalid Base58 program ID".into())} " program!"
            }

        div { class:"flex items-center justify-center",
                button{
                    class: "bg-true-blue  hover:bg-cobalt-blue mt-5 text-sm text-white px-5 py-2 rounded-full",
                    onclick: move |_| {
                        let program_id = program_id.clone();
                        spawn(async move {
                            let pubkey = Pubkey::new_from_array(public_key);

                            let program_id = Pubkey::from_str_const(&program_id);

                            let ix = Instruction {
                                program_id,
                                accounts: vec![],
                                data: discriminant.to_vec(),
                            };

                            match get_blockhash().await {
                                Err(error) => {
                                    GLOBAL_MESSAGE.write().push_back(NotificationInfo::error(format!("Unable to get the blockhash. This transactions is likely to fail. Error: {error:?}!")));
                                },
                                Ok(blockhash) => {
                                    let mut tx = Transaction::new_with_payer(&[ix], Some(&pubkey));
                                    tx.message.recent_blockhash = blockhash;
                                    let tx_bytes = bincode::serialize(&tx).unwrap();
                                    let cluster = CLUSTER_STORAGE.read().active_cluster().cluster();

                                    match WALLET_ADAPTER.read().sign_transaction(&tx_bytes, Some(cluster)).await{
                                        Err(error) => GLOBAL_MESSAGE.write().push_back(
                                                NotificationInfo::error(
                                                    format!("SIGN MESSAGE ERROR: {error:?}")
                                                )
                                            ),
                                        Ok(output) => {
                                            if let Err(error) = bincode::deserialize::<Transaction>(&output[0]){
                                                GLOBAL_MESSAGE.write().push_back(
                                                    NotificationInfo::error(
                                                        format!("SIGN TX ERROR: {error:?}")
                                                    )
                                                );
                                            }else {
                                                GLOBAL_MESSAGE.write().push_back(
                                                    NotificationInfo::new("Sign Transaction Successful")
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    },
                    "SIGN TRANSACTION"
                }
            }
        }
    }
}
