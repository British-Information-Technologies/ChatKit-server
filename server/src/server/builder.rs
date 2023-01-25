use actix::{Actor, Addr};

use super::*;

pub struct ServerBuilder {
	pub(super) name: String,
	pub(super) owner: String,
}

impl<'rhai> ServerBuilder {
	pub(super) fn new() -> Self {
		Self {
			name: "<UNKNOWN>".into(),
			owner: "<UNKNOWN>".into(),
		}
	}

	pub fn name(mut self, name: String) -> Self {
		self.name = name;
		self
	}

	pub fn owner(mut self, owner: String) -> Self {
		self.owner = owner;
		self
	}

	pub fn build(self) -> Addr<Server> {
		Server::from(self).start()
	}
}
