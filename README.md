# Rust Blockchain

This is a simple blockchain project implemented in Rust. It includes core blockchain functionalities like mining blocks, handling transactions, managing a UTXO set, and running a decentralized node network.

## Features
- Create and manage a blockchain
- Generate wallets and manage private keys
- Send transactions between addresses
- Implement a UTXO (Unspent Transaction Output) model
- Mine new blocks and earn rewards
- Run a node to participate in the blockchain network
- Communicate between nodes over a P2P network

## Installation
1. Install Rust and Cargo:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone this repository:
   ```sh
   git clone <your-repo-url>
   cd rust-blockchain
   ```
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage
### Creating a new blockchain
Initialize a new blockchain with a genesis block:
```sh
./target/release/rust-blockchain createblockchain --address <your_wallet_address>
```

### Creating a new wallet
Generate a new wallet and get an address:
```sh
./target/release/rust-blockchain createwallet
```

### Checking wallet balance
Check the balance of a wallet:
```sh
./target/release/rust-blockchain getbalance --address <your_wallet_address>
```

### Starting a node
Run a node to participate in the blockchain network:
```sh
./target/release/rust-blockchain startnode --address <your_wallet_address>
```

### Sending transactions
Send a transaction from one wallet to another:
```sh
./target/release/rust-blockchain send --from <sender_address> --to <receiver_address> --amount <amount> --mine
```

### Viewing the blockchain
Print all blocks in the blockchain:
```sh
./target/release/rust-blockchain printchain
```

### Reindexing the UTXO set
```sh
./target/release/rust-blockchain reindexutxo
```

## How It Works
- Transactions are first stored in a memory pool.
- When a block is mined, transactions from the pool are added to the blockchain.
- A node running on the network validates transactions and propagates blocks.
- The UTXO set maintains spendable outputs for efficient transaction verification.

## To-Do / Future Improvements
- Improve networking for better P2P communication
- Optimize memory pool handling
- Implement better transaction validation and consensus
- Enhance security with better key management
- **Implement quantum-resistant cryptography** to secure transactions against future threats
- **Improve node communication** by allowing in-memory UTXO set access instead of requiring multiple database instances

## Contributing
Feel free to open issues or submit pull requests to improve this project!

## License
This project is open-source and available under the MIT license.

