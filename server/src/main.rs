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
pub(crate) mod bootstrapper;
pub(crate) mod config_manager;

use std::env::args;
use actix::Actor;
use server::Server;
use tokio::time::{sleep, Duration};
use clap::{App, Arg, value_parser};
use openssl::version::version;
use crate::bootstrapper::{Bootstrapper, get_args};

#[actix::main()]
async fn main() {
	let init = Bootstrapper::create()
		.build();
	loop {
		sleep(Duration::from_millis(1000)).await;
	}
}
