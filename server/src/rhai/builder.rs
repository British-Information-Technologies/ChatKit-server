use actix::{Actor, Addr};
use mlua::Lua;
use rhai::{Engine, RegisterNativeFunction, Scope};
use crate::client_management::ClientManager;
use crate::rhai::rhai_manager::RhaiManager;
use crate::network::NetworkManager;
use crate::Server;

pub struct Builder {
	engine: Engine,
	server: Addr<Server>,
	network_manager: Addr<NetworkManager>,
	client_manager: Addr<ClientManager>,
	scope: Scope<'static>,
}

impl Builder {
	pub(super) fn new(
		server: Addr<Server>,
		network_manager: Addr<NetworkManager>,
		client_manager: Addr<ClientManager>,
	) -> Self {
		Builder {
			engine: Engine::new(),
			server,
			network_manager,
			client_manager,
			scope: Default::default()
		}
	}

	pub fn scope_object<T: 'static>(mut self, name: &str, obj: T) -> Self
		where
			T: Clone {
		self.engine.register_type::<T>();
		self.scope.set_value(name, obj);
		self
	}

	pub fn scope_fn<F, A>(mut self, name: &str, func: F ) -> Self
		where
			F: RegisterNativeFunction<A, ()>
	{
		self.engine.register_fn(name, func);
		self
	}


	pub(crate) fn build(self) -> Addr<RhaiManager> {
		RhaiManager {
			engine: self.engine,
			scope: self.scope,
			server: self.server,
			network_manager: self.network_manager,
			client_manager: self.client_manager
		}.start()
	}
}