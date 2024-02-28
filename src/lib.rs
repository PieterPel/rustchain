use chrono::{DateTime, Utc};
use hex;
use rand::prelude::*;
use ring::digest;
use std::vec;

pub struct Block {
    id: u64,
    timestamp: DateTime<Utc>,
    previous_hash: String,
    data: String,
    nonce: u64,
}

pub struct BlockChain {
    chain: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let first_block = Block::get_first_block();

        let start_chain = vec![first_block];

        BlockChain { chain: start_chain }
    }

    pub fn length(&self) -> usize {
        self.chain.len()
    }

    pub fn last_hash(&self) -> String {
        self.chain.last().unwrap().calculate_hash()
    }

    pub fn new_block(&self, data: String) -> Block {
        let last_block = self.chain.last().unwrap();

        let mut rng = rand::thread_rng();

        Block {
            id: last_block.id + 1,
            timestamp: Utc::now(),
            previous_hash: last_block.calculate_hash(),
            data,
            nonce: rng.gen(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }
}

impl Block {
    fn get_first_block() -> Block {
        let opening_message = String::from("This is the first block");

        let mut rng = rand::thread_rng();

        Block {
            id: 0,
            timestamp: Utc::now(),
            previous_hash: String::from("0"),
            data: opening_message,
            nonce: rng.gen(),
        }
    }

    fn calculate_hash(&self) -> String {
        let hash_input = &format!(
            "{}{}{}{}",
            self.id,
            self.timestamp.to_rfc3339(),
            self.previous_hash,
            self.nonce,
        );

        let hash_result = hash_string(hash_input);

        hash_result
    }
}

fn hash_string(input: &str) -> String {
    let hashed = digest::digest(&digest::SHA256, input.as_bytes());
    hex::encode(hashed)
}
