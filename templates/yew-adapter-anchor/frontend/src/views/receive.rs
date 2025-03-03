use yew::{platform::spawn_local, prelude::*};

use crate::{
    copied_address, CloseSvg, CopySvg, GlobalAction, GlobalAppState, NotificationInfo, ReceiveSvg,
};

pub fn ReceiveSol(show_receive_modal: UseStateHandle<bool>, global_state: GlobalAppState) -> Html {
    let address = global_state
        .active_connection
        .borrow()
        .connected_account()
        .cloned()
        .unwrap_or_default()
        .address()
        .to_string();
    let shortened_address = global_state
        .active_connection
        .borrow()
        .connected_account()
        .cloned()
        .unwrap_or_default()
        .shorten_address()
        .unwrap_or_default()
        .to_string();

    let window = global_state.adapter.borrow().window().clone();

    let qrcode = crate::address_qrcode(&address);
    let address_inner = address.clone();

    html! {
        <div class="fixed overflow-y-hidden min-h-screen z-10 flex flex-col w-full bg-[rgba(0,0,0,0.6)] justify-center items-center text-black dark:text-white">
            <div class="flex flex-col w-[90%] sm:w-[80%] md:w-[70%] min-h-64 lg:w-[90%] max-w-screen-sm justify-start items-center bg-white dark:bg-[#0b0414] rounded-3xl">
                <div class="flex w-full justify-end items-center p-5">
                    <button
                        onclick={
                            let show_receive_modal = show_receive_modal.clone();
                            move|_|{ show_receive_modal.set(false); }
                        }
                        class="wallet-adapter-modal-button-close w-[30px] items-center justify-center">
                        <span class="flex w-[20px]"> <CloseSvg/> </span>
                    </button>
                </div>
                <div class="overflow-y-scroll w-full mb-5 items-center justify-center flex flex-col">
                    <div class="flex w-full items-center justify-center text-2xl">
                        <span class="w-[30px] mr-2"> <ReceiveSvg /> </span> {"Receive SOL"}
                    </div>
                    <div class="mb-2 mt-5 rounded-full bg-true-blue hover:bg-cobalt-blue cursor-pointer"
                        onclick={
                            let window = window.clone();
                            Callback::from(move|_| {
                                let address_inner = address_inner.clone();
                                let window = window.clone();
                                let global_state = global_state.clone();

                                spawn_local(async move {
                                    if let Err(error) = copied_address(&address_inner, &window).await {
                                        global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!("COPY ERROR: {:?}", error))));
                                    } else {
                                        global_state.dispatch(GlobalAction::Message(NotificationInfo::new("Copied to clipboard")));
                                    }
                                });
                            })
                        }>
                        <div class="flex justify-left items-center px-2 py-1 text-white rounded-full ">
                            <span class="flex p-2 w-[30px]"> <CopySvg/> </span>
                            <span> { shortened_address } </span>
                        </div>
                    </div>
                    <div class="w-[200px] rounded-xl flex mt-5 mb-5 bg-white"> <img src={qrcode} /> </div>
                </div>
            </div>
        </div>
    }
}
