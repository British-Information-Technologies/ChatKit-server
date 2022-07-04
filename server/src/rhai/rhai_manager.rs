use actix::{Actor, Addr, ArbiterHandle, AsyncContext, Context, Running};
use actix::fut::wrap_future;
use rhai::{Engine, Func, Scope};
use crate::client_management::ClientManager;
use crate::rhai::builder::Builder;
use crate::network::NetworkManager;
use crate::Server;

pub struct RhaiManager {
	pub(super) engine: Engine,
	pub(super) scope: Scope<'static>,
	pub(super) server: Addr<Server>,
	pub(super) network_manager: Addr<NetworkManager>,
	pub(super) client_manager: Addr<ClientManager>,
}

impl RhaiManager {
	pub fn create(
		server: Addr<Server>,
		network_manager: Addr<NetworkManager>,
		client_manager: Addr<ClientManager>
	) -> Builder {
		Builder::new(server.clone(), network_manager.clone(), client_manager.clone())
			.scope_object("server", server)
	}
}

impl Actor for RhaiManager {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		self.engine.run(r#"
			print("hello rhai")
		"#).unwrap();
	}
}

