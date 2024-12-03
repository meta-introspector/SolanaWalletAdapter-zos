use wasm_bindgen::prelude::*;

mod app;
pub(crate) use app::*;

mod signin;
pub(crate) use signin::*;

mod sign_message;
pub(crate) use sign_message::*;

mod sign_tx;
pub(crate) use sign_tx::*;

mod sign_and_send_tx;
pub(crate) use sign_and_send_tx::*;

#[wasm_bindgen(start)]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
