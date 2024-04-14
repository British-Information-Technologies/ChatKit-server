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
use tokio::{
	net::TcpListener,
	select,
	time::{sleep, Duration},
};

/// The main function
#[actix::main()]
async fn main() {
	// creating listeners
	let protobuf_listener = TcpListener::bind("127.0.0.1:6500").await.unwrap();
	// todo: convert the actix stuff to whatever this is.
	// let json_listener = TcpListener::bind("127.0.0.1:5601").await.unwrap();

	let _init = Server::create().build();

	select! {
		Ok((stream, addr)) = protobuf_listener.accept() => {




		},
	};

	loop {
		sleep(Duration::from_millis(1000)).await;
	}
}
