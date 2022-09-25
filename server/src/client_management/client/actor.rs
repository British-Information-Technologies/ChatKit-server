use std::net::SocketAddr;

use actix::{Actor, Addr, AsyncContext, Context, Handler, Recipient};
use foundation::{messages::client::ClientStreamIn, ClientDetails};
use uuid::Uuid;

use crate::{
	client_management::client::messages::{
		ClientDataMessage,
		ClientDataResponse,
		ClientMessage,
		ClientObservableMessage,
		ClientObservableMessage::{GlobalMessage, Message, Update},
	},
	network::{Connection, ConnectionOuput},
	prelude::messages::ObservableMessage,
};

/// messages the client will send to itself
#[allow(dead_code)]
enum SelfMessage {
	ReceivedMessage(ClientStreamIn),
}

/// # Client
/// This represents a connected client.
/// it will handle received message from a connection.
pub struct Client {
	connection: Addr<Connection>,
	details: ClientDetails,
	observers: Vec<Recipient<ClientObservableMessage>>,
}

impl Client {
	pub(crate) fn new(connection: Addr<Connection>, details: ClientDetails) -> Addr<Self> {
		Client {
			connection,
			details,
			observers: Vec::default(),
		}
		.start()
	}

	fn handle_request(
		&mut self,
		ctx: &mut Context<Client>,
		_sender: Addr<Connection>,
		_addr: SocketAddr,
		data: String,
	) {
		use foundation::messages::client::ClientStreamIn::{
			Disconnect,
			SendGlobalMessage,
			SendMessage,
			Update,
		};
		use serde_json::from_str;
		let msg = from_str::<ClientStreamIn>(data.as_str())
			.expect("[Client] failed to decode incoming message");
		match msg {
			Update => self.handle_update(ctx),
			SendMessage { to, content } => self.handle_send(ctx, to, content),
			SendGlobalMessage { content } => self.handle_global_send(ctx, content),
			Disconnect => self.handle_disconnect(ctx),
			_ => todo!(),
		}
	}

	#[inline]
	fn handle_update(&self, ctx: &mut Context<Client>) {
		self.broadcast(Update(ctx.address().downgrade()));
	}

	#[inline]
	fn handle_send(&self, ctx: &mut Context<Client>, to: Uuid, content: String) {
		self.broadcast(Message(ctx.address().downgrade(), to, content));
	}

	#[inline]
	fn handle_global_send(&self, ctx: &mut Context<Client>, content: String) {
		self.broadcast(GlobalMessage(ctx.address().downgrade(), content));
	}

	#[inline]
	fn handle_disconnect(&self, _ctx: &mut Context<Client>) {
		todo!()
	}

	#[inline]
	fn broadcast(&self, message: ClientObservableMessage) {
		for recp in &self.observers {
			recp.do_send(message.clone());
		}
	}
}

impl Actor for Client {
	type Context = Context<Self>;

	// tells the client that it has been connected.
	fn started(&mut self, ctx: &mut Self::Context) {
		use foundation::messages::client::{ClientStreamOut, ClientStreamOut::Connected};
		use serde_json::to_string;

		use crate::{
			network::ConnectionMessage::SendData,
			prelude::messages::ObservableMessage::Subscribe,
		};
		println!("[Client] started");
		self
			.connection
			.do_send::<ObservableMessage<ConnectionOuput>>(Subscribe(
				ctx.address().recipient(),
			));
		self
			.connection
			.do_send(SendData(to_string::<ClientStreamOut>(&Connected).unwrap()));
	}

	fn stopped(&mut self, ctx: &mut Self::Context) {
		use foundation::messages::client::{ClientStreamOut, ClientStreamOut::Disconnected};
		use serde_json::to_string;

		use crate::{
			network::ConnectionMessage::SendData,
			prelude::messages::ObservableMessage::Unsubscribe,
		};
		self
			.connection
			.do_send::<ObservableMessage<ConnectionOuput>>(Unsubscribe(
				ctx.address().recipient(),
			));
		self.connection.do_send(SendData(
			to_string::<ClientStreamOut>(&Disconnected).unwrap(),
		));
	}
}

impl Handler<ClientDataMessage> for Client {
	type Result = ClientDataResponse;
	fn handle(&mut self, msg: ClientDataMessage, _ctx: &mut Self::Context) -> Self::Result {
		match msg {
			ClientDataMessage::Details => ClientDataResponse::Details(self.details.clone()),
			_ => todo!(),
		}
	}
}

// Handles incoming messages to the client.
impl Handler<ClientMessage> for Client {
	type Result = ();
	fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) -> Self::Result {
		use foundation::messages::client::{
			ClientStreamOut,
			ClientStreamOut::{ConnectedClients, GlobalMessage, UserMessage},
		};
		use serde_json::to_string;

		use crate::{
			client_management::client::messages::ClientMessage::{
				GlobalMessage as ClientGlobalMessage,
				Message,
				Update,
			},
			network::ConnectionMessage::SendData,
		};

		match msg {
			Update(clients) => self.connection.do_send(SendData(
				to_string::<ClientStreamOut>(&ConnectedClients { clients })
					.expect("[Client] Failed to encode string"),
			)),
			Message { content, from } => self.connection.do_send(SendData(
				to_string::<ClientStreamOut>(&UserMessage { from, content })
					.expect("[Client] Failed to encode string"),
			)),
			ClientGlobalMessage { from, content } => self.connection.do_send(SendData(
				to_string::<ClientStreamOut>(&GlobalMessage { from, content })
					.expect("[Client] Failed to encode string"),
			)),
		}
	}
}

// Handles outputs from the connection.
impl Handler<ConnectionOuput> for Client {
	type Result = ();

	fn handle(&mut self, msg: ConnectionOuput, ctx: &mut Self::Context) -> Self::Result {
		use crate::network::ConnectionOuput::RecvData;
		match msg {
			RecvData(sender, addr, data) => self.handle_request(ctx, sender, addr, data),
			_ => todo!(),
		}
	}
}

impl Handler<ObservableMessage<ClientObservableMessage>> for Client {
	type Result = ();

	fn handle(
		&mut self,
		msg: ObservableMessage<ClientObservableMessage>,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		use crate::prelude::messages::ObservableMessage::{Subscribe, Unsubscribe};
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
