use crate::{WalletAdapterError, WalletAdapterResult};

/// Solana Mainnet cluster,  https://api.mainnet-beta.solana.com
pub const MAINNET_IDENTIFIER: &'static str = "solana:mainnet";
/// Solana Devnet cluster, e.g. https://api.devnet.solana.com
pub const DEVNET_IDENTIFIER: &'static str = "solana:devnet";
/// Solana Testnet cluster, e.g. https://api.testnet.solana.com
pub const TESTNET_IDENTIFIER: &'static str = "solana:testnet";
/// Solana Localnet cluster, e.g. http://localhost:8899
pub const LOCALNET_IDENTIFIER: &'static str = "solana:localnet";

pub const MAINNET_ENDPOINT: &'static str = "https://api.mainnet-beta.solana.com";
pub const DEVNET_ENDPOINT: &'static str = "https://api.devnet.solana.com";
pub const TESTNET_ENDPOINT: &'static str = "https://api.testnet.solana.com";
pub const LOCALNET_ENDPOINT: &'static str = "http://localhost:8899";

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

impl SolanaChains {
    pub fn endpoint(&self) -> &str {
        match self {
            SolanaChains::MainNet => MAINNET_ENDPOINT,
            SolanaChains::DevNet => DEVNET_ENDPOINT,
            SolanaChains::TestNet => TESTNET_ENDPOINT,
            SolanaChains::LocalNet => LOCALNET_ENDPOINT,
        }
    }

    pub fn from_uri(uri: &str) -> WalletAdapterResult<Self> {
        if !uri.contains("https://") || !uri.contains("http://") {
            return Err(WalletAdapterError::UnsupportedCluster("uri"));
        }

        uri.try_into()
    }
}

impl<'a> TryFrom<&'a str> for SolanaChains {
    type Error = WalletAdapterError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let parsed = match value {
            MAINNET_IDENTIFIER => Self::MainNet,
            DEVNET_IDENTIFIER => Self::DevNet,
            TESTNET_IDENTIFIER => Self::TestNet,
            LOCALNET_IDENTIFIER => Self::LocalNet,
            MAINNET_ENDPOINT => Self::MainNet,
            DEVNET_ENDPOINT => Self::DevNet,
            TESTNET_ENDPOINT => Self::TestNet,
            LOCALNET_ENDPOINT => Self::LocalNet,
            _ => return Err(WalletAdapterError::UnsupportedCluster(value)),
        };

        Ok(parsed)
    }
}

#[cfg(test)]
mod chain_tests {
    use super::*;

    #[test]
    fn is_valid_uri() {
        assert_eq!(MAINNET_ENDPOINT, "https://api.mainnet-beta.solana.com");
        assert_eq!(DEVNET_ENDPOINT, "https://api.devnet.solana.com");
        assert_eq!(TESTNET_ENDPOINT, "https://api.testnet.solana.com");
        assert_eq!(LOCALNET_ENDPOINT, "http://localhost:8899");

        assert_eq!(MAINNET_IDENTIFIER, "solana:mainnet");
        assert_eq!(DEVNET_IDENTIFIER, "solana:devnet");
        assert_eq!(TESTNET_IDENTIFIER, "solana:testnet");
        assert_eq!(LOCALNET_IDENTIFIER, "solana:localnet");
    }

    #[test]
    fn valid_chain() {
        assert_eq!(SolanaChains::MainNet, "solana:mainnet".try_into().unwrap());
        assert_eq!(SolanaChains::DevNet, "solana:devnet".try_into().unwrap());
        assert_eq!(SolanaChains::TestNet, "solana:testnet".try_into().unwrap());
        assert_eq!(
            SolanaChains::LocalNet,
            "solana:localnet".try_into().unwrap()
        );
        assert!({
            let chain: Result<SolanaChains, _> = "solana:localnet2".try_into();

            chain.is_err()
        });

        assert_eq!(
            SolanaChains::MainNet,
            "https://api.mainnet-beta.solana.com".try_into().unwrap()
        );
        assert_eq!(
            SolanaChains::DevNet,
            "https://api.devnet.solana.com".try_into().unwrap()
        );
        assert_eq!(
            SolanaChains::TestNet,
            "https://api.testnet.solana.com".try_into().unwrap()
        );
        assert_eq!(
            SolanaChains::LocalNet,
            "http://localhost:8899".try_into().unwrap()
        );
        assert!({
            let chain: Result<SolanaChains, _> = "https://localhost:8899".try_into();

            chain.is_err()
        });
        assert!({
            let chain: Result<SolanaChains, _> = "https://cluster.foo".try_into();

            chain.is_err()
        });
    }

    #[test]
    fn validate_endpoint() {
        assert_eq!(
            SolanaChains::MainNet.endpoint(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(
            SolanaChains::DevNet.endpoint(),
            "https://api.devnet.solana.com"
        );
        assert_eq!(
            SolanaChains::TestNet.endpoint(),
            "https://api.testnet.solana.com"
        );
        assert_eq!(SolanaChains::LocalNet.endpoint(), "http://localhost:8899");
    }
}
