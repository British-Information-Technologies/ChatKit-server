//! Contains code that handles the lifecycle of connected clients
//!
//! This collects all parts used by the client manager actor
//!
//! It's responsibility is:
//! - to handle client to client communication.
//! - to handle server to client communication.
//! - to handler client lifecycle events such as dicconection.

mod chat_manager;
pub mod client;
mod client_manager;
mod messages;

use chat_manager::ChatManager;
pub(crate) use client_manager::ClientManager;
pub(crate) use messages::{
	ClientManagerDataMessage,
	ClientManagerDataResponse,
	ClientManagerMessage,
	ClientManagerOutput,
};
