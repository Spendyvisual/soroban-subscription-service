# Soroban Subscription Service

> **Stellar Wave Program** � A reusable, deployable subscription billing primitive for SaaS providers on the Stellar network.

[![CI](https://github.com/Spendyvisual/soroban-subscription-service/actions/workflows/ci.yml/badge.svg)](https://github.com/Spendyvisual/soroban-subscription-service/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Stellar](https://img.shields.io/badge/Network-Stellar%20Testnet-blue)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/Runtime-Soroban-purple)](https://soroban.stellar.org)

---

## What is this?

The **Soroban Subscription Service** is a Soroban smart contract framework that allows any SaaS provider to accept **recurring payments in XLM or USDC** natively on the Stellar network � without writing custom settlement logic.

It provides the three primitives that make subscriptions work on-chain:

| Primitive | What it does |
|---|---|
| **Plan Factory** | Provider defines plans with price, interval, asset |
| **Subscriber Registry** | Users authorize the contract once; billing happens automatically |
| **Keeper/Relayer Engine** | Any off-chain bot can charge due subscriptions and earn a fee |

---

## Why Soroban?

- ? **Settlement in seconds** � no 2�7 day card network delays
- ?? **~0.001 XLM per transaction** � vs 2�4% card processing fees
- ?? **Global, permissionless** � works for anyone with a Stellar wallet
- ?? **Non-custodial** � the contract never holds user funds; it uses SAC `transfer_from`

---

## Phase 1 Scope (current)

This release implements the **full contract ABI** with:

- ? Plan CRUD (create, update, deactivate)
- ? Subscription lifecycle (subscribe, cancel, change_plan)
- ? Billing engine (charge_subscriber, batch_charge)
- ? Admin controls (pause, keeper_fee, transfer_admin)
- ? Typed on-chain events for every state change
- ? Full unit test suite (22 tests)
- ? CI: build + test + clippy + audit on every push

Phases 2�6 are tracked as GitHub Issues.

---

## Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Soroban / wasm32 target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI (optional, for testnet deploy)
cargo install --locked stellar-cli --features opt
```

### Build

```bash
git clone https://github.com/Spendyvisual/soroban-subscription-service
cd soroban-subscription-service

# Build for native (development)
cargo build --workspace

# Build for Soroban (wasm32)
cargo build --workspace --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test --workspace -- --nocapture
```

Expected output: all 22+ tests passing.

---

## Contract ABI

### Initialize

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_SECRET> \
  -- initialize \
  --admin <ADMIN_ADDRESS> \
  --provider <PROVIDER_ADDRESS> \
  --keeper_fee_bps 100
```

### Create a Plan

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_SECRET> \
  -- create_plan \
  --name "Pro Monthly" \
  --price_amount 10000000 \
  --price_asset <USDC_SAC_ADDRESS> \
  --interval_secs 2592000 \
  --grace_period_secs 86400
```

### Subscribe

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <SUBSCRIBER_SECRET> \
  -- subscribe \
  --subscriber <SUBSCRIBER_ADDRESS> \
  --plan_id 1
```

### Charge (as a Relayer)

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <KEEPER_SECRET> \
  -- charge_subscriber \
  --subscription_id 1 \
  --keeper <KEEPER_ADDRESS>
```

See [docs/interface.md](docs/interface.md) for the complete ABI reference.

---

## Repository Structure

```
soroban-subscription-service/
+-- PRD.md                     # Product Requirements Document
+-- architecture.md            # Full technical architecture
+-- contracts/subscription/    # Soroban contract (Rust)
�   +-- src/
�       +-- lib.rs             # Contract entrypoint
�       +-- admin.rs           # Admin functions
�       +-- plan.rs            # Plan management
�       +-- subscription.rs    # Subscription lifecycle
�       +-- billing.rs         # Charge engine
�       +-- types.rs           # Shared types
�       +-- errors.rs          # Error enum
�       +-- events.rs          # On-chain events
�       +-- storage.rs         # Storage keys & TTL helpers
�       +-- test.rs            # Unit tests
+-- docs/
�   +-- interface.md           # Full public ABI reference
�   +-- threat-model.md        # Security assumptions
�   +-- adr/                   # Architecture Decision Records
+-- .github/workflows/ci.yml   # CI pipeline
```

---

## Roadmap

| Phase | Status | Description |
|---|---|---|
| **Phase 1** | ? **Complete** | Contract core, tests, CI |
| **Phase 2** | ?? [Issue #2](../../issues/2) | Off-chain relayer/keeper bot |
| **Phase 3** | ?? [Issue #3](../../issues/3) | USDC + multi-asset support |
| **Phase 4** | ?? [Issue #4](../../issues/4) | React frontend + Freighter wallet |
| **Phase 5** | ?? [Issue #5](../../issues/5) | Security audit + testnet deploy |
| **Phase 6** | ?? [Issue #6](../../issues/6) | Mainnet + TypeScript SDK |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). All contributions must pass CI before merge.

---

## License

MIT � see [LICENSE](LICENSE).

---

## Stellar Wave Program

This project is built as part of the [Stellar Wave Program](https://github.com/Spendyvisual), an initiative to accelerate Stellar open-source ecosystem development through incentivized contribution cycles.

