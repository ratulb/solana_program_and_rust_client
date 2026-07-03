# Installation Notes

If you are a first-time user of Rust, the notes below may help you to install some of the dependencies on a Mac or Linux workstation.

## Rust

Install Rust using `rustup`. Rustup will install the latest version of Rust, Cargo, and other binaries used in Solana.

> **Note**: If this is the first time installing Rust on Linux, you may need to install `build-essential`:
> ```bash
> sudo apt install build-essential -y
> ```

Follow the instructions at [Installing Rust](https://www.rust-lang.org/tools/install).

For Mac users, Homebrew is also an option:

```bash
brew install rustup
rustup-init
```

See [Mac Setup](https://sourabhbajaj.com/mac-setup/Rust/) for more details.

After installation, you should have `rustc`, `cargo`, and `rustup`. Ensure `~/.cargo/bin` is in your PATH.

## Clone the Repository

```bash
git clone https://github.com/ratulb/solana_program_and_rust_client.git
cd solana_program_and_rust_client
```

(If you plan to submit pull requests, fork the repository first and then clone your fork.)
