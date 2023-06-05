use cosmos_sdk_proto::cosmos::authz::v1beta1::MsgExec;
use cosmos_sdk_proto::cosmos::distribution::v1beta1::MsgSetWithdrawAddress;
use cosmos_sdk_proto::traits::{Message, MessageExt};
use cosmos_sdk_proto::{
    cosmos::distribution::v1beta1::MsgWithdrawDelegatorReward, prost::EncodeError, Any,
};
use cosmwasm_std::{Addr, Binary, CosmosMsg};

use crate::{
    msg::{AllPendingRewardsResponse, PendingReward},
    ContractError,
};

pub fn withdraw_rewards_msgs(
    target_address: &Addr,
    pending_rewards: Vec<PendingReward>,
) -> Result<Vec<Any>, ContractError> {
    let withdraw_rewards_msgs: Vec<Any> = pending_rewards
        .iter()
        .map(|PendingReward { validator, .. }| {
            MsgWithdrawDelegatorReward {
                validator_address: validator.to_string(),
                delegator_address: target_address.to_string(),
            }
            .to_any()
        })
        .collect::<Result<Vec<_>, EncodeError>>()?;

    Ok(withdraw_rewards_msgs)
}

/// Creates a MsgSetWithdrawAddress message for changing a wallet's delegation rewards withdrawal address
pub fn set_withdraw_rewards_msg(
    delegator_address: &Addr,
    target_withdraw_address: &Addr,
) -> Result<Any, ContractError> {
    let set_withdraw_address_msg = MsgSetWithdrawAddress {
        delegator_address: delegator_address.to_string(),
        withdraw_address: target_withdraw_address.to_string(),
    }
    .to_any()?;

    Ok(set_withdraw_address_msg)
}

/// Creates a MsgExec message
pub fn exec_msg(grantee: &Addr, any_msgs: Vec<Any>) -> Result<CosmosMsg, EncodeError> {
    let exec = MsgExec {
        grantee: grantee.to_string(),
        msgs: any_msgs,
    };

    Ok(CosmosMsg::Stargate {
        type_url: "/cosmos.authz.v1beta1.MsgExec".to_string(),
        value: Binary::from(exec.encode_to_vec()),
    })
}
