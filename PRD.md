# PRD — Soroban Subscription Service

**Status:** Draft v1.0  
**Owner:** broda-spendy  
**Program:** Stellar Wave Program  
**Created:** 2026-08-14

---

## 1. Problem Statement

The global SaaS market exceeds $700B in annual recurring revenue. Virtually all of it is
processed through centralized payment rails (Stripe, PayPal, card networks) that charge 2–4%
fees, exclude the unbanked, and create settlement delays of 2–7 days.

Stellar's Soroban smart contract platform is purpose-built for fast, low-cost programmable
payments. However, there is currently **no native, standardized, reusable primitive for
recurring subscription payments on Soroban**. SaaS providers who want to accept XLM or USDC
on Stellar must either build custom one-shot invoicing logic or rely on off-chain
automation—there is no pull-payment factory they can integrate in hours.

This gap leaves Soroban without one of the most fundamental DeFi and commerce primitives:
**subscriptions**.

---

## 2. Goal

Build a **Soroban smart contract framework** ("Subscription Contract Factory") that allows
SaaS providers to:

1. Define subscription plans (price, interval, asset: XLM or USDC).
2. Accept subscriber sign-ups and hold pre-authorized payment allowances.
3. Trigger recurring charges on-chain via a relayer at the defined interval.
4. Allow subscribers to cancel, upgrade, or downgrade plans.
5. Expose a clean, auditable, on-chain billing history.

The framework must be **modular and reusable**: any SaaS provider deploys their own
instance of the factory contract and configures it without modifying contract code.

---

## 3. Non-Goals (all phases)

- We are not building a fiat on/off-ramp.
- We are not building a full SaaS platform dashboard (UI scope is a reference integration).
- We are not implementing slashing, penalties, or credit-scoring.
- We are not targeting mainnet deployment until Phase 5. All Phases 1-4 target testnet.
- We are not building a generic payment channel; scope is strictly recurring pull payments.

---

## 4. Users & Personas

| Persona | Need |
|---|---|
| **SaaS Provider (Merchant)** | Deploy a subscription contract, define plans, receive periodic payments |
| **Subscriber (End User)** | Sign up for a plan, have subsequent charges happen automatically |
| **Relayer / Keeper** | Off-chain bot that submits charge_subscriber at each billing interval |
| **Developer / Integrator** | Use the contract ABI and SDK to embed subscriptions in their dApp |

---

## 5. Core User Stories

1. As a SaaS provider, I can deploy a Subscription Contract instance with my wallet as admin.
2. As a subscriber, I can authorize the contract to debit my wallet at each interval.
3. As a subscriber, I can cancel my subscription at any time.
4. As a subscriber, I can upgrade or downgrade my plan; billing adjusts at the next interval.
5. As a relayer, I can call charge_subscriber for any active subscriber and earn a keeper fee.
6. As a developer, I can integrate subscription billing without modifying contract source.

---

## 6. Functional Requirements

### 6.1 Plan Management
- create_plan(name, price_amount, price_asset, interval_secs, grace_period_secs) => plan_id
- update_plan(plan_id, ...) => Result  (admin only)
- deactivate_plan(plan_id)  (admin only)
- get_plan(plan_id) => Plan

### 6.2 Subscription Lifecycle
- subscribe(subscriber: Address, plan_id: u32) => subscription_id
- cancel(subscription_id)
- change_plan(subscription_id, new_plan_id)
- get_subscription(subscription_id) => Subscription
- get_active_subscriptions_for(address: Address) => Vec<subscription_id>

### 6.3 Billing / Charging
- charge_subscriber(subscription_id) => Result<ChargeReceipt, SubscriptionError>
- batch_charge(subscription_ids: Vec<u32>) => Vec<Result<...>>

### 6.4 Admin
- initialize(admin: Address, provider: Address, keeper_fee_bps: u32)
- set_keeper_fee(bps: u32)
- transfer_admin(new_admin: Address)
- pause() / unpause()

---

## 7. Phased Delivery

### Phase 1 — Contract Core & Scaffolding (THIS PHASE)
Full Soroban contract with plan management, subscription lifecycle, and simulated billing.
CI on every push. No frontend.

**Definition of Done:** cargo build and cargo test pass in CI; all ABI documented.

### Phase 2 — Relayer & Keeper Bot
Off-chain Node.js relayer that polls Soroban RPC for due subscriptions.

### Phase 3 — USDC & Multi-Asset Support
Integrate Stellar USDC SAC. Allow plans denominated in any SAC.

### Phase 4 — Frontend dApp
React + Freighter wallet integration. Provider dashboard and subscriber portal.

### Phase 5 — Hardening, Audit & Testnet Deployment
Security review, resource optimization, formal audit, full testnet deployment.

### Phase 6 — Mainnet & SDK
Mainnet deployment, TypeScript SDK, partner integrations.

---

## 8. Risks

| Risk | Mitigation |
|---|---|
| Soroban TTL expiry silently deactivates subscriptions | TTL extended on every persistent write |
| No native pull-payment primitive | Use approve/transfer_from on SAC like ERC-20 allowances |
| Relayer downtime causes missed billing | Grace period field; relayer is stateless and restartable |
| Subscriber revokes allowance mid-period | charge_subscriber returns AllowanceRevoked; moves to PastDue |
| Keeper fee abuse | Hard cap at 500 bps enforced in contract |
