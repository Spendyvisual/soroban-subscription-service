# Public Contract ABI Reference

**Contract:** `soroban-subscription`  
**Phase:** 1.0.0

This document is the authoritative reference for all public functions exposed by the
Soroban Subscription Service contract. Any integrator can rely on this ABI without
reading the implementation.

---

## Admin Functions

### `initialize`
One-time setup. Must be called before any other function.

| Parameter | Type | Description |
|---|---|---|
| `admin` | `Address` | Controls plans, fees, and pausing |
| `provider` | `Address` | Receives net subscription revenue |
| `keeper_fee_bps` | `u32` | Relayer fee in basis points (0–500) |

**Returns:** `Result<(), SubscriptionError>`  
**Errors:** `AlreadyInitialized`, `InvalidKeeperFee`

---

### `pause`
Blocks new subscriptions and charges. Admin only.  
**Returns:** `Result<(), SubscriptionError>`

### `unpause`
Resumes normal operation. Admin only.  
**Returns:** `Result<(), SubscriptionError>`

### `set_keeper_fee`
Update the relayer fee. Admin only.

| Parameter | Type | Description |
|---|---|---|
| `bps` | `u32` | New fee in basis points (0–500) |

**Returns:** `Result<(), SubscriptionError>`  
**Errors:** `InvalidKeeperFee`

### `transfer_admin`
Hand the admin role to another address. Current admin must auth.

| Parameter | Type | Description |
|---|---|---|
| `new_admin` | `Address` | New admin address |

**Returns:** `Result<(), SubscriptionError>`

---

## Plan Functions

### `create_plan`
Create a new subscription plan. Admin only.

| Parameter | Type | Description |
|---|---|---|
| `name` | `String` | Human-readable plan name |
| `price_amount` | `i128` | Amount per interval (smallest token unit) |
| `price_asset` | `Address` | SAC address (XLM or USDC) |
| `interval_secs` | `u64` | Billing interval in seconds |
| `grace_period_secs` | `u64` | Grace window after missed payment |

**Returns:** `Result<u32, SubscriptionError>` — the new plan ID  
**Errors:** `InvalidPriceAmount`, `InvalidInterval`

### `update_plan`
Update mutable plan fields. Admin only. Existing subscribers grandfathered.

| Parameter | Type | Description |
|---|---|---|
| `plan_id` | `u32` | Plan to update |
| `name` | `Option<String>` | New name (or None to keep current) |
| `price_amount` | `Option<i128>` | New price |
| `interval_secs` | `Option<u64>` | New interval |
| `grace_period_secs` | `Option<u64>` | New grace period |

**Returns:** `Result<(), SubscriptionError>`

### `deactivate_plan`
Stop accepting new subscribers. Existing subscriptions continue.

| Parameter | Type | Description |
|---|---|---|
| `plan_id` | `u32` | Plan to deactivate |

**Returns:** `Result<(), SubscriptionError>`

### `get_plan`

| Parameter | Type |
|---|---|
| `plan_id` | `u32` |

**Returns:** `Result<Plan, SubscriptionError>`

---

## Subscription Functions

### `subscribe`
Subscribe to a plan. Subscriber must pre-approve SAC allowance.

| Parameter | Type | Description |
|---|---|---|
| `subscriber` | `Address` | The subscribing wallet |
| `plan_id` | `u32` | Plan to subscribe to |

**Returns:** `Result<u32, SubscriptionError>` — the new subscription ID  
**Errors:** `PlanNotFound`, `PlanInactive`, `ContractPaused`

### `cancel`
Cancel an active subscription. Subscriber must auth.

| Parameter | Type |
|---|---|
| `subscription_id` | `u32` |

**Returns:** `Result<(), SubscriptionError>`  
**Errors:** `AlreadyCancelled`, `SubscriptionNotFound`

### `change_plan`
Queue a plan switch effective at the next billing date. Subscriber must auth.

| Parameter | Type | Description |
|---|---|---|
| `subscription_id` | `u32` | Subscription to modify |
| `new_plan_id` | `u32` | Target plan |

**Returns:** `Result<(), SubscriptionError>`

### `get_subscription`

| Parameter | Type |
|---|---|
| `subscription_id` | `u32` |

**Returns:** `Result<Subscription, SubscriptionError>`

### `get_subscriptions_for`
Return all subscription IDs for a given address.

| Parameter | Type |
|---|---|
| `subscriber` | `Address` |

**Returns:** `Vec<u32>`

---

## Billing Functions

### `charge_subscriber`
Permissionless. Any account can call this; the keeper earns the keeper fee.

| Parameter | Type | Description |
|---|---|---|
| `subscription_id` | `u32` | Subscription to charge |
| `keeper` | `Address` | Address that receives the keeper fee |

**Returns:** `Result<ChargeReceipt, SubscriptionError>`  
**Errors:** `NotDue`, `GracePeriodExpired`, `AlreadyCancelled`, `AllowanceRevoked`, `ContractPaused`

### `batch_charge`
Charge up to 50 subscriptions in a single transaction. Per-item failures are skipped.

| Parameter | Type |
|---|---|
| `subscription_ids` | `Vec<u32>` |
| `keeper` | `Address` |

**Returns:** `Result<Vec<ChargeReceipt>, SubscriptionError>`

---

## Types

### `Plan`
```
id:                 u32
name:               String
price_amount:       i128
price_asset:        Address
interval_secs:      u64
grace_period_secs:  u64
active:             bool
```

### `Subscription`
```
id:                   u32
subscriber:           Address
plan_id:              u32
status:               SubscriptionStatus (Active | Cancelled | PastDue)
created_at:           u64
next_billing_ts:      u64
pending_plan_change:  Option<u32>
```

### `ChargeReceipt`
```
subscription_id:  u32
plan_id:          u32
amount_charged:   i128
keeper_fee:       i128
net_to_provider:  i128
timestamp:        u64
```

---

## Error Codes

| Code | Name | Meaning |
|---|---|---|
| 1 | AlreadyInitialized | initialize() called twice |
| 2 | NotInitialized | Contract not yet initialized |
| 3 | Unauthorized | Missing required auth |
| 4 | PlanNotFound | Plan ID does not exist |
| 5 | PlanInactive | Plan not accepting subscribers |
| 6 | SubscriptionNotFound | Subscription ID does not exist |
| 7 | AlreadyCancelled | Subscription already cancelled |
| 8 | NotDue | Billing date not yet reached |
| 9 | GracePeriodExpired | Missed payment beyond grace window |
| 10 | AllowanceRevoked | SAC allowance insufficient |
| 11 | ContractPaused | Contract is paused |
| 12 | InvalidKeeperFee | Fee exceeds 500 bps max |
| 13 | InvalidInterval | Interval must be > 0 |
| 14 | InvalidPriceAmount | Price must be > 0 |
| 15 | InvalidBatchSize | Batch must be 1–50 items |
