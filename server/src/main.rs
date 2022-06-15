//! # actor
//! This is the main module of the actix server.
//! It starts the actor runtime and then sleeps
//! for the duration of the program.

pub(crate) mod client_management;
pub(crate) mod network;
pub(crate) mod prelude;
pub(crate) mod server;

use server::ServerActor;
use tokio::time::{sleep, Duration};

#[actix::main()]
async fn main() {
	let _server = ServerActor::new();
	loop {
		sleep(Duration::from_millis(1000)).await;
	}
}
