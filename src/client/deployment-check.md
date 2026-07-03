# Check Deployment

Before sending transactions, the client verifies the on-chain program is deployed.

## Verification Flow

### 1. Check Program Keypair

The client looks for the program keypair at `./target/deploy/program-keypair.json`. This file is generated during `cargo build-bpf`. If not found, the client exits with an error.

### 2. Retrieve Program Account

The program account is retrieved using the pubkey from the keypair. The client checks two things:

- **The account exists** on-chain
- **The account is executable**

### 3. Handle Upgradable vs BPF Loader

This check alone is not sufficient for programs owned by the **upgradable BPF loader** (`BPFLoaderUpgradeab1e11111111111111111111111`). When a program is closed via `solana program close`, the program data account is wiped, but the program account still reports as executable.

The client handles this by also checking for the **program data account** where actual bytecodes are stored:

![Program account ownership chain](./images/entrypoint.png)

The blue line (BPF Loader) does not allow closing a deployed program. The red underline (Upgradeable BPF Loader) — we query the program data account; if it's empty, the program was closed even though the program account says executable.
