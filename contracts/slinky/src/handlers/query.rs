use crate::{
    contract::{
        AdapterResult, Slinky
    },
    msg::{ConfigResponse, SlinkyQueryMsg, StatusResponse},
    state::{CONFIG, STATUS},
};

use abstract_adapter::objects::AccountId;
use cosmwasm_std::{to_json_binary, Binary, Deps, Env, StdResult};
use neutron_std::types::slinky::oracle::v1::{GetAllCurrencyPairsResponse, GetPriceResponse, OracleQuerier};
use neutron_std::types::slinky::types::v1::CurrencyPair;

pub fn query_handler(
    deps: Deps,
    _env: Env,
    _module: &Slinky,
    msg: SlinkyQueryMsg,
) -> AdapterResult<Binary> {
    match msg {
        SlinkyQueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        SlinkyQueryMsg::Pairs {} => to_json_binary(&query_pairs(deps)?),
        SlinkyQueryMsg::Pair {base,quote} => to_json_binary(&query_pair(deps,_env,base,quote)?),

    }
    .map_err(Into::into)
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let _config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {})
}

fn query_pair(deps: Deps, env: Env,base:String,quote:String) -> StdResult<GetPriceResponse> {
    let querier = OracleQuerier::new(&deps.querier);
    let currency_pair = CurrencyPair {
        base,
        quote,
    };
    let price_response = querier.get_price(Some(currency_pair))?;
    Ok(price_response)
}

fn query_pairs(deps: Deps) -> StdResult<GetAllCurrencyPairsResponse>{
    let querier = OracleQuerier::new(&deps.querier);
    let price_response = querier.get_all_currency_pairs()?;
    // Ok(module.response("reset"))
    Ok(price_response)
}
