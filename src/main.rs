mod block;
mod blockchain;
mod proof_of_work;
mod transaction;
mod utils;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    // Initialize blockchain (loads genesis block or creates one)
    let mut blockchain = Blockchain::new();

    // Create a coinbase transaction (for simplicity, our coinbase transaction sends coins to a dummy address)
    let coinbase_tx = Transaction::new_coinbase_tx("address-1");

    // Add a new block with the coinbase transaction.
    blockchain.add_block(vec![coinbase_tx]);

    // Print the blockchain's blocks.
    for block in blockchain.chain.iter() {
        println!("---------------------------------");
        println!("Block Height: {}", block.get_height());
        println!("Timestamp: {}", block.get_timestamp());
        println!("Prev Hash: {}", block.get_pre_block_hash());
        println!("Hash: {}", block.get_hash());
        println!("Transactions: {:?}", block.get_transactions());
    }
}
