use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub vin: Vec<TXInput>,
    pub vout: Vec<TXOutput>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout: usize,
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXOutput {
    pub value: i32,
    pub pub_key_hash: Vec<u8>,
}

impl Transaction {
    /// Creates a simple coinbase transaction.
    pub fn new_coinbase_tx(to: &str) -> Transaction {
        let subsidy = 50; // example subsidy amount
        let txout = TXOutput {
            value: subsidy,
            pub_key_hash: to.as_bytes().to_vec(),
        };
        Transaction {
            id: vec![], // in a full implementation, you'd compute the tx hash here
            vin: vec![],
            vout: vec![txout],
        }
    }

    /// Returns a reference to the transaction ID.
    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    /// Returns a reference to the transaction inputs.
    pub fn get_vin(&self) -> &[TXInput] {
        self.vin.as_slice()
    }

    /// Returns a reference to the transaction outputs.
    pub fn get_vout(&self) -> &[TXOutput] {
        self.vout.as_slice()
    }
}
