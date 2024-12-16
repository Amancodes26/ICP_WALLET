use crate::wallet::Wallet;
use std::collections::HashMap;

#[derive(Debug)]
pub enum BlockchainError {
    SenderNotFound,
    ReceiverNotFound,
    InsufficientFunds,
    TokenTransferError(String),
}

pub struct Blockchain {
    pub wallets: HashMap<String, Wallet>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            wallets: HashMap::new(),
        }
    }

    pub fn create_wallet(&mut self, address: String, private_key: String) -> Wallet {
        let wallet = Wallet::new(address.clone(), private_key);
        self.wallets.insert(address.clone(), wallet.clone());
        wallet
    }

    pub fn transfer_tokens(
        &mut self,
        from: String,
        to: String,
        amount: u64,
        sender_key: &str,
    ) -> Result<(), BlockchainError> {
        // First check if both wallets exist
        if !self.wallets.contains_key(&from) {
            return Err(BlockchainError::SenderNotFound);
        }
        if !self.wallets.contains_key(&to) {
            return Err(BlockchainError::ReceiverNotFound);
        }

        // Clone the sender wallet
        let mut sender_wallet = self.wallets.get(&from).unwrap().clone();
        let mut receiver_wallet = self.wallets.get(&to).unwrap().clone();

        // Perform the transfer
        sender_wallet.send_tokens(&mut receiver_wallet, amount, sender_key)
            .map_err(|e| BlockchainError::TokenTransferError(e))?;

        // Update both wallets in the blockchain
        self.wallets.insert(from, sender_wallet);
        self.wallets.insert(to, receiver_wallet);

        Ok(())
    }

    pub fn add_wallet(&mut self, address: String, wallet: Wallet) {
        self.wallets.insert(address, wallet);
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }
    
    pub fn fund_wallet(&mut self, address: &str, amount: u64) -> Result<(), BlockchainError> {
        if let Some(wallet) = self.wallets.get_mut(address) {
            wallet.receive_tokens(amount);
            Ok(())
        } else {
            Err(BlockchainError::ReceiverNotFound)
        }
    }
}