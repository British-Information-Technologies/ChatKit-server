use actix::{Actor, Addr, WeakAddr};
use rhai::{Engine, Scope};

use crate::{
	client_management::ClientManager,
	network::NetworkManager,
	rhai::rhai_manager::RhaiManager,
	Server,
};

pub struct Builder {
	engine: Engine,
	server: WeakAddr<Server>,
	network_manager: WeakAddr<NetworkManager>,
	client_manager: WeakAddr<ClientManager>,
	scope: Scope<'static>,
}

impl Builder {
	pub(super) fn new(
		server: WeakAddr<Server>,
		network_manager: WeakAddr<NetworkManager>,
		client_manager: WeakAddr<ClientManager>,
	) -> Self {
		Builder {
			engine: Engine::new(),
			server,
			network_manager,
			client_manager,
			scope: Default::default(),
		}
	}

	pub fn scope_object<T: 'static>(mut self, name: &str, obj: T) -> Self
	where
		T: Clone,
	{
		self.engine.register_type::<T>();
		self.scope.set_value(name, obj);
		self
	}

	// not sure what this is for?
	// pub fn scope_fn<F, A>(mut self, name: &str, func: F) -> Self
	// where
	// 	F: RegisterNativeFunction<A, ()>,
	// {
	// 	self.engine.register_fn(name, func);
	// 	self
	// }

	pub(crate) fn build(self) -> Addr<RhaiManager> {
		RhaiManager {
			engine: self.engine,
			_scope: self.scope,
			_server: self.server,
			_network_manager: self.network_manager,
			_client_manager: self.client_manager,
		}
		.start()
	}
}
