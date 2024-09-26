# SolanaWalletAdapter
A lightweight Rust Solana Wallet that can be used in Rust based frontends and WebAssembly.

#### Features
- [x] Regsiter `wallet-standard:register-wallet` custom event
- [x] Wallet parsing - all fields of a wallet are parsed excluding `features`. Work to detect all features is main priority at the moment
- [ ] SignIn
- [ ] Sign Message
- [ ] Sign Transaction
- [ ] Custom Layouts 

#### Building
1. Install `cargo-make` and `miniserve`
    ```sh
    # cargo-make is the build automation tool used by this project
    
    # miniserve is used to serve our files since it supports
    # serving WebAssembly files `application/wasm`.
    # Any file server that supports `appication/wasm` should work

    $ cargo install miniserve cargo-make
    ```
2. Install `wasm-pack` to build Rust project to WebAssembly. Follow the instructions from wasm-pack project [https://rustwasm.github.io/wasm-pack/installer/](https://rustwasm.github.io/wasm-pack/installer/)
3. Make sure you have the `wasm32-unknown-unknown` toolchain as part of your Rust build pipeline. You can check, install or update using 
    ```sh
    $ rustup target add wasm32-unknown-unknown
    ```
4. Switch to the `examples/simple` directory
    ```sh
    $ cd examples/simple
    ```
5. Run cargo make with the `build` argument to automate the build process
    ```sh
    $ cargo make build
    ```
6. Serve files using miniserve 
    ```sh
    $ cargo make serve
    ```
7. Open browser and navigate to `locahost:5500`


#### LICENSE
Apache-2.0 OR MIT

