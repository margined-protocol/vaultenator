use serde::Serialize;

use crate::{
    admin::Administer, config::Configure, handlers::Handle, ownership::Own, query::Query,
    reply::ReplyHandler, state::ManageState,
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
    // Commenting this for now
    // Generics in enums not easy to solve
    // fn instantiate<M>(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     msg: M,
    // ) -> Result<Response, ContractError>
    // where
    //     M: Serialize + DeserializeOwned;

    // fn execute<M>(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     msg: M,
    // ) -> Result<Response, ContractError>
    // where
    //     M: Serialize + DeserializeOwned;

    // fn query<M>(&self, deps: Deps, env: Env, msg: M) -> StdResult<Binary>;

    // fn reply<M>(&self, deps: DepsMut, env: Env, msg: M) -> Result<Response, ContractError>;

    // fn migrate<M>(&self, deps: DepsMut, env: Env, msg: M) -> Result<Response, ContractError>;
}
