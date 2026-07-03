# Setup Counter Account

Solana on-chain programs are **stateless** — they cannot persist data between invocations. State is stored in **accounts** that the program owns.

> **Note**: Programs are also stored in accounts, marked as executable. Deploy with `solana program deploy` (upgradeable) vs `solana deploy` (final, uses BPFLoader).

## Account Setup Flow

### 1. Retrieve Payer Pubkey

The payer pubkey is derived from `~/.config/solana/id.json`. The program ID is derived from `./target/deploy/program-keypair.json`. If the program has not been built, setup fails immediately.

### 2. Derive Counter Account Address

The counter account pubkey is derived from the payer pubkey, a seed string, and the program ID (owner of the account). A query is made to the chain:

- **Account exists** → early exit, nothing to set up
- **Account does not exist** → proceed to create it

### 3. Calculate Rent-Exempt Balance

Calculate the minimum balance required for the counter account to stay rent-exempt based on its data size. The `Counter` struct:

```rust
struct Counter {
    count: u64,      // 8 bytes
}
```

It derives `BorshSerialize` and `BorshDeserialize` for serialization. A helper computes the struct size:

```rust
const COUNTER_SIZE: usize = std::mem::size_of::<u64>();
```

The rent exemption amount is fetched from the network:

```rust
let lamports = client.get_minimum_balance_for_rent_exemption(COUNTER_SIZE)?;
```

### 4. Construct Create Account Instruction

The system instruction for creating the counter account is constructed with the lamports amount, space, and owner (program ID).

### 5. Query Blockhash

The latest blockhash is retrieved from the network. This measures how recent the client's view of the network is and is used to accept/reject transactions.

### 6. Calculate Fee

The network is queried for the required fee for the transaction message — the cost of executing the transaction.

### 7. Airdrop Lamports

The total cost (rent exemption + transaction fee) is funded via an airdrop:

```rust
client.request_airdrop(&payer, total_lamports)?;
```

The airdrop can be skipped by setting the `skip_airdrop` environment variable to a non-empty value, or if the payer already has sufficient balance.

### 8. Send Transaction

The create-account transaction is sent to the network and a transaction signature is returned.
