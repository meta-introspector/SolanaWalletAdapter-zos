use crate::WalletAdapterError;

#[derive(Debug, PartialEq, Eq)]
pub enum Commitment {
    /// A transaction has been validated and recorded in the blockchain by a single node
    Processed,
    /// A transaction has been validated and recorded by a majority of nodes in the Solana cluster.
    Confirmed,
    /// A has been included in a block that has been committed to the blockchain by the Solana cluster
    /// and is now irreversible.
    Finalized,
}

impl<'a> TryFrom<&'a str> for Commitment {
    type Error = WalletAdapterError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let converted = match value {
            "processed" | "recent" => Self::Processed,
            "confirmed" | "single" | "singleGossip" => Self::Confirmed,
            "finalized" | "root" | "max" => Self::Finalized,
            _ => return Err(WalletAdapterError::UnsupportedCommitment(value)),
        };

        Ok(converted)
    }
}
