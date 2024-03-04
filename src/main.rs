use rustchain::BlockChain;

fn main() {
    let mut block_chain = BlockChain::new();

    let new_data = "I am the second block!".to_string();

    let mut nonce: u64 = 1;
    loop {
        let data = new_data.clone();

        let new_block = block_chain.new_block(data, nonce);

        match block_chain.try_adding_block(new_block) {
            Ok(()) => {
                // Our nonce gives a good hash!
                println!(
                    "Nonce {} gives good hash: {}",
                    nonce,
                    block_chain.last_hash()
                );
                break;
            }
            Err(_error) => {
                //println!("Nonce {} gives a bad hash: {}", nonce, _error);
            }
        }

        nonce = nonce + 1;
    }

    println!("{:?}", block_chain.all_data());

    println!("{:?}", block_chain.verify_hashes());
}
