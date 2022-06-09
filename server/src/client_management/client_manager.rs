use crate::client_management::Client;
use actix::Actor;
use actix::Addr;
use actix::AsyncContext;
use actix::Context;
use actix::Handler;
use actix::Message;
use actix::WeakRecipient;
use std::collections::HashMap;
use uuid::Uuid;

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
	pub(crate) fn new(
		delegate: WeakRecipient<ClientManagerOutput>,
	) -> Addr<Self> {
		ClientManager {
			delegate,
			clients: HashMap::new(),
		}
		.start()
	}

	fn add_client(&mut self, ctx: Context<Self>, uuid: Uuid, addr: Addr<Client>) {
		use crate::prelude::ObservableMessage::Subscribe;
		let recp = ctx.address().recipient().downgrade();
		addr.do_send(Subscribe(recp));
		self.clients.insert(uuid, addr)
	}
}

impl Actor for ClientManager {
	type Context = Context<Self>;
}

impl Handler<ClientManagerMessage> for ClientManager {
	type Result = ();
	fn handle(
		&mut self,
		msg: ClientManagerMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		use ClientManagerMessage::{AddClient, RemoveClient};
		match msg {
			// todo: Add subscription to the client.
			AddClient(uuid, addr) => self.add_client(uuid, addr),
			// todo: remove subscription to client.
			RemoveClient(addr) => {
				if let Some(index) = self.clients.iter().position(|i| i == &addr) {
					self.clients.remove(index);
				}
			}
		}
	}
}
