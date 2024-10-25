use crate::contract::{
    Pop, PopResult
};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, Reply};

pub fn instantiate_reply(_deps: DepsMut, _env: Env, module: Pop, _reply: Reply) -> PopResult {
    Ok(module.response("instantiate_reply"))
}
