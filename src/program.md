# More About the On-Chain Program

To write an on-chain Solana program, follow these steps:

## 1. Implement the Entrypoint Function

Provide a function whose type signature matches [solana_program::entrypoint](https://github.com/solana-labs/solana/blob/f7d557d5ae5d2ebfb70c2eaefa7dd1e2068b748c/sdk/program/src/entrypoint.rs#L25-L26):

```rust
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult
```

- `program_id` — the program's pubkey (can be changed using `solana-keygen grind` to generate a vanity keypair, passed as `--program-id` during deployment).
- `accounts` — consolidated list of all accounts that instructions read and/or write to. They appear in the order `AccountMeta` structs are added to each instruction and the order instructions are added to the containing message. All `AccountInfo` entries must have their writable/readable/signer attributes set. For example, the counter account is constructed with `AccountMeta::new` — writable but not a signer.
- `instruction_data` — raw byte array of data passed from the client.

## 2. Decorate with the Entrypoint Macro

Apply the [entrypoint macro](https://github.com/solana-labs/solana/blob/f7d557d5ae5d2ebfb70c2eaefa7dd1e2068b748c/sdk/program/src/entrypoint.rs#L116):

```rust
entrypoint!(process_instruction);
```

This embeds your implementation inside an external `C` function called `entrypoint` (annotated with `no_mangle` so the compiler keeps the name).

## 3. Define `no-entrypoint` Feature

During development you may depend on other crates that have their own entrypoints. Since there cannot be multiple entrypoints at runtime, declare the entrypoint feature in `program/Cargo.toml`:

```toml
[features]
no-entrypoint = []
```

If another crate wants to use your program's types without including the entrypoint, they add:

```toml
program = { version = "0.1.0", path = "../program", features = ["no-entrypoint"] }
```

Read more in the [Solana docs](https://docs.solana.com/developing/on-chain-programs/developing-rust#project-layout).

## 4. Build the Program

During `cargo build-bpf`, the program is compiled to [Berkeley Packet Filter (BPF)](https://en.wikipedia.org/wiki/Berkeley_Packet_Filter) bytecode and stored as an ELF shared object. A program keypair is also generated — its pubkey becomes the default `program_id`.

## 5. Deploy to the Network

The Solana CLI:
1. Breaks the compiled bytecode into smaller chunks (due to restricted transaction size)
2. Sends chunks to an intermediate on-chain buffer account in a series of transactions
3. Once transmission is complete and verified, a final transaction moves the buffered content to the program's data account

This completes a new deployment or a program upgrade. Transaction costs are deducted from the payer's account.

See [this post](https://jstarry.notion.site/Program-deploys-29780c48794c47308d5f138074dd9838) for more detail.

## State Management

Solana on-chain programs are **stateless** and compiled as shared libraries (`crate-type = ["cdylib"]`). They cannot maintain state across invocations. To persist state, programs use **accounts** that they own.

There is a limit of ~10 MB per account. Space incurs cost (rent). An account can be rent-exempt if it maintains at least two years' worth of rent as balance. On-chain programs are expected to be rent-exempt.

Calculate rent-exempt lamports:

```bash
solana rent 1000   # bytes
```

Or programmatically via [RpcClient::get_minimum_balance_for_rent_exemption](https://docs.rs/solana-client/latest/src/solana_client/rpc_client.rs.html#3531-3536).
