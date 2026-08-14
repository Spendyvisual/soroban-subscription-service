//! Shared types for the Soroban Subscription Service contract.
use soroban_sdk::{contracttype, Address, String};

/// A subscription plan defined by the SaaS provider.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    /// Unique monotonic plan identifier.
    pub id: u32,
    /// Human-readable plan name (e.g. "Pro Monthly").
    pub name: String,
    /// Amount charged per billing interval, in the smallest unit of price_asset.
    pub price_amount: i128,
    /// Stellar Asset Contract address for the payment token (XLM SAC or USDC SAC).
    pub price_asset: Address,
    /// Billing interval in seconds (e.g. 2_592_000 = 30 days).
    pub interval_secs: u64,
    /// Seconds after next_billing_ts before the subscription moves to PastDue.
    pub grace_period_secs: u64,
    /// Whether new subscribers can join this plan.
    pub active: bool,
}

/// Status of a subscriber's subscription.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
    PastDue,
}

/// A single subscriber's subscription record.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    /// Unique monotonic subscription identifier.
    pub id: u32,
    /// The subscriber's Stellar address.
    pub subscriber: Address,
    /// The plan this subscription belongs to.
    pub plan_id: u32,
    /// Current lifecycle status.
    pub status: SubscriptionStatus,
    /// Ledger timestamp when the subscription was created.
    pub created_at: u64,
    /// Ledger timestamp when the next charge is due.
    pub next_billing_ts: u64,
    /// If set, switch to this plan at the next billing cycle.
    pub pending_plan_change: Option<u32>,
}

/// Receipt returned by a successful charge_subscriber call.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChargeReceipt {
    pub subscription_id: u32,
    pub plan_id: u32,
    pub amount_charged: i128,
    pub keeper_fee: i128,
    pub net_to_provider: i128,
    pub timestamp: u64,
}

/// Global contract configuration stored in Instance storage.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Config {
    /// Contract administrator (can manage plans, pause contract, set fees).
    pub admin: Address,
    /// Address that receives net subscription payments.
    pub provider: Address,
    /// Keeper fee in basis points (max 500 = 5%).
    pub keeper_fee_bps: u32,
    /// When true, new subscriptions and charges are blocked.
    pub paused: bool,
}

