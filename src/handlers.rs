use crate::config::Configure;
use crate::contract::Describe;
use crate::errors::ContractError;
use crate::state::ManageState;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use serde::{de::DeserializeOwned, Serialize};

pub const CREATE_STRATEGY_DENOM_REPLY_ID: u64 = 1u64;

pub trait Handle<C, S>: Describe
where
    C: Configure,
    S: ManageState,
{
    fn handle_instantiate<M>(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: M,
    ) -> Result<Response, ContractError>
    where
        M: Serialize + DeserializeOwned;

    fn handle_update_config(
        &self,
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError>;

    fn handle_deposit(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError>;

    fn handle_redeem(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError>;
}
