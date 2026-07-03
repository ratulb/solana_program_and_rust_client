# Establish a Connection

The [Client](https://github.com/ratulb/solana_program_and_rust_client/blob/main/client/src/client.rs) creates an instance of [RpcClient](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html) in its `get_rpc_client` method.

This sets up an HTTP client to the Solana network, configured from `~/.config/solana/cli/config.yml`. Once established, the client can:

- Query accounts
- Send transactions
- Get cluster information
- And much more (see the [RpcClient docs](https://docs.rs/solana-client/latest/solana_client/rpc_client/struct.RpcClient.html#implementations))

The `json_rpc_url` entry in `~/.config/solana/cli/config.yml` is set via:

```bash
solana config set --url localhost
```

Other options: `devnet`, `testnet`, `mainnet-beta`.
