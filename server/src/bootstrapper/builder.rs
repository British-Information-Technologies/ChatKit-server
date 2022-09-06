use crate::{
	bootstrapper::bootstrapper::Bootstrapper, config_manager::get_args,
};
use actix::{Actor, Addr};
use clap::ArgMatches;
use std::fs::OpenOptions;
use tokio::fs::File;

pub struct Builder {
	pub(super) args: ArgMatches,
}

impl Builder {
	pub(super) fn new() -> Self {
		Self { args: get_args() }
	}

	pub fn file(mut self, path: String) -> Self {
		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.read(true)
			.open(path)
			.ok()
			.map(|val| File::from(val));

		self
	}

	pub fn build(self) -> Addr<Bootstrapper> {
		Bootstrapper::from(self).start()
	}
}
