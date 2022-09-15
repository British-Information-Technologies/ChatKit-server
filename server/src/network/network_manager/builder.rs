use crate::network::network_manager::messages::NetworkOutput;
use crate::network::NetworkManager;
use actix::{Actor, Addr, WeakRecipient};

pub struct Builder {
	pub(super) delegate: WeakRecipient<NetworkOutput>,
}

impl Builder {
	pub(super) fn new(delegate: WeakRecipient<NetworkOutput>) -> Self {
		Self { delegate }
	}

	pub fn build(self) -> Addr<NetworkManager> {
		NetworkManager::from(self).start()
	}
}
