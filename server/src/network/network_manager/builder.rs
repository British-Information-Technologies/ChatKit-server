use actix::{Actor, Addr, WeakRecipient};
use crate::network::network_manager::messages::NetworkOutput;
use crate::network::NetworkManager;

pub struct Builder {
	pub(super) port: Option<u16>,
	pub(super) delegate: WeakRecipient<NetworkOutput>,
}

impl Builder {
	pub(super) fn new(delegate: WeakRecipient<NetworkOutput>) -> Self {
		Self {
			port: None,
			delegate,
		}
	}

	pub fn port(mut self, port: u16) -> Self {
		self.port = Some(port);
		self
	}

	pub fn build(self) -> Addr<NetworkManager> {
		NetworkManager::from(self).start()
	}
}