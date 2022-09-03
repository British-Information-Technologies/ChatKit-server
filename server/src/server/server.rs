//! This crate holds the implementations and functions for the server
//! including server boot procedures

use actix::{Actor, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner, Handler};
use actix::dev::MessageResponse;
use actix::fut::wrap_future;
use mlua::Lua;
use foundation::ClientDetails;
use foundation::messages::network::NetworkSockOut::GotInfo;
use crate::client_management::{ClientManager, ClientManagerOutput};
use crate::client_management::client::Client;
use crate::client_management::ClientManagerMessage::AddClient;
use crate::config_manager::ConfigManager;
use crate::lua::LuaManager;
use crate::rhai::RhaiManager;
use crate::network::{Connection, NetworkManager, NetworkMessage, NetworkOutput};
use crate::network::ConnectionMessage::{CloseConnection, SendData};
use crate::network::NetworkOutput::{InfoRequested, NewClient};
use crate::server::{builder, ServerBuilder, ServerDataMessage, ServerDataResponse};
use crate::server::config::ServerConfig;

/// This struct is the main actor of the server.
/// all other actors are ran through here.
pub struct Server {
	config: ServerConfig,
	network_manager: Option<Addr<NetworkManager>>,
	client_management: Option<Addr<ClientManager>>,
	rhai_manager: Option<Addr<RhaiManager>>,
	lua_manager: Option<Addr<LuaManager>>
}

impl Server {
	pub(crate) fn create(config_manager: Addr<ConfigManager>) -> builder::ServerBuilder {
		ServerBuilder::new(config_manager)
	}

	pub(crate) fn client_request(
		&mut self,
		_ctx: &mut <Self as Actor>::Context,
		addr: Addr<Connection>,
		details: ClientDetails,
	) {
		if let Some(mgr) = self.client_management.as_ref() {
			let client = Client::new(addr, details.clone());
			mgr.do_send(AddClient(details.uuid, client));
		}
	}

	pub(crate) fn info_request(
		&mut self,
		ctx: &mut <Self as Actor>::Context,
		sender: Addr<Connection>,
	) {
		let fut = wrap_future(
			sender.send(SendData(
				serde_json::to_string(&GotInfo {
					server_name: self.config.name.clone(),
					server_owner: self.config.owner.clone(),
				})
					.expect("Failed to serialise"),
			)),
		)
			// equivalent to using .then() in js
			.map(move |_out, _act: &mut Self, _ctx| {
				sender.do_send(CloseConnection);
			});
		ctx.spawn(fut);
	}
}

impl Actor for Server {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		let addr = ctx.address().downgrade();

		let nm = NetworkManager::create(addr.clone().recipient())
			.port(self.config.port)
			.build();
		self.network_manager.replace(nm.clone());

		let cm = ClientManager::new(
			addr.clone().recipient(),
		);
		self.client_management.replace(cm.clone());

		let rm = RhaiManager::create(ctx.address(), nm.clone(), cm.clone())
			.build();
		self.rhai_manager.replace(rm);

		let lm = LuaManager::create(ctx.address(), nm, cm)
			.build();
		self.lua_manager.replace(lm);

		if let Some(net_mgr) = self.network_manager.as_ref() {
			net_mgr.do_send(NetworkMessage::StartListening);
		}
	}
}

impl Handler<ServerDataMessage> for Server {
	type Result = ServerDataResponse;

	fn handle(&mut self, msg: ServerDataMessage, ctx: &mut Self::Context) -> Self::Result {
		println!("data message");
		match msg {
			ServerDataMessage::Name => ServerDataResponse::Name(self.config.name.clone()),
			ServerDataMessage::Port => ServerDataResponse::Port(self.config.port.clone()),
			ServerDataMessage::Owner => ServerDataResponse::Owner(self.config.owner.clone()),
			ServerDataMessage::ClientManager => ServerDataResponse::ClientManager(self.client_management.clone()),
			ServerDataMessage::NetworkManager => ServerDataResponse::NetworkManager(self.network_manager.clone()),
		}
	}
}

impl Handler<NetworkOutput> for Server {
	type Result = ();
	fn handle(
		&mut self,
		msg: NetworkOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		println!("[ServerActor] received message");
		match msg {
			// This uses promise like funcionality to queue
			// a set of async operations,
			// so they occur in the right order
			InfoRequested(sender) => self.info_request(ctx, sender),
			// A new client is to be added
			NewClient(addr, details) => self.client_request(ctx, addr, details),
		};
	}
}

impl Handler<ClientManagerOutput> for Server {
	type Result = ();

	fn handle(
		&mut self,
		msg: ClientManagerOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		todo!()
	}
}

impl From<ServerBuilder> for Server {
	fn from(builder: ServerBuilder) -> Self {
		Server {
			config: ServerConfig {
				port: builder.port.unwrap_or(5600),
				name: builder.name.unwrap_or_else(|| "Default Name".to_string()),
				owner: builder.owner.unwrap_or_else(|| "Default owner".to_string()),
			},
			network_manager: None,
			client_management: None,
			rhai_manager: None,
			lua_manager: None
		}
	}
}