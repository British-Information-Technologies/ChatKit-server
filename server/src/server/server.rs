//! This crate holds the implementations and functions for the server
//! including server boot procedures

use crate::client_management::client::Client;
use crate::client_management::ClientManagerMessage::AddClient;
use crate::client_management::{ClientManager, ClientManagerOutput};
use crate::config_manager::{
	ConfigManager, ConfigManagerDataMessage, ConfigManagerDataResponse, ConfigValue,
};
use crate::lua::LuaManager;
use crate::network::ConnectionMessage::{CloseConnection, SendData};
use crate::network::NetworkOutput::{InfoRequested, NewClient};
use crate::network::{Connection, NetworkManager, NetworkOutput};
use crate::rhai::RhaiManager;

use crate::server::{builder, ServerBuilder, ServerDataMessage, ServerDataResponse};

use actix::fut::wrap_future;
use actix::{Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler};
use foundation::messages::network::NetworkSockOut::GotInfo;
use foundation::ClientDetails;

/// This struct is the main actor of the server.
/// all other actors are ran through here.
pub struct Server {
	name: String,
	owner: String,

	network_manager: Option<Addr<NetworkManager>>,
	client_manager: Option<Addr<ClientManager>>,
	rhai_manager: Option<Addr<RhaiManager>>,
	lua_manager: Option<Addr<LuaManager>>,
}

impl Server {
	pub(crate) fn create() -> builder::ServerBuilder {
		ServerBuilder::new()
	}

	pub(crate) fn client_request(
		&mut self,
		_ctx: &mut <Self as Actor>::Context,
		addr: Addr<Connection>,
		details: ClientDetails,
	) {
		if let Some(mgr) = self.client_manager.as_ref() {
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
					server_name: self.name.clone(),
					server_owner: self.owner.clone(),
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
		use ConfigManagerDataMessage::GetValue;
		use ConfigManagerDataResponse::GotValue;

		let addr = ctx.address().downgrade();

		let nm = NetworkManager::create(addr.clone().recipient()).build();
		self.network_manager.replace(nm.clone());

		let cm = ClientManager::new(addr.clone().recipient());
		self.client_manager.replace(cm.clone());

		let rm = RhaiManager::create(ctx.address(), nm.clone(), cm.clone()).build();
		self.rhai_manager.replace(rm);

		let lm = LuaManager::create(ctx.address(), nm, cm).build();
		self.lua_manager.replace(lm);

		let name_fut = wrap_future(
			ConfigManager::shared().send(GetValue("Server.Name".to_owned())),
		)
		.map(|out, actor: &mut Server, _ctx| {
			out.ok().map(|res| {
				if let GotValue(Some(ConfigValue::String(val))) = res {
					actor.name = val
				};
			});
		});

		let owner_fut = wrap_future(
			ConfigManager::shared().send(GetValue("Server.Owner".to_owned())),
		)
		.map(|out, actor: &mut Server, _ctx| {
			out.ok().map(|res| {
				if let GotValue(Some(ConfigValue::String(val))) = res {
					actor.owner = val
				};
			});
		});

		ctx.spawn(name_fut);
		ctx.spawn(owner_fut);
	}
}

impl Handler<ServerDataMessage> for Server {
	type Result = ServerDataResponse;

	fn handle(&mut self, msg: ServerDataMessage, _ctx: &mut Self::Context) -> Self::Result {
		println!("data message");
		match msg {
			ServerDataMessage::Name => ServerDataResponse::Name(self.name.clone()),
			ServerDataMessage::Owner => ServerDataResponse::Owner(self.owner.clone()),
			ServerDataMessage::ClientManager => {
				ServerDataResponse::ClientManager(self.client_manager.clone())
			}
			ServerDataMessage::NetworkManager => {
				ServerDataResponse::NetworkManager(self.network_manager.clone())
			}
		}
	}
}

impl Handler<NetworkOutput> for Server {
	type Result = ();
	fn handle(&mut self, msg: NetworkOutput, ctx: &mut Self::Context) -> Self::Result {
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
		_msg: ClientManagerOutput,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		todo!()
	}
}

impl From<ServerBuilder> for Server {
	fn from(builder: ServerBuilder) -> Self {
		Server {
			name: builder.name,
			owner: builder.owner,

			network_manager: None,
			client_manager: None,
			rhai_manager: None,
			lua_manager: None,
		}
	}
}
