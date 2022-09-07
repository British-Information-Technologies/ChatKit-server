//! # actor
//! This is the main module of the actix server.
//! It starts the actor runtime and then sleeps
//! for the duration of the program.

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

use crate::config_manager::ConfigManager;

#[actix::main()]
async fn main() {
	let init = Server::create(ConfigManager::shared()).build();
	loop {
		sleep(Duration::from_millis(1000)).await;
	}
}
