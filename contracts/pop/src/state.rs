use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};

#[cosmwasm_schema::cw_serde]
pub struct Config {}

pub const CONFIG: Item<Config> = Item::new("config");
pub const COUNT: Item<i32> = Item::new("count");
pub const TRADES: Item<Map<Addr,Vec<Trade>>> = Item::new("trades");


#[cw_serde]
pub struct Trade {
    pub price: Decimal,
    pub dt: i64,
    pub asset: String,
    pub qty: Decimal,
    pub order_type: OrderType,
    pub buy: bool,
    pub expire_time: i64,
    pub tp: Decimal,
    pub sl: Decimal
}

#[cw_serde]
pub enum OrderType {
    Market = 1,
}