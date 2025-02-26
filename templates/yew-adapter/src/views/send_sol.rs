use std::{cell::RefCell, rc::Rc};

use yew::{platform::spawn_local, prelude::*};

use crate::{
    get_input_value, send_sol_req, AvatarSvg, CloseSvg, ClusterStoreState, GlobalAction,
    GlobalAppState, Loader, NotificationInfo, SendSvg,
};

pub fn SendSol(
    show_send_modal: UseStateHandle<bool>,
    global_state: GlobalAppState,
    cluster_store_state: ClusterStoreState,
    trigger: UseStateHandle<bool>,
) -> Html {
    let mut public_key_bytes = [0u8; 32];

    if let Ok(wallet_account) = global_state.active_connection.borrow().connected_account() {
        public_key_bytes = wallet_account.public_key();
    }

    let lamports = Rc::new(RefCell::new(0u64));
    let recipient = Rc::new(RefCell::new(String::default()));

    let active_cluster = cluster_store_state.active_cluster();
    let endpoint = active_cluster.endpoint().to_string();
    let cluster = active_cluster.cluster();
    let window = global_state.adapter.borrow().window().clone();

    html! {
        <div class="fixed z-10 flex flex-col w-full h-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white">
            <div class="flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 max-h-[60%] lg:w-[90%] max-w-screen-sm justify-start items-center bg-white dark:bg-[#0b0414] rounded-3xl">
                <div class="flex w-full justify-end items-center p-5">
                    <button
                        onclick={
                            let show_send_modal = show_send_modal.clone();

                            Callback::from(move|_|{show_send_modal.set(false)})}
                        class="wallet-adapter-modal-button-close w-[30px] items-center justify-center"> <CloseSvg /> </button>
                </div>
                <div class="overflow-y-scroll max-h-[90%] w-full mb-5 items-center justify-center flex flex-col">
                    <div class="flex text-true-blue dark:text-white w-full items-center justify-center">
                        <span class="w-[30px] flex mb-10 mr-2"> <SendSvg/> </span>
                        <span class="flex mb-10 mr-2 text-3xl">{"Send Lamports"} </span>
                    </div>
                    <div class="flex flex-col w-3/5 mt-2 rounded-3x">
                        <div class="flex w-full items-center rounded-xl p-1 bg-transparent">
                            <div class="shrink-0 select-none text-base text-true-blue dark:text-white sm:text-sm/6 mb-10"> { "SOL" } </div>
                            <input
                                onchange={
                                    let lamports = lamports.clone();
                                    Callback::from(move |event: web_sys::Event| {
                                        if let Ok(data) = get_input_value(event).parse::<u64>(){
                                            *lamports.borrow_mut() = data;
                                        }

                                        web_sys::console::log_1(&format!("LAMPORTS INPUT: {:?}", lamports.borrow()).into());
                                    })
                                }
                                class= "focus:outline-none mb-10 bg-transparent border-b-2 border-true-blue block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6"
                                id="lamports"
                                min="0"
                                name="lamports"
                                placeholder="50000000"
                                type="number"
                            />
                        </div>
                        <div class="flex items-center rounded-xl p-1 bg-transparent">
                            <div class="shrink-0 select-none text-base text-gray-500 sm:text-sm/6"> <span class="flex w-[20px]"> <AvatarSvg/> </span> </div>
                            <input
                                onchange={
                                    let recipient = recipient.clone();
                                    Callback::from(move |event: web_sys::Event| {
                                        *recipient.borrow_mut() = get_input_value(event);

                                        web_sys::console::log_1(&format!("RECIPIENT INPUT: {:?}", recipient.borrow()).into());
                                    })
                                }
                                class="w-full focus:outline-none bg-transparent border-b-2 border-true-blue block min-w-0 grow ml-2 text-black dark:text-white placeholder:text-gray-400 sm:text-sm/6"
                                id="address"
                                min="0"
                                name="address"
                                type="text"
                                placeholder="Enter Recipient Address"
                            />
                        </div>
                    </div>
                <div class="flex w-full items-center justify-center mt-4">
                        <button
                            onclick={
                                let endpoint = endpoint.clone();
                                let recipient = recipient.clone();
                                let lamports = lamports.clone();
                                let global_state = global_state.clone();
                                let window = window.clone();
                                let show_send_modal = show_send_modal.clone();
                                let trigger = trigger.clone();

                                Callback::from(move|_|{
                                    global_state.dispatch(GlobalAction::LoadingTrue(trigger.clone()));


                                    let endpoint = endpoint.clone();
                                    let recipient = recipient.clone();
                                    let lamports = lamports.clone();
                                    let global_state = global_state.clone();
                                    let window = window.clone();
                                    let show_send_modal = show_send_modal.clone();
                                    let trigger = trigger.clone();

                                    spawn_local(async move {

                                        if let Err(error) = send_sol_req(
                                            &recipient.borrow(),
                                            *lamports.borrow(),
                                            &endpoint,
                                            &global_state.adapter.borrow(),
                                            public_key_bytes,
                                            cluster,
                                            &window
                                        ).await {
                                            global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!("SEND SOL ERROR: {:?}", error))));
                                        }

                                        global_state.dispatch(GlobalAction::LoadingFalse(trigger.clone()));

                                        show_send_modal.set(false);

                                        global_state.dispatch(GlobalAction::Message(NotificationInfo::new("Sent")));
                                    });
                                })
                            }
                            class="flex bg-true-blue hover:bg-cobalt-blue text-sm mb-10  dark:stext-white text-black px-4 py-1 items-center justify-center rounded-full">
                            if *global_state.loading.borrow() {
                                <Loader/> <span class="text-white"> {"Sending SOL..." }  </span>
                            }else {
                                {"SEND SOL"}
                            }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
