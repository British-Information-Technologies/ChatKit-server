use crate::network::ListenerMessage;
use crate::network::NetworkListener;
use actix::Actor;

use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use actix::Message;
use actix::MessageResponse;
use actix::SpawnHandle;
use std::time::Duration;
use tokio::net::TcpListener;

#[derive(Message)]
#[rtype(result = "NetworkResponse")]
pub(crate) enum NetworkMessage {
	StartListening,
	StopListening,
	Ping,
}

#[derive(MessageResponse, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum NetworkResponse {
	Listening,
	NotListening,
	Pong,
	None,
}

pub(crate) struct NetworkManager {
	listener_addr: Addr<NetworkListener>,
}

impl NetworkManager {
	pub(crate) fn new() -> Addr<NetworkManager> {
		NetworkManager {
			listener_addr: NetworkListener::new("0.0.0.0:5600"),
		}
		.start()
	}

	fn start_listener(
		&mut self,
		_ctx: &mut <Self as actix::Actor>::Context,
	) -> NetworkResponse {
		NetworkResponse::Listening
	}

	fn stop_listener(
		&mut self,
		_ctx: &mut <Self as actix::Actor>::Context,
	) -> NetworkResponse {
		use ListenerMessage::StopListening;
		use NetworkResponse::NotListening;
		self.listener_addr.do_send(StopListening);
		NotListening
	}
}

impl Actor for NetworkManager {
	type Context = Context<Self>;

	fn started(&mut self, _ctx: &mut Self::Context) {}
}

impl Handler<NetworkMessage> for NetworkManager {
	type Result = NetworkResponse;
	fn handle(
		&mut self,
		msg: NetworkMessage,
		ctx: &mut <Self as actix::Actor>::Context,
	) -> <Self as Handler<NetworkMessage>>::Result {
		use NetworkMessage::{Ping, StartListening, StopListening};
		use NetworkResponse::{None, Pong};
		match msg {
			StartListening => self.start_listener(ctx),
			StopListening => None,
			Ping => Pong,
		}
	}
}
