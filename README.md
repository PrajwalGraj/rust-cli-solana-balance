# Solana Wallet Balance Checker

A simple Rust CLI application that fetches the balance of a Solana wallet using the Solana Devnet JSON-RPC API.

## Features

* Takes a Solana wallet address as input
* Sends a JSON-RPC `getBalance` request to Solana Devnet
* Displays the wallet balance in SOL
* Handles RPC errors such as invalid wallet addresses
* Uses async HTTP requests with Tokio and Reqwest

## Tech Stack

* Rust
* Tokio
* Reqwest
* Serde
* Serde JSON
* Solana Devnet RPC

## Run Locally

```bash
cargo run
```

Enter a Solana wallet address when prompted:

```text
Enter Solana Wallet Address:
So11111111111111111111111111111111111111112
```

Example output:

```text
1.000000000 SOL
```

## Concepts Learned

* Async programming with `async` / `await`
* Sending HTTP POST requests with Reqwest
* Building JSON-RPC request payloads using `serde_json::json!`
* Deserializing nested JSON responses using Serde
* Using `Option<T>` to handle success and error responses
* Matching `Some` and `None`
* Converting lamports to SOL

