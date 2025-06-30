# Solana Wallet Info API

A lightweight HTTP API server built with **Rust**, **Axum**, and **solana-client** to fetch SOL balances of any wallet on the Solana blockchain.

## Features

- Query SOL balance of any Solana wallet
- Fast, async, and minimal
- HTTP server powered by Axum
- Built with `solana-client` and `solana-sdk`

---

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum)
- [Solana SDK](https://docs.rs/solana-sdk)
- [Tokio](https://tokio.rs/)

---

## Getting Started

### 1. Clone the Repo

```bash
git clone https://github.com/your-username/solana-wallet-api.git
cd solana-wallet-api
```

### 2.Install Rust if you dont have

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 3.Run the Server

```bash
cargo run
```

---

## Usage

### Get wallet sol balance

```bash
GET /wallet/{address}
```

#### Example

```bash
curl http://127.0.0.1:3000/wallet/Fht9GoUNVZB1xGAbX1V5oBRv7EJ9LD23bE8JMyCmSxB1
```

#### Response

```json
{
  "sol_balance": 2.1357
}
```
