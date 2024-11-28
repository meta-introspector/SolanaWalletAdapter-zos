use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::Wallet;

pub type StorageSchema = HashMap<blake3::Hash, Wallet>;

pub type StorageType = Rc<RefCell<StorageSchema>>;

#[derive(Default, PartialEq, Eq)]
pub struct WalletStorage(StorageType);

impl WalletStorage {
    pub fn clone_inner(&self) -> StorageType {
        Rc::clone(&self.0)
    }

    pub fn get_wallets(&self) -> Vec<Wallet> {
        self.0
            .borrow()
            .values()
            .map(|wallet| wallet.clone())
            .collect::<Vec<Wallet>>()
    }

    pub fn get_wallet(&self, wallet_name: &str) -> Option<Wallet> {
        let storage_ref = self.0.borrow();
        storage_ref
            .get(&blake3::hash(wallet_name.to_lowercase().as_bytes()))
            .map(|wallet| wallet.clone())
    }
}

impl core::fmt::Debug for WalletStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &*self.0.borrow())
    }
}
