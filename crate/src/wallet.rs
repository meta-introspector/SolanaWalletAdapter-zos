use crate::{Cluster, WalletIcon, WALLET_STANDARD_VERSION};

/// Interface of a **Wallet**, also referred to as a **Standard Wallet**.
///
/// A Standard Wallet implements and adheres to the Wallet Standard.
pub trait Wallet {
    /// [WALLET_STANDARD_VERSION] of the Wallet Standard implemented by the Wallet.
    /// It is be read-only, static, and canonically defined by the Wallet Standard.
    fn version() -> &'static str {
        WALLET_STANDARD_VERSION
    }

    /// Name of the Wallet. This may be displayed by the app.
    ///
    /// Must be read-only, static, descriptive, unique,
    /// and canonically defined by the wallet extension or application.
    fn name() -> &'static str;

    ///  Icon of the Wallet displayed by the app.
    ///
    /// Must be read-only, static, and canonically defined by the wallet extension or application.
    fn icon() -> WalletIcon;

    ///
    /// Chains supported by the Wallet.
    ///
    /// A **chain** is an string idnetifier which identifies a blockchain in a canonical,
    /// human-readable format.
    /// [CAIP-2](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-2.md)
    /// chain IDs are compatible with this, but are not required to be used.
    ///
    /// Each blockchain should define its own **chains** by extension of the Wallet Standard,
    /// using its own namespace.
    /// The `standard` and `experimental` namespaces are reserved by the Wallet Standard.
    ///
    /// The event features should be used to notify the app if the value changes.
    fn chains() -> [Cluster];

    /// Features supported by the Wallet.
    ///
    /// A **feature name** is an identifier which identifies a **feature** in a canonical,
    /// human-readable format.
    ///
    /// Each blockchain should define its own features by extension of the Wallet Standard.
    ///
    /// The `standard` and `experimental` namespaces are reserved by the Wallet Standard.
    ///
    /// A **feature** may have any type. It may be a single method or value, or a collection of them.
    ///
    /// A **conventional feature** implements the [Feature] trait.
    /// Example
    ///
    /// ```rust
    /// pub struct Foo {
    ///     ciphers: String,
    ///     encrypt: fn(&[u8]) -> &'static dyn Future<Output = String>,
    /// }
    ///
    /// impl Feature for Foo {
    ///     fn name() -> &'static str {
    ///         "Foo"
    ///     }
    ///
    ///     fn version() -> &'static str {
    ///         "1.0.0"
    ///     }
    /// }
    ///
    /// The `Features` event should be used to notify the app if the value changes.
    ///
    fn features<T>() -> [&'static dyn Feature<T>];

    /// Accounts the app is authorized to use of type [WalletAccount]
    ///
    /// This can be set by the Wallet so the app can use authorized accounts on the initial page load.
    ///
    /// The `ConnectFeature` | `standard:connect` feature should be used to obtain
    /// authorization to the accounts.
    ///
    /// The Feature.events() should be used to notify the app if the value changes.
    fn accounts() -> Vec<WalletAccount>;
}

pub trait Feature<T> {
    fn name(&self) -> &'static str;

    fn version(&self) -> &'static str;
}
