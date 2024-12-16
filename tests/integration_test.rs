use icp_wallet::{wallet::Wallet, blockchain::Blockchain};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_send_receive() {
        let mut alice = Wallet::new(String::from("Alice"), String::from("alice_key"));
        let mut bob = Wallet::new(String::from("Bob"), String::from("bob_key"));

        alice.receive_tokens(100);
        assert_eq!(alice.get_balance(), 100);

        alice.send_tokens(&mut bob, 50, "alice_key").unwrap();
        assert_eq!(alice.get_balance(), 50);
        assert_eq!(bob.get_balance(), 50);
    }

    #[test]
    fn test_blockchain_transfer() {
        let mut chain = Blockchain::new();
        let alice = chain.create_wallet(String::from("Alice"), String::from("alice_key"));
        let bob = chain.create_wallet(String::from("Bob"), String::from("bob_key"));
    
        // Fund Alice's wallet using the new method
        chain.fund_wallet(&alice.address, 100).unwrap();
    
        chain.transfer_tokens(alice.address.clone(), bob.address.clone(), 50, "alice_key").unwrap();
        assert_eq!(chain.get_wallet(&alice.address).unwrap().get_balance(), 50);
        assert_eq!(chain.get_wallet(&bob.address).unwrap().get_balance(), 50);
    }
}