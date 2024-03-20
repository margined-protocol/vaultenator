use crate::config::Configure;
use crate::contract::Describe;
use crate::errors::ContractError;
use crate::msg::{create_denom_message, InstantiateMsg};
use crate::state::ManageState;
use crate::state::{CONTRACT_VERSION, OWNER};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, SubMsg};
use cw2::set_contract_version;

pub const CREATE_STRATEGY_DENOM_REPLY_ID: u64 = 1u64;

pub trait Handle<C, S>: Describe
where
    C: Configure,
    S: ManageState,
{
    fn handle_instantiate(
        &self,
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(
            deps.storage,
            format!("crates.io:{}", Self::CONTRACT_NAME),
            CONTRACT_VERSION,
        )?;

        C::init_config(&mut deps, &msg)?;
        S::init_state(&mut deps, &env)?;

        OWNER.set(deps, Some(info.sender))?;

        let create_denom_sub_msg = SubMsg::reply_always(
            create_denom_message(&env.contract.address, Self::CONTRACT_NAME.to_string()),
            CREATE_STRATEGY_DENOM_REPLY_ID,
        );

        Ok(Response::new()
            .add_submessages([create_denom_sub_msg])
            .add_attribute("action", "instantiate"))
    }

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
