//! Typed event helpers for the Soroban Subscription Service.
//!
//! All state-changing operations emit events so off-chain indexers (and the
//! future relayer bot) can efficiently track subscription state without
//! scanning all contract storage.
use soroban_sdk::{symbol_short, Address, Env};

use crate::types::{ChargeReceipt, Plan};

/// Emitted when a new plan is created.
pub fn plan_created(env: &Env, plan: &Plan) {
    env.events().publish(
        (symbol_short!("plan_new"), plan.id),
        plan.clone(),
    );
}

/// Emitted when a plan is updated.
pub fn plan_updated(env: &Env, plan_id: u32) {
    env.events().publish(
        (symbol_short!("plan_upd"), plan_id),
        plan_id,
    );
}

/// Emitted when a plan is deactivated.
pub fn plan_deactivated(env: &Env, plan_id: u32) {
    env.events().publish(
        (symbol_short!("plan_off"), plan_id),
        plan_id,
    );
}

/// Emitted when a subscriber successfully subscribes to a plan.
pub fn subscribed(
    env: &Env,
    subscription_id: u32,
    subscriber: &Address,
    plan_id: u32,
    next_billing_ts: u64,
) {
    env.events().publish(
        (symbol_short!("sub"), subscriber.clone(), plan_id),
        (subscription_id, next_billing_ts),
    );
}

/// Emitted when a subscription is successfully charged.
pub fn charged(env: &Env, receipt: &ChargeReceipt) {
    env.events().publish(
        (symbol_short!("charge"), receipt.subscription_id),
        receipt.clone(),
    );
}

/// Emitted when a subscription is cancelled.
pub fn cancelled(env: &Env, subscription_id: u32, subscriber: &Address, timestamp: u64) {
    env.events().publish(
        (symbol_short!("cancel"), subscription_id),
        (subscriber.clone(), timestamp),
    );
}

/// Emitted when a plan change is queued for the next billing cycle.
pub fn plan_change_queued(
    env: &Env,
    subscription_id: u32,
    old_plan_id: u32,
    new_plan_id: u32,
) {
    env.events().publish(
        (symbol_short!("pl_chg"), subscription_id),
        (old_plan_id, new_plan_id),
    );
}

/// Emitted when the contract is paused or unpaused.
pub fn paused(env: &Env, is_paused: bool) {
    env.events().publish(
        (symbol_short!("paused"),),
        is_paused,
    );
}
