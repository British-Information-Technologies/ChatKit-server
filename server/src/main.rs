//! # actor
//! This is the main module of the actix server.
//! It starts the actor runtime and then sleeps
//! for the duration of the program.

pub(crate) mod server;
pub(crate) mod client_management;
pub(crate) mod network;
pub(crate) mod prelude;
pub(crate) mod rhai;
pub(crate) mod lua;
pub(crate) mod scripting;

use std::env::args;
use server::Server;
use tokio::time::{sleep, Duration};
use clap::{App, Arg, value_parser};
use openssl::version::version;

#[actix::main()]
async fn main() {

	let args = App::new("Rust Chat Server")
		.author("Michael Bailey & Mitchel Hardie")
		.version("0.1.0")
		.about("A chat server written in rust, with a custom json protocol, based on serde and actix")
		.arg(
			Arg::new("port")
				.short('p')
				.long("port")
				.takes_value(true)
				.value_parser(value_parser!(u16))
				.default_value("5600")
				.help("overrides the default port")
		)
		.arg(
			Arg::new("server name")
				.short('n')
				.long("name")
				.takes_value(true)
				.help("overrides the default port of the server")
		)
		.arg(
			Arg::new("server owner")
				.short('o')
				.long("owner")
				.takes_value(true)
				.help("overrides the owner of the server")
		)
		.after_help("This is a chat server made to test out writing a full application in rust \
											It has evolved over time to use different frameworks\
											It is currently using actix")
		.get_matches();

	let mut server_builder = Server::create();

	if let Some(port) = args.get_one::<u16>("port") {
		server_builder = server_builder.port(*port);
		println!("got port number {:?}", port);
	}
	if let Some(name) = args.get_one::<String>("server name") {
		server_builder = server_builder.name(name.clone());
		println!("got server name number {:?}", name)
	}
	if let Some(owner) = args.get_one::<String>("server owner") {
		server_builder = server_builder.owner(owner.clone());
		println!("got server owner number {:?}", owner)
	}

	let _server = server_builder.build();

	loop {
		sleep(Duration::from_millis(1000)).await;
	}
}
