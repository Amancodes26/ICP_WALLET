# ICP Wallet

> A Rust implementation of cryptocurrency wallet system for Internet Computer Protocol.

---

## Project Overview

The **ICP Token Wallet** project is designed to showcase how to create a simple token wallet that interacts with the **Internet Computer** blockchain. It allows users to:
- Send tokens between wallets.
- Receive tokens into a wallet.
- Display the current balance of the wallet.

The project includes:
- **Smart Contract** (in Rust) deployed on the ICP blockchain.
- **Backend** for interacting with the deployed smart contract and managing wallets.
- **Unit tests** to validate functionality of the wallet and blockchain interactions.

---

## 🚀 Features

- ✅ Wallet Management
- ✅ Secure Transactions
- ✅ Balance Tracking
- ✅ Token System
---

## Technologies Used

- **Rust**: For the smart contract logic (Canister).
- **ICP (Internet Computer Protocol)**: Blockchain for decentralized app deployment.
- **dfx**: The official tool for interacting with the Internet Computer blockchain and deploying smart contracts.
- **ic-agent**: Rust library for interacting with the ICP blockchain.

---

## 📁 Project Structure

```text
icp_wallet/
├── src/
│   ├── main.rs         # Entry point
│   ├── lib.rs          # Library exports
│   ├── wallet.rs       # Wallet logic
│   ├── blockchain.rs   # Chain operations
│   └── token.rs        # Token management
└── tests/
    └── integration_test.rs

``` 

## Installation and Setup

Follow these steps to set up the project and deploy it on a local ICP test network.

### Step 1: Install Prerequisites

1. **Install `dfx` (Internet Computer SDK)**:

   To interact with the ICP network and deploy canisters, you need the `dfx` tool. Install it by running:

   ```bash
   sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"



`dfx --version`

1.  **Install Rust**:

    If you don't already have Rust installed, follow the official guide to install it: <https://www.rust-lang.org/learn/get-started>.

    Alternatively, you can install Rust via `rustup`:

   

    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

    After installation, verify Rust:

    

    `rustc --version`

### Step 2: Clone the Repository

Clone the project repository to your local machine:



`git clone https://github.com/yourusername/icp-token-wallet.git
cd icp-token-wallet`

### Step 3: Build the Project

To build the Rust-based smart contract (Canister), run:



`cargo build --release`

### Step 4: Start the Local ICP Test Network

Start the local ICP test network with:


`dfx start`

This will start a local replica of the Internet Computer blockchain, allowing you to deploy and test your contract.

### Step 5: Deploy the Smart Contract

Now deploy your Rust-based token wallet contract to the local ICP test network:



`dfx deploy`

Once deployed, `dfx` will provide you with the canister ID, which uniquely identifies your deployed contract on the ICP network.

* * * * *

Usage
-----

After deploying the contract, you can interact with it and test the wallet functionality.

### Step 1: Interact with the Contract

You can interact with your deployed smart contract using the `dfx` CLI.

#### Example: Sending Tokens

To send tokens between two wallets, use the following command:



`dfx canister call my_canister send_tokens '("Alice", "Bob", 50, "alice_key")'`

This example sends 50 tokens from Alice to Bob.

#### Example: Checking Wallet Balance

To check the balance of a wallet, use:



`dfx canister call my_canister get_balance '("Alice")'`

### Step 2: Testing

# Run all tests
cargo test

# Run integration tests
cargo test --test integration_test

* * * * *

Contributing
------------

Feel free to fork the repository, submit pull requests, or open issues if you encounter any bugs or have suggestions for improvements.

To contribute:

1.  Fork the repository on GitHub.
2.  Create a new branch.
3.  Commit your changes.
4.  Submit a pull request.