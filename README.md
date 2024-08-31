## Substreams Solana Tokens (showcase)

This showcase extractions of SPL Token `Transfer` instruction and storing them in a Clickhouse database. This can be used as a base for future improvements.

First ensure you have Docker Compose environment up and running:

```bash
docker compose up
```

And then:

```bash
export DSN="clickhouse://default:@localhost:9000/default"

substreams-sink-sql setup "$DSN" tokens/solana-tokens-v0.1.0.spkg
substreams-sink-sql sink --undo-buffer-size 12 "$DSN" tokens/solana-tokens-v0.1.0.spkg "200_000_000:201_000_000"
```

### From Source

```bash
cargo build --target wasm32-unknown-unknown --release

export DSN="clickhouse://default:@localhost:9000/default"

substreams-sink-sql setup "$DSN" tokens/substreams.yaml
substreams-sink-sql sink --undo-buffer-size 12 "$DSN" tokens/substreams.yaml "200_000_000:201_000_000"
```