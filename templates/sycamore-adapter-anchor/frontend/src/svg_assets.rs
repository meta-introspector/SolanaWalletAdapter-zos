use sycamore::prelude::*;

pub fn Loader() -> View {
    view! {
        svg ("aria-hidden"= "true",
            class= "inline mr-3 w-4 h-4 dark:text-true-blue text-blue-900 animate-spin",
            fill= "none",
            "role"= "status",
            "viewBox"= "0 0 100 101",
            xmlns= "http://www.w3.org/2000/svg"){
            path (
                d= "M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z",
                fill= "#E5E7EB"
            )
            path (
                d= "M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z",
                fill= "currentColor",
            )
        }
    }
}

pub fn asset_base64_svg(svg_contents: &'static str) -> String {
    String::from("data:image/svg+xml;base64,")
        + data_encoding::BASE64
            .encode(svg_contents.as_bytes())
            .as_str()
}

pub fn airdrop_svg() -> String {
    asset_base64_svg(AIRDROP_SVG)
}

pub fn avatar_svg() -> String {
    asset_base64_svg(AVATAR_SVG)
}

pub fn balance_svg() -> String {
    asset_base64_svg(BALANCE_SVG)
}

pub fn change_wallet_svg() -> String {
    asset_base64_svg(CHANGE_WALLET_SVG)
}

pub fn check_svg() -> String {
    asset_base64_svg(CHECK_SVG)
}

pub fn close_svg() -> String {
    asset_base64_svg(CLOSE_SVG)
}

pub fn clusters_svg() -> String {
    asset_base64_svg(CLUSTERS_SVG)
}

pub fn copy_svg() -> String {
    asset_base64_svg(COPY_SVG)
}

pub fn devnet_svg() -> String {
    asset_base64_svg(DEVNET_SVG)
}

pub fn disconnect_svg() -> String {
    asset_base64_svg(DISCONNECT_SVG)
}

pub fn error_svg() -> String {
    asset_base64_svg(ERROR_SVG)
}

pub fn gradient_wallet_svg() -> String {
    asset_base64_svg(GRADIENT_WALLET_SVG)
}

pub fn localnet_svg() -> String {
    asset_base64_svg(LOCALNET_SVG)
}

pub fn mainnet_svg() -> String {
    asset_base64_svg(MAINNET_SVG)
}

pub fn notification_svg() -> String {
    asset_base64_svg(NOTIFICATION_SVG)
}

pub fn receive_svg() -> String {
    asset_base64_svg(RECEIVE_SVG)
}

pub fn send_svg() -> String {
    asset_base64_svg(SEND_SVG)
}

pub fn signature_svg() -> String {
    asset_base64_svg(SIGNATURE_SVG)
}

pub fn sign_message_svg() -> String {
    asset_base64_svg(SIGN_MESSAGE_SVG)
}

pub fn sign_tx_svg() -> String {
    asset_base64_svg(SIGN_TX_SVG)
}

pub fn siws_svg() -> String {
    asset_base64_svg(SIWS_SVG)
}

pub fn testnet_svg() -> String {
    asset_base64_svg(TESTNET_SVG)
}

pub fn timestamp_svg() -> String {
    asset_base64_svg(TIMESTAMP_SVG)
}

pub fn token_ata_svg() -> String {
    asset_base64_svg(TOKEN_ATA_SVG)
}

pub fn token_mint_svg() -> String {
    asset_base64_svg(TOKEN_MINT_SVG)
}

pub fn trash_svg() -> String {
    asset_base64_svg(TRASH_SVG)
}

pub fn wallet_svg() -> String {
    asset_base64_svg(WALLET_SVG)
}

pub fn link_svg() -> String {
    asset_base64_svg(LINK_SVG)
}

pub fn id_svg() -> String {
    asset_base64_svg(ID_SVG)
}

const LINK_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/link.svg"
));

const ID_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/id.svg"
));

const AIRDROP_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/airdrop.svg"
));

const AVATAR_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/avatar.svg"
));

const BALANCE_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/balance.svg"
));

const CHANGE_WALLET_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/change-wallet.svg"
));

const CHECK_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/check.svg"
));

const CLOSE_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/close.svg"
));

const CLUSTERS_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/clusters.svg"
));

const COPY_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/copy.svg"
));

const DEVNET_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/devnet.svg"
));

const DISCONNECT_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/disconnect.svg"
));

const ERROR_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/error.svg"
));

const GRADIENT_WALLET_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/Gradient-Wallet-Icon.svg"
));

const LOCALNET_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/localnet.svg"
));

const MAINNET_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/mainnet.svg"
));

const NOTIFICATION_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/notification-bell.svg"
));

const RECEIVE_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/receive.svg"
));

const SEND_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/send.svg"
));

const SIGNATURE_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/signature.svg"
));

const SIGN_MESSAGE_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/sign-message.svg"
));

const SIGN_TX_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/sign-tx.svg"
));

const SIWS_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/siws.svg"
));

const TESTNET_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/testnet.svg"
));

const TIMESTAMP_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/timestamp.svg"
));

const TOKEN_ATA_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/token-ata.svg"
));

const TOKEN_MINT_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/token-mint.svg"
));

const TRASH_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/trash.svg"
));

const WALLET_SVG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/svg_assets/wallet.svg"
));
