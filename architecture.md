# Architecture — Soroban Subscription Service

**Status:** v1.0 — Phase 1 Scaffolding  
**Updated:** 2026-08-14

---

## 1. High-Level Overview

```
  SaaS Provider (Admin)          Subscriber (End User)          Relayer / Keeper Bot
         |                              |                               |
         | initialize()                 | subscribe(plan_id)            | charge_subscriber(id)
         | create_plan()                | cancel()                      | batch_charge([ids])
         |                              | change_plan()                 |
         v                              v                               v
 +-------------------------------------------------------------------------+
 |                   Soroban Smart Contract (contracts/subscription)        |
 |                                                                         |
 |  +-------------------+   +--------------------+   +------------------+  |
 |  |   Admin Module     |   |   Plan Module       |   |  Subscription    |  |
 |  |  initialize()      |   |  create_plan()      |   |  Module          |  |
 |  |  pause/unpause()   |   |  update_plan()      |   |  subscribe()     |  |
 |  |  set_keeper_fee()  |   |  deactivate_plan()  |   |  cancel()        |  |
 |  |  transfer_admin()  |   |  get_plan()         |   |  change_plan()   |  |
 |  +-------------------+   +--------------------+   +------------------+  |
 |                                                                         |
 |  +-------------------+   +--------------------+   +------------------+  |
 |  |  Billing Module    |   |   Storage Module    |   |  Events Module   |  |
 |  |  charge_subscriber |   |  Instance storage   |   |  Charged         |  |
 |  |  batch_charge()    |   |  Persistent storage |   |  Subscribed      |  |
 |  |  verify_due()      |   |  TTL management     |   |  Cancelled       |  |
 |  +-------------------+   +--------------------+   +------------------+  |
 |                                                                         |
 +-------------------------------------------------------------------------+
                                    |
                                    | Stellar Asset Contract (SAC)
                                    | transfer_from(subscriber -> provider)
                                    v
                     +-----------------------------+
                     |  Stellar Network (testnet)  |
                     +-----------------------------+
```

---

## 2. Repository Layout

```
soroban-subscription-service/
+-- PRD.md                          # Product Requirements Document
+-- architecture.md                 # This file
+-- README.md                       # Getting started guide
+-- CONTRIBUTING.md                 # Contribution guidelines
+-- Cargo.toml                      # Workspace root
+-- rustfmt.toml                    # Formatting config
+-- .github/
¦   +-- workflows/
¦       +-- ci.yml                  # CI: build + test on every push/PR
+-- contracts/
¦   +-- subscription/               # Core Soroban contract crate
¦       +-- Cargo.toml
¦       +-- src/
¦           +-- lib.rs              # Contract entrypoint, #[contract] impl
¦           +-- admin.rs            # initialize, pause, transfer_admin
¦           +-- plan.rs             # Plan CRUD, plan storage
¦           +-- subscription.rs     # subscribe, cancel, change_plan
¦           +-- billing.rs          # charge_subscriber, batch_charge
¦           +-- storage.rs          # StorageKey enum, TTL helpers
¦           +-- types.rs            # Plan, Subscription, ChargeReceipt types
¦           +-- errors.rs           # SubscriptionError enum
¦           +-- events.rs           # typed event helpers
¦           +-- test.rs             # Full unit test suite
+-- docs/
¦   +-- interface.md                # Public ABI reference (all function signatures)
¦   +-- threat-model.md             # Security assumptions & Phase 1 caveats
¦   +-- adr/
¦       +-- README.md               # ADR index and process
¦       +-- 001-storage-strategy.md # Why Instance vs Persistent
¦       +-- 002-pull-payment.md     # Why approve/transfer_from pattern
¦       +-- 003-keeper-model.md     # Relayer incentive design
+-- scripts/
¦   +-- test-charge.sh              # Shell script to simulate a charge cycle locally
+-- examples/
    +-- subscribe-flow.md           # Step-by-step walkthrough of full subscribe->charge->cancel
```

---

## 3. Contract Design (Phase 1)

### 3.1 Core Types (`types.rs`)

```rust
pub struct Plan {
    pub id: u32,
    pub name: String,
    pub price_amount: i128,
    pub price_asset: Address,   // SAC address (XLM or USDC)
    pub interval_secs: u64,     // e.g. 2_592_000 = 30 days
    pub grace_period_secs: u64, // e.g. 86_400 = 1 day
    pub active: bool,
}

pub struct Subscription {
    pub id: u32,
    pub subscriber: Address,
    pub plan_id: u32,
    pub status: SubscriptionStatus,  // Active | Cancelled | PastDue
    pub created_at: u64,             // ledger timestamp
    pub next_billing_ts: u64,        // when next charge is due
    pub pending_plan_change: Option<u32>, // plan_id effective next period
}

pub struct ChargeReceipt {
    pub subscription_id: u32,
    pub amount_charged: i128,
    pub keeper_fee: i128,
    pub timestamp: u64,
}

pub enum SubscriptionStatus {
    Active,
    Cancelled,
    PastDue,
}
```

### 3.2 Storage Strategy (`storage.rs`)

Following Soroban best practices:

| Data | Storage Type | Rationale |
|---|---|---|
| Contract config (admin, provider, keeper_fee_bps, paused) | **Instance** | Loaded with every invocation; small |
| Plan definitions | **Persistent** | Long-lived; TTL bumped on every plan write |
| Subscription records | **Persistent** | Long-lived per user; TTL bumped on charge |
| Plan counter, subscription counter | **Instance** | Monotonic; small |

TTL extension is performed on every write to a Persistent entry to prevent silent archival.

### 3.3 Admin Module (`admin.rs`)

- `initialize` is callable exactly once (guarded by checking for existing config).
- All privileged calls use `require_auth(&admin)`.
- `pause()` sets a boolean in Instance storage; all billing and subscribe calls check this guard.

### 3.4 Plan Module (`plan.rs`)

- Plans are stored as `StorageKey::Plan(plan_id)` in Persistent storage.
- `create_plan` increments a plan counter (Instance) and writes the new Plan.
- `deactivate_plan` marks `active = false`; existing subscriptions are NOT automatically cancelled.
- `update_plan` is admin-only and updates mutable fields; existing subscriber billing is not retroactively changed.

### 3.5 Subscription Module (`subscription.rs`)

- `subscribe` verifies the plan is active and not paused, creates a Subscription record with `next_billing_ts = now + interval_secs`, and emits `Subscribed` event.
- `cancel` requires `require_auth(&subscriber)` or admin; sets `status = Cancelled`.
- `change_plan` sets `pending_plan_change`; the change takes effect when `charge_subscriber` runs next.

### 3.6 Billing Module (`billing.rs`)

Core logic for Phase 1:

```
charge_subscriber(subscription_id):
  1. Load subscription — error if not found
  2. Check status == Active — error if Cancelled
  3. Check current_ts >= next_billing_ts — error if too early (NotDue)
  4. Check current_ts <= next_billing_ts + grace_period — if exceeded, mark PastDue, error
  5. Apply pending plan change if present
  6. Load plan — get price_amount, price_asset, interval
  7. Compute keeper_fee = price_amount * keeper_fee_bps / 10_000
  8. net_amount = price_amount - keeper_fee
  9. Call SAC.transfer_from(subscriber, provider, net_amount)   [Phase 1: simulated in tests]
 10. Call SAC.transfer_from(subscriber, invoker, keeper_fee)    [Phase 1: simulated in tests]
 11. Update subscription.next_billing_ts += interval_secs
 12. Bump persistent TTL for subscription
 13. Emit Charged event with ChargeReceipt
 14. Return Ok(ChargeReceipt)
```

**Phase 1 note:** The actual SAC `transfer_from` calls are present in the code but the tests mock the token client. The full flow is wired up so Phase 2 only requires deploying against a real SAC.

### 3.7 Events (`events.rs`)

All state transitions emit typed events:

| Event | Topics | Data |
|---|---|---|
| `Subscribed` | `["sub", subscriber, plan_id]` | `subscription_id, next_billing_ts` |
| `Charged` | `["charge", subscription_id]` | `ChargeReceipt` |
| `Cancelled` | `["cancel", subscription_id]` | `subscriber, timestamp` |
| `PlanChanged` | `["plan_change", subscription_id]` | `old_plan_id, new_plan_id` |
| `PlanCreated` | `["plan_new", plan_id]` | `Plan` |

### 3.8 Errors (`errors.rs`)

```rust
pub enum SubscriptionError {
    AlreadyInitialized   = 1,
    NotInitialized       = 2,
    Unauthorized         = 3,
    PlanNotFound         = 4,
    PlanInactive         = 5,
    SubscriptionNotFound = 6,
    AlreadyCancelled     = 7,
    NotDue               = 8,
    PastDue              = 9,
    AllowanceRevoked     = 10,
    ContractPaused       = 11,
    InvalidKeeperFee     = 12,
    InvalidInterval      = 13,
}
```

---

## 4. Security Model (Phase 1)

- All admin operations require `require_auth(&admin_address)`.
- Subscriber operations (`cancel`, `change_plan`) require `require_auth(&subscriber_address)`.
- `charge_subscriber` is permissionless (callable by any relayer) but the recipient is always the stored `provider` address — the keeper cannot redirect funds.
- Keeper fee is bounded at contract level (`max 500 bps`).
- The contract does not hold funds directly; it uses SAC `transfer_from` which requires the subscriber to have pre-approved an allowance.

---

## 5. Testing Strategy

- **Unit tests** (`test.rs`): full coverage of plan CRUD, subscribe/cancel/change flows, billing arithmetic, error paths, keeper fee edge cases.
- **Mock token client**: soroban_sdk test utilities mock the SAC so billing tests run without a live network.
- **CI** (`ci.yml`): `cargo build --workspace` + `cargo test --workspace` on every push and PR.

---

## 6. Architecture Decision Records

| ADR | Title | Decision |
|---|---|---|
| 001 | Storage Strategy | Instance for config, Persistent for plans/subscriptions |
| 002 | Pull Payment Pattern | Use SAC approve/transfer_from (ERC-20 equivalent) |
| 003 | Keeper Incentive Model | Permissionless relayers, fee taken from subscription payment |
