//! This is the main module of the actix server.
//! It starts the server and sleeps for the remainder of the program

pub(crate) mod client_management;
pub(crate) mod config_manager;

pub(crate) mod network;
pub(crate) mod prelude;

pub(crate) mod scripting;
pub(crate) mod server;

pub mod listener_manager;
pub mod network_connection;
pub mod os_signal_manager;
pub mod server_va;

use crate::server_va::Server;

/// The main function
#[actix::main()]
async fn main() {
	// creating listeners

	Server::default().run().await;
}
