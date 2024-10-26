use crate::contract::Pop;

use cosmwasm_schema::QueryResponses;
use neutron_std::types::slinky::oracle::v1::{GetAllCurrencyPairsResponse, GetPriceResponse, OracleQuerier};
use neutron_std::types::slinky::types::v1::CurrencyPair;
use crate::state::Trade;

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(Pop, PopExecuteMsg, PopQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
pub struct PopInstantiateMsg {
    pub count: i32,
}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum PopExecuteMsg {
    UpdateConfig {},
    /// Increment count by 1
    Increment {},
    /// Admin method - reset count
    Reset {
        /// Count value after reset
        count: i32,
    },
    Trade(Trade),
}

#[cosmwasm_schema::cw_serde]
pub struct PopMigrateMsg {}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum PopQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[cw_orch(fn_name("query_trade"))]
    #[returns(TradeResponse)]
    Trade {
        base: String,
        quote: String
    },
    // #[cw_orch(fn_name("query_ps"))]
    // #[returns(TradeResponse)]
    // Pairs {
    //     base: String,
    //     quote: String
    // },

}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct TradeResponse {
    pub price: GetPriceResponse,
    pub trade: Trade
}

