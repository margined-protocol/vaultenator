use cosmwasm_std::{OverflowError, StdError};
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("Strategy denom not initialised")]
    DenomNotInitialized {},

    #[error("Expired")]
    Expired {},

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid reply id")]
    InvalidReplyId,

    #[error("Invalid ownership, new owner cannot be the same as existing")]
    InvalidOwnership {},

    #[error("Invalid duration cannot be greater than {0}")]
    InvalidDuration(u64),

    #[error("Invalid funds")]
    InvalidFunds {},

    #[error("Invalid message")]
    InvalidMessage { msg: String },

    #[error("Invalid Pool Route: {reason:?}")]
    InvalidPoolRoute { reason: String },

    #[error("Contract is already open")]
    IsOpen {},

    #[error("Contract is not paused")]
    NotPaused {},

    #[error("Cannot perform action as contract is not open")]
    NotOpen {},

    #[error("Not found")]
    NotFound { msg: String },

    #[error("Owner not set")]
    NoOwner {},

    #[error("Overflow occurred: {0}")]
    Overflow(OverflowError),

    #[error("Cannot perform action as contract is paused")]
    Paused {},

    #[error("Proposal not found")]
    ProposalNotFound {},

    #[error("Strategy Cap Exceeded")]
    StrategyCapExceeded {},

    #[error("Serialization error")]
    SerializationError { msg: String },

    #[error("Unauthorized")]
    Unauthorized {},
}

impl From<serde_json::Error> for ContractError {
    fn from(err: serde_json::Error) -> Self {
        ContractError::SerializationError {
            msg: err.to_string(),
        }
    }
}
