use actix::{Actor, Addr};
use super::*;

pub struct ServerBuilder {
	pub(super) name: Option<String>,
	pub(super) port: Option<usize>,
	pub(super) owner: Option<String>,
}

impl ServerBuilder {
	pub(super) fn new() -> Self {
		Self {
			name: None,
			port: None,
			owner: None,
		}
	}

	pub fn port(mut self, port: usize) -> Self {
		self.port = Some(port);
		self
	}

	pub fn name(mut self, name: String) -> Self {
		self.name = Some(name);
		self
	}

	pub fn owner(mut self, owner: String) -> Self {
		self.owner = Some(owner);
		self
	}

	pub fn build(self) -> Addr<Server> {
		Server::from(self).start()
	}
}