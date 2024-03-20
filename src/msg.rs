use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, CosmosMsg};
use cw_vault_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};

use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

#[cw_serde]
pub struct InstantiateMsg {
    pub base_denom: String,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExtensionExecuteMsg {
    Vaultenator(VaultenatorExtensionExecuteMsg),
}

#[cw_serde]
pub enum ExtensionQueryMsg {
    Vaultenator(VaultenatorExtensionQueryMsg),
}

#[cw_serde]
pub enum VaultenatorExtensionQueryMsg {
    Owner {},
    Config {},
    State {},
    OwnershipProposal {},
}

#[cw_serde]
pub enum VaultenatorExtensionExecuteMsg {
    ClaimOwnership {},
    Pause {},
    ProposeNewOwner { new_owner: String, duration: u64 },
    RejectOwner {},
    SetOpen {},
    UpdateConfig {},
    UnPause {},
}

pub type ExecuteMsg = VaultStandardExecuteMsg<ExtensionExecuteMsg>;
pub type QueryMsg = VaultStandardQueryMsg<ExtensionQueryMsg>;

pub fn create_denom_message(contract_address: &Addr, subdenom: String) -> CosmosMsg {
    MsgCreateDenom {
        sender: contract_address.to_string(),
        subdenom,
    }
    .into()
}
