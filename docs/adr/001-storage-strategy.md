# ADR 001 — Storage Strategy

**Status:** Accepted  
**Date:** 2026-08-14

## Context
Soroban has three storage tiers: Temporary, Instance, and Persistent.
Each has different cost and TTL semantics.

## Decision
- **Instance storage:** Contract configuration (admin, provider, keeper_fee_bps, paused flag), plan counter, subscription counter.
- **Persistent storage:** Individual plan records, individual subscription records, subscriber index.

## Rationale
Instance storage is loaded with every contract invocation and is cheap for small, frequently-read data.
Persistent storage requires explicit TTL management but is appropriate for long-lived per-user data.

## Consequences
Every write to Persistent storage must call `extend_ttl` to prevent silent archival. This is codified in the `bump_persistent` helper.
