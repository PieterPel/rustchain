use hex;
use ring::digest;
use std::{error, fmt};

const NUMBER_OF_LEADING_ZEROS: usize = 5;

pub fn hash_string(input: &str) -> String {
    let hashed = digest::digest(&digest::SHA256, input.as_bytes());
    hex::encode(hashed)
}

pub fn check_hash(hash: &str) -> bool {
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
