# Query the Counter Account

Each time the client runs, it increments the counter value stored in the counter account owned by the on-chain program.

## Reading the Counter Value

1. The counter account is loaded by its derived pubkey
2. The `data` field of the account is deserialized into a `Counter` struct using Borsh
3. The `count` field is printed

```rust
let account = client.get_account(&counter_address)?;
let counter = Counter::try_from_slice(&account.data)?;
println!("Counter value {}", counter.count);
```

Every execution of `cargo run` increments the counter by 1.

## Experiment: Concurrent Increments

If you start the validator in a clean state and run:

```bash
for _ in {0..99}; do cargo run; done
```

from **two terminals simultaneously** — the counter value should be **200**. But it won't be!

Why? Check all the entries in `~/.config/solana/cli/config.yml` for the answer.

## Using `solana deploy` Instead

If you deploy with `solana deploy program.so` instead of `solana program deploy`:

- The program is owned by the BPF loader (not upgradeable)
- A randomly generated program ID is assigned
- Pass it to the client via the `program_id` environment variable:

```bash
program_id=<generated_id> cargo run
```
