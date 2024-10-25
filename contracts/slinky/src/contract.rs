use crate::{
    error::SlinkyError,
    handlers,
    msg::{
        SlinkyExecuteMsg, SlinkyInstantiateMsg, SlinkyQueryMsg
    },
    ADAPTER_VERSION, SLINKY_ID,
};

use abstract_adapter::AdapterContract;
use cosmwasm_std::Response;

/// The type of the adapter that is used to build your Adapter and access the Abstract SDK features.
pub type Slinky = AdapterContract<
    SlinkyError,
    SlinkyInstantiateMsg,
    SlinkyExecuteMsg,
    SlinkyQueryMsg,
>;
/// The type of the result returned by your Adapter's entry points.
pub type AdapterResult<T = Response> = Result<T, SlinkyError>;

const SLINKY: Slinky = Slinky::new(SLINKY_ID, ADAPTER_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler);

// Export handlers
#[cfg(feature = "export")]
abstract_adapter::export_endpoints!(SLINKY, Slinky);

abstract_adapter::cw_orch_interface!(
    SLINKY,
    Slinky,
    SlinkyInstantiateMsg,
    SlinkyInterface
);
