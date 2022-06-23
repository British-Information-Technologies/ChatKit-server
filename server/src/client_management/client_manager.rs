use std::collections::HashMap;

use actix::{
	fut::{wrap_future, wrap_stream},
	Actor,
	ActorFutureExt,
	ActorStreamExt,
	Addr,
	ArbiterHandle,
	AsyncContext,
	Context,
	Handler,
	MailboxError,
	Message,
	MessageResponse,
	Recipient,
	Running,
	StreamHandler,
	WeakAddr,
	WeakRecipient,
};
use foundation::{
	messages::client::{ClientStreamIn, ClientStreamIn::SendGlobalMessage},
	ClientDetails,
};
use futures::{SinkExt, TryStreamExt};
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
	client_management::{
		client::{
			ClientDataMessage,
			ClientMessage,
			ClientMessage::SendMessage,
			ClientObservableMessage,
		},
		Client,
	},
	network::NetworkOutput,
	prelude::messages::ObservableMessage,
};

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) enum ClientManagerMessage {
	AddClient(Uuid, Addr<Client>),
	RemoveClient(Uuid),
}

#[derive(Message)]
#[rtype(result = "()")]
pub enum ClientManagerOutput {
	UpdateRequest(Addr<ClientManager>),
}

pub struct ClientManager {
	clients: HashMap<Uuid, Addr<Client>>,
	delegate: WeakRecipient<ClientManagerOutput>,
}

impl ClientManager {
	pub(crate) fn send_update(
		&mut self,
		ctx: &mut Context<Self>,
		addr: WeakAddr<Client>,
	) {
		println!("[ClientManager] sending update to client");
		use ClientMessage::SendUpdate;
		let self_addr = ctx.address();
		if let Some(to_send) = addr.upgrade() {
			let client_addr: Vec<Addr<Client>> =
				self.clients.iter().map(|(_, v)| v).cloned().collect();

			let collection = tokio_stream::iter(client_addr)
				.then(|addr| addr.send(ClientDataMessage))
				.map(|val| val.unwrap().0)
				// .filter(|val| )
				.collect();

			let fut = wrap_future(async move {
				let a: Vec<_> = collection.await;
				to_send.send(SendUpdate(a)).await;
			});

			ctx.spawn(fut);
		}
	}

	pub(crate) fn send_message_request(
		&self,
		ctx: &mut Context<ClientManager>,
		sender: WeakAddr<Client>,
		uuid: Uuid,
		content: String,
	) {
		println!("[ClientManager] sending message to client");
		let client_addr: Vec<Addr<Client>> =
			self.clients.iter().map(|(_, v)| v).cloned().collect();

		let collection = tokio_stream::iter(client_addr)
			.then(|addr| addr.send(ClientDataMessage))
			.map(|val| val.unwrap().0)
			.collect();

		let fut = wrap_future(async move {
			if let Some(sender) = sender.upgrade() {
				let from: Uuid =
					sender.send(ClientDataMessage).await.unwrap().0.uuid;
				let client_details: Vec<ClientDetails> = collection.await;
				let pos = client_details.iter().position(|i| i.uuid == from);
				if let Some(pos) = pos {
					sender.send(SendMessage { content, from }).await;
				}
			}
		});

		ctx.spawn(fut);
	}

	pub(crate) fn send_global_message_request(
		&self,
		ctx: &mut Context<ClientManager>,
		sender: WeakAddr<Client>,
		content: String,
	) {
		use ClientMessage::SendGlobalMessage;
		let client_addr: Vec<Addr<Client>> =
			self.clients.iter().map(|(_, v)| v).cloned().collect();

		if let Some(sender) = sender.upgrade() {
			let fut = wrap_future(async move {
				let from: Uuid =
					sender.send(ClientDataMessage).await.unwrap().0.uuid;
				let collection = tokio_stream::iter(client_addr)
					.then(move |addr| {
						addr.send(SendGlobalMessage {
							content: content.clone(),
							from,
						})
					})
					.collect();
				let a: Vec<_> = collection.await;
			});
			ctx.spawn(fut);
		}
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

	fn add_client(
		&mut self,
		ctx: &mut Context<ClientManager>,
		uuid: Uuid,
		addr: Addr<Client>,
	) {
		println!("[ClientManager] adding client");
		use crate::prelude::messages::ObservableMessage::Subscribe;
		let recp = ctx.address().recipient::<ClientObservableMessage>();
		addr.do_send(Subscribe(recp));
		self.clients.insert(uuid, addr);
	}

	fn remove_client(&mut self, ctx: &mut Context<ClientManager>, uuid: Uuid) {
		println!("[ClientManager] removing client");
		use crate::prelude::messages::ObservableMessage::Unsubscribe;
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

	fn handle(
		&mut self,
		msg: ClientObservableMessage,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use ClientObservableMessage::{
			SendGlobalMessageRequest,
			SendMessageRequest,
			UpdateRequest,
		};
		match msg {
			SendMessageRequest(addr, uuid, content) => {
				self.send_message_request(ctx, addr, uuid, content)
			}
			SendGlobalMessageRequest(addr, content) => {
				self.send_global_message_request(ctx, addr, content)
			}
			UpdateRequest(addr) => self.send_update(ctx, addr),
			_ => todo!(),
		}
	}
}
