use std::net::SocketAddr;

use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClientInfo {
	uuid: Uuid,
	username: String,
	addr: SocketAddr,
}

impl ClientInfo {
	pub fn new(uuid: Uuid, username: String, addr: SocketAddr) -> Self {
		Self {
			uuid,
			username,
			addr,
		}
	}

	pub fn get_uuid(&self) -> Uuid {
		self.uuid
	}

	pub fn get_username(&self) -> String {
		self.username.clone()
	}

	pub fn get_addr(&self) -> SocketAddr {
		self.addr
	}
}
