use super::*;
use actix::{Actor, Addr};

pub struct ServerBuilder {
	pub(super) name: Option<String>,
	pub(super) port: Option<u16>,
	pub(super) owner: Option<String>,
}

impl<'rhai> ServerBuilder {
	pub(super) fn new() -> Self {
		Self {
			name: None,
			port: None,
			owner: None,
		}
	}

	pub fn port(mut self, port: Option<u16>) -> Self {
		self.port = port;
		self
	}

	pub fn name(mut self, name: Option<String>) -> Self {
		self.name = name;
		self
	}

	pub fn owner(mut self, owner: Option<String>) -> Self {
		self.owner = owner;
		self
	}

	pub fn build(self) -> Addr<Server> {
		Server::from(self).start()
	}
}
