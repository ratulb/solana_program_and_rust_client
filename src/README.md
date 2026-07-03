# Rust On-Chain Contract and Client on Solana

## What is Solana?

Solana is a high-performance blockchain designed for decentralized applications and cryptocurrencies. It achieves speed and scalability through several innovations:

- **Proof of History (PoH)** — a cryptographic clock that timestamps transactions before consensus, enabling parallel processing without the bottleneck of sequential execution found in chains like Ethereum.
- **Parallel transaction processing** — Solana's Seaport scheduler processes non-overlapping transactions concurrently. Unlike Ethereum's single-threaded EVM, Solana can handle thousands of transactions per second.
- **Low fees (~$0.0002)** and fast finality (~400ms) — achieved through PoH combined with Tower BFT consensus.
- **Stateless programs + account model** — on-chain programs (smart contracts) are stateless shared libraries. They persist state by owning dedicated accounts. Clients must declare all accounts an instruction touches upfront, enabling the runtime to parallelize safely.
- **Rust-based development** — programs compile to BPF (Berkeley Packet Filter) bytecode, not EVM bytecode. Development is in Rust or C, giving access to the full Rust ecosystem and strong memory safety guarantees.
- **No mempool** — transactions are forwarded directly to the current leader, unlike Ethereum's public mempool where pending transactions are visible to all.

These design choices make Solana fundamentally different from Ethereum, both in architecture and development model.

---

## This Project

This project demonstrates how to use Solana Rust APIs to write a Solana on-chain program (contract) and an off-chain client program in Rust via a simple counter example.

The project comprises:

- An on-chain counter program
- A Rust client that can send an "Increment" counter message to the on-chain program and get the current reading

## Source

The full source code is available at [github.com/ratulb/solana_program_and_rust_client](https://github.com/ratulb/solana_program_and_rust_client).

## About the Author

Created by [Ratul Buragohain](https://github.com/ratulb). Licensed under GPL-3.0.
