# MerchPay

**MerchPay** is a decentralized payment platform on the Stellar network, created to empower small merchants in underserved and emerging markets by providing an accessible digital payment solution.

## Overview

MerchPay offers a Stellar-powered payment network for small businesses, enabling merchants to:

- Accept low-cost digital payments
- Provide loyalty rewards
- Easily cash out with services like MoneyGram
- Enable seamless customer transactions via [Lumina Wallet on Telegram](https://t.me/luminawalletbot/wallet)

This platform is designed to help merchants and customers in cash-dominant economies transition to digital finance, promoting financial resilience and economic growth.

## Key Features

- **Accessible Payment Processing**: Low fees make it affordable for regions with limited banking access.
- **Loyalty and Rewards**: Merchants can reward loyal customers, fostering engagement and repeat business.
- **Lumina Wallet Integration**: Customers pay with Lumina Wallet on Telegram, solving day-to-day payment needs.
- **Cash-In/Cash-Out Flexibility**: Convert digital payments to local currency with services like MoneyGram.

## Technologies Used

- **Rust & Soroban**: For secure payment processing and smart contracts.
- **TypeScript & Next Js**: Frontend for a user-friendly experience.
- **Telegram (Lumina Wallet)**: Simplifies payments for customers on the go.

## Getting Started

### Prerequisites

- **Node.js** and **Rust** installed
- Stellar account for development setup

### Installation

1. Clone the repository:
   ```bash
   git clone git@github.com:karangoraniya/merchpay-con.git
   ```
   ```bash
   cd merchpay-con
   ```
2. Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Install Soroban CLI:

```bash
cargo install soroban-cli
```

4. Build contract:

```bash
stellar contract build
```

## Testing

Run tests:

```bash
cargo test
```

## Deployment

1. Build for release:

```bash
stellar contract build --release
```

2. Deploy:

```bash
stellar contract install --wasm target/wasm32-unknown-unknown/release/merchpay.wasm --network testnet
```

3. Generate SDK:

```bash
stellar contract bindings typescript --network testnet --contract-id YOUR_CONTRACT_ID --output-dir ../frontend/contract-sdk
```
