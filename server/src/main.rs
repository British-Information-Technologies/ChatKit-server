// pub mod chat_manager;
pub mod client;
pub mod client_manager;
mod event_type;
mod lua;
pub mod messages;
mod network;
pub mod network_manager;
// mod plugin;
mod prelude;
pub mod server;

use std::io;

use clap::{App, Arg};

use server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
	let server = Server::new().await.unwrap();

	server.start().await;
	Ok(())
}
