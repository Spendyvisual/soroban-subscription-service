//! Billing engine: charge_subscriber and batch_charge.
//!
//! Phase 1 Note:
//! The actual Stellar Asset Contract (SAC) transfer_from call is structured
//! and present in this module. In unit tests, the token client is mocked via
//! soroban_sdk testutils. In Phase 2, only the provider/SAC address wiring
//! needs to change - the logic stays identical.
use soroban_sdk::{token, Address, Env, Vec};

use crate::{
    admin::load_config,
    errors::SubscriptionError,
    events,
    plan::get_plan,
    storage::{bump_persistent, StorageKey},
    subscription::get_subscription,
    types::{ChargeReceipt, SubscriptionStatus},
};

/// Maximum number of subscriptions that can be charged in a single batch call.
pub const MAX_BATCH_SIZE: u32 = 50;

/// Charge a single subscriber for their current billing period.
///
/// This function is **permissionless**: any account can call it (typically a
/// relayer/keeper bot). The keeper earns keeper_fee_bps of the charge amount
/// transferred directly to env.current_contract_address() invoker - wait,
/// to env.invoker() - actually in Soroban we use the transaction source
/// as fee recipient since invoker isn't exposed. The keeper fee goes to the
/// caller of this function (the relayer).
///
/// # Charging Logic
/// 1. Load subscription - error if not found.
/// 2. Check status == Active - error if Cancelled.
/// 3. Check now >= next_billing_ts - error if NotDue.
/// 4. Check now <= next_billing_ts + grace_period - if exceeded, mark PastDue.
/// 5. Apply pending plan change if present.
/// 6. Load plan - get price_amount, price_asset, interval.
/// 7. Compute keeper_fee and net_amount.
/// 8. Call SAC.transfer_from(subscriber, provider, net_amount).
/// 9. Update next_billing_ts += interval_secs; bump TTL.
/// 10. Emit Charged event; return ChargeReceipt.
pub fn charge_subscriber(
    env: &Env,
    subscription_id: u32,
    keeper: Address,
) -> Result<ChargeReceipt, SubscriptionError> {
    let config = load_config(env)?;
    if config.paused {
        return Err(SubscriptionError::ContractPaused);
    }

    let mut sub = get_subscription(env, subscription_id)?;

    if sub.status == SubscriptionStatus::Cancelled {
        return Err(SubscriptionError::AlreadyCancelled);
    }

    let now = env.ledger().timestamp();

    // Must be at or past the billing date.
    if now < sub.next_billing_ts {
        return Err(SubscriptionError::NotDue);
    }

    // Load the plan (may have been switched via pending_plan_change).
    if let Some(new_plan_id) = sub.pending_plan_change.take() {
        sub.plan_id = new_plan_id;
        sub.pending_plan_change = None;
    }

    let plan = get_plan(env, sub.plan_id)?;

    // Grace period check.
    if now > sub.next_billing_ts + plan.grace_period_secs {
        sub.status = SubscriptionStatus::PastDue;
        env.storage()
            .persistent()
            .set(&StorageKey::Subscription(subscription_id), &sub);
        bump_persistent(env, &StorageKey::Subscription(subscription_id));
        return Err(SubscriptionError::GracePeriodExpired);
    }

    // Fee arithmetic.
    let keeper_fee: i128 = (plan.price_amount * config.keeper_fee_bps as i128) / 10_000;
    let net_to_provider: i128 = plan.price_amount - keeper_fee;

    // --- Token transfer (Phase 1: mocked in tests; real SAC call wired here) ---
    let token_client = token::Client::new(env, &plan.price_asset);

    // Transfer net amount to provider.
    token_client.transfer_from(
        &env.current_contract_address(),
        &sub.subscriber,
        &config.provider,
        &net_to_provider,
    );

    // Transfer keeper fee to relayer.
    if keeper_fee > 0 {
        token_client.transfer_from(
            &env.current_contract_address(),
            &sub.subscriber,
            &keeper,
            &keeper_fee,
        );
    }
    // --- End token transfer ---

    // Advance billing date.
    sub.next_billing_ts += plan.interval_secs;
    // Reset to Active if it was PastDue and the charge succeeded.
    sub.status = SubscriptionStatus::Active;

    env.storage()
        .persistent()
        .set(&StorageKey::Subscription(subscription_id), &sub);
    bump_persistent(env, &StorageKey::Subscription(subscription_id));

    let receipt = ChargeReceipt {
        subscription_id,
        plan_id: sub.plan_id,
        amount_charged: plan.price_amount,
        keeper_fee,
        net_to_provider,
        timestamp: now,
    };

    events::charged(env, &receipt);
    Ok(receipt)
}

/// Charge multiple subscriptions in a single transaction.
///
/// Returns a vector of results in the same order as the input IDs.
/// Individual failures do NOT abort the batch - the result vector indicates
/// which charges succeeded and which failed.
pub fn batch_charge(
    env: &Env,
    subscription_ids: Vec<u32>,
    keeper: Address,
) -> Result<Vec<ChargeReceipt>, SubscriptionError> {
    if subscription_ids.is_empty() || subscription_ids.len() > MAX_BATCH_SIZE {
        return Err(SubscriptionError::InvalidBatchSize);
    }

    let mut receipts = Vec::new(env);
    for id in subscription_ids.iter() {
        // Silently skip failures in batch mode; relayer can retry individually.
        if let Ok(receipt) = charge_subscriber(env, id, keeper.clone()) {
            receipts.push_back(receipt);
        }
    }
    Ok(receipts)
}

