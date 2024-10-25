pub mod api;
pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
pub mod state;

pub use contract::interface::SlinkyInterface;
pub use error::SlinkyError;
pub use msg::{
    SlinkyExecuteMsg, SlinkyInstantiateMsg
};

/// The version of your Adapter
pub const ADAPTER_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const POP_NAMESPACE: &str = "pop";
pub const SLINKY_NAME: &str = "slinky";
pub const SLINKY_ID: &str = const_format::concatcp!(POP_NAMESPACE, ":", SLINKY_NAME);
