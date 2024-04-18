//! This crate holds the implementations and functions for the server
//! including server boot procedures

use actix::{
	fut::wrap_future,
	Actor,
	ActorFutureExt,
	Addr,
	AsyncContext,
	Context,
	Handler,
};
use foundation::{messages::network::NetworkSockOut::GotInfo, ClientDetails};

use crate::{
	client_management::{
		client::Client,
		ClientManager,
		ClientManagerMessage::AddClient,
		ClientManagerOutput,
	},
	config_manager::{
		ConfigManager,
		ConfigManagerDataMessage,
		ConfigManagerDataResponse,
		ConfigValue,
	},
	network::{
		Connection,
		ConnectionMessage::{CloseConnection, SendData},
		NetworkManager,
		NetworkOutput,
		NetworkOutput::{InfoRequested, NewClient},
	},
	prelude::messages::NetworkMessage,
	server::{builder, ServerBuilder, ServerDataMessage, ServerDataResponse},
};

/// This struct is the main actor of the server.
/// all other actors are ran through here.
pub struct Server {
	name: String,
	owner: String,

	network_manager: Option<Addr<NetworkManager>>,
	client_manager: Option<Addr<ClientManager>>,
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
		let cm = ClientManager::new(addr.recipient());

		self.network_manager.replace(nm.clone());
		self.client_manager.replace(cm.clone());

		nm.do_send(NetworkMessage::StartListening);

		let name_fut = wrap_future(
			ConfigManager::shared().send(GetValue("Server.Name".to_owned())),
		)
		.map(|out, actor: &mut Server, _ctx| {
			if let Ok(GotValue(Some(ConfigValue::String(val)))) = out {
				actor.name = val
			}
		});

		let owner_fut = wrap_future(
			ConfigManager::shared().send(GetValue("Server.Owner".to_owned())),
		)
		.map(|out, actor: &mut Server, _ctx| {
			if let Ok(GotValue(Some(ConfigValue::String(val)))) = out {
				actor.owner = val
			}
		});

		ctx.spawn(name_fut);
		ctx.spawn(owner_fut);
	}
}

impl Handler<ServerDataMessage> for Server {
	type Result = ServerDataResponse;

	fn handle(
		&mut self,
		msg: ServerDataMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		println!("[Server] got data message");
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
		}
	}
}
