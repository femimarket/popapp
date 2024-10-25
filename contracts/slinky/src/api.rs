use crate::{
    msg::{
        SlinkyExecuteMsg, SlinkyQueryMsg
    },
    SLINKY_ID,
};

use abstract_adapter::sdk::{
    features::{AccountIdentification, Dependencies, ModuleIdentification},
    AbstractSdkResult, AdapterInterface,
};
use abstract_adapter::std::objects::module::ModuleId;
use cosmwasm_schema::serde::de::DeserializeOwned;
use cosmwasm_std::{CosmosMsg, Deps, Uint128};
use neutron_std::types::slinky::oracle::v1::{GetAllCurrencyPairsResponse, GetPriceResponse, OracleQuerier};
use neutron_std::types::slinky::types::v1::CurrencyPair;

// API for Abstract SDK users
/// Interact with your adapter in other modules.
pub trait SlinkyApi: AccountIdentification + Dependencies + ModuleIdentification {
    /// Construct a new adapter interface.
    fn slinky<'a>(&'a self, deps: Deps<'a>) -> Slinky<Self> {
        Slinky {
            base: self,
            deps,
            module_id: SLINKY_ID,
        }
    }
}

impl<T: AccountIdentification + Dependencies + ModuleIdentification> SlinkyApi for T {}

#[derive(Clone)]
pub struct Slinky<'a, T: SlinkyApi> {
    pub base: &'a T,
    pub module_id: ModuleId<'a>,
    pub deps: Deps<'a>,
}

impl<'a, T: SlinkyApi> Slinky<'a, T> {
    /// Set the module id
    pub fn with_module_id(self, module_id: ModuleId<'a>) -> Self {
        Self { module_id, ..self }
    }

    /// returns the HUB module id
    fn module_id(&self) -> ModuleId {
        self.module_id
    }

    /// Executes a [SlinkyExecuteMsg] in the adapter
    fn request(&self, msg: SlinkyExecuteMsg) -> AbstractSdkResult<CosmosMsg> {
        let adapters = self.base.adapters(self.deps);

        adapters.execute(self.module_id(), msg)
    }

    /// Route message
    pub fn update_config(&self) -> AbstractSdkResult<CosmosMsg> {
        self.request(SlinkyExecuteMsg::UpdateConfig {})
    }
}

/// Queries
impl<'a, T: SlinkyApi> Slinky<'a, T> {
    /// Query your adapter via message type
    pub fn query<R: DeserializeOwned>(&self, query_msg: SlinkyQueryMsg) -> AbstractSdkResult<R> {
        let adapters = self.base.adapters(self.deps);
        adapters.query(self.module_id(), query_msg)
    }

    /// Query config
    pub fn config(&self) -> AbstractSdkResult<Uint128> {
        self.query(SlinkyQueryMsg::Config {})
    }

    pub fn pairs(&self) -> AbstractSdkResult<GetAllCurrencyPairsResponse>{
        self.query(SlinkyQueryMsg::Pairs {})
    }

    pub fn pair(&self, base: String, quote: String) -> AbstractSdkResult<GetPriceResponse> {
        self.query(SlinkyQueryMsg::Pair { base, quote })
    }
}
