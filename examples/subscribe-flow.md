# Soroban Subscription Service — Subscribe Flow Walkthrough

This document walks through the complete lifecycle of a subscription,
from plan creation to renewal to cancellation.

## Step 1: Provider Initializes the Contract

The SaaS provider deploys the contract and calls initialize:

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source $ADMIN_SECRET \
  -- initialize \
  --admin $ADMIN_ADDR \
  --provider $PROVIDER_ADDR \
  --keeper_fee_bps 100
```

keeper_fee_bps = 100 means relayers earn 1% of each charge.

## Step 2: Provider Creates a Plan

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source $ADMIN_SECRET \
  -- create_plan \
  --name "Pro Monthly" \
  --price_amount 10000000 \
  --price_asset $USDC_SAC \
  --interval_secs 2592000 \
  --grace_period_secs 86400
```

Returns: plan_id = 1

## Step 3: Subscriber Approves Allowance

The subscriber must approve the contract to spend USDC on their behalf.
This is done via the USDC SAC:

```bash
stellar contract invoke \
  --id $USDC_SAC \
  --source $SUBSCRIBER_SECRET \
  -- approve \
  --from $SUBSCRIBER_ADDR \
  --spender $CONTRACT_ID \
  --amount 120000000 \
  --expiration_ledger 9999999
```

120000000 = 12 months of 10 USDC each (7 decimals).

## Step 4: Subscriber Subscribes

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source $SUBSCRIBER_SECRET \
  -- subscribe \
  --subscriber $SUBSCRIBER_ADDR \
  --plan_id 1
```

Returns: subscription_id = 1
Contract records: next_billing_ts = now + 2592000 (30 days from now)

## Step 5: Relayer Charges After 30 Days

Any relayer can call this. They earn 100,000 stroops (1% of 10 USDC):

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source $KEEPER_SECRET \
  -- charge_subscriber \
  --subscription_id 1 \
  --keeper $KEEPER_ADDR
```

Result:
- Provider receives: 9,900,000 (9.90 USDC)
- Keeper receives: 100,000 (0.10 USDC)
- next_billing_ts advances by 2,592,000 seconds

## Step 6: Subscriber Cancels

```bash
stellar contract invoke \
  --id $CONTRACT_ID \
  --source $SUBSCRIBER_SECRET \
  -- cancel \
  --subscription_id 1
```

Status changes to Cancelled. No further charges are attempted.
