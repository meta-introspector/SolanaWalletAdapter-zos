use crate::WalletAdapterError;

/// Solana Mainnet cluster,  https://api.mainnet-beta.solana.com
pub const SOLANA_MAINNET_CHAIN: &str = "solana:mainnet";

/// Solana Devnet cluster, e.g. https://api.devnet.solana.com
pub const SOLANA_DEVNET_CHAIN: &str = "solana:devnet";

/// Solana Testnet cluster, e.g. https://api.testnet.solana.com
pub const SOLANA_TESTNET_CHAIN: &str = "solana:testnet";

/// Solana Localnet cluster, e.g. http://localhost:8899
pub const SOLANA_LOCALNET_CHAIN: &str = "solana:localnet";

/// Solana Clusters
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum SolanaChains {
    /// Solana Mainnet cluster,  https://api.mainnet-beta.solana.com
    MainNet,
    /// Solana Devnet cluster, e.g. https://api.devnet.solana.com
    DevNet,
    /// Solana Testnet cluster, e.g. https://api.testnet.solana.com
    TestNet,
    /// Solana Localnet cluster, e.g. http://localhost:8899
    LocalNet,
}

impl TryFrom<&str> for SolanaChains {
    type Error = WalletAdapterError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = match value {
            "solana:mainnet" => Self::MainNet,
            "solana:devnet" => Self::DevNet,
            "solana:testnet" => Self::TestNet,
            "solana:localnet" => Self::LocalNet,
            _ => return Err(WalletAdapterError::UnsupportedCluster(value.into())),
        };

        Ok(parsed)
    }
}
