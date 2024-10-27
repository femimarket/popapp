use cosmwasm_std::HexBinary;
use cw_storage_plus::{Item, Map};

use crate::msg::execute::{Request, TradeRequest};

pub const STATE: Item<HexBinary> = Item::new("state");
pub const TRADE_STATE: Item<HexBinary> = Item::new("trade_state");
pub const REQUESTS: Item<Vec<Request>> = Item::new("requests");
pub const TRADE_REQUESTS: Item<Vec<TradeRequest>> = Item::new("trade_requests");
pub const DENOM: Item<String> = Item::new("donation_denom");
pub const BALANCES: Map<&str, HexBinary> = Map::new("balances");
