mod blockchain;
mod consensus;
mod smart_contract;
mod transaction;
mod wallet;

pub use blockchain::Blockchain;
pub use consensus::Consensus;
pub use smart_contract::SmartContract;
pub use transaction::Transaction;
pub use wallet::Wallet;

use tensornetics_decentralized_ledger::{Blockchain, Consensus, SmartContract, Transaction, Wallet};

fn main() {
    let blockchain = Blockchain::new();
    let consensus = Consensus::new(blockchain);
    let smart_contract = SmartContract::new(blockchain);
    let transaction = Transaction::new("Alice".to_string(), "Bob".to_string(), 10);
    let wallet = Wallet::new("Alice".to_string());
}
