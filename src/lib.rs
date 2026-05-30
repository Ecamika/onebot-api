//! OneBot V11

pub mod api;
pub mod communication;
pub mod error;
pub mod event;
pub mod message;

#[cfg(feature = "quick_operation")]
pub mod quick_operation;
#[cfg(feature = "selector")]
pub mod selector;

#[cfg(feature = "selector")]
pub use onebot_api_macros::Selector;
