use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, Coin, CosmosMsg, Decimal, StdResult, WasmMsg};

use crate::msg::ExecuteMsg;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

/// combines two vectors of coins into just one where any overlapping denoms are added together
pub fn sum_coins(xs: Vec<Coin>, ys: Vec<Coin>) -> Vec<Coin> {
    let mut coins = xs;
    for y in ys {
        let mut found = false;
        for x in coins.iter_mut() {
            if x.denom == y.denom {
                x.amount += y.amount;
                found = true;
                break;
            }
        }
        if !found {
            coins.push(y);
        }
    }
    coins
}

/// Splits the given coins into two vectors based on the percentage given
pub fn partition_coins_by_percentage(
    percentage: Decimal,
    coins: Vec<Coin>,
) -> (Vec<Coin>, Vec<Coin>) {
    let mut percentage_coins = vec![];
    let mut remaining_coins = vec![];

    for Coin { amount, denom } in coins {
        let amount_to_send = amount * percentage;
        let amount_to_remain = amount - amount_to_send;

        percentage_coins.push(Coin {
            denom: denom.clone(),
            amount: amount_to_send,
        });
        remaining_coins.push(Coin {
            denom,
            amount: amount_to_remain,
        });
    }

    (percentage_coins, remaining_coins)
}
