use std::net::SocketAddr;
use crate::network::{Connection, ConnectionOuput};
use crate::prelude::ObservableMessage;
use actix::{Actor, Addr, Context, Handler, Message, MessageResponse, WeakAddr, Recipient, Running, ArbiterHandle, AsyncContext};
use serde_json::{from_str, to_string};
use foundation::ClientDetails;
use crate::network::ConnectionMessage;
use uuid::Uuid;
use foundation::messages::client::{ClientStreamIn, ClientStreamOut};
use crate::client_management::client::ClientObservableMessage::{SendGlobalMessageRequest, SendMessageRequest, UpdateRequest};
use crate::network::ConnectionMessage::SendData;
use crate::prelude::ObservableMessage::{Subscribe, Unsubscribe};

/// Message sent ot the clients delegate
#[derive(Message)]
#[rtype(result = "()")]
pub enum ClientMessage {
	SendUpdate(Vec<ClientDetails>),
	SendMessage {
		from: Uuid,
		content: String,
	},
	SendGlobalMessage {
		from: Uuid,
		content: String,
	}
}

#[derive(Message)]
#[rtype(result = "ClientDetailsResponse")]
pub struct ClientDataMessage;

#[derive(MessageResponse)]
pub struct ClientDetailsResponse(pub ClientDetails);

/// messages the client will send to itself
enum SelfMessage {
	ReceivedMessage(ClientStreamIn)
}

/// message that is sent to all observers of the current client.
#[derive(Message, Clone)]
#[rtype(result = "()")]
pub enum ClientObservableMessage {
	SendMessageRequest(WeakAddr<Client>, Uuid, String),
	SendGlobalMessageRequest(WeakAddr<Client>, String),
	UpdateRequest(WeakAddr<Client>),
}

/// # Client
/// This represents a connected client.
/// it will handle received message from a connection.
pub struct Client {
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

	fn handle_request(
		&mut self,
		ctx: &mut Context<Client>,
		sender: Addr<Connection>,
		addr: SocketAddr,
		data: String
	) {
		use ClientStreamIn::{Update, SendMessage, SendGlobalMessage, Disconnect};
		let msg = from_str::<ClientStreamIn>(data.as_str()).expect("[Client] failed to decode incoming message");
		match msg {
			Update => self.handle_update(ctx),
			SendMessage { to, content } => self.handle_send(ctx, to, content),
			SendGlobalMessage { content } => self.handle_global_send(ctx, content),
			Disconnect => self.handle_disconnect(ctx),
			_ => todo!()
		}
	}

	#[inline]
	fn handle_update(&self,
	 	ctx: &mut Context<Client>,
	) {
		self.broadcast(UpdateRequest(ctx.address().downgrade()));
	}

	#[inline]
	fn handle_send(&self, ctx: &mut Context<Client>, to: Uuid, content: String) {
		self.broadcast(SendMessageRequest(
			ctx.address().downgrade(),
			to,
			content
		));
	}

	#[inline]
	fn handle_global_send(&self, ctx: &mut Context<Client>, content: String) {
		self.broadcast(SendGlobalMessageRequest(ctx.address().downgrade(), content));
	}

	#[inline]
	fn handle_disconnect(&self, ctx: &mut Context<Client>) {
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
		use ClientStreamOut::Connected;
		use ConnectionMessage::{SendData};
		println!("[Client] started");
		self.connection.do_send(Subscribe(ctx.address().recipient()));
		self.connection.do_send(SendData(to_string::<ClientStreamOut>(&Connected).unwrap()));
	}

	fn stopped(&mut self, ctx: &mut Self::Context) {
		use ClientStreamOut::Disconnected;
		use ConnectionMessage::{SendData};
		self.connection.do_send(Unsubscribe(ctx.address().recipient()));
		self.connection.do_send(SendData(to_string::<ClientStreamOut>(&Disconnected).unwrap()));
	}
}

impl Handler<ClientDataMessage> for Client {
	type Result = ClientDetailsResponse;
	fn handle(&mut self, msg: ClientDataMessage, ctx: &mut Self::Context) -> Self::Result {
		ClientDetailsResponse(self.details.clone())
	}
}

// Handles incoming messages to the client.
impl Handler<ClientMessage> for Client {
	type Result = ();
	fn handle(
		&mut self,
		msg: ClientMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		use ClientMessage::{SendUpdate, SendMessage, SendGlobalMessage};
		use ClientStreamOut::{ConnectedClients, UserMessage, GlobalMessage};

		match msg {
			SendUpdate(clients) => self.connection.do_send(
				SendData(to_string::<ClientStreamOut>(
					&ConnectedClients { clients }
				).expect("[Client] Failed to encode string"))),
			SendMessage {content, from} => self.connection.do_send(
				SendData(to_string::<ClientStreamOut>(
					&UserMessage {from,content}
				).expect("[Client] Failed to encode string"))),
			SendGlobalMessage { from, content } => self.connection.do_send(
				SendData(to_string::<ClientStreamOut>(
					&GlobalMessage {from,content}
				).expect("[Client] Failed to encode string"))),
			_ => todo!(),
		}
	}
}

// Handles outputs from the connection.
impl Handler<ConnectionOuput> for Client {
	type Result = ();

	fn handle(
		&mut self,
		msg: ConnectionOuput,
		ctx: &mut Self::Context
	) -> Self::Result {
		use ConnectionOuput::RecvData;
		match msg {
			RecvData(sender, addr, data) => self.handle_request(ctx, sender, addr, data),

			_ => todo!()
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
