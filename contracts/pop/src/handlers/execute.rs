use std::str::FromStr;
use abstract_app::sdk::TransferInterface;
use crate::{contract::{
    Pop, PopResult
}, msg::PopExecuteMsg, state::{CONFIG, COUNT}, PopError};

use abstract_app::traits::AbstractResponse;
use cosmwasm_std::{Decimal, DepsMut, Env, Event, HexBinary, MessageInfo, Response, StdError, StdResult};
use neutron_std::types::slinky::oracle::v1::{GetPriceResponse, OracleQuerier};
use neutron_std::types::slinky::types::v1::CurrencyPair;
use quartz_common::contract::handler::RawHandler;
use crate::state::{EncryptedTrade, Request, Trade, STATE, TRADES, TRADE_REQUESTS};
use slinky::api::SlinkyApi;
use crate::msg::{AttestedMsg, ConfigResponse, TradeResponse, UpdateMsg};

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
        PopExecuteMsg::TradeCiphertext {ciphertext,digest} => trade_ciphertext(deps, env, info, ciphertext,digest, module),
        PopExecuteMsg::Update { data }  => execute_update(deps, env, info, data, module),
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

    let price_response:GetPriceResponse = slinky.pair(
        split[0].to_string(),
        split[1].to_string()
    )?;

    // let bank = module.ibc_client().ics20_transfer()
    // bank.send(
    //
    // )


    let mut trades = TRADES.load(deps.storage)?;
    // let mut trades2 = TRADES2.load(deps.storage)?;

    // let new_trade = Trade{
    //     price: Decimal::from_str(&price_response.price.unwrap().price.to_string()).unwrap(),
    //     dt: env.block.time.to_string().parse().unwrap(),
    //     asset: trade.asset.to_string(),
    //     ..trade
    // };


    // trades.push()
    //
    // ts.push(HexBinary::from(serde_json_wasm::to_vec(&new_trade).unwrap()));
    //
    // let _ = trades.save(deps.storage,info.sender.to_string(),&ts).unwrap();
    //
    // // let ts = HexBinary::from(ts);
    // // let _ = trades2.save(deps.storage,&info.sender.to_string(),&ts).unwrap();
    //




    Ok(module.response("reset"))
}

fn trade_ciphertext(deps: DepsMut, env: Env, info: MessageInfo, ciphertext: Vec<u8>, digest:HexBinary, module: Pop)
    -> PopResult<Response> {

    let slinky = module.slinky(deps.as_ref());

    let price_response:GetPriceResponse = slinky.pair(
        "BTC".to_string(),
        "USD".to_string()
    )?;

    let enc_trade = EncryptedTrade {
        price: Decimal::from_str(&price_response.price.unwrap().price.to_string()).unwrap(),
        data: HexBinary::from(ciphertext)
    };

    let mut requests = TRADE_REQUESTS.load(deps.storage)?;
    requests.push(enc_trade);

    TRADE_REQUESTS.save(deps.storage, &requests)?;

    let event = Event::new("trade_request").add_attribute("action", "user");
    let resp = Response::new().add_event(event);

    Ok(resp)
}

fn execute_update(mut deps: DepsMut, env: Env, info: MessageInfo, data: AttestedMsg<UpdateMsg>, module: Pop)
                  -> PopResult<Response> {
    let _ = data
        .clone()
        .handle_raw(deps.branch(), &env, &info)
        .map_err(|x|PopError::Std(StdError::generic_err(x.to_string())))?;

    STATE.save(deps.storage, &data.msg.0.ciphertext)?;

    let event = Event::new("execute_update").add_attribute("action", "user");
    let resp = Response::new().add_event(event);

    Ok(resp)
}
