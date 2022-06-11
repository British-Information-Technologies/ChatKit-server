use crate::network::Connection;
use crate::prelude::ObservableMessage;
use actix::{Actor, Addr, Context, Handler, Message, WeakAddr, Recipient, Running, ArbiterHandle};
use serde_json::to_string;
use foundation::ClientDetails;
use crate::network::ConnectionMessage;
use uuid::Uuid;
use foundation::messages::client::ClientStreamOut;

/// Message sent ot the clients delegate
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientMessage {

}

/// message that is sent to all observers of the current client.
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientObservableMessage {
	SendMessageRequest(WeakAddr<Client>, Uuid, String),
}

/// # Client
/// This represents a connected client.
/// it will handle received message from a connection.
pub(crate) struct Client {
	connection: Addr<Connection>,
	details: ClientDetails,
	observers: Vec<Recipient<ClientObservableMessage>>
}

impl Client {
	pub(crate) fn new(
		connection: Addr<Connection>,
		details: ClientDetails,
	) -> Addr<Self> {
		Client {
			connection,
			details,
			observers: Vec::default(),
		}
		.start()
	}
}

impl Actor for Client {
	type Context = Context<Self>;

	// tells the client that it has been connected
	fn started(&mut self, ctx: &mut Self::Context) {
		use ClientStreamOut::Connected;
		use ConnectionMessage::{SendData};
		println!("[Client] started");
		self.connection.do_send(SendData(to_string::<ClientStreamOut>(&Connected).unwrap()));
	}
}

impl Handler<ClientMessage> for Client {
	type Result = ();
	fn handle(
		&mut self,
		msg: ClientMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		match msg {
			_ => todo!(),
		}
	}
}

impl Handler<ObservableMessage<ClientObservableMessage>> for Client {
	type Result = ();

	fn handle(&mut self, msg: ObservableMessage<ClientObservableMessage>, ctx: &mut Self::Context) -> Self::Result {
		use ObservableMessage::{Subscribe,Unsubscribe};
		match msg {
			Subscribe(r) => {
				println!("[Client] adding subscriber");
				self.observers.push(r);
			}
			Unsubscribe(r) => {
				println!("[Client] removing subscriber");
				self.observers = self
					.observers
					.clone()
					.into_iter()
					.filter(|a| a != &r)
					.collect();
			}
		}
	}
}
