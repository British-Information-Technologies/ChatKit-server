//! This is the main module of the actix server.
//! It starts the server and sleeps for the remainder of the program

pub(crate) mod network;

pub mod connection;
pub mod os_signal_manager;
pub mod server_va;

use crate::server_va::Server;

/// The main function
#[actix::main()]
async fn main() {
	// creating listeners

	Server::default().run().await;
}
