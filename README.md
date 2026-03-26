# spl-forge

Simple CLI to create and manage SPL assets on Solana.

## What It Does

- Setup and manage local CLI config
- Create mint accounts
- Create token/NFT flows (market/pool/launch planned)

## Quick Start

Build and run:

```bash
cargo check
spl-forge help
```

On first run, config is auto-created under `~/.config/spl-forge/`.

## Command Usage

General form:

```bash
spl-forge <command> <subcommand> [flags]
```

## Commands

### `help`

```bash
spl-forge help
```

### `config`

Get current config:

```bash
spl-forge config get
```

Quick network switch (recommended):

```bash
spl-forge config set devnet
spl-forge config set localhost
spl-forge config set mainnet
```

These presets update RPC URL, WebSocket URL, and commitment together.

Set custom RPC URL:

```bash
spl-forge config set --url https://api.devnet.solana.com
```

Use Solana CLI wallet keypair:

```bash
spl-forge config set --keypair solana-cli
```

Reset config:

```bash
spl-forge config reset
```

### `wallet`

Show active wallet address:

```bash
spl-forge wallet address
```

Show active wallet balance:

```bash
spl-forge wallet balance
```

Show balance for any wallet:

```bash
spl-forge wallet balance --address <PUBKEY>
```

Show full wallet status:

```bash
spl-forge wallet status
```

Request SOL airdrop (devnet/localnet only):

```bash
spl-forge wallet airdrop 2
```

### `create mint`

```bash
spl-forge create mint \
  --mint-authority <PUBKEY> \
  --decimals <DECIMALS> \
  --initial-supply <AMOUNT> \
  [--freeze-authority <PUBKEY>]
```

### `create metadata`

```bash
spl-forge create metadata \
  --mint-address <MINT_PUBKEY> \
  --name "My Token" \
  --symbol "MTK" \
  --uri "https://example.com/metadata.json" \
  --immutable
```

### `create token`

```bash
spl-forge create token \
  --name "My Token" \
  --symbol "MTK" \
  --decimals <DECIMALS> \
  --initial-supply <AMOUNT> \
  --uri "https://example.com/metadata.json" \
  [--freeze-authority <PUBKEY>] \
  [--immutable]
```

### `create nft`

```bash
spl-forge create nft \
  --name "My NFT" \
  --symbol "MNFT" \
  --uri "https://example.com/nft.json" \
  [--freeze-authority <PUBKEY>] \
  [--collection-mint <PUBKEY>] \
  [--immutable]
```

### Planned (not implemented yet)

- `create market`
- `create pool`
- `create launch`

## Testing

Quick compile check:

```bash
cargo check
```

Run normal smoke tests:

```bash
cargo test --test commands_smoke -- --nocapture
```

Run aggressive localnet sweep:

```bash
cargo test --test commands_localnet -- --nocapture
```

Notes:

- Localnet tests expect `solana-test-validator` on `127.0.0.1:8899`.
- `commands_smoke` skips localnet-only create checks if validator is not reachable.
- `create metadata`, `create token`, and `create nft` are currently expected to fail in tests until metadata support is re-enabled.
