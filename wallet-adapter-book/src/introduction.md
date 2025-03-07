# Rust Wallet Adapter for Solana Dapps.

<img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/LOGO.svg" alt="Rust Wallet-Adapter Logo" style="zoom:5%;" />

[![crates.io](https://img.shields.io/crates/v/wallet-adapter.svg)](https://crates.io/crates/wallet-adapter)[![Docs](https://docs.rs/wallet-adapter/badge.svg)](https://docs.rs/wallet-adapter)[![Rust](https://github.com/JamiiDao/SolanaWalletAdapter/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/JamiiDao/SolanaWalletAdapter/actions/workflows/rust.yml)![License](https://img.shields.io/crates/l/wallet-adapter)![Passively Maintained](https://img.shields.io/badge/status-passively%20maintained-cyan)

The wallet-adapter library is a Rust crate that performs actions between a Rust WebAssembly frontend and browser wallet extensions that implement the [wallet-standard](https://github.com/wallet-standard/wallet-standard).

### Links

1. [Github](https://github.com/JamiiDao/SolanaWalletAdapter)
2. [Github Templates](https://github.com/JamiiDao/SolanaWalletAdapter/tree/master/templates)
3. [crates.io](https://crates.io/crates/wallet-adapter)
4. [docs.rs](https://docs.rs/wallet-adapter/)

### Features
It implements the following from [wallet-standard](https://github.com/wallet-standard/wallet-standard):
- [x] `standard:connect` - Performs a connection operation to a browser wallet 
- [x] `standard:disconnect` - Performs a connection operation to a browser wallet 
- [x] `standard:events` - Listens for connected and disconnected events emitted from a wallet.
- [x] `solana:signIn` - Performs a Sign In With Solana (SIWS) operations for wallet authentication/authorization.
- [x] `solana:signMessage` - An operation that signs a message
- [x] `solana:signTransaction` - An operation that signs a transaction
- [x] `solana:signAndSendTransaction` - an operation that signs a transaction and sends it to a Solana RPC.

Any wallet that implements the [wallet-standard](https://github.com/wallet-standard/wallet-standard) will be detected by this crate.


### Events
Wallets register themselves by listening for one or both of these browser custom events as defined by the wallet-standard.
- [x] `wallet-standard:register-wallet` - listens for wallets that emit browser window event as a way to register themselves.
- [x] `wallet-standard:app-ready` - the library dispatches this event in order to inform browser extension wallets that the frontend application is ready to receive registration events.