## Rust Wallet Adapter Templates

### Supported templates

- [x] Dioxus
- [x] Yew
- [x] Sycamore

### Maintenance 

New releases will be for bug fixes or to support a new release based on a new version of the supported frameworks.

### Template Features

1. Connect wallet first for routes that require a connected wallet

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/accounts-disconnected.png" alt="Connect Wallet First" width="80%">
2. Connect a wallet

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/connect-modal.png" alt="Connect Modal" width="80%">
3. Connection Loading

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/connect-loading.png" alt="Connect Wallet First" width="80%">
4. Notifications

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/notifications.png" alt="Connect Wallet First" width="50%">
5. Connected Wallet Options

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/dropdown-options.png" alt="Connect Wallet First" width="50%">
6. Homepage

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/Dashboard.png" alt="Connect Wallet First" width="80%">
7. Clusters View

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/clusters-view.png" alt="Connect Wallet First" width="80%">
8. Add clusters view

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/add-cluster-view.png" alt="Connect Wallet First" width="80%">
9. Account Info Loading

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/account-loading.png" alt="Connect Wallet First" width="80%">
10. Account Info Loaded (view balance, token accounts and transactions)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/accounts-loaded.png" alt="Connect Wallet First" width="80%">
11. Send SOL (sign and send tx example)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/send.png" alt="Connect Wallet First" width="80%">
12. Receive SOL (with copy address to clipboard support and displaying QR code for receive address)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/receive.png" alt="Connect Wallet First" width="80%">
13. Request Airdrop (not available on Mainnet cluster)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/airdrop.png" alt="Connect Wallet First" width="80%">
14. Failed transaction 

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/failed-tx.png" alt="Connect Wallet First" width="80%">
15. Refresh button to refresh account info after transactions or in case template like Yew returns stale state due to Yew bug
16. Extras route (SIWS, Sign Message, Sign Transaction)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/extras.png" alt="Connect Wallet First" width="80%">

17. Unsupported extra features for example wallets that don't support SIWS
    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/siws-unsupported.png" alt="Connect Wallet First" width="50%">

### Choosing a template

- Dioxus is the recommended template. Dioxus web framework `(>=0.6.0)` enables the best developer experience of the templates due to easy state management (like global hooks) and robust tools (like hot-reloading and bundling) implemented by the Dioxus team. The only issue with Dioxus in this implementation is that it only supports tailwind `<=0.3`. Support for tailwind `v4` will be released with upcoming dioxus `0.7`. The dioxus template will be updated to support tailwind v4 when dioxus 0.7 is released.

- Sycamore also has good state management but lacks hot reloading.

- ***Yew is not recommended,*** it has a lot of issues that haven't been address for a while. Issues like:

  - Fills temporary directory of the operating system quickly causing Trunk build tool to panic with error `no space left on the device`

  - Stale state management, for example the `accounts` route doesn't refresh unless the `Refresh` button is tapped on. On other routes, state management must be triggered manually by passing around a boolean value that is then set to true to cause the page to reload. Using `use_effect` hooks on a component routinely causes infinite renders. See issue [#3796](https://github.com/yewstack/yew/issues/3796)

  - Some events don't fire on input unless workaround events like `onchange` instead of `oninput` are used. See a similar issue [#3792](https://github.com/yewstack/yew/issues/3792)

  - Difficult to use `Option<T>` type for optional types. See issue [#3747](https://github.com/yewstack/yew/issues/3747)

  - Firefox doesn't update the clusters in the `select` element in the template header if the element changes from within another component in the page. See issue [#3745]

    ***Until these issues are fixed, using Yew is not recommended.***

### To generate the starter code for various templates, first install `cargo generate`

```sh
cargo install cargo-generate

# or if you have `cargo-binstall` already installed, then run:
cargo binstall cargo-generate
```

[Cargo generate](https://crates.io/crates/cargo-generate) is a common rust tool used to generate templates.

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

##### NOTE: [trunk](https://dioxuslabs.com/learn/0.6/getting_started/) build tool is a tool used to build and bundle Rust code into web-assembly. It has support for tailwind `v4` without the need to install any node modules or the tailwind cli. It achieves this by bundling all the tools required to build and bundle Rust code and tailwind styles to web-assembly.

### Running the templates

- Dioxus
  Install dioxus tools from [https://dioxuslabs.com/learn/0.6/getting_started/](https://dioxuslabs.com/learn/0.6/getting_started/)
  To make modifications to tailwind css follow instructions from [Using tailwind with Dioxus](https://dioxuslabs.com/learn/0.6/cookbook/tailwind/)
  Then run:

  ```sh
   npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
  ```

  To run Dioxus cli
  ```sh
  dx serve #Automatically run with hot-reload
  ```

  Lastly, when build is complete, open the URL shown by the output of `dx serve` above (mostly defaults to http://localhost:8080)

- Sycamore & Yew
  Install Trunk build tool from [https://trunkrs.dev/](https://trunkrs.dev/) 

  Navigate to the template root.
  Trunk automatically compiles modifications to tailwind css.

  To run Trunk cli

  ```sh
  trunk serve -p 9000 # You can replace `9000` with port of your choice
  ```

  Lastly, when build is complete, open the URL shown by the output of `dx serve` above (mostly defaults to http://localhost:8080)
