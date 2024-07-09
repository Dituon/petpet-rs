#[cfg(feature = "server")]
pub mod server;
pub mod service;
#[cfg(feature = "server")]
pub mod config;

mod query_template;