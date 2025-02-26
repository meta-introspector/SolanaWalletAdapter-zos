A template for Sycamore with TailwindCSS
========================================

Requirements:

 - [Sycamore](https://sycamore.dev/)
 - [Trunk](https://trunkrs.dev/)


### Trunk
Trunk is a build and bundler tool for Rust frontend code. 
***NOTE***: When debugging using code while using Trunk build tool, some wallets don't show up quickly compared to using `wasm-pack` or ` cargo build --target wasm32-unknown-unknown`. This happens on the `register` event, specifically the `wallet-standard:register-wallet` event from wallet-standard. Give the wallets a few seconds to register themselves and they will show up.

### Tailwind CSS
This template uses tailwind CSS to render stylesheets. Trunk already supports this by bundling the tailwind CLI so no need to install the Tailwind CLI or node modules while using Trunk build tool [https://trunkrs.dev/assets/#tailwind](https://trunkrs.dev/assets/#tailwind)

***NOTE*** that Sycamore can be cumbersome to build complex UIs for a beginner, so if you find any bugs while customizing this template, open a pull request.

### Building the template
Switch to your template directory and run
```sh
trunk serve -p 9000 --open
```
The `9000` is the port so you can customize that.
