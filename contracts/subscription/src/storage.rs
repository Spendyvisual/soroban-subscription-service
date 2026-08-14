//! Storage key definitions and TTL management helpers.
use soroban_sdk::{contracttype, Env};

/// Maximum TTL for instance storage entries (approximately 30 days of ledgers).
pub const INSTANCE_TTL_BUMP: u32 = 518_400;
/// Maximum TTL for persistent storage entries (approximately 1 year of ledgers).
pub const PERSISTENT_TTL_BUMP: u32 = 6_307_200;
/// Minimum TTL threshold before bumping (half of bump amount).
pub const PERSISTENT_TTL_THRESHOLD: u32 = 3_153_600;

/// All storage keys used by the contract.
#[contracttype]
#[derive(Clone, Debug)]
pub enum StorageKey {
    /// Contract configuration (Config struct). Stored in Instance storage.
    Config,
    /// Monotonic counter for plan IDs. Stored in Instance storage.
    PlanCounter,
    /// Monotonic counter for subscription IDs. Stored in Instance storage.
    SubscriptionCounter,
    /// Individual plan record keyed by plan ID. Stored in Persistent storage.
    Plan(u32),
    /// Individual subscription record keyed by subscription ID. Stored in Persistent storage.
    Subscription(u32),
    /// Index: subscriber address -> list of subscription IDs. Stored in Persistent storage.
    SubscriberIndex(soroban_sdk::Address),
}

/// Bump the TTL on the contract instance storage so configuration never expires.
pub fn bump_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(INSTANCE_TTL_BUMP, INSTANCE_TTL_BUMP);
}

/// Bump the TTL on a persistent storage entry.
pub fn bump_persistent(env: &Env, key: &StorageKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, PERSISTENT_TTL_THRESHOLD, PERSISTENT_TTL_BUMP);
}

