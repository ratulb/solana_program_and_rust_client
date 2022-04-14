<p align="center">
  <a href="https://solana.com">
    <img alt="Solana" src="https://i.imgur.com/uBVzyX3.png" width="250" />
  </a>
</p>

[![Build status][travis-image]][travis-url] [![Gitpod
Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/solana-labs/example-helloworld)

[travis-image]:
https://travis-ci.org/solana-labs/example-helloworld.svg?branch=master
[travis-url]: https://travis-ci.org/solana-labs/example-helloworld

# Rust counter program and client on solana

This project demonstrates how to use solana rust APIs to write a counter program and client in rust.

The project comprises of:

* An on-chain counter program
* A rust client that can send a "Increament" counter message to the on-chain program and get the current reading.

## Table of Contents
- [Counter program and client](#counter_program_and_client)
  - [Table of Contents](#table-of-contents)
  - [Quick Start](#quick-start)
    - [Configure CLI](#configure-cli)
    - [Start local Solana cluster](#start-local-solana-cluster)
    - [Build the on-chain program](#build-the-on-chain-program)
    - [Deploy the on-chain program locally](#deploy-the-on-chain-program-locally)
    - [Deploy to devnet](#deploy-to-devnet)
    - [Deploy to testnet](#deploy-to-testnet)
    - [Run the rust client](#run-the-rust-client)
    - [Expected output](#expected-output)
      - [Not seeing the expected output?](#not-seeing-the-expected-output)
    - [Project structure](#project-structure)

  - [More about the client](#more-about-the-client)
    - [Entrypoint](#entrypoint)
    - [Establish a connection to the cluster](#establish-a-connection-to-the-cluster)
    - [Load the helloworld on-chain program if not already loaded](#load-the-helloworld-on-chain-program-if-not-already-loaded)
    - [Send a "Hello" transaction to the on-chain program](#send-a-hello-transaction-to-the-on-chain-program)
    - [Query the Solana account used in the Hello transaction](#query-the-solana-account-used-in-the-hello-transaction)
  - [More about the on-chain program](#more-about-the-on-chain-program)
    - [Programming on Solana](#programming-on-Solana)
  - [Pointing to a public Solana cluster](#pointing-to-a-public-solana-cluster)
  - [Expand your skills with advanced examples](#expand-your-skills-with-advanced-examples)

## Quick Start

[![Open in
Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/solana-labs/example-helloworld)

If you decide to open in Gitpod then refer to
[README-gitpod.md](README-gitpod.md), otherwise continue reading.

The following dependencies are required to build and run this example:

- Install Rust v1.60.0 or later from https://rustup.rs/
- Install Solana v1.10.5 or later from
  https://docs.solana.com/cli/install-solana-cli-tools

If this is your first time using Rust, these [Installation
Notes](README-installation-notes.md) might be helpful.

### Configure CLI

> If you're on Windows, it is recommended to use [WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) to run these commands

1. Set CLI config url to localhost cluster

```bash
solana config set --url localhost
or 
solana config set -ul

```

2. Create CLI Keypair

If this is your first time using the Solana CLI, you will need to generate a new keypair:

```bash
solana-keygen new
```

### Start local Solana cluster

This example connects to a local Solana cluster by default.

Start a local Solana cluster:
```bash
solana-test-validator
```
If you want start with a clean slate after couple of trials, you can do:

```bash
solana-test-validator --reset
```

> **Note**: You may need to do some [system tuning](https://docs.solana.com/running-validator/validator-start#system-tuning) (and restart your computer) to get the validator to run

On-chain deployed program's logs can be viewed by launcing a separate terminal and firing the following command:
```bash
solana logs
```
> **None**: For logging messages inside the on-chain program, we should use the `msg!` [macro](https://docs.solana.com/developing/on-chain-programs/developing-rust#logging).

### Build the on-chain program

Go inside the 'solana_counter_program' directory if not already done:

```bash
cd solana_counter_program

cargo build-bpf
```
### Deploy the on-chain program locally

```bash
solana program deploy target/deploy/program.so
```

### Run the rust client

```bash
cargo run
```

### Expected output

Values will differ!

```bash
Connecting to cluster...http://localhost:8899
Connection to cluster established
Cluster node solana version 1.10.5
Counter account B6rWFbQ4pmb4pvcZstFCjLXffZSaqqn6c8fdXzpK3WSX already exists. Owner program: HGsPi7r4MEeUSC74vzx9qCqJvuuBb3AcjNc5MrtEjCGu
Binary address 8cRrhLjJ7sSbSa1kuaShq2Ywu1otyRhkNwTQ3E1Bqr4T
Fee for message 5000
Counter value 1

```

#### Not seeing the expected output?

- Ensure you've [started the local cluster](#start-local-solana-cluster),
  [built the on-chain program](#build-the-on-chain-program) and [deployed the program to the cluster](#deploy-the-on-chain-program).

### Project structure

The following image shows the project layout. We are making use of cargo [workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).
- [program](https://github.com/ratulb/solana_counter_program/tree/main/program) - this is the on-chain counter program
- [client](https://github.com/ratulb/solana_counter_program/tree/main/client) - this is the rust client program that invokes the program to increament the counter.
- [common](https://github.com/ratulb/solana_counter_program/tree/main/common) - this crate contains the enum/structs shared by both program and client.

<p align="left">
  <a href="#project_structure">
    <img alt="Project structure" src="solana_counter_program.png" width="350" height="100"/>
  </a>
</p>

For experimentation, tweaking files under the program folder would require [rebuild](#build-the-on-chain-program) and [redeployment](#deploy-the-on-chain-program-locally).

Now when you rerun `cargo run`, you should see the results of your changes.

### Deploy to devnet

```bash
solana config set --url d
solana program deploy target/deploy/program.so
```
#### Or
### Deploy to testnet

```bash
solana config set -ut

solana program deploy target/deploy/program.so
```

> **Note**: You may not have required sol balance to deploy and run transactions in devnet or testnet. To request sol into your account do an airdrop:

#### Check account sol balance:
```bash
solana balance
```
#### Request sol airdrop:
```bash
solana airdrop 1
```

#### Run the client:
```bash
cargo run
```
#### Output when connected to devnet:
Connecting to cluster...https://api.devnet.solana.com
Connection to cluster established
Cluster node solana version 1.9.15
Counter account does not exist AccountNotFound: pubkey=3MMqqJqpQkTR7rXotYBrZoYear33cQ6Mgdmb5omjh5s7. Would create
Freestay lamports : 946560
Fee for message 5000
Total amount for transaction 951560 lamports
Binary address HnkMPHhde9UCq3bRGLFs26HnZ42r1yRVHn5jgyiDGvxq
Fee for message 5000
Counter value 1


## More about the client

The client is a rust cli [program](https://github.com/ratulb/solana_counter_program/blob/main/client/src/main.rs) with a main function.

### Main function

The [main function](https://github.com/ratulb/solana_counter_program/blob/27d5aa6aa4c7ec14fe049837e8beff4bb7548f3e/client/src/main.rs#L5) does following five things:


### Instantiates the client that wraps up an underlying RpcClient

[Client](https://github.com/ratulb/solana_counter_program/blob/main/client/src/client.rs)
creates an instance of [RpcClient](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html) in its [get_rpc_client](https://github.com/ratulb/solana_counter_program/blob/8ca2bd8130d0f385d2720b2623abf7f0965e0566/client/src/client.rs#L48) methhod. This sets up a Http client to the solana network that is picked up from `~/.config/solana/cli/config.yml`. Once the client has been setup - we can start interacting with solana network for things like querying about accounts, sending transactions, getting cluster related information and many more. The exhaustive list can be found [here](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#implementations).

The `json_rpc_url` entry in the `config.yaml` file gets configured via the following command:
```bash
solana config set --url localhost[devnet, testnet etc]
```

### Setup an account to store counter program's state

The client ensures there is an account available to pay for transactions,
and creates one if there is not, by calling
[`establishPayer`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L102).

### Check if the helloworld on-chain program has been deployed

In [`checkProgram`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L144),
the client loads the keypair of the deployed program from `./dist/program/helloworld-keypair.json` and uses
the public key for the keypair to fetch the program account. If the program doesn't exist, the client halts
with an error. If the program does exist, it will create a new account with the program assigned as its owner
to store program state (number of hello's processed).

### Send a "Hello" transaction to the on-chain program

The client then constructs and sends a "Hello" transaction to the program by
calling
[`sayHello`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L209).
The transaction contains a single very simple instruction that primarily carries
the public key of the helloworld program account to call and the "greeter"
account to which the client wishes to say "Hello" to.

### Query the Solana account used in the "Hello" transaction

Each time the client says "Hello" to an account, the program increments a
numerical count in the "greeter" account's data.  The client queries the
"greeter" account's data to discover the current number of times the account has
been greeted by calling
[`reportGreetings`](https://github.com/solana-labs/example-helloworld/blob/ad52dc719cdc96d45ad8e308e8759abf4792b667/src/client/hello_world.ts#L226).

## More about the on-chain program

The [on-chain counter program](program/Cargo.toml) is a Rust program
compiled to [Berkeley Packet Filter
(BPF)](https://en.wikipedia.org/wiki/Berkeley_Packet_Filter) bytecode and stored as an
[Executable and Linkable Format (ELF) shared
object](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format).

The program is written using:
- [Solana Rust SDK](https://github.com/solana-labs/solana/tree/master/sdk)

### Programming on Solana

To learn more about Solana programming model refer to the [Programming Model
Overview](https://docs.solana.com/developing/programming-model/overview).

To learn more about developing programs on Solana refer to the [On-Chain
Programs Overview](https://docs.solana.com/developing/on-chain-programs/overview)

## Pointing to a public Solana cluster

Solana maintains three public clusters:
- `devnet` - Development cluster with airdrops enabled
- `testnet` - Tour De Sol test cluster without airdrops enabled
- `mainnet-beta` -  Main cluster

Use the Solana CLI to configure which cluster to connect to.

To point to `devnet`:
```bash
solana config set --url devnet
```

To point back to the local cluster:
```bash
solana config set --url localhost
```

## Writing the client in Rust

This example details writing the client code in typescript; however
the Solana client program can be written in any language. For an
example client written in Rust and an accompanying write up see [this
repo](https://github.com/ezekiiel/simple-solana-program).

## Expand your skills with advanced examples

There is lots more to learn; The following examples demonstrate more advanced
features like custom errors, advanced account handling, suggestions for data
serialization, benchmarking, etc...

- [Programming
  Examples](https://github.com/solana-labs/solana-program-library/tree/master/examples)
- [Token
  Program](https://github.com/solana-labs/solana-program-library/tree/master/token)
- [Token Swap
  Program](https://github.com/solana-labs/solana-program-library/tree/master/token-swap)
