use crate::{contract::{
    Pop, PopResult
}, msg::PopExecuteMsg, state::{CONFIG, COUNT}, PopError};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{DepsMut, Env, MessageInfo, StdError};
use neutron_std::types::slinky::oracle::v1::OracleQuerier;
use neutron_std::types::slinky::types::v1::CurrencyPair;
use crate::state::Trade;

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
        PopExecuteMsg::Trade(t) => trade(deps, env, info, t, module),
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

    let querier = OracleQuerier::new(&deps.querier);
    let currency_pair = CurrencyPair {
        base: "EUR".to_string(),
        quote: "USD".to_string(),
    };
    let price_response = querier.get_price(Some(currency_pair))?;

    if price_response.nonce == 0 {
        return Err(
            PopError::Std(StdError::generic_err("Count not instantiated yet"))
        )
        // return Err(
        //     StdError::generic_err("Count not instantiated yet")
        // )
        // return Err(ContractError::PriceIsNil {
        //     symbol: pair.base.clone(),
        //     quote: pair.quote.clone(),
        // });
    }

    let price = price_response
        .price
        .ok_or_else(|| {
            // ContractError::PriceNotAvailable {
            //     symbol: pair.base.clone(),
            //     quote: pair.quote.clone(),
            // }
            // let err = format!("symbol:{}, quote:{}")
            return PopError::Std(StdError::generic_err("PriceNotAvailable"))
        })?;

    let max_price_age: u64 = 3; // adjust based on appetite for price freshness
    let price_age = env.block.height - price.block_height;
    if price_age > max_price_age {
        return Err(
            PopError::Std(StdError::generic_err("price is too old"))
        );
    }


    Ok(module.response("reset"))
}
