/// Name of the feature.
pub const SOLANA_SIGN_IN: &str = "solana:signIn";

pub struct SolanaSignInOutput<'sw> {
    account: WalletAccount,
    signed_message: &'sw [u8],
    signature: [u8; 64],
    signature_type: SignatureType,
}

/// Optional type of the message signature produced.
/// If not provided, the signature must be Ed25519.
#[derive(Debug, Default)]
pub enum SignatureType {
    #[default]
    Ed25519,
}
