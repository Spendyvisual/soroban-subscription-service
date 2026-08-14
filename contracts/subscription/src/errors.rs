//! Error types for the Soroban Subscription Service contract.
use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SubscriptionError {
    /// Contract has already been initialized.
    AlreadyInitialized = 1,
    /// Contract has not been initialized yet.
    NotInitialized = 2,
    /// Caller is not authorized to perform this action.
    Unauthorized = 3,
    /// The requested plan does not exist.
    PlanNotFound = 4,
    /// The plan exists but is no longer accepting new subscribers.
    PlanInactive = 5,
    /// The requested subscription does not exist.
    SubscriptionNotFound = 6,
    /// The subscription has already been cancelled.
    AlreadyCancelled = 7,
    /// The subscription is not yet due for billing.
    NotDue = 8,
    /// The grace period has expired; subscription is past due.
    GracePeriodExpired = 9,
    /// The subscriber has insufficient allowance for the token transfer.
    AllowanceRevoked = 10,
    /// The contract is paused; no new subscriptions or charges allowed.
    ContractPaused = 11,
    /// Keeper fee basis points exceed the maximum allowed (500 bps).
    InvalidKeeperFee = 12,
    /// Interval must be greater than zero.
    InvalidInterval = 13,
    /// Price amount must be greater than zero.
    InvalidPriceAmount = 14,
    /// Batch charge vector is empty or too large.
    InvalidBatchSize = 15,
}
