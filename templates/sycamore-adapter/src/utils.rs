use qrcodegen::{QrCode, QrCodeEcc};
use sycamore::prelude::*;
use wallet_adapter::{
    wasm_bindgen_futures::JsFuture, web_sys::wasm_bindgen::JsCast, Cluster, WalletResult,
};
use web_sys::{HtmlInputElement, HtmlSelectElement};

use crate::{devnet_svg, localnet_svg, mainnet_svg, testnet_svg, AdapterCluster};

pub fn trunk_cluster_name(name: &str) -> String {
    if name.len() > 10 {
        name.chars().take(10).collect::<String>() + "..."
    } else {
        name.to_string()
    }
}

pub fn get_cluster_svg(cluster: Cluster) -> String {
    if cluster == Cluster::MainNet {
        mainnet_svg()
    } else if cluster == Cluster::TestNet {
        testnet_svg()
    } else if cluster == Cluster::DevNet {
        devnet_svg()
    } else if cluster == Cluster::LocalNet {
        localnet_svg()
    } else {
        devnet_svg()
    }
}

const EXPLORER: &str = "https://explorer.solana.com/";

pub fn format_address_url(address: &str, active_cluster: &AdapterCluster) -> String {
    String::new() + EXPLORER + "address/" + address + &adapter_query_string(active_cluster)
}

pub fn format_tx_url(tx: &str, active_cluster: &AdapterCluster) -> String {
    String::new() + EXPLORER + "tx/" + tx + &adapter_query_string(active_cluster)
}

pub fn adapter_query_string(active_cluster: &AdapterCluster) -> String {
    active_cluster.query_string().to_owned()
}

pub(crate) fn link_target_blank(href: String, text: String) -> View {
    view! {a (class="underline", href=href, target="_blank", rel="noopener noreferrer"){ (text)"⇗"}}
}

pub async fn copied_address(address: &str) -> WalletResult<()> {
    let pending: JsFuture = window().navigator().clipboard().write_text(address).into();

    pending.await?;

    Ok(())
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

pub fn get_select_value(event: web_sys::Event) -> String {
    let target = event.target().unwrap();
    target.dyn_into::<HtmlSelectElement>().unwrap().value()
}

pub fn get_input_value(event: web_sys::Event) -> String {
    let target = event.target().unwrap();
    target.dyn_into::<HtmlInputElement>().unwrap().value()
}
