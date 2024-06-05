# DecentralizedVotingApp

This project is a decentralized voting application that allows users to participate in and create decentralized votes using Rust for the backend, Solidity for the smart contracts, and HTML/CSS/JavaScript for the frontend.

## Structure

- **HTML/CSS/JavaScript**: Handles the frontend interface.
- **Rust**: Manages backend processing and database interactions.
- **Solidity**: Manages decentralized voting processes on the Ethereum blockchain.

## Setup

### HTML/CSS/JavaScript
1. Install the required dependencies:
    ```
    npm install
    ```
2. Start the development server:
    ```
    npm start
    ```

### Rust
1. Install Rust if it is not already installed.
2. Navigate to the project directory and set up the environment variables by creating a `.env` file in the `root` directory with the following content:
    ```
    DATABASE_URL=postgres://username:password@localhost/decentralized_voting_app
    PORT=8080
    INFURA_PROJECT_ID=your_infura_project_id
    MNEMONIC=your_wallet_mnemonic
    ```
3. Run the backend server:
    ```
    cargo run
    ```

### Solidity
1. Install Truffle if it is not already installed:
    ```
    npm install -g truffle
    ```
2. Navigate to the `contracts` directory.
3. Compile the smart contract:
    ```
    truffle compile
    ```
4. Deploy the smart contract:
    ```
    truffle migrate
    ```

## Overview

1. **HTML/CSS/JavaScript**: Provides a user interface for creating and participating in votes, viewing vote results, and user registration/login.
2. **Rust**: Processes the vote and user data, interacts with a PostgreSQL database, and handles requests from the frontend.
3. **Solidity**: Defines the smart contract for managing decentralized voting processes and ensuring transparency and security.
