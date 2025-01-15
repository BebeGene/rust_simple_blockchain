// This script manages the blockchain, adding blocks and printing the chain.

use crate::block::Block;  // Imports the Block struct from the block module

#[derive(Debug)]  // Derives the Debug trait for the Blockchain struct to enable easy printing
pub struct Blockchain {
    pub chain: Vec<Block>,  // Holds the list (vector) of blocks in the blockchain
}

impl Blockchain {
    // Initializes a new blockchain with a genesis block
    pub fn new() -> Self {
        let mut chain = Vec::new();  // Creates an empty vector to hold the blocks
        let genesis_block = Block::new("Genesis Block".to_string(), String::from("0"));  // Creates the first block (genesis block)
        chain.push(genesis_block);  // Adds the genesis block to the chain

        Blockchain { chain }  // Returns a new Blockchain instance containing the genesis block
    }

    // Adds a new block to the blockchain
    pub fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap();  // Retrieves the last block in the chain
        let previous_hash = previous_block.hash.clone();  // Gets the hash of the previous block
        let new_block = Block::new(data, previous_hash);  // Creates a new block with the provided data and previous hash
        self.chain.push(new_block);  // Adds the new block to the blockchain
    }

    // Prints the entire blockchain to the console
    pub fn print_chain(&self) {
        for block in &self.chain {  // Iterates over each block in the blockchain
            block.display();  // Displays the block's information
            println!("-------------------------");  // Prints a separator line for clarity
        }
    }
}
