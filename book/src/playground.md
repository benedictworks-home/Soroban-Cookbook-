# Playground Infrastructure

The Soroban Cookbook playground is the local Cargo workspace at the root of
this repository. There is no separate service to install — cloning the repo
gives you a working playground for every example in the book.

## Framework

The playground is a standard Rust/Cargo workspace (see the root `Cargo.toml`).
Every example lives under `examples/` as its own workspace member, so you can
build, edit, and test any single example without touching the others.

## Soroban SDK Integration

The `soroban-sdk` version is pinned once, at the workspace level:

```toml
[workspace.dependencies]
soroban-sdk = "26.0.0-rc.1"
```

Each example crate depends on `soroban-sdk.workspace = true`, so every
contract in the playground builds against the same SDK version.

## Build System

```bash
# Add the Wasm target once
rustup target add wasm32-unknown-unknown

# Build a single example
cargo build --manifest-path examples/basics/01-hello-world/Cargo.toml \
            --target wasm32-unknown-unknown --release

# Build every workspace member
cargo build --workspace --target wasm32-unknown-unknown --release
```

## Testing Capability

```bash
# Test a single example
cargo test --manifest-path examples/basics/01-hello-world/Cargo.toml

# Test the full workspace
cargo test --workspace
```

Cross-example integration and security tests live in `tests/integration`
and `tests/security`, and are included as workspace members so
`cargo test --workspace` runs them alongside the individual examples.

## Documentation

- [Interactive Playground — Basic Examples](./examples/playground.md) walks
  through all 14 basics examples with runnable snippets.
- [Setup Environment](./guides/getting-started.md) covers installing Rust,
  the Wasm target, and the Soroban CLI from scratch.
- [Testing Guide](./guides/testing.md) covers mocking auth, time
  manipulation, and event assertions when writing your own tests.
