use actix::{Actor, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner, Handler};
use actix::fut::wrap_future;
use foundation::ClientDetails;
use foundation::messages::network::NetworkSockOut::GotInfo;
use crate::client_management::{Client, ClientManager, ClientManagerOutput};
use crate::client_management::ClientManagerMessage::AddClient;
use crate::network::{Connection, NetworkManager, NetworkMessage, NetworkOutput};
use crate::network::ConnectionMessage::{CloseConnection, SendData};
use crate::network::NetworkOutput::{InfoRequested, NewClient};
use crate::server::{builder, ServerBuilder};
use crate::server::config::ServerConfig;

/// This struct is the main actor of the server.
/// all other actors are ran through here.
pub struct Server {
	config: ServerConfig,
	network_manager: Option<Addr<NetworkManager>>,
	client_management: Option<Addr<ClientManager>>,
}

impl Server {
	pub(crate) fn new() -> Addr<Self> {
		Server {
			config: Default::default(),
			network_manager: None,
			client_management: None,
		}
			.start()
	}

	pub fn create() -> builder::ServerBuilder {
		ServerBuilder::new()
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
					server_name: "String".to_owned(),
					server_owner: "String".to_owned(),
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
		let addr = ctx.address();

		self.network_manager
			.replace(NetworkManager::new(addr.clone().recipient().downgrade()));

		self.client_management.replace(ClientManager::new(
			addr.clone().recipient::<ClientManagerOutput>().downgrade(),
		));

		if let Some(net_mgr) = self.network_manager.as_ref() {
			net_mgr.do_send(NetworkMessage::StartListening);
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

impl From<builder::ServerBuilder> for Server {
	fn from(builder: ServerBuilder) -> Self {
		Server {
			config: ServerConfig {
				port: builder.port.unwrap_or(5600),
				name: builder.name.unwrap_or_else(|| "Default Name".to_string()),
				owner: builder.owner.unwrap_or_else(|| "Default owner".to_string()),
			},
			network_manager: None,
			client_management: None
		}
	}
}