//! Comprehensive unit test suite for the Soroban Subscription Service.
//!
//! Tests cover:
//! - Contract initialization (happy path, double-init guard)
//! - Plan CRUD (create, update, deactivate, invalid inputs)
//! - Subscription lifecycle (subscribe, cancel, change_plan)
//! - Billing arithmetic (keeper fee, net amount, next_billing_ts advance)
//! - Error paths (NotDue, GracePeriodExpired, AlreadyCancelled, ContractPaused)
//! - Admin controls (pause/unpause, set_keeper_fee, transfer_admin)
//! - Subscriber index integrity
#![cfg(test)]

extern crate std;

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env, String, Vec,
};

use crate::{errors::SubscriptionError, types::SubscriptionStatus, SubscriptionContract, SubscriptionContractClient};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

struct TestContext {
    env: Env,
    contract_id: Address,
    client: SubscriptionContractClient<'static>,
    admin: Address,
    provider: Address,
    token_id: Address,
}

impl TestContext {
    fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let provider = Address::generate(&env);

        // Deploy a mock token contract for the payment asset.
        let token_id = env.register_stellar_asset_contract_v2(admin.clone()).address();

        let contract_id = env.register(SubscriptionContract, ());
        let client = SubscriptionContractClient::new(&env, &contract_id);

        client
            .initialize(&admin, &provider, &100u32) // 1% keeper fee
            .expect("initialize failed");

        // Re-create client with 'static lifetime for convenience.
        // SAFETY: env outlives the test.
        let client: SubscriptionContractClient<'static> =
            unsafe { std::mem::transmute(SubscriptionContractClient::new(&env, &contract_id)) };

        TestContext {
            env,
            contract_id,
            client,
            admin,
            provider,
            token_id,
        }
    }

    fn mint_to(&self, recipient: &Address, amount: i128) {
        let token = token::StellarAssetClient::new(&self.env, &self.token_id);
        token.mint(recipient, &amount);
    }

    fn create_monthly_plan(&self) -> u32 {
        self.client
            .create_plan(
                &String::from_str(&self.env, "Pro Monthly"),
                &10_000_000i128, // 10 USDC (7 decimals)
                &self.token_id,
                &2_592_000u64, // 30 days
                &86_400u64,    // 1 day grace
            )
            .expect("create_plan failed")
    }
}

fn advance_time(env: &Env, seconds: u64) {
    env.ledger().with_mut(|l| {
        l.timestamp += seconds;
    });
}

// ---------------------------------------------------------------------------
// Initialization tests
// ---------------------------------------------------------------------------

#[test]
fn test_initialize_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let contract_id = env.register(SubscriptionContract, ());
    let client = SubscriptionContractClient::new(&env, &contract_id);

    assert!(client.initialize(&admin, &provider, &100u32).is_ok());
}

#[test]
fn test_initialize_double_init_fails() {
    let ctx = TestContext::new();
    let result = ctx.client.try_initialize(&ctx.admin, &ctx.provider, &100u32);
    assert_eq!(
        result.unwrap_err().unwrap(),
        SubscriptionError::AlreadyInitialized
    );
}

#[test]
fn test_initialize_keeper_fee_too_high_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract_id = env.register(SubscriptionContract, ());
    let client = SubscriptionContractClient::new(&env, &contract_id);

    let result = client.try_initialize(&admin, &provider, &501u32);
    assert_eq!(
        result.unwrap_err().unwrap(),
        SubscriptionError::InvalidKeeperFee
    );
}

// ---------------------------------------------------------------------------
// Plan management tests
// ---------------------------------------------------------------------------

#[test]
fn test_create_plan_returns_sequential_ids() {
    let ctx = TestContext::new();

    let id1 = ctx.client
        .create_plan(&String::from_str(&ctx.env, "Basic"), &1_000_000i128, &ctx.token_id, &86_400u64, &3_600u64)
        .unwrap();
    let id2 = ctx.client
        .create_plan(&String::from_str(&ctx.env, "Pro"), &5_000_000i128, &ctx.token_id, &86_400u64, &3_600u64)
        .unwrap();

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
}

#[test]
fn test_create_plan_zero_price_fails() {
    let ctx = TestContext::new();
    let result = ctx.client.try_create_plan(
        &String::from_str(&ctx.env, "Free"),
        &0i128,
        &ctx.token_id,
        &86_400u64,
        &0u64,
    );
    assert_eq!(
        result.unwrap_err().unwrap(),
        SubscriptionError::InvalidPriceAmount
    );
}

#[test]
fn test_create_plan_zero_interval_fails() {
    let ctx = TestContext::new();
    let result = ctx.client.try_create_plan(
        &String::from_str(&ctx.env, "Instant"),
        &1_000_000i128,
        &ctx.token_id,
        &0u64,
        &0u64,
    );
    assert_eq!(
        result.unwrap_err().unwrap(),
        SubscriptionError::InvalidInterval
    );
}

#[test]
fn test_get_plan_not_found() {
    let ctx = TestContext::new();
    let result = ctx.client.try_get_plan(&999u32);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::PlanNotFound);
}

#[test]
fn test_deactivate_plan_blocks_new_subscriptions() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();

    ctx.client.deactivate_plan(&plan_id).unwrap();

    let subscriber = Address::generate(&ctx.env);
    let result = ctx.client.try_subscribe(&subscriber, &plan_id);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::PlanInactive);
}

#[test]
fn test_update_plan_changes_price() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();

    ctx.client
        .update_plan(&plan_id, &None, &Some(20_000_000i128), &None, &None)
        .unwrap();

    let plan = ctx.client.get_plan(&plan_id).unwrap();
    assert_eq!(plan.price_amount, 20_000_000i128);
}

// ---------------------------------------------------------------------------
// Subscription lifecycle tests
// ---------------------------------------------------------------------------

#[test]
fn test_subscribe_creates_record_with_correct_billing_ts() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);

    let now = ctx.env.ledger().timestamp();
    let sub_id = ctx.client.subscribe(&subscriber, &plan_id).unwrap();

    let sub = ctx.client.get_subscription(&sub_id).unwrap();
    assert_eq!(sub.subscriber, subscriber);
    assert_eq!(sub.plan_id, plan_id);
    assert_eq!(sub.status, SubscriptionStatus::Active);
    assert_eq!(sub.next_billing_ts, now + 2_592_000u64);
}

#[test]
fn test_subscribe_appears_in_subscriber_index() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);

    let sub_id = ctx.client.subscribe(&subscriber, &plan_id).unwrap();

    let ids = ctx.client.get_subscriptions_for(&subscriber);
    assert_eq!(ids.len(), 1);
    assert_eq!(ids.get(0).unwrap(), sub_id);
}

#[test]
fn test_cancel_subscription_sets_cancelled_status() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);

    let sub_id = ctx.client.subscribe(&subscriber, &plan_id).unwrap();
    ctx.client.cancel(&sub_id).unwrap();

    let sub = ctx.client.get_subscription(&sub_id).unwrap();
    assert_eq!(sub.status, SubscriptionStatus::Cancelled);
}

#[test]
fn test_cancel_twice_returns_error() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);

    let sub_id = ctx.client.subscribe(&subscriber, &plan_id).unwrap();
    ctx.client.cancel(&sub_id).unwrap();

    let result = ctx.client.try_cancel(&sub_id);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::AlreadyCancelled);
}

#[test]
fn test_change_plan_queues_pending_change() {
    let ctx = TestContext::new();
    let plan_id_a = ctx.create_monthly_plan();
    let plan_id_b = ctx.client
        .create_plan(
            &String::from_str(&ctx.env, "Enterprise"),
            &50_000_000i128,
            &ctx.token_id,
            &2_592_000u64,
            &86_400u64,
        )
        .unwrap();

    let subscriber = Address::generate(&ctx.env);
    let sub_id = ctx.client.subscribe(&subscriber, &plan_id_a).unwrap();

    ctx.client.change_plan(&sub_id, &plan_id_b).unwrap();

    let sub = ctx.client.get_subscription(&sub_id).unwrap();
    assert_eq!(sub.pending_plan_change, Some(plan_id_b));
    assert_eq!(sub.plan_id, plan_id_a); // still on old plan until next charge
}

// ---------------------------------------------------------------------------
// Billing tests
// ---------------------------------------------------------------------------

#[test]
fn test_charge_not_due_returns_error() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);
    ctx.mint_to(&subscriber, 100_000_000i128);

    let sub_id = ctx.client.subscribe(&subscriber, &plan_id).unwrap();
    let keeper = Address::generate(&ctx.env);

    // Don't advance time — billing date not yet reached.
    let result = ctx.client.try_charge_subscriber(&sub_id, &keeper);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::NotDue);
}

#[test]
fn test_charge_after_billing_date_succeeds_and_advances_ts() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);
    ctx.mint_to(&subscriber, 100_000_000i128);

    let sub_id = ctx.client.subscribe(&subscriber, &plan_id).unwrap();
    let sub_before = ctx.client.get_subscription(&sub_id).unwrap();

    // Advance time past the billing date.
    advance_time(&ctx.env, 2_592_001u64);

    let keeper = Address::generate(&ctx.env);
    let receipt = ctx.client.charge_subscriber(&sub_id, &keeper).unwrap();

    assert_eq!(receipt.subscription_id, sub_id);
    assert_eq!(receipt.amount_charged, 10_000_000i128);
    assert_eq!(receipt.keeper_fee, 100_000i128);        // 1% of 10_000_000
    assert_eq!(receipt.net_to_provider, 9_900_000i128); // 10_000_000 - 100_000

    let sub_after = ctx.client.get_subscription(&sub_id).unwrap();
    assert_eq!(
        sub_after.next_billing_ts,
        sub_before.next_billing_ts + 2_592_000u64
    );
}

#[test]
fn test_charge_after_grace_period_marks_past_due() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan(); // 1-day grace period
    let subscriber = Address::generate(&ctx.env);
    ctx.mint_to(&subscriber, 100_000_000i128);

    let sub_id = ctx.client.subscribe(&subscriber, &plan_id).unwrap();
    let keeper = Address::generate(&ctx.env);

    // Advance past billing date + grace period (30 days + 1 day + 1 second).
    advance_time(&ctx.env, 2_592_000u64 + 86_400u64 + 1u64);

    let result = ctx.client.try_charge_subscriber(&sub_id, &keeper);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::GracePeriodExpired);

    let sub = ctx.client.get_subscription(&sub_id).unwrap();
    assert_eq!(sub.status, SubscriptionStatus::PastDue);
}

#[test]
fn test_charge_cancelled_subscription_fails() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);

    let sub_id = ctx.client.subscribe(&subscriber, &plan_id).unwrap();
    ctx.client.cancel(&sub_id).unwrap();

    advance_time(&ctx.env, 2_592_001u64);

    let keeper = Address::generate(&ctx.env);
    let result = ctx.client.try_charge_subscriber(&sub_id, &keeper);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::AlreadyCancelled);
}

#[test]
fn test_charge_applies_pending_plan_change() {
    let ctx = TestContext::new();
    let plan_id_a = ctx.create_monthly_plan(); // 10_000_000
    let plan_id_b = ctx.client
        .create_plan(
            &String::from_str(&ctx.env, "Enterprise"),
            &50_000_000i128,
            &ctx.token_id,
            &2_592_000u64,
            &86_400u64,
        )
        .unwrap();

    let subscriber = Address::generate(&ctx.env);
    ctx.mint_to(&subscriber, 500_000_000i128);

    let sub_id = ctx.client.subscribe(&subscriber, &plan_id_a).unwrap();
    ctx.client.change_plan(&sub_id, &plan_id_b).unwrap();

    advance_time(&ctx.env, 2_592_001u64);

    let keeper = Address::generate(&ctx.env);
    let receipt = ctx.client.charge_subscriber(&sub_id, &keeper).unwrap();

    // Charge should be at the new plan's price.
    assert_eq!(receipt.amount_charged, 50_000_000i128);
    assert_eq!(receipt.plan_id, plan_id_b);

    let sub = ctx.client.get_subscription(&sub_id).unwrap();
    assert_eq!(sub.plan_id, plan_id_b);
    assert!(sub.pending_plan_change.is_none());
}

#[test]
fn test_zero_keeper_fee_sends_full_amount_to_provider() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let contract_id = env.register(SubscriptionContract, ());
    let client = SubscriptionContractClient::new(&env, &contract_id);

    client.initialize(&admin, &provider, &0u32).unwrap(); // 0% keeper fee

    let plan_id = client
        .create_plan(
            &String::from_str(&env, "Zero Fee"),
            &10_000_000i128,
            &token_id,
            &86_400u64,
            &3_600u64,
        )
        .unwrap();

    let subscriber = Address::generate(&env);
    let token = token::StellarAssetClient::new(&env, &token_id);
    token.mint(&subscriber, &100_000_000i128);

    let sub_id = client.subscribe(&subscriber, &plan_id).unwrap();
    env.ledger().with_mut(|l| l.timestamp += 86_401u64);

    let keeper = Address::generate(&env);
    let receipt = client.charge_subscriber(&sub_id, &keeper).unwrap();

    assert_eq!(receipt.keeper_fee, 0i128);
    assert_eq!(receipt.net_to_provider, 10_000_000i128);
}

// ---------------------------------------------------------------------------
// Admin control tests
// ---------------------------------------------------------------------------

#[test]
fn test_pause_blocks_subscriptions() {
    let ctx = TestContext::new();
    ctx.client.pause().unwrap();

    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);
    let result = ctx.client.try_subscribe(&subscriber, &plan_id);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::ContractPaused);
}

#[test]
fn test_unpause_restores_subscriptions() {
    let ctx = TestContext::new();
    ctx.client.pause().unwrap();
    ctx.client.unpause().unwrap();

    let plan_id = ctx.create_monthly_plan();
    let subscriber = Address::generate(&ctx.env);
    assert!(ctx.client.try_subscribe(&subscriber, &plan_id).is_ok());
}

#[test]
fn test_set_keeper_fee_above_max_fails() {
    let ctx = TestContext::new();
    let result = ctx.client.try_set_keeper_fee(&501u32);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::InvalidKeeperFee);
}

#[test]
fn test_transfer_admin_changes_admin() {
    let ctx = TestContext::new();
    let new_admin = Address::generate(&ctx.env);
    ctx.client.transfer_admin(&new_admin).unwrap();
    // New admin can now create plans (implicitly tested via mock_all_auths).
    assert!(ctx.client
        .try_create_plan(
            &String::from_str(&ctx.env, "Post-Transfer"),
            &1_000_000i128,
            &ctx.token_id,
            &86_400u64,
            &0u64,
        )
        .is_ok());
}

#[test]
fn test_batch_charge_empty_fails() {
    let ctx = TestContext::new();
    let keeper = Address::generate(&ctx.env);
    let empty_ids: Vec<u32> = Vec::new(&ctx.env);
    let result = ctx.client.try_batch_charge(&empty_ids, &keeper);
    assert_eq!(result.unwrap_err().unwrap(), SubscriptionError::InvalidBatchSize);
}

#[test]
fn test_batch_charge_succeeds_for_valid_subscriptions() {
    let ctx = TestContext::new();
    let plan_id = ctx.create_monthly_plan();

    let subscriber_a = Address::generate(&ctx.env);
    let subscriber_b = Address::generate(&ctx.env);
    ctx.mint_to(&subscriber_a, 100_000_000i128);
    ctx.mint_to(&subscriber_b, 100_000_000i128);

    let sub_a = ctx.client.subscribe(&subscriber_a, &plan_id).unwrap();
    let sub_b = ctx.client.subscribe(&subscriber_b, &plan_id).unwrap();

    advance_time(&ctx.env, 2_592_001u64);

    let keeper = Address::generate(&ctx.env);
    let mut ids = Vec::new(&ctx.env);
    ids.push_back(sub_a);
    ids.push_back(sub_b);

    let receipts = ctx.client.batch_charge(&ids, &keeper).unwrap();
    assert_eq!(receipts.len(), 2);
}
