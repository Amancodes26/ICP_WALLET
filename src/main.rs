mod wallet;

fn main() {
    // Create wallets with a private key
    let mut alice_wallet = wallet::Wallet::new(String::from("Alice"), String::from("alice_key"));
    let mut bob_wallet = wallet::Wallet::new(String::from("Bob"), String::from("bob_key"));

    // Add initial balance to Alice
    alice_wallet.receive_tokens(100);
    println!("Alice's Balance: {}", alice_wallet.get_balance());

    // Alice sends tokens to Bob
    if let Err(e) = alice_wallet.send_tokens(&mut bob_wallet, 30, "alice_key") {
        println!("Error: {}", e);
    }

    // Print updated balances
    println!("Alice's Balance: {}", alice_wallet.get_balance());
    println!("Bob's Balance: {}", bob_wallet.get_balance());
}
