use crate::contract::Slinky;

use abstract_adapter::objects::AccountId;
use cosmwasm_schema::QueryResponses;
use neutron_std::types::slinky::oracle::v1::{GetAllCurrencyPairsResponse, GetPriceResponse, OracleQuerier};
use neutron_std::types::slinky::types::v1::CurrencyPair;

/// Adapter execute messages


// This is used for type safety and re-exporting the contract endpoint structs.
abstract_adapter::adapter_msg_types!(Slinky, SlinkyExecuteMsg, SlinkyQueryMsg);

/// Adapter instantiate message
#[cosmwasm_schema::cw_serde]
pub struct SlinkyInstantiateMsg {}

#[cosmwasm_schema::cw_serde]
pub enum SlinkyExecuteMsg {
    /// Admin method: Update the configuration of the adapter
    UpdateConfig {},
}

/// Adapter query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum SlinkyQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(GetAllCurrencyPairsResponse)]
    Pairs {},
    #[returns(GetPriceResponse)]
    Pair {
        base: String,
        quote: String
    },
}

#[cosmwasm_schema::cw_serde]
pub struct ConfigResponse {}

#[cosmwasm_schema::cw_serde]
pub struct StatusResponse {
    pub status: Option<String>,
}
