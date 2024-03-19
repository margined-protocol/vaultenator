use crate::errors::ContractError;
use cw_storage_plus::Item;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::ownership::OwnerProposal;
// use crate::structs::{Config, State};
use cosmwasm_std::{Deps, DepsMut, Env};
use cw_controllers::Admin;

pub const DEFAULT_STRATEGY_CAP: u128 = 10_000_000_000_000_000_000_000u128;
pub const OWNER: Admin = Admin::new("owner");
// pub const STATE: Item<State> = Item::new("state");
// pub const CONFIG: Item<Config> = Item::new("config");
pub const OWNERSHIP_PROPOSAL: Item<OwnerProposal> = Item::new("ownership_proposals");
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub trait ManageState: Serialize + DeserializeOwned + Sized {
    const STATE_KEY: &'static str;

    fn is_contract_open(deps: Deps) -> Result<bool, ContractError>;
    fn is_contract_paused(deps: Deps) -> Result<bool, ContractError>;
    fn set_open(&mut self, open: bool);
    fn set_paused(&mut self, paused: bool);

    fn init_state(deps: &mut DepsMut, env: &Env) -> Result<(), ContractError>
    where
        Self: Sized;

    fn update_state(&mut self, deps: &mut DepsMut) -> Result<(), ContractError>;

    fn save_to_storage(&self, deps: &mut DepsMut) -> Result<(), ContractError> {
        let state_item = Item::<Self>::new(Self::STATE_KEY);
        state_item
            .save(deps.storage, self)
            .map_err(ContractError::from)
    }

    fn get_from_storage(deps: Deps) -> Result<Self, ContractError>
    where
        Self: Sized,
    {
        let state_item = Item::<Self>::new(Self::STATE_KEY);
        state_item.load(deps.storage).map_err(ContractError::from)
    }

    fn is_open_and_unpaused(deps: Deps) -> Result<(), ContractError> {
        if Self::is_contract_open(deps)? {
            return Err(ContractError::IsOpen {});
        }
        if !Self::is_contract_paused(deps)? {
            return Err(ContractError::Paused {});
        }
        Ok(())
    }
}
