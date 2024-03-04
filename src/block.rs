use crate::hash;
use crate::utils::ValidData;
use chrono::{DateTime, Utc};

pub struct Block<T>
where
    T: ValidData,
{
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub previous_hash: String,
    pub data: T,
    pub nonce: u64,
}

impl<T> Block<T>
where
    T: ValidData,
{
    pub fn get_first_block() -> Block<T> {
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
            "{}{}{}{}{}",
            self.id,
            self.timestamp.to_rfc3339(),
            self.previous_hash,
            self.data.to_string(),
            self.nonce,
        );

        let hash_result = hash::hash_string(hash_input);

        hash_result
    }
}
