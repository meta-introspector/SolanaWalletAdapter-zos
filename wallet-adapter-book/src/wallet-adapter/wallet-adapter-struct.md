# `WalletAdapter` struct
Contains types and implements methods to dispatch events, listen for events and perform actions specified by the `wallet-standard`.

#### Structure
```rust,no_run
#[derive(Debug, Clone)]

pub struct WalletAdapter {
    window: Window,
    document: Document,
    storage: WalletStorage,
    connection_info: ConnectionInfoInner,
    wallet_events: WalletEventReceiver,
    wallet_events_sender: WalletEventSender,
    //...
}
```

#### window
Represents an optional [Browser Window Object](https://docs.rs/web-sys/latest/web_sys/struct.Window.html) object.
Handles both Dispatch (`wallet-standard:app-ready`) and listen (`wallet-standard:register-wallet`) events.
If the Browser window is not found the `WalletError::MissingAccessToBrowserWindow` error is returned.

#### document
Represent the [Browser Document Object](https://docs.rs/web-sys/latest/web_sys/struct.Document.html).
If the document object is not found the `WalletError::MissingAccessToBrowserDocument` is returned.

#### storage field
[WalletStorage](https://docs.rs/wallet-adapter/latest/wallet_adapter/struct.WalletStorage.html#) is where the registered wallets are stored. It is an in-memory store of the registered wallets whose key is a hash that ensures no two wallets with the same name can be registered. All wallet names are case-insensitive.
The internal structure is:
```rust,no_run
Rc<RefCell<HashMap<blake3::Hash, Wallet>>>
```
Only the register event adds to the storage. 

 - Methods on storage
```rust.no_run
use wallet_adapter::WalletAdapter;

// Initializing a wallet adapter
let adapter = WalletAdapter::init()?;

// Fetch the account storage
let storage = adapter.storage();

// Cheaply clone the storage since it's an Rc<RefCell<T>>
storage.clone_inner();

// Get all the wallets
storage.get_wallets();

// Get a wallet
storage.get_wallet("pHantom"); // wallet names are case-insensitive so this still works
```

#### connection_info field
This field represents the [ConnectionInfo](https://docs.rs/wallet-adapter/latest/wallet_adapter/struct.ConnectionInfo.html) struct which contains the connected wallet and connected account.

It's internal structure:

```rust,no_run
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionInfo {
    wallet: Option<Wallet>,
    account: Option<WalletAccount>,
    previous_accounts: Vec<WalletAccount>,
}
```

- `wallet` field holds a [connected wallet](https://docs.rs/wallet-adapter/latest/wallet_adapter/struct.Wallet.html). When `WalletAdapter.connect()` method is called and a connection with a browser wallet is established successfully, this field is `Option::Some(Wallet)` else it is `Option::None`
- `account` field holds a [connected account](https://docs.rs/wallet-adapter/latest/wallet_adapter/struct.WalletAccount.html). When `WalletAdapter.connect()` method is called and a connection with a browser wallet is established successfully, this field is `Option::Some(WalletAccount)` else it is `Option::None`
- `previous_accounts` field contains a sequence of connected accounts. Some browser extension wallets emit events that can emit a connected event without disconnecting previous accounts. This account can be used to check if it is part of the sequence in the `previous_accounts` field therefore the `WalletAdapter` can emit the events `Reconnected` and `AccountChanged`.

#### wallet_events field

An [asynchronous listener](https://docs.rs/wallet-adapter/latest/wallet_adapter/type.WalletEventReceiver.html) of type:
```rust
pub type WalletEventReceiver = async_channel::Receiver<wallet_adapter::WalletEvent>;
```

It is an asynchronous type that can be used to listen for [events](https://docs.rs/wallet-adapter/latest/wallet_adapter/enum.WalletEvent.html) emitted by the [WalletAdapter](https://docs.rs/wallet-adapter/latest/wallet_adapter/struct.WalletAdapter.html) and can be accessed using the [WalletAdapter.events()](https://docs.rs/wallet-adapter/latest/wallet_adapter/struct.WalletAdapter.html#method.events) method. 

#### wallet_events_sender

An [async sender](https://docs.rs/wallet-adapter/latest/wallet_adapter/type.WalletEventSender.html) of type:

```rust
pub type WalletEventSender = Sender<WalletEvent>;
```

It is an asynchronous type that is used to send [wallet events](https://docs.rs/wallet-adapter/latest/wallet_adapter/enum.WalletEvent.html) to an [asynchronous listener](https://docs.rs/wallet-adapter/latest/wallet_adapter/type.WalletEventReceiver.html).