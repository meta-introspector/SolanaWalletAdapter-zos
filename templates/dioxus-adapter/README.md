## Dioxus Template with Tailwind CSS


### Overview
This template contains the following features:
- Dashboard with all important links to resources
- Wallet selection
- Viewing and adding clusters
- Handling errors by showing user a 15 seconds(configurable) notification error. 
- Notifications contain a unique random generated key to allow dioxus to perform comparisons for removal after timeout expires
- Viewing account balance, token accounts and transactions
- Mobile friendly header and responsive UI which can be used in in-browser wallets on mobile devices
- Request airdrop (excluding mainnet)
- Send SOL (if account balance is not zero)
- Receive SOL, showing account with `Copy to Clipboard` support and QR Code for easy scanning
- Refresh Accounts button since RPCs mostly don't update balances immediately
- Custom error for unreachable cluster endpoints
- SIWS, Sign Message, Sign Transaction and Sign and Send Transaction (Send SOL component)
  
### Requirements
1. Install tailwind since this template uses tailwind CSS. Currently on tailwind version 3.x.x is supported by Dioxus 0.6.x . This template will be updated when Dioxus version 0.7.x is released which is to supported tailwind 0.4.x .
    ```sh
    npm install -D tailwindcss@3 --save
    ```
2. Install Dioxus CLI for building the Rust code to WASM

### Running the dev server
1. Start the tailwind CLI
    ```sh
    npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
    ```
2. Start the Dioxus CLI
    ```sh
    dx serve
    ```

- Open the browser at default port http://localhost:8080 or the port described by Dioxus CLI in case port `8080` was already in use

- Sometimes there are warning in the browser console, use `dx check` command to find if there are fixes that need to be done.