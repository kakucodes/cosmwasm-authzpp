use cosmwasm_std::{Addr, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg},
    ContractError,
};

pub struct OutpostContract(Addr);

impl OutpostContract {
    pub fn addr(&self) -> &Addr {
        &self.0
    }

    pub fn store_code(app: &mut App) -> u64 {
        let contract = ContractWrapper::new(execute, instantiate, query);
        app.store_code(Box::new(contract))
    }

    #[track_caller]
    pub fn instantiate<'a>(
        app: &mut App,
        code_id: u64,
        sender: &Addr,
        admin: impl Into<Option<&'a Addr>>,
        label: &str,
        // funds: &[Coin],
        instantiate_msg: &InstantiateMsg,
    ) -> StdResult<OutpostContract> {
        let admin = admin.into();

        app.instantiate_contract(
            code_id,
            sender.clone(),
            &instantiate_msg,
            &vec![],
            label,
            admin.map(Addr::to_string),
        )
        .map_err(|err| err.downcast().unwrap())
        .map(OutpostContract)
    }
}
