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
                    VaultenatorExtensionExecuteMsg::RejectOwner {} => self
                        .handle_ownership_proposal_rejection(deps, info, OWNER, OWNERSHIP_PROPOSAL),
                    VaultenatorExtensionExecuteMsg::SetOpen {} => {
                        self.handle_open_contract(deps, info)
                    }
                    VaultenatorExtensionExecuteMsg::Unpause {} => {
                        self.handle_unpause_contract(deps, info)
                    }
                    VaultenatorExtensionExecuteMsg::UpdateConfig {} => {
                        unimplemented!("not implemented")
                        //handle_update_config(deps, info, new_config)
                    }
                },
            },
        }
    }

    fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::VaultStandardInfo {} => to_json_binary(&VaultStandardInfoResponse {
                version: Self::VAULT_STANDARD_VERSION,
                extensions: Self::VAULT_STANDARD_EXTENSIONS
                    .iter()
                    .map(|&s| s.into())
                    .collect(),
            }),
            QueryMsg::Info {} => Self::query_info(deps, env),
            QueryMsg::PreviewDeposit { amount } => Self::query_preview_deposit(amount, deps, env),
            QueryMsg::PreviewRedeem { amount } => Self::query_preview_redeem(amount, deps, env),
            QueryMsg::TotalAssets {} => Self::query_total_assets(deps, env),
            QueryMsg::TotalVaultTokenSupply {} => Self::query_total_vault_token_supply(deps, env),
            QueryMsg::ConvertToShares { amount } => {
                Self::query_convert_to_shares(amount, deps, env)
            }
            QueryMsg::ConvertToAssets { amount } => {
                Self::query_convert_to_assets(amount, deps, env)
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

    fn migrate(&self, deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
        self.handle_migrate(deps, env, msg)
    }
}
