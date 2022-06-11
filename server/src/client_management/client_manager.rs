use crate::client_management::Client;
use actix::{Actor, ArbiterHandle, Recipient, Running, WeakAddr};
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use actix::Message;
use actix::WeakRecipient;
use std::collections::HashMap;
use uuid::Uuid;
use crate::client_management::client::ClientObservableMessage;
use crate::prelude::ObservableMessage;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientManagerMessage {
	AddClient(Uuid, Addr<Client>),
	RemoveClient(Uuid),
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientManagerOutput {}

pub(crate) struct ClientManager {
	clients: HashMap<Uuid, Addr<Client>>,
	delegate: WeakRecipient<ClientManagerOutput>,
}

impl ClientManager {
	pub(crate) fn send_message_request(&self, ctx: &mut Context<ClientManager>, addr: WeakAddr<Client>, uuid: Uuid, content: String) {
		todo!()
	}
}

impl ClientManager {
	pub(crate) fn new(
		delegate: WeakRecipient<ClientManagerOutput>,
	) -> Addr<Self> {
		ClientManager {
			delegate,
			clients: HashMap::new(),
		}
		.start()
	}

	fn add_client(&mut self, ctx: &mut Context<ClientManager>, uuid: Uuid, addr: Addr<Client>) {
		use crate::prelude::ObservableMessage::Subscribe;
		let recp = ctx.address().recipient::<ClientObservableMessage>();
		addr.do_send(Subscribe(recp));
		self.clients.insert(uuid, addr);
	}

	fn remove_client(&mut self, ctx: &mut Context<ClientManager>, uuid: Uuid) {
		use crate::prelude::ObservableMessage::Unsubscribe;
		let recp = ctx.address().recipient::<ClientObservableMessage>();
		if let Some(addr) = self.clients.remove(&uuid) {
			addr.do_send(Unsubscribe(recp));
		}
	}
}

impl Actor for ClientManager {
	type Context = Context<Self>;

	fn started(&mut self, ctx: &mut Self::Context) {
		println!("[ClientManager] started");

	}
}

impl Handler<ClientManagerMessage> for ClientManager {
	type Result = ();
	fn handle(
		&mut self,
		msg: ClientManagerMessage,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ClientManagerMessage::{AddClient, RemoveClient};
		match msg {
			// todo: Add subscription to the client.
			AddClient(uuid, addr) => self.add_client(ctx, uuid, addr),
			// todo: remove subscription to client.
			RemoveClient(uuid) => self.remove_client(ctx, uuid),
		}
	}
}

impl Handler<ClientObservableMessage> for ClientManager {
	type Result = ();

	fn handle(&mut self, msg: ClientObservableMessage, ctx: &mut Self::Context) -> Self::Result {
		use ClientObservableMessage::SendMessageRequest;
		match msg {
			SendMessageRequest(addr, uuid, content) => self.send_message_request(ctx, addr, uuid, content),
		}
	}
}
