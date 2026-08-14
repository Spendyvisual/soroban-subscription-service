//! Admin functions: initialize, pause/unpause, transfer admin, set keeper fee.
use soroban_sdk::{Address, Env};

use crate::{
    errors::SubscriptionError,
    events,
    storage::{bump_instance, StorageKey},
    types::Config,
};

/// Maximum keeper fee: 5% = 500 basis points.
pub const MAX_KEEPER_FEE_BPS: u32 = 500;

/// Initialize the contract. Can only be called once.
///
/// # Arguments
/// * `admin`           - Address that will control plans, fees, and pausing.
/// * `provider`        - Address that receives net subscription revenue.
/// * `keeper_fee_bps`  - Relayer reward in basis points (max 500 = 5%).
pub fn initialize(
    env: &Env,
    admin: Address,
    provider: Address,
    keeper_fee_bps: u32,
) -> Result<(), SubscriptionError> {
    if env.storage().instance().has(&StorageKey::Config) {
        return Err(SubscriptionError::AlreadyInitialized);
    }
    if keeper_fee_bps > MAX_KEEPER_FEE_BPS {
        return Err(SubscriptionError::InvalidKeeperFee);
    }

    let config = Config {
        admin,
        provider,
        keeper_fee_bps,
        paused: false,
    };
    env.storage().instance().set(&StorageKey::Config, &config);
    env.storage().instance().set(&StorageKey::PlanCounter, &0u32);
    env.storage()
        .instance()
        .set(&StorageKey::SubscriptionCounter, &0u32);

    bump_instance(env);
    Ok(())
}

/// Load the Config from storage, returning NotInitialized if absent.
pub fn load_config(env: &Env) -> Result<Config, SubscriptionError> {
    env.storage()
        .instance()
        .get(&StorageKey::Config)
        .ok_or(SubscriptionError::NotInitialized)
}

/// Save an updated Config back to instance storage.
pub fn save_config(env: &Env, config: &Config) {
    env.storage().instance().set(&StorageKey::Config, config);
    bump_instance(env);
}

/// Require that the caller is the contract admin.
pub fn require_admin(env: &Env, config: &Config) {
    config.admin.require_auth();
    let _ = env; // silence unused warning
}

/// Pause the contract. Blocks new subscriptions and charges.
pub fn pause(env: &Env) -> Result<(), SubscriptionError> {
    let mut config = load_config(env)?;
    require_admin(env, &config);
    config.paused = true;
    save_config(env, &config);
    events::paused(env, true);
    Ok(())
}

/// Unpause the contract, resuming normal operation.
pub fn unpause(env: &Env) -> Result<(), SubscriptionError> {
    let mut config = load_config(env)?;
    require_admin(env, &config);
    config.paused = false;
    save_config(env, &config);
    events::paused(env, false);
    Ok(())
}

/// Update the keeper fee. Admin only; capped at MAX_KEEPER_FEE_BPS.
pub fn set_keeper_fee(env: &Env, bps: u32) -> Result<(), SubscriptionError> {
    let mut config = load_config(env)?;
    require_admin(env, &config);
    if bps > MAX_KEEPER_FEE_BPS {
        return Err(SubscriptionError::InvalidKeeperFee);
    }
    config.keeper_fee_bps = bps;
    save_config(env, &config);
    Ok(())
}

/// Transfer admin role to a new address. Requires auth from current admin.
pub fn transfer_admin(env: &Env, new_admin: Address) -> Result<(), SubscriptionError> {
    let mut config = load_config(env)?;
    require_admin(env, &config);
    config.admin = new_admin;
    save_config(env, &config);
    Ok(())
}

