use crate::{contract::{
    Pop, PopResult
}, msg::PopInstantiateMsg, state::{Config, CONFIG, COUNT}, PopError};

use cosmwasm_std::{DepsMut, Env, HexBinary, MessageInfo, Response, StdError};
use quartz_common::contract::handler::RawHandler;
use crate::state::{EncryptedTrade, STATE, TRADE_REQUESTS};

pub fn instantiate_handler(
    mut deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _module: Pop,
    msg: PopInstantiateMsg,
) -> PopResult {
    let config: Config = Config {};

    msg.quartz.handle_raw(deps.branch(), &_env, &_info)
        .map_err(|x|PopError::Std(StdError::generic_err(x.to_string())))?;

    CONFIG.save(deps.storage, &config)?;
    COUNT.save(deps.storage, &2)?;

    let state = HexBinary::from(&[0x00]);
    STATE.save(deps.storage, &state)?;

    let requests: Vec<EncryptedTrade> = Vec::new();
    TRADE_REQUESTS.save(deps.storage, &requests)?;

    Ok(Response::new())
}
