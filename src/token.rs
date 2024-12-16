use std::collections::HashMap;

pub struct Token {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub balances: HashMap<String, u64>,
}

impl Token {
    pub fn new(name: String, symbol: String, total_supply: u64) -> Self {
        Token {
            name,
            symbol,
            total_supply,
            balances: HashMap::new(),
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        if let Some(balance) = self.balances.get_mut(from) {
            if *balance < amount {
                return Err(String::from("Insufficient funds."));
            }
            *balance -= amount;
            *self.balances.entry(to.to_string()).or_insert(0) += amount;
            Ok(())
        } else {
            Err(String::from("Sender does not exist."))
        }
    }

    pub fn balance_of(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
}
