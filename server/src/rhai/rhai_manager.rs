use actix::{Actor, Addr, Context, WeakAddr};
use rhai::{Engine, Scope};

use crate::{
	client_management::ClientManager,
	network::NetworkManager,
	rhai::builder::Builder,
	Server,
};

pub struct RhaiManager {
	pub(super) engine: Engine,
	pub(super) _scope: Scope<'static>,
	pub(super) _server: WeakAddr<Server>,
	pub(super) _network_manager: WeakAddr<NetworkManager>,
	pub(super) _client_manager: WeakAddr<ClientManager>,
}

impl RhaiManager {
	pub fn create(
		server: WeakAddr<Server>,
		network_manager: WeakAddr<NetworkManager>,
		client_manager: WeakAddr<ClientManager>,
	) -> Builder {
		Builder::new(
			server.clone(),
			network_manager.clone(),
			client_manager.clone(),
		)
		.scope_object("server", server)
	}
}

impl Actor for RhaiManager {
	type Context = Context<Self>;

	fn started(&mut self, _ctx: &mut Self::Context) {
		self
			.engine
			.run(
				r#"
			print("hello rhai")
		"#,
			)
			.unwrap();
	}
}
