#![allow(non_snake_case)]

mod components;
pub(crate) use components::*;

mod views;
pub(crate) use views::*;

mod types;

mod utils;
pub(crate) use utils::*;

mod svg_assets;
pub(crate) use svg_assets::*;

mod app;

pub(crate) const IDL_RAW_DATA: &str = partial_idl_parser::get_idl!();

fn main() {
    yew::Renderer::<app::App>::new().render();
}
