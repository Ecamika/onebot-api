//! OneBot V11

pub mod api;
pub mod communication;
pub mod error;
pub mod event;
pub mod extension;
pub mod message;

#[cfg(feature = "quick_operation")]
pub mod quick_operation;
#[cfg(feature = "selector")]
pub use tynavi;

#[cfg(feature = "selector")]
pub mod selector {
	pub use tynavi::selector::Selector;
	pub use tynavi::traits::*;
}
