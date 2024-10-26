use crate::{contract::{
    Pop, PopResult
}, msg::PopExecuteMsg, state::{CONFIG, COUNT}, PopError};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo, StdError};
use neutron_std::types::slinky::oracle::v1::OracleQuerier;
use neutron_std::types::slinky::types::v1::CurrencyPair;
use crate::state::Trade;
use slinky::api::SlinkyApi;

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    module: Pop,
    msg: PopExecuteMsg,
) -> PopResult {
    match msg {
        PopExecuteMsg::UpdateConfig {} => update_config(deps, env, info, module),
        PopExecuteMsg::Increment {} => increment(deps, module),
        PopExecuteMsg::Reset { count } => reset(deps, env, info, count, module),
        PopExecuteMsg::Trade {data:t} => trade(deps, env, info, t, module),
    }
}

/// Update the configuration of the app
fn update_config(deps: DepsMut, env: Env, msg_info: MessageInfo, module: Pop) -> PopResult {
    // Only the admin should be able to call this
    module.admin.assert_admin(deps.as_ref(), &env, &msg_info.sender)?;
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(module.response("update_config"))
}

fn increment(deps: DepsMut, module: Pop) -> PopResult {
    COUNT.update(deps.storage, |count| PopResult::Ok(count + 1))?;

    Ok(module.response("increment"))
}

fn reset(deps: DepsMut, env: Env, info: MessageInfo, count: i32, module: Pop) -> PopResult {
    module.admin.assert_admin(deps.as_ref(), &env, &info.sender)?;
    COUNT.save(deps.storage, &count)?;

    Ok(module.response("reset"))
}

fn trade(deps: DepsMut, env: Env, info: MessageInfo, trade: Trade, module: Pop) -> PopResult {
    module.admin.assert_admin(deps.as_ref(), &env, &info.sender)?;

    let slinky = module.slinky(deps.as_ref());

    let split = trade.asset.split("_").collect::<Vec<&str>>();

    let price_response = slinky.pair(
        split[0].to_string(),
        split[1].to_string()
    )?;




    Ok(module.response("reset"))
}
