//! Soroban Subscription Service - Contract Entrypoint
//!
//! This contract provides a reusable, deployable subscription billing
//! primitive for SaaS providers on the Stellar network.
//!
//! ## Architecture
//! See architecture.md at the repository root for the full design.
//!
//! ## Phase 1 Note
//! The Stellar Asset Contract (SAC) transfer_from calls in billing.rs are
//! wired but tested with mocked token clients. Phase 2 connects to a real
//! SAC on testnet.
#![no_std]

mod admin;
mod billing;
mod errors;
mod events;
mod plan;
mod storage;
mod subscription;
mod types;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec};

use crate::{
    billing::{batch_charge, charge_subscriber},
    errors::SubscriptionError,
    plan::{create_plan, deactivate_plan, get_plan, update_plan},
    subscription::{cancel, change_plan, get_subscriber_ids, get_subscription, subscribe},
    types::{ChargeReceipt, Plan, Subscription},
};

#[contract]
pub struct SubscriptionContract;

#[contractimpl]
impl SubscriptionContract {
    // -------------------------------------------------------------------------
    // Admin
    // -------------------------------------------------------------------------

    /// Initialize the contract. Must be called once before any other function.
    pub fn initialize(
        env: Env,
        admin: Address,
        provider: Address,
        keeper_fee_bps: u32,
    ) -> Result<(), SubscriptionError> {
        admin::initialize(&env, admin, provider, keeper_fee_bps)
    }

    /// Pause the contract, blocking new subscriptions and charges.
    pub fn pause(env: Env) -> Result<(), SubscriptionError> {
        admin::pause(&env)
    }

    /// Resume normal contract operation after a pause.
    pub fn unpause(env: Env) -> Result<(), SubscriptionError> {
        admin::unpause(&env)
    }

    /// Update the keeper/relayer fee. Admin only; max 500 bps (5%).
    pub fn set_keeper_fee(env: Env, bps: u32) -> Result<(), SubscriptionError> {
        admin::set_keeper_fee(&env, bps)
    }

    /// Transfer the admin role to a new address.
    pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), SubscriptionError> {
        admin::transfer_admin(&env, new_admin)
    }

    // -------------------------------------------------------------------------
    // Plans
    // -------------------------------------------------------------------------

    /// Create a new subscription plan. Admin only.
    pub fn create_plan(
        env: Env,
        name: String,
        price_amount: i128,
        price_asset: Address,
        interval_secs: u64,
        grace_period_secs: u64,
    ) -> Result<u32, SubscriptionError> {
        create_plan(&env, name, price_amount, price_asset, interval_secs, grace_period_secs)
    }

    /// Update mutable plan fields. Admin only. Existing subscribers are grandfathered.
    pub fn update_plan(
        env: Env,
        plan_id: u32,
        name: Option<String>,
        price_amount: Option<i128>,
        interval_secs: Option<u64>,
        grace_period_secs: Option<u64>,
    ) -> Result<(), SubscriptionError> {
        update_plan(&env, plan_id, name, price_amount, interval_secs, grace_period_secs)
    }

    /// Deactivate a plan (no new subscribers). Admin only.
    pub fn deactivate_plan(env: Env, plan_id: u32) -> Result<(), SubscriptionError> {
        deactivate_plan(&env, plan_id)
    }

    /// Read a plan by ID.
    pub fn get_plan(env: Env, plan_id: u32) -> Result<Plan, SubscriptionError> {
        get_plan(&env, plan_id)
    }

    // -------------------------------------------------------------------------
    // Subscriptions
    // -------------------------------------------------------------------------

    /// Subscribe to a plan. Requires the subscriber to have approved the SAC allowance.
    pub fn subscribe(
        env: Env,
        subscriber: Address,
        plan_id: u32,
    ) -> Result<u32, SubscriptionError> {
        subscribe(&env, subscriber, plan_id)
    }

    /// Cancel a subscription. Callable by the subscriber.
    pub fn cancel(env: Env, subscription_id: u32) -> Result<(), SubscriptionError> {
        cancel(&env, subscription_id)
    }

    /// Queue a plan change effective at the next billing cycle.
    pub fn change_plan(
        env: Env,
        subscription_id: u32,
        new_plan_id: u32,
    ) -> Result<(), SubscriptionError> {
        change_plan(&env, subscription_id, new_plan_id)
    }

    /// Read a subscription by ID.
    pub fn get_subscription(
        env: Env,
        subscription_id: u32,
    ) -> Result<Subscription, SubscriptionError> {
        get_subscription(&env, subscription_id)
    }

    /// Return all subscription IDs for a subscriber address.
    pub fn get_subscriptions_for(env: Env, subscriber: Address) -> Vec<u32> {
        get_subscriber_ids(&env, &subscriber)
    }

    // -------------------------------------------------------------------------
    // Billing
    // -------------------------------------------------------------------------

    /// Charge a single subscriber. Permissionless - any relayer can call this.
    /// The keeper (caller) receives keeper_fee_bps of the payment.
    pub fn charge_subscriber(
        env: Env,
        subscription_id: u32,
        keeper: Address,
    ) -> Result<ChargeReceipt, SubscriptionError> {
        charge_subscriber(&env, subscription_id, keeper)
    }

    /// Charge up to 50 subscriptions in a single transaction.
    pub fn batch_charge(
        env: Env,
        subscription_ids: Vec<u32>,
        keeper: Address,
    ) -> Result<Vec<ChargeReceipt>, SubscriptionError> {
        batch_charge(&env, subscription_ids, keeper)
    }
}

