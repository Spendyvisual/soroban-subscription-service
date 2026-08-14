# ADR 003 — Keeper/Relayer Incentive Design

**Status:** Accepted  
**Date:** 2026-08-14

## Context
Soroban contracts cannot execute autonomously; charging requires an external trigger.
We need an economic model that incentivizes external parties to submit charge transactions.

## Decision
`charge_subscriber` is permissionless: any account can call it.
The caller (keeper) receives `keeper_fee_bps` of the subscription payment,
transferred directly from the subscriber to the keeper address.

The maximum keeper fee is hard-capped at 500 bps (5%) in the contract.

## Rationale
- Permissionless design means no single relayer is a point of failure.
- Economic incentive (keeper fee) makes it profitable to run a relayer.
- Fee cap protects subscribers from provider abuse.
- Providers can set keeper_fee=0 and run their own relayer if they prefer.

## Consequences
In Phase 2, a reference Node.js relayer implementation will be provided.
Anyone can run it and earn fees. The fee rate is set by the provider (within the cap).
