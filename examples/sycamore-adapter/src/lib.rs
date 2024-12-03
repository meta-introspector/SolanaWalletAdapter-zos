use sycamore::prelude::*;
use wallet_adapter::{InitEvents, Wallet, WalletAccount, WalletStorage};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, MouseEvent};

mod header;
use header::*;

mod signin;
use signin::*;

mod sign_message;
use sign_message::*;

mod sign_tx;
use sign_tx::*;

mod sign_and_send_tx;
use sign_and_send_tx::*;

#[wasm_bindgen(start)]
pub fn main() {
    sycamore::render(App);
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct SycamoreWalletAdapter {
    storage: Signal<WalletStorage>,
    connected: Signal<Option<ConnectedArgs>>,
}

impl SycamoreWalletAdapter {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ConnectedArgs {
    pub(crate) connected_account: WalletAccount,
    pub(crate) connected_wallet: Wallet,
}

#[derive(Debug, Props)]
pub struct Controller<'a> {
    pub(crate) connected_account: &'a WalletAccount,
    pub(crate) connected_wallet: &'a Wallet,
}

#[component]
fn App() -> View {
    let adapter = SycamoreWalletAdapter::new();

    RootPage(adapter)
}

#[component]
fn RootPage(adapter: SycamoreWalletAdapter) -> View {
    let mut storage = WalletStorage::default();

    InitEvents::new(&window()).init(&mut storage).unwrap();
    adapter.storage.set(storage);

    let show_modal = create_signal(false);

    view! {
        (if adapter.connected.get_clone().is_some(){
            view!{
                (Header(adapter.connected))
                div(id="body-content"){
                    SignIn(connected_wallet= &adapter.connected.get_clone().as_ref().unwrap().connected_wallet,
                     connected_account =&adapter.connected.get_clone().as_ref().unwrap().connected_account,
                    )
                    SignMessage(connected_wallet= &adapter.connected.get_clone().as_ref().unwrap().connected_wallet,
                        connected_account =&adapter.connected.get_clone().as_ref().unwrap().connected_account,
                    )
                    SignTx(connected_wallet= &adapter.connected.get_clone().as_ref().unwrap().connected_wallet,
                        connected_account =&adapter.connected.get_clone().as_ref().unwrap().connected_account,
                    )
                    SignAndSendTx(connected_wallet= &adapter.connected.get_clone().as_ref().unwrap().connected_wallet,
                        connected_account =&adapter.connected.get_clone().as_ref().unwrap().connected_account,
                    )
                }
            }
        }else {

            view!{
                div(id="disconnected-content") {
                    h1 {"Rust Wallet Adapter Demo"}
                    div {"WALLET DISCONNECTED"}

                    div {
                        button(id="btn-primary", on:click=move |_| {
                            show_modal.set(true)
                        }) { "CONNECT" }
                    }
                }

                (if show_modal.get() {
                    let wallets = create_signal(adapter.storage.get_clone().get_wallets());
                    let list_entries = wallets.get_clone().iter().map(|wallet| {
                        let wallet_icon = wallet.icon().as_ref().unwrap().to_string();
                        let wallet_name = wallet.name().to_string();
                        let data_id = wallet_name.clone();

                        view!{
                            div(class="wallet-list-entry", data-wallet-name=data_id, on:click=move|event: MouseEvent|{
                                let target = event.current_target().unwrap();
                                let target_as_element = target.dyn_ref::<HtmlElement>().unwrap();
                                let attribute_value = target_as_element.get_attribute("data-wallet-name").unwrap();
                                wasm_bindgen_futures::spawn_local(async move {

                                    let target_wallet = adapter.storage.get_clone().get_wallet(&attribute_value).unwrap();
                                    let account_connected = target_wallet.connect().await.unwrap();
                                    adapter.connected.set(
                                        Some(ConnectedArgs {
                                            connected_account: account_connected,
                                            connected_wallet: target_wallet
                                        })
                                    );
                                    show_modal.set(false);
                                });
                            }) {
                                span(class="wallet-icon"){img(src=wallet_icon)}
                                span(class="wallet-name"){ (wallet_name)}
                            }
                        }
                    }).collect::<Vec<View>>();

                   view!{
                        div (id="modal-container"){
                            div(id="modal-content") {
                                (list_entries)
                                button(class="btn-primary", on:click=move |_| {
                                    show_modal.set(false)
                                }) { "CANCEL" }
                            }
                        }
                    }
                }else { view!() })
            }
        })
    }
}
