#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

use wallet_adapter::{Cluster, WalletAdapter};

mod views;
use views::*;

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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const NORMALIZE_CSS: Asset = asset!("/assets/normalize.css");
const SOLANA_CSS: Asset = asset!("/assets/solana.css");
const MAIN_CSS: Asset = asset!("/assets/main.css");
pub(crate) const LOGO: Asset = asset!("/assets/logo.png");

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct AdapterCluster {
    name: String,
    cluster: Cluster,
    endpoint: String,
    identifier: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct DioxusWalletAdapter {
    connection: WalletAdapter,
    clusters: Vec<AdapterCluster>,
    active_cluster: AdapterCluster,
    show_modal: bool,
}

#[component]
fn App() -> Element {
    let adapter = WalletAdapter::init().unwrap();

    let active_cluster = AdapterCluster {
        name: "devnet".to_string(),
        cluster: Cluster::DevNet,
        endpoint: Cluster::DevNet.endpoint().to_string(),
        identifier: Cluster::DevNet.display().to_string(),
    };

    let clusters = vec![
        active_cluster.clone(),
        AdapterCluster {
            name: "mainnet".to_string(),
            cluster: Cluster::MainNet,
            endpoint: Cluster::MainNet.endpoint().to_string(),
            identifier: Cluster::MainNet.display().to_string(),
        },
        AdapterCluster {
            name: "testnet".to_string(),
            cluster: Cluster::TestNet,
            endpoint: Cluster::TestNet.endpoint().to_string(),
            identifier: Cluster::TestNet.display().to_string(),
        },
        AdapterCluster {
            name: "localhost".to_string(),
            cluster: Cluster::LocalNet,
            endpoint: Cluster::LocalNet.endpoint().to_string(),
            identifier: Cluster::LocalNet.display().to_string(),
        },
    ];

    use_context_provider(|| {
        Signal::new(DioxusWalletAdapter {
            connection: adapter,
            active_cluster,
            clusters,
            show_modal: false,
        })
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: NORMALIZE_CSS }
        document::Link { rel: "stylesheet", href: SOLANA_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div {id:"root",
            div {class:"h-full flex flex-col",
                Router::<Route> {}
            }
        }
    }
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Header)]
        #[route("/")]
        WalletDashboard {},
        // Add additional routes here
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
