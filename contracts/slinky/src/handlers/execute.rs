use crate::{
    contract::{
        AdapterResult, Slinky
    },
    msg::SlinkyExecuteMsg,
    state::{CONFIG, STATUS},
    SlinkyError, POP_NAMESPACE,
};

use abstract_adapter::{
    objects::namespace::Namespace,
    sdk::{AccountVerification, ModuleRegistryInterface},
    traits::AbstractResponse,
};
use cosmwasm_std::{ensure_eq, DepsMut, Env, MessageInfo};

pub fn execute_handler(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    module: Slinky,
    msg: SlinkyExecuteMsg,
) -> AdapterResult {
    match msg {
        SlinkyExecuteMsg::UpdateConfig {} => update_config(deps, env, info, module),
    }
}

/// Update the configuration of the adapter
fn update_config(deps: DepsMut, env: Env, _msg_info: MessageInfo, module: Slinky) -> AdapterResult {
    // Only admin(namespace owner) can change recipient address
    let namespace = module
        .module_registry(deps.as_ref(), &env)?
        .query_namespace(Namespace::new(POP_NAMESPACE)?)?;

    // unwrap namespace, since it's unlikely to have unclaimed namespace as this adapter installed
    let namespace_info = namespace.unwrap();
    ensure_eq!(
        namespace_info.account,
        module.target_account.clone().unwrap(),
        SlinkyError::Unauthorized {}
    );
    let mut _config = CONFIG.load(deps.storage)?;

    Ok(module.response("update_config"))
}
