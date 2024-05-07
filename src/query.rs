use crate::config::Configure;
use crate::state::ManageState;
use cosmwasm_std::{Deps, Env, StdError, StdResult, Uint128};
use cw_vault_standard::VaultInfoResponse;
use serde::Serialize;

pub trait Query<C, S>
where
    C: Configure + Serialize,
    S: ManageState + Serialize,
{
    fn query_config(deps: Deps) -> StdResult<C> {
        match C::get_from_storage(deps) {
            Ok(config) => Ok(config),
            Err(e) => Err(StdError::generic_err(e.to_string())),
        }
    }

    fn query_state(deps: Deps) -> StdResult<S> {
        match S::get_from_storage(deps) {
            Ok(state) => Ok(state),
            Err(e) => Err(StdError::generic_err(e.to_string())),
        }
    }

    fn query_info(deps: Deps, env: Env) -> StdResult<VaultInfoResponse>;
    fn query_preview_deposit(amount: Uint128, deps: Deps, env: Env) -> StdResult<Uint128>;
    fn query_preview_redeem(amount: Uint128, deps: Deps, env: Env) -> StdResult<Uint128>;
    fn query_total_assets(deps: Deps, env: Env) -> StdResult<Uint128>;
    fn query_total_vault_token_supply(deps: Deps, env: Env) -> StdResult<Uint128>;
    fn query_convert_to_shares(amount: Uint128, deps: Deps, env: Env) -> StdResult<Uint128>;
    fn query_convert_to_assets(amount: Uint128, deps: Deps, env: Env) -> StdResult<Uint128>;
}
