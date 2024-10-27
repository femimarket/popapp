use crate::contract::Pop;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, HexBinary, Uint128};
use neutron_std::types::slinky::oracle::v1::{GetAllCurrencyPairsResponse, GetPriceResponse, GetPricesResponse, OracleQuerier};
use neutron_std::types::slinky::types::v1::CurrencyPair;
use crate::state::{EncryptedTrade, Trade};
use quartz_common::contract::handler::RawHandler;
use quartz_common::contract::{
    msg::execute::attested::{RawAttested, RawAttestedMsgSansHandler, RawDefaultAttestation},
    prelude::*,
};
use quartz_common::contract::msg::execute::attested::HasUserData;
use quartz_common::contract::state::UserData;
use sha2::{Digest, Sha256};

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(Pop, PopExecuteMsg, PopQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
pub struct PopInstantiateMsg<RA = RawDefaultAttestation> {
    pub quartz: QuartzInstantiateMsg<RA>,
}

pub type AttestedMsg<M, RA = RawDefaultAttestation> = RawAttested<RawAttestedMsgSansHandler<M>, RA>;

#[cw_serde]
pub struct UpdateMsg {
    pub ciphertext: HexBinary,
    pub quantity: u32,
    pub withdrawals: Vec<(Addr, Uint128)>,
    // pub proof: π
}

impl HasUserData for UpdateMsg {
    fn user_data(&self) -> UserData {
        let mut hasher = Sha256::new();
        hasher.update(serde_json_wasm::to_string(&self).expect("infallible serializer"));
        let digest: [u8; 32] = hasher.finalize().into();

        let mut user_data = [0u8; 64];
        user_data[0..32].copy_from_slice(&digest);
        user_data
    }
}

/// App execute messages
#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum PopExecuteMsg<RA = RawDefaultAttestation> {
    UpdateConfig {},
    /// Increment count by 1
    Increment {},
    /// Admin method - reset count
    Reset {
        /// Count value after reset
        count: i32,
    },
    Trade {
        data: Trade
    },
    TradeCiphertext {
        ciphertext: Vec<u8>,
        digest: HexBinary,
    },
    Update {
        data:AttestedMsg<UpdateMsg, RA>
    }
}

#[cosmwasm_schema::cw_serde]
pub struct PopMigrateMsg {}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[derive(QueryResponses, cw_orch::QueryFns)]
pub enum PopQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(TradeResponse)]
    QueryTrade {
        base: String,
        quote: String
    },
    #[returns(QueryRequestsResponse)]
    QueryRequests {},
    #[returns(QueryStateResponse)]
    QueryState {},
    #[returns(QueryPairsResponse)]
    QueryPairs {},
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
pub struct QueryRequestsResponse {
    pub requests:Vec<EncryptedTrade>
}

#[cosmwasm_schema::cw_serde]
pub struct QueryStateResponse {
    pub data:HexBinary
}

#[cosmwasm_schema::cw_serde]
pub struct TradeResponse {
    pub price: GetPriceResponse,
    pub trade: Trade
}

#[cw_serde]
pub struct QueryPairsResponse {
    pub pairs: GetAllCurrencyPairsResponse,
}

