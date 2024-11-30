use sycamore::prelude::*;
use wasm_bindgen::prelude::*;

use crate::ConnectedArgs;

#[component]
pub fn Header(adapter: Signal<Option<ConnectedArgs>>) -> View {
    let address = adapter
        .get_clone()
        .as_ref()
        .unwrap()
        .connected_account
        .address
        .clone();
    let short_address = adapter
        .get_clone()
        .as_ref()
        .unwrap()
        .connected_account
        .shorten_address()
        .unwrap()
        .to_string();

    view! {
            div(id="header"){
                span {"SYCAMORE DEMO"}
                span(class="menu-item"){
                    a(id="address", href="#",
                        data-wallet-address=address)
                    {
                        (short_address)
                    }
                    ul(class="dropdown"){
                        li(class="dropdown-entry") {
                            a(
                            on:click=move |_| {
                                let address_element = document().get_element_by_id("address").unwrap();
                                let text_to_copy = address_element.get_attribute("data-wallet-address").unwrap();
                                let address_menu = document().get_element_by_id("copy-address").unwrap();
                                address_menu.set_text_content(Some("Copied"));
                                wasm_bindgen_futures::spawn_local(async move{
                                let writer_promise = window().navigator().clipboard().write_text(&text_to_copy);
                                wasm_bindgen_futures::JsFuture::from(writer_promise).await.unwrap();
                                let set_timeout = wasm_bindgen_futures::wasm_bindgen::closure::Closure::wrap(Box::new(move |_: web_sys::Event| {
                                    address_menu.set_text_content(Some("Copy Address"));
                                }) as Box<dyn Fn(_)>);

                                use wasm_bindgen_futures::wasm_bindgen::JsCast;
                                let set_timeout_fn = set_timeout.as_ref().dyn_ref::<wasm_bindgen_futures::js_sys::Function>().unwrap();
                                window()
                                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                                        set_timeout_fn,
                                        1000,
                                    )
                                    .expect("failed to set timeout");
                                    set_timeout.forget();
                                });
                            },
                        id="copy-address",
                        href="#"){
                        "Copy Address"
                    }}
                    // li{class:"dropdown-entry", a {
                    //     onclick: move |_| {
                    //         show_modal.set(true);
                    //     },
                    //     href:"#",
                    //     "Change Wallet"
                    // }}
                    li(class="dropdown-entry") {
                        a(
                            on:click=move |_| {
                                let connected_wallet = adapter.get_clone().as_ref().unwrap().connected_wallet.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                connected_wallet.disconnect().await.unwrap();
                                adapter.set(Option::None);
                            });
                        },
                        href="#",){
                        "Disconnect"
                    }}
                }
            }
        }
    }
}
