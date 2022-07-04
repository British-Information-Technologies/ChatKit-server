use actix::{Actor, Addr};
use mlua::Lua;
use rhai::{Engine, RegisterNativeFunction, Scope};
use crate::client_management::ClientManager;
use crate::lua::lua_manager::LuaManager;
use crate::network::NetworkManager;
use crate::Server;

pub struct Builder {
	pub(crate) engine: Lua,
	pub(super) server: Addr<Server>,
	pub(super) network_manager: Addr<NetworkManager>,
	pub(super) client_manager: Addr<ClientManager>,
}

impl Builder {
	pub(super) fn new(
		server: Addr<Server>,
		network_manager: Addr<NetworkManager>,
		client_manager: Addr<ClientManager>,
	) -> Self {
		Builder {
			engine: Lua::new(),
			server,
			network_manager,
			client_manager,
		}
	}

	pub(crate) fn build(self) -> Addr<LuaManager> {
		Addr::from(self)
	}
}