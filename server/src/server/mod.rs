//! # actix_server
//! this holds the server actor
//! the server acts as teh main actor
//! and supervisor to the actor system.

mod server;

mod builder;
mod messages;

pub use builder::ServerBuilder;
pub use messages::*;
pub use server::Server;
