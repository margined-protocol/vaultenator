use crate::errors::ContractError;
use cosmwasm_std::{Deps, DepsMut};
use cw_storage_plus::Item;
use serde::{de::DeserializeOwned, Serialize};

pub trait Configure: Serialize + DeserializeOwned + Sized {
    const CONFIG_KEY: &'static str;

    fn init_config<M>(deps: &mut DepsMut, msg: &M) -> Result<Self, ContractError>
    where
        M: Serialize + DeserializeOwned,
        Self: Sized;

    fn update_strategy_denom(&mut self, denom: String);

    fn save_to_storage(&self, deps: &mut DepsMut) -> Result<(), ContractError> {
        let config_item = Item::<Self>::new(Self::CONFIG_KEY);
        config_item
            .save(deps.storage, self)
            .map_err(ContractError::from)
    }

    fn get_from_storage(deps: Deps) -> Result<Self, ContractError> {
        let config_item = Item::<Self>::new(Self::CONFIG_KEY);
        config_item.load(deps.storage).map_err(ContractError::from)
    }
}
