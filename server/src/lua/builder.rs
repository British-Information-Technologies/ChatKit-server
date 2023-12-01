use actix::{Addr, WeakAddr};

use crate::{
	client_management::ClientManager,
	lua::lua_manager::LuaManager,
	network::NetworkManager,
	Server,
};

pub struct Builder {
	pub(super) server: WeakAddr<Server>,
	pub(super) network_manager: WeakAddr<NetworkManager>,
	pub(super) client_manager: WeakAddr<ClientManager>,
}

impl Builder {
	pub(super) fn new(
		server: WeakAddr<Server>,
		network_manager: WeakAddr<NetworkManager>,
		client_manager: WeakAddr<ClientManager>,
	) -> Self {
		Builder {
			server,
			network_manager,
			client_manager,
		}
	}

	pub(crate) fn build(self) -> Addr<LuaManager> {
		Addr::from(self)
	}
}
