//! OneBot V11

pub mod api;
pub mod communication;
pub mod error;
pub mod event;
pub mod message;

#[cfg(feature = "quick_operation")]
pub mod quick_operation;
pub mod selector;
