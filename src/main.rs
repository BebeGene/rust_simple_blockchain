// This script initializes the blockchain, adds blocks, and prints the chain.

mod blockchain; // Imports the 'blockchain' module
mod block;      // Imports the 'block' module

// Brings the Blockchain and Block structs into scope
use crate::{blockchain::Blockchain, block::Block};

fn main() {
    // Creates a new blockchain instance (with the genesis block)
    let mut blockchain = Blockchain::new();

    // Adds a new block with the data "First Block" to the blockchain
    blockchain.add_block("First Block".to_string());

    // Adds another block with the data "Second Block" to the blockchain
    blockchain.add_block("Second Block".to_string());

    // Adds yet another block with the data "Third Block" to the blockchain
    blockchain.add_block("Third Block".to_string());

    // Prints out the entire blockchain to the console
    blockchain.print_chain();
}
