use std::fs::OpenOptions;
use actix::{Actor, Addr};
use clap::ArgMatches;
use tokio::fs::File;
use crate::bootstrapper::bootstrapper::Bootstrapper;
use super::get_args;

pub struct Builder {
	pub(super) args: ArgMatches,
}

impl Builder {
	pub(super) fn new() -> Self {
		Self {
			args: get_args(),
		}
	}

	pub fn file(mut self, path: String) -> Self {
		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.read(true)
			.open(path)
			.ok().map(|val| File::from(val));

		self
	}

	pub fn build(self) -> Addr<Bootstrapper> {
		Bootstrapper::from(self).start()
	}
}