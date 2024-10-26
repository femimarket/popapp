use crate::{contract::{
    Pop, PopResult
}, msg::{ConfigResponse, TradeResponse, PopQueryMsg}, state::{CONFIG, COUNT}, PopError};

use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdError, StdResult};
use neutron_std::types::slinky::oracle::v1::{GetAllCurrencyPairsResponse, GetPriceResponse, OracleQuerier};
use neutron_std::types::slinky::types::v1::CurrencyPair;
use slinky::api::{Slinky, SlinkyApi};
use crate::state::{OrderType, Trade};

pub fn query_handler(
    deps: Deps,
    _env: Env,
    module: &Pop,
    msg: PopQueryMsg,
) -> PopResult<Binary> {
    match msg {
        PopQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        PopQueryMsg::Trade {base,quote} => to_json_binary(&query_trade(deps,_env, module, base,quote)?),
    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_trade(deps: Deps, env: Env, module: &Pop, base: String,quote: String) -> PopResult<TradeResponse> {
    let slinky = module.slinky(deps);

    let price_response = slinky.pair(base, quote)?;

    // if price_response.nonce == 0 {x
    //     return Err(
    //         StdError::generic_err("Count not instantiated yet")
    //     )
    //     // return Err(
    //     //     StdError::generic_err("Count not instantiated yet")
    //     // )
    //     // return Err(ContractError::PriceIsNil {
    //     //     symbol: pair.base.clone(),
    //     //     quote: pair.quote.clone(),
    //     // });
    // }
    //
    // let price = price_response
    //     .price
    //     .ok_or_else(|| {
    //         // ContractError::PriceNotAvailable {
    //         //     symbol: pair.base.clone(),
    //         //     quote: pair.quote.clone(),
    //         // }
    //         // let err = format!("symbol:{}, quote:{}")
    //         return StdError::generic_err("PriceNotAvailable")
    //     })?;
    //
    // let max_price_age: u64 = 3; // adjust based on appetite for price freshness
    // let price_age = env.block.height - price.block_height;
    // if price_age > max_price_age {
    //     return Err(
    //         StdError::generic_err("price is too old")
    //     );
    // }


    // Ok(module.response("reset"))
    Ok(TradeResponse {
        price: price_response,
        trade:Trade{
            price: Default::default(),
            dt: 0,
            asset: "".to_string(),
            qty: Default::default(),
            order_type: OrderType::Market,
            buy: false,
            expire_time: 0,
            tp: Default::default(),
            sl: Default::default(),
        }
    })
}

