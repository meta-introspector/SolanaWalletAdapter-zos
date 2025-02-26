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
cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/dioxus-adapter
```

- Yew template

```sh
cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/yew-adapter
```

- Sycamore Template

```sh
cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/sycamore-adapter
```
