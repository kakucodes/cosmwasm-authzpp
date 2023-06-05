use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Decimal, Timestamp};
use serde::Serialize;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(VersionResponse)]
    Version {},

    /// Returns the grant information for the given granter.
    /// Will either return the active grant info or nothing if there is no active grant.
    #[returns(Option<GrantQueryResponse>)]
    ActiveGrantsByGranter(String),

    /// Returns the grant information for the given grantee.
    /// Will return a list of all grants that the grantee has access to.
    #[returns(Vec<GrantQueryResponse>)]
    ActiveGrantsByGrantee(String),

    /// Returns the pending rewards for the given grantee.
    #[returns(AllPendingRewardsResponse)]
    PendingRewards(String),
}

#[cw_serde]
pub struct AllPendingRewardsResponse {
    pub rewards: Vec<PendingReward>,
    pub total: Vec<Coin>,
}

#[cw_serde]
pub struct PendingReward {
    pub validator: String,
    pub amount: Vec<Coin>,
}

#[cw_serde]
pub struct GrantQueryResponse {
    pub granter: Addr,
    pub allowed_withdrawls: AllowedWithdrawlSettings,
}

#[cw_serde]
pub struct VersionResponse {
    pub version: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Creates a new grant that allows portions of one's staking rewards to be claimed by other addresses
    Grant(AllowedWithdrawlSettings),

    /// Revokes an existing grant so that it can no longer be used
    Revoke(),

    /// Withdraws a user's rewards to the address that was granted access to them and the remainder to the grantee's address
    Execute(ExecuteSettings),
}

#[cw_serde]
pub struct ExecuteSettings {
    /// address to withdraw the rewards for
    pub granter: String,
    /// the percentage of rewards to be withdrawn to the grantee. if none is specified, the max is used
    pub percentage: Option<Decimal>,
}

#[cw_serde]
pub struct AllowedWithdrawlSettings {
    /// address to withdraw portion of rewards to
    pub grantee: String,
    /// percentage of rewards that can be withdrawn to the given address
    pub max_percentage: Decimal,
    /// expiration date of the grant as a unix timestamp
    pub expiration: Timestamp,
}
