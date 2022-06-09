//! # actix_server
//! this holds the server actor
//! the server acts as teh main actor 
//! and supervisor to the actor system.

use crate::client_management::Client;
use crate::client_management::ClientManager;
use crate::client_management::ClientManagerOutput;
use crate::network::Connection;
use crate::network::ConnectionInitiator;
use crate::network::ConnectionMessage;
use crate::network::NetworkOutput;
use actix::fut::wrap_future;
use actix::Actor;
use actix::ActorFutureExt;
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use crate::client_management::ClientManagerMessage;
use foundation::messages::network::NetworkSockOut;
use foundation::ClientDetails;
use crate::network::{NetworkManager, NetworkMessage};


/// This struct is the main actor of the server.
/// all other actors are ran through here.
pub struct ServerActor {
	network_manager: Option<Addr<NetworkManager>>,
	client_management: Option<Addr<ClientManager>>,
}

impl ServerActor {
	pub(crate) fn new() -> Addr<Self> {
		ServerActor {
			network_manager: None,
			client_management: None,
		}
		.start()
	}

	pub(crate) fn client_request(
		&mut self,
		ctx: &mut <Self as Actor>::Context,
		addr: Addr<Connection>,
		details: ClientDetails
	) {
		use ClientManagerMessage::{AddClient};
		if let Some(mgr) = self.client_management.as_ref() {
			let client = Client::new(addr, details);
			mgr.do_send(AddClient(client));
		}
	}

	pub(crate) fn info_request(
		&mut self,
		ctx: &mut <Self as Actor>::Context,
		sender: Addr<Connection>,
	) {
		use NetworkSockOut::GotInfo;
		use ConnectionMessage::{CloseConnection, SendData};
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

impl Actor for ServerActor {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		let recp = ctx.address();

		self
			.network_manager
			.replace(NetworkManager::new(recp.clone().recipient().downgrade()));

		self
			.client_management
			.replace(ClientManager::new(recp.clone().recipient().downgrade()));

		if let Some(net_mgr) = self.network_manager.as_ref() {
			net_mgr.do_send(NetworkMessage::StartListening);
		}
	}
}

impl Handler<NetworkOutput> for ServerActor {
	type Result = ();
	fn handle(
		&mut self,
		msg: NetworkOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ConnectionMessage::{CloseConnection, SendData};
		use NetworkOutput::{InfoRequested, NewClient,NewConnection};
		use NetworkSockOut::GotInfo;
		println!("[ServerActor] received message");
		match msg {
			// This uses promise like funcionality to queue
			// a set of async operations,
			// so they occur in the right order
			InfoRequested(sender) => self.info_request(ctx, sender),
			// A new client is to be added
			NewClient(addr, details) => {

			}
			// A new client is to be added
			NewConnection(_) => todo!(),
		};
	}
}

impl Handler<ClientManagerOutput> for ServerActor {
	type Result = ();
	fn handle(
		&mut self,
		_msg: ClientManagerOutput,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		use ClientManagerOutput::{};
	}
}

