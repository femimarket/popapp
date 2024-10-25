pub mod contract;
pub mod error;
mod handlers;
pub mod msg;
mod replies;
pub mod state;



pub use error::PopError;

/// The version of your app
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub use contract::interface::PopInterface;

pub const POP_NAMESPACE: &str = "pop";
pub const POP_NAME: &str = "pop";
pub const POP_ID: &str = const_format::concatcp!(POP_NAMESPACE, ":", POP_NAME);
