# Threat Model — Soroban Subscription Service

**Phase:** 1  
**Date:** 2026-08-14

---

## Phase 1 Security Posture

Phase 1 establishes the correct contract structure and authorization model.
It does NOT provide any additional privacy beyond Stellar's base layer.

### What IS secured in Phase 1

| Guarantee | Mechanism |
|---|---|
| Admin functions require admin auth | `require_auth(&config.admin)` on all admin calls |
| Subscriber cancel/change requires subscriber auth | `require_auth(&sub.subscriber)` |
| Provider address is immutable after initialize | Stored in Instance config; only admin can update via set_provider (future) |
| Keeper fee is capped at 5% | Hard-coded `MAX_KEEPER_FEE_BPS = 500` |
| Funds flow only to stored provider address | `transfer_from` target is `config.provider`, never the relayer |
| Double-initialization blocked | Config existence check in `initialize` |

### What is NOT secured in Phase 1

- Subscription amounts and billing dates are visible in contract storage.
- There is no on-chain enforcement of SAC allowances before subscribe() — the failure mode is that charge_subscriber() fails gracefully.
- No rate limiting on charge_subscriber(); relayers can spam failed calls (costs them fees, not the contract).

---

## Trust Assumptions

| Actor | Trusted for | Not trusted for |
|---|---|---|
| Admin | Managing plans and fees correctly | Admin key compromise could deactivate plans (but cannot redirect funds) |
| Relayer | Calling charge_subscriber at the right time | Keeper cannot redirect provider payments; worst case: missed billing |
| Subscriber | Maintaining SAC allowance | Revocation causes PastDue status; no funds locked |
| Soroban runtime | Correct execution of contract logic | N/A |

---

## Phase 2+ Additions

Phase 2 will add:
- Allowance validation before subscribe() (read SAC allowance on-chain)
- Rate limiting via subscription state machine
- Optional subscriber notification events
