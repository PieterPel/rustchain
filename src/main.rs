use rustchain::BlockChain;

fn main() {
    let mut block_chain = BlockChain::new();

    let new_data = "I am the second block!".to_string();

    let new_block = block_chain.new_block(new_data);

    block_chain.add_block(new_block);

    println!("{}", block_chain.last_hash().to_string())
}
