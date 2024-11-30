use std::{cell::RefCell, rc::Rc};

use wallet_adapter::{Wallet, WalletAccount, WalletAdapter};
use yew::prelude::*;

use crate::{SignAndSendTxComponent, SignInComponent, SignMessageComponent, SignTxComponent};

#[derive(Properties, PartialEq)]
pub struct YewAdapter {
    controller: UseStateHandle<WalletAdapter>,
    show_modal: UseStateHandle<bool>,
    connected_wallet: UseStateHandle<Rc<RefCell<Option<Wallet>>>>,
    connected_account: UseStateHandle<Option<WalletAccount>>,
}

#[derive(Properties, PartialEq)]
pub struct AdapterActions {
    pub connected_wallet: Wallet,
    pub connected_account: WalletAccount,
}

#[function_component(App)]
pub fn app() -> Html {
    let adapter = WalletAdapter::init().unwrap();

    let adapter = use_state(|| adapter);
    let show_modal = use_state(|| false);
    let show_modal_cloned = show_modal.clone();
    let connected_wallet =
        use_state::<Rc<RefCell<Option<Wallet>>>, _>(|| Rc::new(RefCell::new(Option::None)));
    let connected_account = use_state::<Option<WalletAccount>, _>(|| Option::None);

    html! {
        if connected_account.is_some() {
            <Connected
                controller= {adapter}
                show_modal={show_modal}
                connected_wallet={connected_wallet.clone()}
                connected_account={connected_account.clone()}
            />
            <div id="body-content">
                <SignInComponent
                    connected_wallet= {connected_wallet.clone().as_ref().borrow().clone().unwrap()}
                    connected_account={connected_account.clone().as_ref().unwrap().clone()} />

                <SignMessageComponent
                    connected_wallet= {connected_wallet.clone().as_ref().borrow().clone().unwrap()}
                    connected_account={connected_account.clone().as_ref().unwrap().clone()} />

                <SignTxComponent
                    connected_wallet= {connected_wallet.clone().as_ref().borrow().clone().unwrap()}
                    connected_account={connected_account.clone().as_ref().unwrap().clone()} />

                <SignAndSendTxComponent
                    connected_wallet= {connected_wallet.clone().as_ref().borrow().clone().unwrap()}
                    connected_account={connected_account.clone().as_ref().unwrap().clone()} />
            </div>

        }else {
            <div id="disconnected-content">
                <h1> {"Rust Wallet Adapter Demo"}</h1>
                <div> {"WALLET DISCONNECTED"} </div>
                <button id="btn-primary" onclick={Callback::from(move |_| {

                    show_modal_cloned.set(true);
                })}>{"CONNECT"}</button>

                if *show_modal {
                    <ShowModalComponent
                    controller= {adapter}
                    show_modal={show_modal}
                    connected_wallet={connected_wallet}
                    connected_account={connected_account}
                    />
                }else {
                }
            </div>
        }

    }
}

#[function_component]
pub fn Connected(adapter: &YewAdapter) -> Html {
    html! {
        <Header
            controller= {adapter.controller.clone()}
            show_modal={adapter.show_modal.clone()}
            connected_wallet={adapter.connected_wallet.clone()}
            connected_account={adapter.connected_account.clone()}
        />
    }
}

#[function_component]
pub fn ShowModalComponent(adapter: &YewAdapter) -> Html {
    let modal = adapter.show_modal.clone();

    html! {
        <div id="modal-container">
            <div id="modal-content">
                    { adapter.controller.wallets().into_iter().map(move |wallet| {
                        let wallet_name = wallet.name().to_string();
                        let wallet_icon = wallet.icon().unwrap().to_string();
                        let show_modal_cloned = adapter.show_modal.clone();
                        let return_wallet = wallet.clone();
                        let connected_wallet = adapter.connected_wallet.clone();
                        let connected_account = adapter.connected_account.clone();

                        html!{
                            <div class={classes!("wallet-list-entry")} data-wallet-name={wallet_name.clone()}
                                onclick={
                                    Callback::from(move|_| {
                                        let wallet = wallet.clone();
                                        let return_wallet = return_wallet.clone();
                                        let connected_account_inner = connected_account.clone();


                                        wasm_bindgen_futures::spawn_local(async move {
                                          let connected_to = wallet.connect().await.unwrap();
                                           connected_account_inner.set(Some(connected_to));

                                        });
                                        connected_wallet.set(Rc::new(RefCell::new(Some(return_wallet))));
                                        show_modal_cloned.set(false);
                                })}
                            >
                                <span class={classes!("wallet-icon")}>
                                    <img src={wallet_icon} />
                                </span>
                                <span class={classes!("wallet-name")}>{wallet_name}</span>
                            </div>
                        }
                    }).collect::<Html>() }
                <button id="btn-primary" onclick={Callback::from(move |_| {
                    modal.set(false);
                })}>{"CANCEL"}</button>
            </div>
        </div>
    }
}

#[function_component]
pub fn Header(adapter: &YewAdapter) -> Html {
    let wallet_address = adapter.connected_account.as_ref().unwrap().address.clone();
    let shortened_address = adapter
        .connected_account
        .as_ref()
        .unwrap()
        .shorten_address()
        .unwrap();
    let connected_account = adapter.connected_account.clone();
    let connected_wallet = adapter.connected_wallet.clone();
    html! {
        <div id="header">
                <span>{"YEW DEMO"}</span>
                <span class={classes!("menu-item")}>
                    <a id="address" href="" data-wallet-address={wallet_address}>
                        {shortened_address}
                    </a>
                    <ul class={classes!("dropdown")}>
                        <li class={classes!("dropdown-entry")}>
                            <a onclick={
                                Callback::from(move |_| {
                                    let window = web_sys::window().unwrap();
                                    let document = window.document().unwrap();

                                    let address_element= document.get_element_by_id("address").unwrap();
                                    let text_to_copy = address_element.get_attribute("data-wallet-address").unwrap();
                                    let address_menu = document.get_element_by_id("copy-address").unwrap();
                                    address_menu.set_text_content(Some("Copied"));
                                    wasm_bindgen_futures::spawn_local(async move{
                                        let writer_promise = window.navigator().clipboard().write_text(&text_to_copy);
                                        wasm_bindgen_futures::JsFuture::from(writer_promise).await.unwrap();
                                        let set_timeout = wasm_bindgen_futures::wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                                            address_menu.set_text_content(Some("Copy Address"));
                                        }) as Box<dyn Fn(_)>);

                                        use wasm_bindgen_futures::wasm_bindgen::JsCast;
                                        let set_timeout_fn = set_timeout.as_ref().dyn_ref::<wasm_bindgen_futures::js_sys::Function>().unwrap();
                                        window
                                            .set_timeout_with_callback_and_timeout_and_arguments_0(
                                                set_timeout_fn,
                                                1000,
                                            )
                                            .expect("failed to set timeout");
                                            set_timeout.forget();
                                    });
                                })
                            }
                                id="copy-address"
                                href="#">{"Copy Address"}
                            </a>
                        </li>
                        // <li class={classes!("dropdown-entry")}>
                        //     <a
                        //     // onclick: move |_| {
                        //     //     show_modal.set(true);
                        //     // },
                        //     href="#">{"Change Wallet"} </a>
                        // </li>
                        <li class="dropdown-entry">
                            <a
                            onclick={
                                Callback::from(move|_| {
                                    let connected_wallet_inner = connected_wallet.clone();

                                    wasm_bindgen_futures::spawn_local(async move {
                                        connected_wallet_inner.as_ref().borrow().as_ref().unwrap().disconnect().await.unwrap();
                                    });
                                    connected_wallet.set(Rc::new(RefCell::new(Option::None)));
                                    connected_account.set(Option::None);


                            })}
                            href="#">{"Disconnect"}</a>
                        </li>
                    </ul>
                </span>
        </div>
    }
}
