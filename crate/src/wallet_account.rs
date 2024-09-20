use crate::{Cluster, PublicKey};

// https://github.com/wallet-standard/wallet-standard/blob/master/packages/core/base/src/wallet.ts
/// A `WalletAccount`, also referred to as an `Account`.
///
/// An account is a _read-only data object_ that is provided from the Wallet to the app,
/// authorizing the app to use it.
///
/// The app can use an account to display and query information from a chain.
///
/// The app can also act using an account by passing it to {@link Wallet.features | features} of the Wallet.
///
/// TODO Check the correctness of statement below in Rust traits
/// Wallets may use or extend ReadonlyWalletAccount which implements this interface.
pub trait WalletAccount {
    /// Address of the account, corresponding with a public key.
    fn address(&self) -> &'static str;

    /// Public key of the account, corresponding with a secret key to use.
    fn public_key(&self) -> PublicKey;

    /// Chains supported by the account.
    ///
    /// This must be a subset of the clusters of the Wallet.
    ///
    fn chains(&self) -> &'static [Cluster];

    /// Feature names supported by the account.
    ///
    /// This must be a subset of the names of {@link Wallet.features | features} of the Wallet.
    ///
    fn features(&self) -> &'static [Features];

    /// Optional user-friendly descriptive label or name for the account.
    /// This may be displayed by the app.
    fn label(&self) -> Option<&'static str>;

    /// Optional user-friendly icon for the account. This may be displayed by the app.
    fn icon(&self) -> Option<WalletIcon>;
}

/// A data URI containing a base64-encoded SVG, WebP, PNG, or GIF image.
pub struct WalletIcon(
    /// Format `data:image/${'svg+xml' | 'webp' | 'png' | 'gif'};base64,${string}`
    &'static str,
);
