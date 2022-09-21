use crate::client_management::ClientManager;
use crate::network::NetworkManager;
use crate::rhai::builder::Builder;
use crate::Server;

use actix::{Actor, Addr, Context};
use rhai::{Engine, Scope};

pub struct RhaiManager {
	pub(super) engine: Engine,
	pub(super) _scope: Scope<'static>,
	pub(super) _server: Addr<Server>,
	pub(super) _network_manager: Addr<NetworkManager>,
	pub(super) _client_manager: Addr<ClientManager>,
}

impl RhaiManager {
	pub fn create(
		server: Addr<Server>,
		network_manager: Addr<NetworkManager>,
		client_manager: Addr<ClientManager>,
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
