//! # actix_server
//! this holds the server actor
//! the server acts as teh main actor
//! and supervisor to the actor system.

mod server;
mod config;
mod builder;

use config::ServerConfig;
pub use server::Server;
pub(crate) use builder::ServerBuilder;