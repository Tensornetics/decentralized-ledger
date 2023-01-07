use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub proof: u64,
    pub previous_hash: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, proof: u64, previous_hash: String) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            index,
            timestamp,
            transactions,
            proof,
            previous_hash,
        }
    }

    pub fn hash(&self) -> String {
        let data = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.input(data);
        format!("{:x}", hasher.result())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}

#[derive(Default)]
pub struct Blockchain {
    pub chain: VecDeque<Block>,
    pub current_transactions: VecDeque<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Self::default();

        blockchain.new_block(100, String::new());

        blockchain
    }

    pub fn new_block(&mut self, proof: u64, previous_hash: String) -> &Block {
        let block = Block::new(
            self.chain.len() as u64 + 1,
            self.current_transactions.drain(..).collect(),
            proof,
            previous_hash,
        );

        self.chain.push_back(block);
        self.chain.back().unwrap()
    }

    pub fn new_transaction(&mut self, transaction: Transaction) -> u64 {
        self.current_transactions.push_back(transaction);
        self.chain.
    pub fn new_transaction(&mut self, transaction: Transaction) -> u64 {
        self.current_transactions.push_back(transaction);
        self.chain.len() as u64 + 1
    }

    pub fn proof_of_work(&self, last_proof: u64) -> u64 {
        let mut proof = 0;
        while !self.valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    fn valid_proof(&self, last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = Sha256::digest(guess.as_bytes());
        guess_hash[0] == 0
    }
}
