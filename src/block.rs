use crate::Transaction;
use crate::proof_of_work::ProofOfWork;
use crate::utils::{current_timestamp,sha256_digest};
use serde::{Deserialize, Serialize};
use sled::IVec;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block {
    /// Creates a new block by initializing fields and then mining it via proof-of-work.
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
        };
        print!("first------");
        println!("{:?}",block);
        // Run proof-of-work on the block to compute a valid nonce and hash.
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        block
    }

    /// Serializes the block into a vector of bytes.
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    /// Deserializes a block from a slice of bytes.
    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
    }

    /// Generates the genesis block (the first block in the blockchain).
    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        Block::new_block(String::from("None"), &transactions, 0)
    }

    /// Computes a hash of all transactions in the block.
    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        sha256_digest(txhashs.as_slice())
    }

    // Helper getter methods:
    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }
    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }
    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }
    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }
    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}
