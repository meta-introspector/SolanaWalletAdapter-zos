use std::borrow::BorrowMut;

use wallet_adapter::Wallet;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::{Route, YewAdapter};

#[function_component(Header)]
pub fn header() -> Html {
    let adapter = use_context::<YewAdapter>().expect("no global ctx `YewAdapter` found");
    let connection = adapter.clone();
    let show_dropdown = use_state(|| "wallet-adapter-dropdown-list");
    let show_modal = use_state(|| false);
    let show_dropdow_inner = show_dropdown.clone();

    let toggle_dropdown = move || {
        let show_dropdow_inner = show_dropdow_inner.clone();
        if *show_dropdow_inner == "wallet-adapter-dropdown-list" {
            show_dropdow_inner
                .clone()
                .set("wallet-adapter-dropdown-list wallet-adapter-dropdown-list-active");
        } else {
            show_dropdow_inner.set("wallet-adapter-dropdown-list");
        }
    };

    html! {
        <div class="navbar bg-base-300 text-neutral-content flex-col md:flex-row space-y-2 md:space-y-0">
            <div class="flex-1">
                <a class="btn btn-ghost normal-case text-xl" href="/">
                    <img class="h-4 md:h-6" alt="Logo" src="/logo.png" />
                </a>
                <ul class="menu menu-horizontal px-1 space-x-2">
                    <li> <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>> </li>
                </ul>
            </div>

            <div class="flex-none space-x-2">
                <div class="wallet-adapter-dropdown">
                    if adapter.as_ref().borrow().is_connected() {
                        <div class="wallet-adapter-dropdown">
                            <button
                                onclick={
                                    let toggle_dropdown = toggle_dropdown.clone();
                                    Callback::from(move |_| {
                                    toggle_dropdown();
                                })}
                                onblur={Callback::from(move |_| {
                                    toggle_dropdown();
                                })}

                                type="button"
                                tabindex="0"
                                class="wallet-adapter-button wallet-adapter-button-trigger">
                                    <i class={"wallet-adapter-button-start-icon"}>
                                        <img src={adapter.as_ref().borrow().connected_wallet().as_ref().unwrap().icon().as_ref().unwrap().to_string()}
                                            alt="{adapter.read().connection.connected_wallet().as_ref().unwrap().name()} icon" />
                                    </i>
                                    {adapter.as_ref().borrow().connected_account().as_ref().unwrap().shorten_address().unwrap()}
                                </button>
                            <ul role="menu" aria-label="dropdown-list"
                                class={*show_dropdown}>
                                <li role="menuitem" class="wallet-adapter-dropdown-list-item"> {"Copy address"} </li>
                                <li onclick={
                                    let show_modal = show_modal.clone();

                                    Callback::from(move|_| {
                                        show_modal.clone().borrow_mut().set(true)})}
                                    role="menuitem" class="wallet-adapter-dropdown-list-item">
                                    {"Change wallet"}
                                </li>
                                <li onclick={
                                    let connection = connection.clone();
                                    Callback::from(move|_| {
                                        let connection = connection.clone();

                                        spawn_local(async move{
                                            connection.as_ref().borrow_mut().disconnect().await.unwrap();
                                        });
                                    })}
                                    role="menuitem" class="wallet-adapter-dropdown-list-item"> {"Disconnect"}
                                </li>
                            </ul>
                        </div>
                    }else {
                        <button
                        onclick={
                            let show_modal = show_modal.clone();
                            Callback::from(move |_| {
                                show_modal.set(true);
                            })
                        }
                        class="wallet-adapter-button wallet-adapter-button-trigger"
                        style="pointer-events: auto;" tabindex="0" type="button">{"Select Wallet"}</button>
                    }
                </div>
            </div>
            if *show_modal {
                {show_modal_component(show_modal, &adapter)}
            }
        </div>
    }
}

pub fn show_modal_component(show_modal: UseStateHandle<bool>, adapter: &YewAdapter) -> Html {
    let show_modal = show_modal.clone();
    let connection = adapter.clone();

    let build_node = |wallet: Wallet| -> Html {
        let icon = wallet.icon().as_ref().unwrap().to_string();
        let alt = wallet.name().to_string() + " icon";

        html! {
            <li>
                <button
                    onclick={
                        let wallet = wallet.clone();
                        let show_modal = show_modal.clone();
                        let connection = connection.clone();

                        Callback::from(move|_|{
                        let wallet = wallet.clone();
                        let show_modal = show_modal.clone();
                        let connection = connection.clone();

                        spawn_local(async move {
                            connection.as_ref().borrow_mut().connect(wallet).await.unwrap();
                            show_modal.set(false)
                        });
                    })}
                    class="wallet-adapter-button" tabindex= "0" type= "button">
                    <i class="wallet-adapter-button-start-icon">
                        <img src={icon} alt={alt} />
                    </i>
                    {wallet.name()}
                    <span> { "Detected" }</span>
                </button>
            </li>
        }
    };

    html! {
        <div aria-labelledby="wallet-adapter-modal-title" aria-modal="true"
            class="wallet-adapter-modal wallet-adapter-modal-fade-in" role="dialog">
            <div class="wallet-adapter-modal-container">
                <div class="wallet-adapter-modal-wrapper">
                    <button
                        onclick={
                            let show_modal = show_modal.clone();
                            Callback::from(move |_|{
                                show_modal.clone().set(false)
                            })}
                        class="wallet-adapter-modal-button-close">
                        <CloseSvg/>
                    </button>
                    <h1 class="wallet-adapter-modal-title"> {"Connect a wallet on Solana to continue"} </h1>
                    <ul class="wallet-adapter-modal-list">
                        { adapter.as_ref().borrow().wallets().iter().map(|wallet|{
                            {build_node(wallet.clone())}
                        }).collect::<Vec<Html>>() }
                    </ul>
                </div>
                <div class="wallet-adapter-modal-overlay"></div>
            </div>
        </div>
    }
}

#[function_component(CloseSvg)]
pub fn close_svg() -> Html {
    html! {
        <svg width="14" height="14">
            <path d="M14 12.461 8.3 6.772l5.234-5.233L12.006 0 6.772 5.234 1.54 0 0 1.539l5.234 5.233L0 12.006l1.539 1.528L6.772 8.3l5.69 5.7L14 12.461z"/>
        </svg>
    }
}
