use cosmwasm_std::{Addr, CosmosMsg};

use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

pub fn create_denom_message(contract_address: &Addr, subdenom: String) -> CosmosMsg {
    MsgCreateDenom {
        sender: contract_address.to_string(),
        subdenom,
    }
    .into()
}
