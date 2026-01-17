use async_trait::async_trait;

mod communication_utils;
pub mod http;
pub mod http_post;
mod sse;
pub mod ws;
pub mod ws_reverse;

pub use communication_utils::Client;
