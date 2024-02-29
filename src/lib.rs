use chrono::{DateTime, Utc};
use hex;
use ring::digest;
use std::{error, fmt, vec};

const NUMBER_OF_LEADING_ZEROS: usize = 5;

pub trait ValidData: ToString + Default {}

impl ValidData for String {}

pub struct Block<T>
where
    T: ValidData,
{
    id: u64,
    timestamp: DateTime<Utc>,
    previous_hash: String,
    data: T,
    nonce: u64,
}

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
}

impl<T> Block<T>
where
    T: ValidData,
{
    fn get_first_block() -> Block<T> {
        Block {
            id: 0,
            timestamp: Utc::now(),
            previous_hash: String::from("0"),
            data: T::default(),
            nonce: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
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

fn check_hash(hash: &str) -> bool {
    hash.chars().take(NUMBER_OF_LEADING_ZEROS).all(|c| c == '0')
}

#[derive(Debug)]
pub enum HashError {
    InvalidHash(String),
}

impl error::Error for HashError {}

impl fmt::Display for HashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashError::InvalidHash(hash) => write!(f, "Invalid hash: {}", hash),
        }
    }
}
