// This script defines the structure of a Block and methods for creating and displaying a block.

use chrono::prelude::*;       // Imports chrono library to handle timestamps
use sha2::{Sha256, Digest};   // Imports Sha256 hashing algorithm and Digest trait

#[derive(Clone, Debug)]       // Derives Clone and Debug traits for the Block struct
pub struct Block {
    pub timestamp: String,      // Stores the timestamp of the block creation
    pub data: String,           // Stores the data or message of the block
    pub previous_hash: String,  // Stores the hash of the previous block in the chain
    pub hash: String,           // Stores the hash of the current block
}

impl Block {
    // Constructor that creates a new block
    pub fn new(data: String, previous_hash: String) -> Self {
        // Gets the current timestamp in RFC3339 format
        let timestamp = Utc::now().to_rfc3339();

        // Creates a new Sha256 hasher
        let mut hasher = Sha256::new();

        // Updates the hasher with the current timestamp, data, and previous hash
        hasher.update(&timestamp);
        hasher.update(&data);
        hasher.update(&previous_hash);

        // Finalizes the hash and formats it as a hexadecimal string
        let hash = format!("{:x}", hasher.finalize());

        // Returns a new Block instance with the generated timestamp, data, previous hash, and the calculated hash
        Block {
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    // Displays block information in a user-friendly format
    pub fn display(&self) {
        // Prints the block's timestamp, data, previous hash, and hash to the console
        println!("Block: {{");
        println!("  Timestamp: {}", self.timestamp);
        println!("  Data: {}", self.data);
        println!("  Previous Hash: {}", self.previous_hash);
        println!("  Hash: {}", self.hash);
        println!("}}");
    }
}
