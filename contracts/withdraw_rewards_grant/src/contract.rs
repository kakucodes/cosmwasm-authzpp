#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Order, Response};

use crate::error::ContractError;

use crate::helpers::partition_coins_by_percentage;
use crate::msg::{
    AllPendingRewardsResponse, AllowedWithdrawlSettings, ExecuteMsg, ExecuteSettings,
    InstantiateMsg, QueryMsg,
};
use crate::msg_gen::{exec_msg, set_withdraw_rewards_msg, withdraw_rewards_msgs};
use crate::queries::query_pending_rewards;
use crate::state::GRANTS;
use crate::{execute, queries};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:withdraw-rewards-grant";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: InstantiateMsg) -> Result<Response, ContractError> {
    // let version: Version = CONTRACT_VERSION.parse()?;
    // let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

    // if storage_version < version {
    //     set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // }
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Grant(grant_spec) => {
            // validate that the grantee address is valid
            let grantee_addr = deps
                .api
                .addr_validate(&grant_spec.grantee)
                .map_err(|_| ContractError::InvalidGranteeAddress(grant_spec.clone().grantee))?;

            GRANTS.save(deps.storage, info.sender.clone(), &grant_spec)?;

            Ok(Response::default()
                .add_attribute("action", "grant")
                .add_attribute("granter", info.sender)
                .add_attribute("grantee", grantee_addr.to_string()))
        }
        ExecuteMsg::Revoke() => {
            // remove the grant from state
            GRANTS.remove(deps.storage, info.sender.clone());

            Ok(Response::default()
                .add_attribute("action", "revoke")
                .add_attribute("granter", info.sender))
        }
        ExecuteMsg::Execute(ExecuteSettings {
            granter,
            percentage,
        }) => {
            // validate the granter address
            let granter_addr = deps
                .api
                .addr_validate(&granter)
                .map_err(|_| ContractError::InvalidGranterAddress(granter.clone()))?;

            // get the grant settings from state
            let grant = GRANTS.load(deps.storage, granter_addr.clone())?;

            let grantee_addr = deps.api.addr_validate(&grant.grantee)?;

            // validate that the grant is currently active
            if env.block.time > grant.expiration {
                return Err(ContractError::Unauthorized {});
            }

            // validate that the executor is either the granter or the grantee
            if !info.sender.eq(&granter_addr) && !info.sender.eq(&grantee_addr) {
                return Err(ContractError::Unauthorized {});
            }

            let AllPendingRewardsResponse { total, rewards } =
                query_pending_rewards(&deps.querier, &granter_addr)?;

            // figure out what percentage of the rewards to send to the grantee
            let percentage_to_send = percentage
                .unwrap_or(grant.max_percentage)
                .min(grant.max_percentage);

            // get the list of tokens that the granter and grantee should each recieve
            let (grantee_coins, granter_coins) =
                partition_coins_by_percentage(percentage_to_send, total);

            let mut claim_rewards_msgs = vec![];

            // first set the withdraw rewards address to this contract
            claim_rewards_msgs.push(set_withdraw_rewards_msg(
                &granter_addr,
                &env.contract.address,
            )?);

            // claim all of the users rewards. these should now be sent into this contract
            claim_rewards_msgs.extend(withdraw_rewards_msgs(&granter_addr, rewards)?);

            // put the delegator's withdraw address back to them so they dont accidentally send us tokens
            claim_rewards_msgs.push(set_withdraw_rewards_msg(&granter_addr, &granter_addr)?);

            // wrap the messages for claiming into a single exec message as these will get done via native Authz
            let withdraw_rewards_exec_msg = exec_msg(&env.contract.address, claim_rewards_msgs)?;

            // send the grantee their share of the rewards
            let grantee_send_msg = BankMsg::Send {
                to_address: grantee_addr.to_string(),
                amount: grantee_coins,
            };

            // send the granter their share of the rewards
            let granter_send_msg = BankMsg::Send {
                to_address: granter_addr.to_string(),
                amount: granter_coins,
            };

            Ok(Response::default()
                .add_message(withdraw_rewards_exec_msg)
                .add_message(grantee_send_msg)
                .add_message(granter_send_msg)
                .add_attribute("action", "execute_withdraw_rewards_split")
                .add_attribute("granter", granter_addr)
                .add_attribute("grantee", grantee_addr)
                .add_attribute("percentage_split", percentage_to_send.to_string()))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Version {} => to_binary(&queries::query_version()).map_err(ContractError::Std),
        QueryMsg::ActiveGrantsByGranter(granter) => {
            // validate the granter address
            let granter = deps.api.addr_validate(&granter)?;
            // .map_err(|_| ContractError::InvalidGranterAddress(granter_address))?;

            let grant = GRANTS.load(deps.storage, granter)?;

            let grant: Option<AllowedWithdrawlSettings> =
                Option::from(grant).filter(|grant| env.block.time <= grant.expiration);

            to_binary(&grant).map_err(ContractError::Std)
        }
        QueryMsg::ActiveGrantsByGrantee(grantee) => {
            // validate the grantee address
            let grantee = deps.api.addr_validate(&grantee)?;
            // .map_err(|_| ContractError::InvalidGranteeAddress(grantee_address))?;

            // check all the grants to see if the grantee is the one being queried
            // and that the grant is active
            let grants = GRANTS
                .range(deps.storage, None, None, Order::Ascending)
                .filter_map(|item| {
                    if let Ok((_, withdrawl_setting)) = item {
                        if env.block.time <= (withdrawl_setting.expiration)
                            && withdrawl_setting.grantee.eq(&grantee)
                        {
                            return Some(withdrawl_setting);
                        }
                    }
                    None
                })
                // .map(|item| item.1)
                .collect::<Vec<_>>();

            to_binary(&grants).map_err(ContractError::Std)
        }
        QueryMsg::PendingRewards(delegator_address) => {
            let delegator = deps.api.addr_validate(&delegator_address)?;
            // .map_err(|_| ContractError::InvalidGranteeAddress(delegator_address))?;

            let rewards = query_pending_rewards(&deps.querier, &delegator)?;

            to_binary(&rewards).map_err(ContractError::Std)
        }
    }
}
