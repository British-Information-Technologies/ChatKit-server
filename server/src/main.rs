// pub mod chat_manager;
pub mod client;
pub mod client_manager;
mod lua;
pub mod messages;
pub mod network_manager;
mod plugin;
pub mod server;

use std::io;

use clap::{App, Arg};

use server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
	let _args = App::new("--rust chat server--")
		.version("0.1.5")
		.author("Mitchel Hardie <mitch161>, Michael Bailey <michael-bailey>")
		.about(
			"this is a chat server developed in rust, depending on the version one of two implementations will be used",
		)
		.arg(
			Arg::with_name("config")
				.short("p")
				.long("port")
				.value_name("PORT")
				.help("sets the port the server runs on.")
				.takes_value(true),
		)
		.get_matches();

	let server = Server::new().await.unwrap();

	server.start().await;
	Ok(())
}
