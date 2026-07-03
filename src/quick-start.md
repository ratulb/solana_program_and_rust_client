# Quick Start

The following dependencies are required to build and run this example:

- Install Rust v1.60.0 or later from [rustup.rs](https://rustup.rs/)
- Install Solana v1.10.8 or later from [docs.solana.com/cli/install-solana-cli-tools](https://docs.solana.com/cli/install-solana-cli-tools)

If this is your first time using Rust, see the [Installation Notes](./installation.md).

## Configure CLI

Set CLI config url to localhost cluster:

```bash
solana config set --url localhost
```

Create CLI Keypair (if first time):

```bash
solana-keygen new
```

## Start Local Solana Cluster

```bash
solana-test-validator
```

For a clean slate after trials:

```bash
solana-test-validator --reset
```

> **Note**: You may need to do some [system tuning](https://docs.solana.com/running-validator/validator-start#system-tuning) to get the validator to run.

View on-chain program logs in a separate terminal:

```bash
solana logs
```

> **Note**: Use `msg!` [macro](https://docs.solana.com/developing/on-chain-programs/developing-rust#logging) for logging inside the on-chain program.

## Build the On-Chain Program

```bash
cd program
cargo build-bpf
```

## Deploy the On-Chain Program Locally

```bash
solana program deploy target/deploy/program.so
```

## Run the Rust Client

```bash
cargo run
```

### Expected Output

```
Connecting to cluster...http://localhost:8899
Connection to cluster established
Cluster node solana version 1.10.5
Counter account B6rWFbQ4pmb4pvcZstFCjLXffZSaqqn6c8fdXzpK3WSX already exists. Owner program: HGsPi7r4MEeUSC74vzx9qCqJvuuBb3AcjNc5MrtEjCGu
Binary address 8cRrhLjJ7sSbSa1kuaShq2Ywu1otyRhkNwTQ3E1Bqr4T
Fee for message 5000
Counter value 1
```

Values will differ!

> **Not seeing the expected output?** Ensure you've started the local cluster, built the on-chain program, and deployed it.

## Deploy to Devnet

```bash
solana config set --url d
solana program deploy target/deploy/program.so
```

## Deploy to Testnet

```bash
solana config set -ut
solana program deploy target/deploy/program.so
```

> **Note**: You may not have required SOL balance. Airdrop SOL into your account:

```bash
solana balance
solana airdrop 1
```
