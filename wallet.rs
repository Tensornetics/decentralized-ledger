use rand::{thread_rng, Rng};
use tensornetics_decentralized_ledger::Transaction;

pub struct Wallet {
    pub id: String,
    pub balance: u64,
}

impl Wallet {
    pub fn new(id: String) -> Self {
        Self { id, balance: 0 }
    }

    pub fn generate_keypair() -> (String, String) {
        let mut rng = thread_rng();
        let private_key: u64 = rng.gen();
        let public_key = format!("{:x}", private_key);
        (public_key, private_key.to_string())
    }

    pub fn create_transaction(&self, recipient: &str, amount: u64) -> Transaction {
        Transaction::new(self.id.clone(), recipient.to_string(), amount)
    }
}
