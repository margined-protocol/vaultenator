use crate::config::Configure;
use crate::state::ManageState;
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdError, StdResult, Uint128};
use serde::Serialize;

pub trait Query<C, S>
where
    C: Configure + Serialize,
    S: ManageState + Serialize,
{
    fn query_config(deps: Deps) -> StdResult<Binary> {
        match C::get_from_storage(deps) {
            Ok(config) => to_json_binary(&config),
            Err(e) => Err(StdError::generic_err(e.to_string())),
        }
    }

    fn query_state(deps: Deps) -> StdResult<Binary> {
        match S::get_from_storage(deps) {
            Ok(config) => to_json_binary(&config),
            Err(e) => Err(StdError::generic_err(e.to_string())),
        }
    }

    fn query_info(deps: Deps, env: Env) -> StdResult<Binary>;
    fn query_preview_deposit(amount: Uint128, deps: Deps, env: Env) -> StdResult<Binary>;
    fn query_preview_redeem(amount: Uint128, deps: Deps, env: Env) -> StdResult<Binary>;
    fn query_total_assets(deps: Deps, env: Env) -> StdResult<Binary>;
    fn query_total_vault_token_supply(deps: Deps, env: Env) -> StdResult<Binary>;
    fn query_convert_to_shares(amount: Uint128, deps: Deps, env: Env) -> StdResult<Binary>;
    fn query_convert_to_assets(amount: Uint128, deps: Deps, env: Env) -> StdResult<Binary>;
}
