#!/usr/bin/env bash
# scripts/test-charge.sh
# Simulates a full subscribe -> charge -> cancel cycle on testnet.
# Usage: ./scripts/test-charge.sh [CONTRACT_ID] [USDC_SAC]

set -euo pipefail

CONTRACT_ID="${1:-}"
USDC_SAC="${2:-}"

if [[ -z "$CONTRACT_ID" ]]; then
    echo "Usage: $0 <contract_id> <usdc_sac_address>"
    exit 1
fi

echo "=== Soroban Subscription Service — Test Charge Script ==="
echo "Contract: $CONTRACT_ID"
echo "USDC SAC: $USDC_SAC"

# Create a test plan (30-day interval, 1-day grace)
echo ""
echo "[1/4] Creating test plan..."
PLAN_ID=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source admin \
    --network testnet \
    -- create_plan \
    --name '"Test Plan"' \
    --price_amount 10000000 \
    --price_asset "$USDC_SAC" \
    --interval_secs 86400 \
    --grace_period_secs 3600 \
    2>&1 | tail -n1 | tr -d '"')

echo "Plan ID: $PLAN_ID"

echo "[2/4] Subscribing test account..."
SUB_ID=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source subscriber \
    --network testnet \
    -- subscribe \
    --subscriber "$(stellar keys address subscriber)" \
    --plan_id "$PLAN_ID" \
    2>&1 | tail -n1 | tr -d '"')

echo "Subscription ID: $SUB_ID"

echo "[3/4] Waiting 1 day (simulated)..."
echo "  NOTE: On testnet, advance ledger timestamp manually or wait."

echo "[4/4] Charging subscription as keeper..."
stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source keeper \
    --network testnet \
    -- charge_subscriber \
    --subscription_id "$SUB_ID" \
    --keeper "$(stellar keys address keeper)"

echo ""
echo "=== Done ==="
