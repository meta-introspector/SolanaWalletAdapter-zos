use std::{cell::RefCell, rc::Rc};

use solana_sdk::native_token::LAMPORTS_PER_SOL;
use yew::{platform::spawn_local, prelude::*};

use crate::{
    get_input_value, request_airdrop, AirdropSvg, CloseSvg, ClusterStoreState, GlobalAction,
    GlobalAppState, Loader, NotificationInfo,
};

pub fn Airdrop(
    show_airdrop_modal: UseStateHandle<bool>,
    global_state: GlobalAppState,
    cluster_store_state: ClusterStoreState,
    trigger: UseStateHandle<bool>,
) -> Html {
    let lamports = Rc::new(RefCell::new(LAMPORTS_PER_SOL));
    let address = global_state
        .active_connection
        .borrow()
        .connected_account()
        .unwrap()
        .address()
        .to_string();
    let window = global_state.adapter.borrow().window().clone();
    let endpoint = cluster_store_state.active_cluster().endpoint().to_string();

    html! {
        <div class="fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white">
            <div class="flex flex-col bg-rich-black w-[60%] min-h-[40vh] rounded-lg p-5 shadow-2xl">
                <div class="flex w-full justify-end items-center p-5">
                    <button
                        onclick={

                            let show_airdrop_modal = show_airdrop_modal.clone();
                            Callback::from(move |_| {
                                show_airdrop_modal.set(bool::default());
                            })
                        }
                        class="wallet-adapter-modal-button-close w-[30px] items-center justify-center"
                    >
                        <span class="flex w-[20px]"> <CloseSvg/> </span>
                    </button>
                </div>
                <div class="overflow-y-scroll max-h-[90%] w-full mb-5 items-center justify-center flex flex-col">
                    <div class="flex w-full items-center justify-center text-2xl">
                        <span class="w-[50px] mr-2"> <AirdropSvg/> </span> {"Request Airdrop"}
                    </div>
                    <div class="mt-2 rounded-3x">
                        <div class="flex items-center rounded-xl p-1 bg-transparent">
                            <div class="shrink-0 select-none text-base text-gray-500 sm:text-sm/6">{"SOL" }</div>
                            <input
                                onchange={
                                    let lamports = lamports.clone();
                                    Callback::from(move |event: web_sys::Event| {
                                        let element =  get_input_value(event);

                                        let data = element.parse::<u64>().unwrap_or(1);
                                        *lamports.borrow_mut() = data*LAMPORTS_PER_SOL;
                                    })
                                }
                                class="focus:outline-none bg-transparent border-b-2 border-white block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6"
                                id="airdrop" min="0" name="airdrop" placeholder="2" type="number" value="2"/>
                        </div>
                    </div>
                    <div class="flex-w-full items-center justify-center mt-4">
                        <button
                        disabled={*global_state.loading.borrow()}
                            onclick={

                                let endpoint = endpoint.clone();
                                let address = address.clone();
                                let lamports = lamports.clone();
                                let window = window.clone();
                                let global_state = global_state.clone();
                                let show_airdrop_modal = show_airdrop_modal.clone();
                                let trigger = trigger.clone();

                                Callback::from(move|_|{
                                    let endpoint = endpoint.clone();
                                    let address = address.clone();
                                    let lamports = lamports.clone();
                                    let window = window.clone();
                                    let global_state = global_state.clone();
                                    let show_airdrop_modal = show_airdrop_modal.clone();
                                    let trigger = trigger.clone();

                                    spawn_local(async move {

                                        global_state.dispatch(GlobalAction::LoadingTrue(trigger.clone()));

                                        if request_airdrop(*lamports.borrow(), &address, &endpoint, &window).await.is_err() {
                                            global_state.dispatch(GlobalAction::Message(
                                                NotificationInfo::error("REQUEST AIRDROP ERROR: You might have reached your daily limit.")
                                            ));
                                        }else {
                                            global_state.dispatch(GlobalAction::Message(
                                                NotificationInfo::new("REQUESTED AIRDROP")
                                            ));
                                        }

                                        show_airdrop_modal.set(bool::default());
                                        global_state.dispatch(GlobalAction::LoadingFalse(trigger.clone()));
                                    });
                                })
                            }
                            class="flex text-sm bg-true-blue hover:bg-cobalt-blue text-white px-4 py-2 items-center justify-center rounded-full">
                            if *global_state.loading.borrow() {
                                <Loader/> {"Requesting Airdrop..."}
                            }else {
                                {"REQUEST AIRDROP"}
                            }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
