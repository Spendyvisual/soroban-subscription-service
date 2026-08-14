//! Plan management: create, update, deactivate, and query subscription plans.
use soroban_sdk::{Address, Env, String};

use crate::{
    admin::{load_config, require_admin},
    errors::SubscriptionError,
    events,
    storage::{bump_instance, bump_persistent, StorageKey},
    types::Plan,
};

/// Create a new subscription plan. Admin only.
///
/// Returns the new plan ID on success.
pub fn create_plan(
    env: &Env,
    name: String,
    price_amount: i128,
    price_asset: Address,
    interval_secs: u64,
    grace_period_secs: u64,
) -> Result<u32, SubscriptionError> {
    let config = load_config(env)?;
    require_admin(env, &config);

    if price_amount <= 0 {
        return Err(SubscriptionError::InvalidPriceAmount);
    }
    if interval_secs == 0 {
        return Err(SubscriptionError::InvalidInterval);
    }

    // Allocate a new plan ID.
    let plan_id: u32 = env
        .storage()
        .instance()
        .get(&StorageKey::PlanCounter)
        .unwrap_or(0u32)
        + 1;
    env.storage()
        .instance()
        .set(&StorageKey::PlanCounter, &plan_id);
    bump_instance(env);

    let plan = Plan {
        id: plan_id,
        name,
        price_amount,
        price_asset,
        interval_secs,
        grace_period_secs,
        active: true,
    };

    env.storage()
        .persistent()
        .set(&StorageKey::Plan(plan_id), &plan);
    bump_persistent(env, &StorageKey::Plan(plan_id));

    events::plan_created(env, &plan);
    Ok(plan_id)
}

/// Update a plan's mutable fields. Admin only.
/// Existing subscriptions are grandfathered on the old terms until they
/// explicitly call change_plan.
pub fn update_plan(
    env: &Env,
    plan_id: u32,
    name: Option<String>,
    price_amount: Option<i128>,
    interval_secs: Option<u64>,
    grace_period_secs: Option<u64>,
) -> Result<(), SubscriptionError> {
    let config = load_config(env)?;
    require_admin(env, &config);

    let mut plan = get_plan(env, plan_id)?;

    if let Some(n) = name {
        plan.name = n;
    }
    if let Some(p) = price_amount {
        if p <= 0 {
            return Err(SubscriptionError::InvalidPriceAmount);
        }
        plan.price_amount = p;
    }
    if let Some(i) = interval_secs {
        if i == 0 {
            return Err(SubscriptionError::InvalidInterval);
        }
        plan.interval_secs = i;
    }
    if let Some(g) = grace_period_secs {
        plan.grace_period_secs = g;
    }

    env.storage()
        .persistent()
        .set(&StorageKey::Plan(plan_id), &plan);
    bump_persistent(env, &StorageKey::Plan(plan_id));

    events::plan_updated(env, plan_id);
    Ok(())
}

/// Deactivate a plan, preventing new subscribers. Admin only.
/// Existing active subscriptions continue until the period ends.
pub fn deactivate_plan(env: &Env, plan_id: u32) -> Result<(), SubscriptionError> {
    let config = load_config(env)?;
    require_admin(env, &config);

    let mut plan = get_plan(env, plan_id)?;
    plan.active = false;

    env.storage()
        .persistent()
        .set(&StorageKey::Plan(plan_id), &plan);
    bump_persistent(env, &StorageKey::Plan(plan_id));

    events::plan_deactivated(env, plan_id);
    Ok(())
}

/// Read a plan by ID. Returns PlanNotFound if it does not exist.
pub fn get_plan(env: &Env, plan_id: u32) -> Result<Plan, SubscriptionError> {
    env.storage()
        .persistent()
        .get::<StorageKey, Plan>(&StorageKey::Plan(plan_id))
        .ok_or(SubscriptionError::PlanNotFound)
}
