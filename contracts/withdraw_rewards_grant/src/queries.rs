use crate::{
    helpers::sum_coins,
    msg::{AllPendingRewardsResponse, PendingReward, VersionResponse},
};
use cosmwasm_std::{Addr, Coin, Decimal, Deps, FullDelegation, QuerierWrapper, StdResult, Uint128};

use crate::ContractError;

pub fn query_version() -> VersionResponse {
    VersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

/// Queries the pending staking rewards for a given delegator
pub fn query_pending_rewards(
    querier: &QuerierWrapper,
    delegator: &Addr,
) -> Result<AllPendingRewardsResponse, ContractError> {
    // gets all of the individual delegations for the delegator
    let rewards_query: Result<Vec<PendingReward>, ContractError> = querier
        .query_all_delegations(delegator)?
        .into_iter()
        .map(
            // each delegation is queried for its pending rewards
            |delegation| match querier.query_delegation(delegator, delegation.validator) {
                Ok(Some(FullDelegation {
                    validator,
                    accumulated_rewards,
                    ..
                })) => Ok(PendingReward {
                    validator,
                    amount: accumulated_rewards,
                }),
                _ => Err(ContractError::QueryPendingRewardsFailure),
            },
        )
        .collect();

    let rewards = rewards_query?;

    // sums the rewards
    let total = rewards.clone().into_iter().fold(vec![], |mut acc, reward| {
        acc = sum_coins(acc, reward.amount);
        acc
    });

    Ok(AllPendingRewardsResponse { rewards, total })
}
