use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, HexBinary, Int64};
use cw_storage_plus::{Item, Map};

#[cosmwasm_schema::cw_serde]
pub struct Config {}

pub const TRADE_REQUESTS: Item<Vec<EncryptedTrade>> = Item::new("trade_requests");
pub const STATE: Item<HexBinary> = Item::new("state");

pub const CONFIG: Item<Config> = Item::new("config");
pub const COUNT: Item<i32> = Item::new("count");
pub const TRADES: Item<Vec<HexBinary>> = Item::new("trades");

// pub const TRADES2: Item<Map<&str, HexBinary>> = Item::new("trades2");

#[cw_serde]
pub enum Request {
    PriceRequest(Vec<u8>,Vec<u8>),
}

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
pub struct EncryptedTrade {
    pub price: Decimal,
    pub data: HexBinary,
}

#[cw_serde]
pub struct DecryptedTrade {
    pub price: Decimal,
    pub data: Trade,
}

#[cw_serde]
pub enum OrderType {
    Market = 1,
}
