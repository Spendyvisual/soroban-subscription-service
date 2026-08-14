# Contributing to Soroban Subscription Service

Thank you for contributing to the Stellar Wave Program! This guide covers everything you need to get started.

## Code of Conduct

Be respectful, inclusive, and constructive. We follow the [Contributor Covenant](https://www.contributor-covenant.org/).

## Development Setup

```bash
# Clone
git clone https://github.com/Spendyvisual/soroban-subscription-service
cd soroban-subscription-service

# Install Rust + wasm target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI (optional)
cargo install --locked stellar-cli --features opt

# Build
cargo build --workspace

# Test
cargo test --workspace

# Format
cargo fmt --all

# Lint
cargo clippy --workspace --all-targets -- -D warnings
```

## Workflow

1. Fork the repository.
2. Create a feature branch: `git checkout -b feat/my-feature`.
3. Make your changes, including tests.
4. Run `cargo test --workspace` � all tests must pass.
5. Run `cargo fmt --all` and `cargo clippy`.
6. Open a Pull Request against `main`.

## Pull Request Requirements

- [ ] All CI checks pass (build, test, clippy, fmt, audit).
- [ ] New logic is covered by unit tests in `test.rs`.
- [ ] Public ABI changes are reflected in `docs/interface.md`.
- [ ] Significant architectural changes have a corresponding ADR in `docs/adr/`.

## Issue Labels

| Label | Meaning |
|---|---|
| `phase-1` | Scope of current release |
| `phase-2` | Relayer/keeper bot |
| `phase-3` | Multi-asset support |
| `phase-4` | Frontend |
| `phase-5` | Audit/testnet |
| `phase-6` | Mainnet/SDK |
| `bug` | Something is broken |
| `enhancement` | New feature or improvement |
| `good first issue` | Good for newcomers |

## Questions?

Open a Discussion or join the Stellar Wave Program Discord.

