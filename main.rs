use tensornetics_decentralized_ledger::{Blockchain, Transaction};

fn main() {
    let mut blockchain = Blockchain::new();

    println!("{:?}", blockchain);

    let tx1 = Transaction {
        sender: String::from("Alice"),
        recipient: String::from("Bob"),
        amount: 50,
    };

    let tx2 = Transaction {
        sender: String::from("Bob"),
        recipient: String::from("Charlie"),
        amount: 25,
    };

    let tx3 = Transaction {
        sender: String::from("Charlie"),
        recipient: String::from("Alice"),
        amount: 75,
    };

    let block1 = blockchain.new_block(blockchain.proof_of_work(blockchain.chain.back().unwrap().proof), String::new());
    let block2 = blockchain.new_block(blockchain.proof_of_work(blockchain.chain.back().unwrap().proof), String::new());
    let block3 = blockchain.new_block(blockchain.proof_of_work(blockchain.chain.back().unwrap().proof), String::new());

    println!("{:?}", blockchain);
}
