#![feature(arbitrary_self_types)]

// mod chat_manager;
mod client;
mod client_manager;
mod messages;
mod network_manager;
mod server;
mod lua;
mod plugin_manager;
pub mod plugin;

pub use server::Server;
