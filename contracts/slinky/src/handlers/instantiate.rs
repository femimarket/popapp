use crate::{
    contract::{
        AdapterResult, Slinky
    },
    msg::SlinkyInstantiateMsg,
    state::{Config, CONFIG},
};

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn instantiate_handler(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _module: Slinky,
    _msg: SlinkyInstantiateMsg,
) -> AdapterResult {
    let config: Config = Config {};

    CONFIG.save(deps.storage, &config)?;

    // Example instantiation that doesn't do anything
    Ok(Response::new())
}
