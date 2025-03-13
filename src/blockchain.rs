use crate::block::Block;
use sled;
use bincode;
use std::sync::{Arc, RwLock};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub tip_hash: Arc<RwLock<String>>,
    pub db: sled::Db,
}

impl Blockchain {
    /// Creates a new blockchain by loading from the embedded database or creating a genesis block.
    pub fn new() -> Blockchain {
        let db = sled::open("blockchain_db").expect("Failed to open database");
        let mut chain = Vec::new();

        // Use key "0" for the genesis block.
        if let Ok(Some(genesis_bytes)) = db.get("0") {
            let genesis_block: Block = bincode::deserialize(&genesis_bytes).unwrap();
            chain.push(genesis_block.clone());
            // Currently, there's no println here. You can add one if you want:
            println!("Loaded genesis block from DB: {:?}\n", genesis_block);
            Blockchain {
                chain,
                tip_hash: Arc::new(RwLock::new(genesis_block.get_hash().to_string())),
                db,
            }
        } else {
            let coinbase_tx = crate::Transaction::new_coinbase_tx("genesis-address");
            let genesis_block = Block::generate_genesis_block(&coinbase_tx);
            println!("Created genesis block: {:?}\n", genesis_block);
            db.insert("0", bincode::serialize(&genesis_block).unwrap()).unwrap();
            chain.push(genesis_block.clone());
            Blockchain {
                chain,
                tip_hash: Arc::new(RwLock::new(genesis_block.get_hash().to_string())),
                db,
            }
        }
        
        /* if let Ok(Some(genesis_bytes)) = db.get("0") {
            let genesis_block: Block = bincode::deserialize(&genesis_bytes).unwrap();
            chain.push(genesis_block.clone());
            Blockchain {
                chain,
                tip_hash: Arc::new(RwLock::new(genesis_block.get_hash().to_string())),
                db,
            }
        } else {
            // If no genesis block exists, create one.
            let coinbase_tx = crate::Transaction::new_coinbase_tx("genesis-address");
            let genesis_block = Block::generate_genesis_block(&coinbase_tx);
            println!("{:?}",genesis_block);
            db.insert("0", bincode::serialize(&genesis_block).unwrap()).unwrap();
            chain.push(genesis_block.clone());
            Blockchain {
                chain,
                tip_hash: Arc::new(RwLock::new(genesis_block.get_hash().to_string())),
                db,
            }
        } */
    }

    /// Adds a new block with the given transactions.
    pub fn add_block(&mut self, transactions: Vec<crate::Transaction>) {
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new_block(prev_block.get_hash().to_string(), &transactions, prev_block.get_height() + 1);

        // Save block to the database using its height as key.
        let key = new_block.get_height().to_string();
        self.db.insert(key, bincode::serialize(&new_block).unwrap()).unwrap();

        self.chain.push(new_block);
        // Update tip_hash.
        let latest_hash = self.chain.last().unwrap().get_hash().to_string();
        let mut tip = self.tip_hash.write().unwrap();
        *tip = latest_hash;
    }
}
