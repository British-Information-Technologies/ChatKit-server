//! # actix_server
//! this holds the server actor
//! the server acts as teh main actor
//! and supervisor to the actor system.

mod server;
mod config;
mod builder;
mod messages;

use config::ServerConfig;
pub use server::Server;
pub use builder::ServerBuilder;
pub use messages::*;