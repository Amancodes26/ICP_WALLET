#[allow(dead_code)]
#[derive(Clone)]
pub struct Wallet {
    pub address: String,
    pub balance: u64,
    private_key: String, // Private to the struct
}

impl Wallet {
    pub fn new(address: String, private_key: String) -> Self {
        Wallet {
            address,
            balance: 0,
            private_key,
        }
    }

    pub fn send_tokens(
        &mut self,
        recipient: &mut Wallet,
        amount: u64,
        sender_key: &str,
    ) -> Result<(), String> {
        if self.private_key != sender_key {
            return Err(String::from("Unauthorized transaction."));
        }
        if self.balance < amount {
            return Err(String::from("Insufficient balance."));
        }
        self.balance -= amount;
        recipient.balance += amount;
        Ok(())
    }

    pub fn receive_tokens(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn get_balance(&self) -> u64 {
        self.balance
    }
}