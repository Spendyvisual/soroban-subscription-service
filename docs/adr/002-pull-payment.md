# ADR 002 — Pull Payment Pattern via SAC

**Status:** Accepted  
**Date:** 2026-08-14

## Context
Soroban does not have a native "pull payment" or "stream" primitive.
Recurring payments require the contract to debit the subscriber at each interval.

## Decision
Use the Stellar Asset Contract (SAC) `approve` + `transfer_from` pattern,
identical in semantics to ERC-20 allowances on Ethereum.

Subscribers call `sac.approve(contract_address, allowance, expiry_ledger)` once.
The contract then calls `sac.transfer_from(contract, subscriber, provider, amount)` on each charge.

## Rationale
- Already available on Stellar mainnet via the Protocol 20 SAC.
- Subscriber retains full control: they can revoke the allowance at any time.
- The contract never holds subscriber funds in custody.

## Consequences
If the subscriber revokes their allowance, `charge_subscriber` returns `AllowanceRevoked`
and the subscription transitions to `PastDue`. This is a graceful failure — no funds are locked.
