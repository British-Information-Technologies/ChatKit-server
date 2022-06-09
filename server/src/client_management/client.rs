use crate::network::Connection;
use crate::prelude::ObservableMessage;
use actix::{Actor, Addr, Context, Handler, Message, WeakAddr};
use foundation::ClientDetails;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientMessage {
	AddClient(Uuid, Addr<Client>),
	RemoveClient(Uuid),
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientObservableMessage {
	SendRequest(WeakAddr<Client>, Uuid, String),
}

/// # Client
/// This represents a connected client.
/// it will handle received message from a connection.
pub(crate) struct Client {
	connection: Addr<Connection>,
	details: ClientDetails,
}

impl Client {
	pub(crate) fn new(
		connection: Addr<Connection>,
		details: ClientDetails,
	) -> Addr<Self> {
		Client {
			connection,
			details,
		}
		.start()
	}
}

impl Actor for Client {
	type Context = Context<Self>;
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

impl Handler<ObservableMessage<ClientObservableMessage>> for Client {}
