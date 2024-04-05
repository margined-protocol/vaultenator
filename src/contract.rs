use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw_vault_standard::msg::VaultStandardInfoResponse;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    admin::Administer,
    config::Configure,
    errors::ContractError,
    handlers::Handle,
    msg::{
        ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, MigrateMsg, QueryMsg,
        VaultenatorExtensionExecuteMsg, VaultenatorExtensionQueryMsg,
    },
    ownership::Own,
    query::Query,
    reply::ReplyHandler,
    state::{ManageState, OWNER, OWNERSHIP_PROPOSAL},
};

pub trait Describe {
    const CONTRACT_NAME: &'static str;
    const VAULT_STANDARD_VERSION: u16;
    const VAULT_STANDARD_EXTENSIONS: [&'static str; 2];
}

pub trait Vaultenator<Config, State>:
    Describe
    + Own
    + Handle<Config, State>
    + Administer<State>
    + Query<Config, State>
    + ReplyHandler<Config>
where
    Config: Configure + Serialize,
    State: ManageState + Serialize,
{
    fn instantiate<Msg>(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Msg,
    ) -> Result<Response, ContractError>
    where
        Msg: Serialize + DeserializeOwned,
    {
        self.handle_instantiate(deps, env, info, msg)
    }

    fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Deposit {
                amount: _,
                recipient: _,
            } => self.handle_deposit(deps, env, info),
            ExecuteMsg::Redeem {
                recipient: _,
                amount: _,
            } => self.handle_redeem(deps, env, info),
            ExecuteMsg::VaultExtension(msg) => match msg {
                ExtensionExecuteMsg::Vaultenator(msg) => match msg {
                    VaultenatorExtensionExecuteMsg::ClaimOwnership {} => {
                        self.handle_claim_ownership(deps, info, env, OWNER, OWNERSHIP_PROPOSAL)
                    }
                    VaultenatorExtensionExecuteMsg::UnPause {} => {
                        self.handle_unpause_contract(deps, info)
                    }
                    VaultenatorExtensionExecuteMsg::SetOpen {} => {
                        self.handle_open_contract(deps, info)
                    }
                    VaultenatorExtensionExecuteMsg::UpdateConfig {} => {
                        unimplemented!("not implemented")
                        //handle_update_config(deps, info, new_config)
                    }
                    VaultenatorExtensionExecuteMsg::RejectOwner {} => self
                        .handle_ownership_proposal_rejection(deps, info, OWNER, OWNERSHIP_PROPOSAL),
                    VaultenatorExtensionExecuteMsg::Pause {} => {
                        self.handle_pause_contract(deps, info)
                    }
                    VaultenatorExtensionExecuteMsg::ProposeNewOwner {
                        new_owner,
                        duration,
                    } => self.handle_ownership_proposal(
                        deps,
                        info,
                        env,
                        new_owner,
                        duration,
                        OWNER,
                        OWNERSHIP_PROPOSAL,
                    ),
                },
            },
        }
    }

    fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::VaultStandardInfo {} => to_json_binary(&VaultStandardInfoResponse {
                version: Self::VAULT_STANDARD_VERSION,
                extensions: Self::VAULT_STANDARD_EXTENSIONS
                    .iter()
                    .map(|&s| s.into())
                    .collect(),
            }),
            QueryMsg::Info {} => {
                unimplemented!("not implemented")
            }
            QueryMsg::PreviewDeposit { amount: _ } => {
                unimplemented!("not implemented")
            }
            QueryMsg::PreviewRedeem { amount: _ } => {
                unimplemented!("not implemented")
            }
            QueryMsg::TotalAssets {} => {
                unimplemented!("not implemented")
            }
            QueryMsg::TotalVaultTokenSupply {} => {
                unimplemented!("not implemented")
            }
            QueryMsg::ConvertToShares { amount: _ } => {
                unimplemented!("not implemented")
            }
            QueryMsg::ConvertToAssets { amount: _ } => {
                unimplemented!("not implemented")
            }
            QueryMsg::VaultExtension(msg) => match msg {
                ExtensionQueryMsg::Vaultenator(msg) => match msg {
                    VaultenatorExtensionQueryMsg::Config {} => Self::query_config(deps),
                    VaultenatorExtensionQueryMsg::State {} => Self::query_state(deps),
                    VaultenatorExtensionQueryMsg::Owner {} => {
                        let owner = Self::query_owner(deps)?;
                        to_json_binary(&owner)
                    }
                    VaultenatorExtensionQueryMsg::OwnershipProposal {} => {
                        let owner = Self::query_ownership_proposal(deps, OWNERSHIP_PROPOSAL)?;
                        to_json_binary(&owner)
                    }
                },
            },
        }
    }

    fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        self.handle_reply(deps, env, msg)
    }

    fn migrate(
        &self,
        _deps: DepsMut,
        _env: Env,
        _msg: MigrateMsg,
    ) -> Result<Response, ContractError> {
        unimplemented!("not implemented")
    }
}
