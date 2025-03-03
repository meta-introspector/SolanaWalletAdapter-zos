#![allow(non_snake_case)]

mod views;
use views::*;

mod header;
use header::*;

mod types;
use types::*;

mod utils;
use utils::*;

mod fetch_parser;
use fetch_parser::*;

mod svg_assets;
pub(crate) use svg_assets::*;

mod fetch_util;
pub(crate) use fetch_util::*;

mod app;

pub(crate) const IDL_RAW_DATA: &str = partial_idl_parser::get_idl!();

fn main() {
    app::run()
}
