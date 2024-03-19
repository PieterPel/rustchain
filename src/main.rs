use rustchain::BlockChain;
use rustchain::Person;

mod utils;

use crate::utils::StringBytes;

fn main() {
    let mut block_chain = BlockChain::<StringBytes>::new();

    let alice = Person::new();
    let bob = Person::new();

    let message = "Hello Alice, I am Bob!".to_string();

    bob.send_message(&message, &alice.public_key, &mut block_chain);

    println!("{:?}", block_chain.all_data());

    println!("{:?}", block_chain.verify_hashes());
}
