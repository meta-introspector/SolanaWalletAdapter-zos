# SolanaWalletAdapter
A lightweight Rust Solana Wallet that can be used in Rust based frontends and WebAssembly.

### Documentation Links
- [Usage](#usage) - How to add this library and required features for `web-sys` crate
- [Initializing](#initializing-register-and-appready) - How `AppReady` and `Register` wallet events are initialized
- [Wallet Storage](#in-memory-storage-for-registered-wallets) - How the wallets registered are stored in memory within the dapp
- [Connect and Check for Supported Features](#connecting-to-a-browser-extension-wallet-and-checking-for-features) - How to connect to a browser wallet and check which features the connected wallet supports
- [Disconnect](#disconnecting-from-the-wallet) - Disconnected an account from the connected wallet
- [Sign In With Solana](#sign-in-with-solana-siws) - Sign In With Solana (SIWS)
- [Sign Message](#sign-message) - Signing a message with a browser wallet
- [Sign Transaction](#sign-transaction) - Signing a transaction with a browser wallet
- [Sign and Send Transaction](#sign-and-send-transaction) - Sign and Send Transaction with a browser wallet
- [Examples](#examples) - Where to find examples
- [License](#license) - Licensed under Apache-2.0 or MIT
- [Features](#features) - What features of the wallet standard are supported by this library
- [Templates](#templates) - Which Rust frontend framework templates have been implemented
- [Template Examples](#template-examples) - Which Rust frontend framework templates examples have been implemented
- [Build Requirement](#requirements) - What tools are required to build a working project
- [Build and Run A Template](#template-usage) - How to build a template or project to WebAssembly and run in the browser

### Usage
If the project does not have it's own build tool ensure the `Cargo.toml` manifest specified that this is a shared library by adding
```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

Add library to `Cargo.toml` manifest file.
The features for `web-sys` crate below are required for all features to work.
```toml
[dependencies]
wallet-adapter = "<latest version>"
wasm-bindgen = "<latest version>"
web-sys = { version = "<latest version>", features = [
"Window",
"Document",
"Event",
"EventTarget",
"CustomEvent",
"CustomEventInit",
"Element",
"HtmlElement",
"Location",
"Request",
"RequestInit",
"RequestMode",
"Response",
"Headers",
"PointerEvent",
"Clipboard",
"Navigator",
"console",
] }

[profile.release]
# Tell `rustc` to optimize for small code size.
opt-level = "s"
```

See [Template Usage](#template-usage) for more details


### Initializing `Register` and `AppReady`
This is done automatically when calling `WalletAdapter::init()`. The `Register` and `AppReady` events are registered to the browser window and document in the current page allowing browser extension wallet to register themselves as per the wallet standard specification.
```rust
use wallet_adapter::WalletAdapter;

fn foo() -> wallet_adapter::WalletResult<()>{
// Initializing the wallet adapter
let adapter = WalletAdapter::init()?;

// Get wallets registered
adapter.wallets();

// Get a wallet by name
adapter.get_wallet("Phantom");

// Get the storage where the registered wallets are stored
adapter.storage();

// Expose the browser window
adapter.window();
// Expose the browser document
adapter.document();

Ok::<(), wallet_adapter::WalletError>(())
}
```


Alternatively, for some templates where handling events like `onclick` requires `FnMut` closures, `InitEvents` can be used instead.
The `wallet_adapter::InitEvents` requires an in-memory storage solution to store the registered wallets for retrieval. The `wallet_adapter::WalletStorage` can be used instead which wraps a HashMap with a `Rc<RefCell<>>` to allow usage in closures that move variables out of their environment.

```rust
use wallet_adapter::{WalletStorage, InitEvents};

fn foo() -> wallet_adapter::WalletResult<()> {

// Get the Window object where the `AppReady` and `Register` events will be initialized
let window = web_sys::window().expect("No Browser Window Detected");
let mut storage = WalletStorage::default();
// Initialize the events
let init_events = InitEvents::new(&window).init(&mut storage)?;

Ok::<(), wallet_adapter::WalletError>(())
}
```
#### In-memory storage for registered wallets.
`wallet_adapter::WalletStorage` handles storage of registered wallets. The in-memory storage is a `HashMap<hash, Wallet>`
where the `hash` is the hash of the wallet name.
```rust
use wallet_adapter::WalletStorage;

let storage = WalletStorage::default();

// Get all registered wallets
storage.get_wallets();

// Get a wallet by its name
storage.get_wallet("Phantom");

// Clone the storage inside a closure, method or function that moves variables out of their environment
// `WalletStorage` internally representation is `Rc<RefCell<HashMap<hash, Wallet>>>`
// this makes it cheap to clone `WalletStorage` where one needs to access `HashMap<hash, Wallet>`
storage.clone_inner();
```

### Connecting to a browser extension wallet and checking for features
```rust
use wallet_adapter::WalletAdapter;

async fn foo() -> wallet_adapter::WalletResult<()> {

let mut adapter = WalletAdapter::init()?;
adapter.connect("Phantom").await?;

// Is MainNet cluster supported
adapter.mainnet();

// Is DevNet cluster supported
adapter.devnet();

// Is TestNet cluster supported
adapter.testnet();

// Is LocalNet cluster supported
adapter.localnet();

// Is `standard:connect` feature specified in wallet standard supported
adapter.standard_connect();

// Is `standard:disconnect` feature specified in wallet standard supported
adapter.standard_disconnect();

// Is `standard:events` feature specified in wallet standard supported
adapter.standard_events();

// Is `solana:signIn` feature specified in wallet standard supported
adapter.solana_signin();

// Is `solana:signMessage` feature specified in wallet standard supported
adapter.solana_sign_message();

// Is `solana:signTransaction` feature specified in wallet standard supported
adapter.solana_sign_transaction();

// Is `solana:signAndSendTransaction` feature specified in wallet standard supported
adapter.solana_sign_and_send_transaction();

Ok::<(), wallet_adapter::WalletError>(())
}
```

### Disconnecting from the wallet
```rust
use wallet_adapter::{WalletAdapter, SigninInput};

async fn foo() -> wallet_adapter::WalletResult<()> {
let mut adapter = WalletAdapter::init()?;
adapter.connect("Phantom").await?;

// Disconnect from the wallet
adapter.disconnect().await?;

Ok::<(), wallet_adapter::WalletError>(())
}
```

### Sign In With Solana (SIWS)
```rust
use wallet_adapter::{WalletAdapter, SigninInput};

async fn foo() -> wallet_adapter::WalletResult<()> {
let mut adapter = WalletAdapter::init()?;
adapter.connect("Phantom").await?;

// The message to show the user
let statement = "Login To Dev Website";

// Get the public key bytes of the connected account within the connected wallet
let public_key = adapter.connected_account()?.public_key;
// Get the address of the connected account within the connected wallet
let address = adapter.connected_account()?.address.clone();

let mut signin_input = SigninInput::new();
signin_input
    .set_domain(&adapter.window())?
    .set_statement(statement)
    .set_chain_id(wallet_adapter::Cluster::DevNet)
    // NOTE: Some wallets require this field or the wallet adapter
    // will return an error `MessageResponseMismatch` which is as
    // a result of the sent message not corresponding with the signed message
    .set_address(&address)?;

// Get the public key in bytes of the connected 
let signin_output = adapter.sign_in(&signin_input, public_key).await.unwrap();

Ok::<(), wallet_adapter::WalletError>(())
}
```
Sign In With Solana (SIWS) supports more options for the Sign In With Solana Standard. Check the methods on the [SigninInput] struct.
**NOTE** that an error is thrown by the library in case the message signed, public key don't match or if the signature is not valid for the signing public key.

### Sign Message
All messages must be UTF-8 encoded string of bytes
```rust
use wallet_adapter::{WalletAdapter, SigninInput};

async fn foo() -> wallet_adapter::WalletResult<()> {
let mut adapter = WalletAdapter::init()?;
adapter.connect("Phantom").await?;
// Check if the wallet supports signing a message
if adapter.solana_sign_message()? {
    adapter.sign_message(b"SOLANA ROCKS!!!").await?;
}else {
    // Tell user Sign message is not supported
}
Ok::<(), wallet_adapter::WalletError>(())
}
```
**NOTE** that an error is thrown by the library in case the message, public key don't match or if the signature is not valid for the signing public key.
### Sign Transaction
Here, we simulate signing a SOL transfer instruction
```rust
use wallet_adapter::{WalletAdapter, Cluster, Utils,};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_instruction, transaction::Transaction,
};

async fn foo() -> wallet_adapter::WalletResult<()> {
let mut adapter = WalletAdapter::init()?;
adapter.connect("Phantom").await?;

// Construct a transaction in a manner that the browser wallet extension
// can deserialize the transaction from bytes.
// Here we will use `solana-sdk` crate since it can be converted to
// bytes using a crate like `bincode` that understands serializing
// and deserializing the transaction to and from bytes.
//
// Get the public key bytes from the connected account
let public_key = adapter.connected_account()?.public_key;

// Convert the public key bytes of the sender to a `solana_sdk::pubkey::Pubkey`
let pubkey = Pubkey::new_from_array(public_key);

// Convert the public key bytes of the recipient to a `solana_sdk::pubkey::Pubkey`.
// Here we use `wallet_adapter::Utils::public_key_rand()` to generate unique public key bytes
// for testing. Make sure you use a valid public key with corresponding private key
// or your funds will be lost.
let recipient_pubkey = Pubkey::new_from_array(Utils::public_key_rand());

// How many SOL to send.
// The `solana_sdk::native_token::LAMPORTS_PER_SOL` constant contains the number of lamports
// equal to `1 SOL` so calculating `2 SOL` can be achieved using `2 * LAMPORTS_PER_SOL`
let sol = LAMPORTS_PER_SOL;

// Create an instruction to transfer the SOL
let instr = system_instruction::transfer(&pubkey, &recipient_pubkey, sol);
// Create a new unsigned transaction
let tx = Transaction::new_with_payer(&[instr], Some(&pubkey));
// Serialize the transaction into bytes using `bincode`
let tx_bytes = bincode::serialize(&tx).unwrap();

// Specify to use devnet cluster
let cluster = Cluster::DevNet;

// You can check if a wallet is connected first to display
// a certain view to a user or make a user connect first if the account was disconnected
if adapter.is_connected() {
    // Request the browser wallet to sign the transaction.
    let output = adapter.sign_transaction(&tx_bytes, Some(cluster)).await?;

    // Deserialize the signed transaction bytes back into a transaction
    let deser_tx_output = bincode::deserialize::<Transaction>(&output[0]).unwrap();
}

Ok::<(), wallet_adapter::WalletError>(())
}

```
Remember to add the necessary dependencies for this part in the `Cargo.toml` manifest.
```toml
[dependencies]
# Add these
solana-sdk = "2.1.2"
bincode = "1.3.3"
```
**NOTE** that if the signed transaction is verified by the library and an error is thrown in case of signature mismatch.

### Sign And Send Transaction
Here, we simulate signing and sending a SOL transfer instruction
```rust
use std::str::FromStr;

use wallet_adapter::{WalletAdapter, Cluster, Utils, SendOptions};
use serde::Deserialize;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, system_instruction, transaction::Transaction,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{wasm_bindgen::JsCast, Headers, Request, RequestInit, Response};

async fn foo() -> wallet_adapter::WalletResult<()> {
let mut adapter = WalletAdapter::init()?;
adapter.connect("Phantom").await?;

// The variables for the code is the same as the one for Sign Transaction

let public_key = adapter.connected_account()?.public_key;
let pubkey = Pubkey::new_from_array(public_key);
let recipient_pubkey = Pubkey::new_from_array(Utils::public_key_rand());
let sol = LAMPORTS_PER_SOL;
let instr = system_instruction::transfer(&pubkey, &recipient_pubkey, sol);

// This part is different from Sign Transaction above since we need a valid recent blockhash
// as part of the `SendAndSignTransaction` specification.


// First let's construct structs from serde that will allow us to deserialize the 
// response of the recent blockhash from a Solana cluster

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockHashResponse<'a> {
    #[serde(borrow)]
    pub jsonrpc: &'a str,
    pub id: u8,
    pub result: ResponseResult<'a>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseResult<'a> {
    #[serde(borrow)]
    pub context: Context<'a>,
    #[serde(borrow)]
    pub value: ResponseValue<'a>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context<'a> {
    #[serde(borrow)]
    pub api_version: &'a str,
    pub slot: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseValue<'a> {
    #[serde(borrow)]
    pub blockhash: &'a str,
    pub last_valid_block_height: u64,
}

// Create code to use browser fetch API to request a recent blockhash from the
// Solana cluster, in our case Devnet cluster.
// NOTE: You can use other crates like `reqwest` and `gloo-net` do this. However,
// this example will use browser fetch API to give an example of how you would use
// Fetch API or when you don't want to add external dependencies to do this
async fn get_blockhash() -> solana_sdk::hash::Hash {
    let devnet_uri = Cluster::DevNet.endpoint();
    let body = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method":"getLatestBlockhash",
        "params":[

        ]
    };

    // NOTE: You can use Reqwest crate instead to fetch the blockhash but
    // this code shows how to use the browser `fetch` api

    let headers = Headers::new().unwrap();
    headers.append("content-type", "application/json").unwrap();
    headers.append("Accept", "application/json").unwrap();

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_headers(&headers);
    opts.set_body(&body.to_string().as_str().into());

    let request = Request::new_with_str_and_init(&devnet_uri, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let fetch_promise = window.fetch_with_request(&request);

    // Await the fetch promise to get a `Response` object
    let resp_value = JsFuture::from(fetch_promise).await.unwrap();
    let resp = resp_value.dyn_into::<Response>().unwrap();

    let body_as_str = JsFuture::from(resp.text().unwrap())
        .await
        .unwrap()
        .as_string()
        .unwrap();

    let deser = serde_json::from_str::<GetBlockHashResponse>(&body_as_str).unwrap();

    solana_sdk::hash::Hash::from_str(deser.result.value.blockhash).unwrap()
}

// Create a new mutable unsigned transaction
let mut tx = Transaction::new_with_payer(&[instr], Some(&pubkey));

// You can check if a wallet is connected first to display
// a certain view to a user or make a user connect first if the account was disconnected
if adapter.is_connected() {
    // Get the blockhash
    let blockhash = get_blockhash().await;
    // Add the blockhash to the transaction
    tx.message.recent_blockhash = blockhash;
    // Serialize the transaction into bytes
    let tx_bytes = bincode::serialize(&tx).unwrap();

    // Specify which options to pass to the browser wallet.
    // Here we use default options
    let send_options = SendOptions::default();

    // Request the wallet to sign and send the transaction, returning the signature
    let signature = adapter.sign_and_send_transaction(&tx_bytes, Cluster::DevNet, send_options).await?;
    let signature_with_link = String::from("https://explorer.solana.com/tx/") + &Utils::base58_signature(signature).as_str() + "?cluster=devnet";
}
Ok::<(), wallet_adapter::WalletError>(())
}

```
Remember to add the necessary dependencies for this part in the `Cargo.toml` manifest.
```toml
[dependencies]
# Add these
solana-sdk = "2.1.2"
bincode = "1.3.3"
jzon = "0.12.5"
serde_json = "1.0.133"
serde = { version = "1.0.215", features = ["derive"] }
```
**NOTE** that if the signed transaction is verified by the library and an error is thrown in case of signature mismatch.


### Examples
Examples can be found at [examples directory](https://github.com/JamiiDao/SolanaWalletAdapter/tree/master/examples)

### LICENSE
Apache-2.0 OR MIT

### Features
- [x] Register `wallet-standard:register-wallet` custom event
- [x] App Ready `wallet-standard:app-ready` custom event
- [x] Wallet Info
- [x] Wallet Account parsing
- [x] Wallet Icon
- [x] Chains
- [x] Clusters
- [x] Version (Semver Versionin)
- [x] Features
- [x] Connect Wallet `standard:connect`
- [x] Disconnect Wallet `standard:disconnect`
- [x] SignIn (Sign In With Solana SIWS)
- [x] Sign Message
- [x] Sign Transaction
- [x] Sign and Send Transaction

### Templates
- [ ] WebAssembly (No Frontend Framework) 
- [x] Sycamore
- [x] Yew
- [x] Dioxus 

### Template Examples
- [ ] WebAssembly (No Frontend Framework) 
- [x] Sycamore
- [x] Yew
- [x] Dioxus



### Requirements
1. Install `wasm32-unknown-unknown` toolchain to compile against for the browser
    ```sh
    rustup target add wasm32-unknown-unknown
    ```
2. Install `wasm-pack` to build Rust projects for Rust WebAssembly with no framework, Yew and Sycamore templates and examples. 
   Dioxus template and examples require using the Dioxus cli which you can install from [Dioxus Website](https://dioxuslabs.com/).
   Follow the instructions from wasm-pack project [https://rustwasm.github.io/wasm-pack/installer/](https://rustwasm.github.io/wasm-pack/installer/)


### Template Usage
To add the library to any project Run
```sh
cargo add wallet-adapter
```

To get starter code for various templates, first install `cargo generate`
```sh
cargo install cargo-generate
```

Then run `cargo generate --name temp-wasm https://github.com/JamiiDao/Solana-Rust-Wallet-Adapter-Templates <template subfolder name>`
where `<template subfolder name>` is the name of the directory containing the template. Examples:
- Dioxus template
```sh
cargo generate --name <project name> https://github.com/JamiiDao/Solana-Rust-Wallet-Adapter-Templates dioxus
```
- Yew template
```sh
cargo generate --name <project name> https://github.com/JamiiDao/Solana-Rust-Wallet-Adapter-Templates yew
```
- Sycamore Template
```sh
cargo generate --name <project name> https://github.com/JamiiDao/Solana-Rust-Wallet-Adapter-Templates sycamore
```
All templates can be found at [Solana-Rust-Wallet-Adapter-Templates(https://github.com/JamiiDao/Solana-Rust-Wallet-Adapter-Templates)

##### Running Dioxus examples and templates using Dioxus cli.
- Install dioxus cli from [https://dioxuslabs.com/](https://dioxuslabs.com/).

- Build, run and serve a dioxus project
  ```sh
    dx serve --hot-reload
   ```


Templates like Yew and Sycamore don't come with there own build tool like Dioxus. For such templates install a server like `miniserve`
that supports sending the WebAssembly files to the browser with the MIME `application/wasm` .
If you decided to use miniserve install with command
```sh
cargo install miniserve
```
Then compile the Rust code to wasm format using
```sh
wasm-pack build --dev --target web --out-name wasm --out-dir ./resources/pkg
```
This outputs all the wasm in the ./resources/pkg

#### Serve files
Install `miniserve` crate or any other http serve that supports serving wasm files as `application/wasm` MIME

```sh
miniserve -p 5500 ./resources --index index.html
```

Now the files are available at port [localhost:5500](http://localhost:5500).

The `resources/index.html` can be used as a reference on how the wasm files are imported an initialized.
