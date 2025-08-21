# ⚓️ Anchor Swap Escrow

A 🦀 Rust & ⚡️ TypeScript-based escrow program for atomic swaps on the Solana blockchain, powered by the [Anchor framework](https://project-serum.github.io/anchor/).

---

## 🚀 Overview

**anchor-swap-escrow** enables trustless, atomic token swaps between parties using on-chain escrow accounts. Perfect for decentralized exchanges, peer-to-peer trading, or any application where secure settlement is a must!

---

## ✨ Features

- 🔒 Trustless escrow for atomic token swaps on Solana
- 🛠 Built with the Anchor framework for security & developer productivity
- 💱 Supports custom Solana token programs (SPL tokens)
- 🧩 TypeScript client for seamless web integration

---

## 🛠 Getting Started

### 📋 Prerequisites

- 🦀 [Rust](https://www.rust-lang.org/tools/install)
- 🪙 [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- ⛓ [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html)
- 🟦 [Node.js](https://nodejs.org/) (for TypeScript client)

### ⬇️ Installation

```bash
git clone https://github.com/bytehash69/anchor-swap-escrow.git
cd anchor-swap-escrow
```

### 🏗 Build the Program

```bash
anchor build
```

### 🧪 Run Tests

```bash
anchor test
```

### 🛳 Deploy to Localnet

```bash
anchor deploy
```

---

## 📦 Usage

### 📝 Rust (On-Chain Program)

- Logic lives in `programs/anchor-swap-escrow`.
- Implement & customize instructions for initializing escrows, swapping, and closing accounts.

### 🟦 TypeScript (Client SDK)

- The TypeScript client (see `tests/` or `client/`) lets you interact with the program.
- Example scripts show how to create escrows and execute swaps.

---

## 💡 Example: Initiating an Escrow (TypeScript)

```typescript
import { Program, AnchorProvider, web3 } from "@project-serum/anchor";
import { AnchorSwapEscrow } from "../target/types/anchor_swap_escrow";
// ...setup provider and program...

// Call initializeEscrow, exchange, and closeEscrow instructions accordingly
```

For full examples, check out the `tests/` directory.

---

## 🤝 Contributing

Pull requests & issues are welcome! For big changes, open an issue to discuss what you’d like to change first.

---

## 📄 License

Licensed under the MIT License.

---

## ✉️ Contact

Maintained by [bytehash69](https://github.com/bytehash69).

---

> ⚠️ **This project is not audited. Use at your own risk!**
