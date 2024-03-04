use crate::block::Block;
use crate::hash::{check_hash, HashError};
use crate::utils::ValidData;
use chrono::Utc;
use std::vec;

pub struct BlockChain<T>
where
    T: ValidData,
{
    chain: Vec<Block<T>>,
}

impl<T> BlockChain<T>
where
    T: ValidData,
{
    pub fn new() -> BlockChain<T> {
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

    pub fn all_data(&self) -> Vec<&T> {
        self.chain.iter().map(|block| &block.data).collect()
    }

    pub fn new_block(&self, data: T, nonce: u64) -> Block<T> {
        let last_block = self.chain.last().unwrap();

        Block {
            id: last_block.id + 1,
            timestamp: Utc::now(),
            previous_hash: last_block.calculate_hash(),
            data,
            nonce,
        }
    }

    pub fn try_adding_block(&mut self, block: Block<T>) -> Result<(), HashError> {
        let block_hash = block.calculate_hash();

        if check_hash(&block_hash) {
            self.chain.push(block);
            Ok(())
        } else {
            Err(HashError::InvalidHash(block_hash))
        }
    }

    pub fn verify_hashes(&self) -> bool {
        let mut iter = self.chain.iter().peekable();

        let first_hash = self.chain[0].calculate_hash();

        // check the hashes of all blocks
        while let Some(block) = iter.next() {
            let hash = block.calculate_hash();

            // Invalid chain if the hash is not correct
            if hash != first_hash && !check_hash(&hash) {
                return false;
            }

            // Do not check the next blok if already at the last
            if let Some(next_block) = iter.peek() {
                // Invalid chain if the hashes do not match
                if hash != next_block.previous_hash {
                    return false;
                }
            }
        }

        true
    }
}
