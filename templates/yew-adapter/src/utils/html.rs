use qrcodegen::{QrCode, QrCodeEcc};
use wallet_adapter::Cluster;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::{DevnetSvg, LocalnetSvg, MainnetSvg, TestnetSvg};

pub fn get_select_value(event: web_sys::Event) -> String {
    let target = event.target().unwrap();
    target.dyn_into::<HtmlSelectElement>().unwrap().value()
}

pub fn get_input_value(event: web_sys::Event) -> String {
    let target = event.target().unwrap();
    target.dyn_into::<HtmlInputElement>().unwrap().value()
}

pub fn get_cluster_svg(cluster: Cluster) -> Html {
    if cluster == Cluster::MainNet {
        html! {<MainnetSvg/> }
    } else if cluster == Cluster::TestNet {
        html! {<TestnetSvg/> }
    } else if cluster == Cluster::DevNet {
        html! {<DevnetSvg/> }
    } else if cluster == Cluster::LocalNet {
        html! {<LocalnetSvg/> }
    } else {
        html! {<DevnetSvg/> }
    }
}

// Creates a single QR Code, then prints it to the console.
pub fn address_qrcode(address: &str) -> String {
    let errcorlvl: QrCodeEcc = QrCodeEcc::High; // Error correction level

    // Make and print the QR Code symbol
    let qr: QrCode = QrCode::encode_text(address, errcorlvl).unwrap();
    qr_to_svg(&qr, 4)
}

fn qr_to_svg(qr: &QrCode, border: i32) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result = String::new();
    result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let dimension = qr
        .size()
        .checked_add(border.checked_mul(2).unwrap())
        .unwrap();
    result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";

    String::from("data:image/svg+xml;base64,")
        + data_encoding::BASE64.encode(result.as_bytes()).as_str()
}
