use crate::{
    error::PopError,
    handlers,
    msg::{
        PopExecuteMsg, PopInstantiateMsg, PopMigrateMsg, PopQueryMsg
    },
    replies::{self, INSTANTIATE_REPLY_ID},
    APP_VERSION, POP_ID,
};

use abstract_app::AppContract;
use abstract_app::objects::dependency::StaticDependency;
use cosmwasm_std::{Empty, Response};
use slinky::SLINKY_ID;

/// The type of the result returned by your app's entry points.
pub type PopResult<T = Response> = Result<T, PopError>;

/// The type of the app that is used to build your app and access the Abstract SDK features.
pub type Pop =
    AppContract<PopError, PopInstantiateMsg, PopExecuteMsg, PopQueryMsg, Empty>;

const APP: Pop = Pop::new(POP_ID, APP_VERSION, None)
    .with_instantiate(handlers::instantiate_handler)
    .with_execute(handlers::execute_handler)
    .with_query(handlers::query_handler)
    // .with_migrate(handlers::migrate_handler)
    .with_replies(&[(INSTANTIATE_REPLY_ID, replies::instantiate_reply)])
    .with_dependencies(&[
        StaticDependency::new(SLINKY_ID, &[">=0.0.1"])
    ]);

// Export handlers
#[cfg(feature = "export")]
abstract_app::export_endpoints!(APP, Pop);

abstract_app::cw_orch_interface!(APP, Pop, PopInterface);

#[cfg(not(target_arch = "wasm32"))]
use abstract_app::std::account::ModuleInstallConfig;

// TODO: add to docmuentation
// https://linear.app/abstract-sdk/issue/ABS-414/add-documentation-on-dependencycreation-trait
#[cfg(not(target_arch = "wasm32"))]
impl<Chain: cw_orch::environment::CwEnv> abstract_interface::DependencyCreation
    for crate::PopInterface<Chain>
{
    type DependenciesConfig = cosmwasm_std::Empty;

    fn dependency_install_configs(
        _configuration: Self::DependenciesConfig,
    ) -> Result<Vec<ModuleInstallConfig>, abstract_app::abstract_interface::AbstractInterfaceError>
    {
        Ok(vec![ModuleInstallConfig::new(
            abstract_app::objects::module::ModuleInfo::from_id(
                SLINKY_ID,
                slinky::ADAPTER_VERSION.into(),
            )?,
            None,
        )])
    }
}
