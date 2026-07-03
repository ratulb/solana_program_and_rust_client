# Send a Counter Increment Transaction

The client submits a transaction to the on-chain counter program to increment the counter value stored in the owned account.

## How Instructions Work

To invoke a Solana on-chain program, send a **Transaction** containing a **Message**, which encapsulates one or more **Instructions**.

Each instruction has a `data` field — a `Vec<u8>` of program-specific bytes. The Solana runtime is **agnostic** about the data format, but provides APIs for constructing instructions from:
- [Borsh](https://borsh.io/) — preferred, has a stable specification
- [Bincode](https://github.com/bincode-org/bincode) — has a published spec but noted as computationally expensive

You can also invent your own serialization and use the [low-level API](https://github.com/solana-labs/solana/blob/d71986cecf062e2bbbe291e018bf0a4c33e192a5/sdk/program/src/instruction.rs#L492).

## In This Project

The client uses Borsh serialization. The custom data is an enum:

```rust
enum CounterInstruction {
    Increment,
}
```

This enum is defined in the `common` crate and is shared between the client (for serialization) and the program (for deserialization).

### Instruction Construction

```rust
let instruction = Instruction {
    program_id: counter_program_id,
    accounts: vec![
        AccountMeta::new(counter_account, false),  // writable, not signer
    ],
    data: CounterInstruction::Increment.try_to_vec()?,
};
```

### Passing Accounts

Accounts that the program reads or modifies during execution must be passed in `AccountMeta` structs:

- `AccountMeta::new(pubkey, is_signer)` — writable account
- `AccountMeta::new_readonly(pubkey, is_signer)` — read-only account

Passing accounts lets the Solana runtime parallelize transactions.

> **Try this**: Comment out the single instruction and uncomment the line that clones the instruction and packs it twice. What happens when you send two increment instructions in one transaction?

### Transaction Submission

The usual steps:
1. Load payer keypair and program ID
2. Query latest blockhash
3. Calculate fee for message
4. Create the transaction with the instruction(s)
5. Sign and send
