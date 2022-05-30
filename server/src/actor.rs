mod network;
pub(crate) mod prelude;

use crate::network::ConnectionMessage;
use crate::network::NetworkOutput;
use actix::clock::sleep;
use actix::fut::wrap_future;
use actix::Actor;
use actix::ActorFutureExt;
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use foundation::messages::network::NetworkSockOut;
use network::{NetworkManager, NetworkMessage};
use std::time::Duration;

/// This struct is the main actor of teh server.
/// all other actors are ran through here.
struct Server {
	network_manager: Option<Addr<NetworkManager>>,
}

impl Server {
	pub(crate) fn new() -> Addr<Self> {
		Server {
			network_manager: None,
		}
		.start()
	}
}

impl Actor for Server {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		self
			.network_manager
			.replace(NetworkManager::new(ctx.address().recipient()));

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
		use ConnectionMessage::{CloseConnection, SendData};
		use NetworkOutput::{InfoRequested, NewClient};
		use NetworkSockOut::GotInfo;
		println!("server received message");
		match msg {
			// This uses promise like funcionality to queue
			// a set of async operations,
			// so they occur in the right order
			InfoRequested(sender) => {
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
				.map(move |out, act: &mut Self, ctx| {
					sender.do_send(CloseConnection);
				});
				ctx.spawn(fut);
			}
			NewClient(_, _) => todo!(),
		};
	}
}

#[actix::main()]
async fn main() {
	let server = Server::new();
	loop {
		sleep(Duration::from_millis(500)).await;
	}
}
