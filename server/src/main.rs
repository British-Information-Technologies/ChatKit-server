//! This is the main module of the actix server.
//! It starts the server and sleeps for the remainder of the program

pub(crate) mod client_management;
pub(crate) mod config_manager;
pub(crate) mod lua;
pub(crate) mod network;
pub(crate) mod prelude;
pub(crate) mod rhai;
pub(crate) mod scripting;
pub(crate) mod server;

use server::Server;

use tokio::time::{sleep, Duration};

/// The main function
#[actix::main()]
async fn main() {
	let _init = Server::create().build();
	loop {
		sleep(Duration::from_millis(1000)).await;
	}
}
