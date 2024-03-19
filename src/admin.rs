use crate::errors::ContractError;
use crate::state::ManageState;
use crate::state::OWNER;
use cosmwasm_std::{ensure, DepsMut, Event, MessageInfo, Response};
use serde::Serialize;

pub trait Administer<S>
where
    S: ManageState + Serialize,
{
    fn handle_open_contract(
        &self,
        mut deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        let mut state = S::get_from_storage(deps.as_ref())?;

        let is_open = S::is_contract_open(deps.as_ref())?;
        ensure!(!is_open, ContractError::IsOpen {});

        state.set_open(true);
        state.set_paused(false);

        state.save_to_storage(&mut deps)?;

        Ok(Response::new().add_event(Event::new("open_contract")))
    }

    fn handle_pause_contract(
        &self,
        mut deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        let mut state = S::get_from_storage(deps.as_ref())?;

        let is_paused = S::is_contract_paused(deps.as_ref())?;
        ensure!(!is_paused, ContractError::Paused {});

        state.set_paused(true);
        state.save_to_storage(&mut deps)?;

        Ok(Response::default().add_event(Event::new("paused")))
    }

    fn handle_unpause_contract(
        &self,
        mut deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        OWNER.assert_admin(deps.as_ref(), &info.sender)?;

        let mut state = S::get_from_storage(deps.as_ref())?;

        let is_paused = S::is_contract_paused(deps.as_ref())?;
        ensure!(is_paused, ContractError::NotPaused {});

        state.set_paused(false);
        state.save_to_storage(&mut deps)?;

        Ok(Response::default().add_event(Event::new("unpaused")))
    }
}
