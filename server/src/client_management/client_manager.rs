use std::collections::HashMap;

use actix::{
	fut::wrap_future,
	Actor,
	Addr,
	AsyncContext,
	Context,
	Handler,
	WeakAddr,
	WeakRecipient,
};
use foundation::ClientDetails;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::client_management::{
	chat_manager::{ChatManager, ChatManagerMessage},
	client::{
		Client,
		ClientDataMessage,
		ClientDataResponse,
		ClientDataResponse::Details,
		ClientMessage::SendMessage,
		ClientObservableMessage,
	},
	messages::{
		ClientManagerDataMessage,
		ClientManagerDataResponse,
		ClientManagerDataResponse::{ClientCount, Clients},
		ClientManagerMessage,
		ClientManagerOutput,
	},
};

pub struct ClientManager {
	clients: HashMap<Uuid, Addr<Client>>,
	chat_manager: Addr<ChatManager>,
	_delegate: WeakRecipient<ClientManagerOutput>,
}

impl ClientManager {
	pub(crate) fn new(delegate: WeakRecipient<ClientManagerOutput>) -> Addr<Self> {
		ClientManager {
			_delegate: delegate,
			clients: HashMap::new(),
			chat_manager: ChatManager::new(),
		}
		.start()
	}

	pub(crate) fn send_update(&mut self, ctx: &mut Context<Self>, addr: WeakAddr<Client>) {
		println!("[ClientManager] sending update to client");
		use crate::client_management::client::ClientMessage::SendUpdate;
		if let Some(to_send) = addr.upgrade() {
			let client_addr: Vec<Addr<Client>> =
				self.clients.iter().map(|(_, v)| v).cloned().collect();

			let collection = tokio_stream::iter(client_addr)
				.then(|addr| addr.send(ClientDataMessage::Details))
				.map(|val| {
					if let Details(details) = val.unwrap() {
						details
					} else {
						ClientDetails::default()
					}
				})
				.collect();

			let fut = wrap_future(async move {
				let a: Vec<_> = collection.await;
				let _ = to_send.send(SendUpdate(a)).await;
			});

			ctx.spawn(fut);
		}
	}

	pub(crate) fn send_message_request(
		&self,
		ctx: &mut Context<ClientManager>,
		sender: WeakAddr<Client>,
		_uuid: Uuid,
		content: String,
	) {
		println!("[ClientManager] sending message to client");
		let client_addr: Vec<Addr<Client>> =
			self.clients.iter().map(|(_, v)| v).cloned().collect();

		let collection = tokio_stream::iter(client_addr)
			.then(|addr| addr.send(ClientDataMessage::Details))
			.map(|val| val.unwrap())
			.map(|val: ClientDataResponse| {
				if let Details(details) = val {
					details
				} else {
					ClientDetails::default()
				}
			})
			.collect();

		let fut = wrap_future(async move {
			if let Some(sender) = sender.upgrade() {
				let details: ClientDataResponse =
					sender.send(ClientDataMessage::Details).await.unwrap();

				let from = if let Details(details) = details {
					details.uuid
				} else {
					ClientDetails::default().uuid
				};

				let client_details: Vec<ClientDetails> = collection.await;
				let pos = client_details.iter().position(|i| i.uuid == from);
				if pos.is_some() {
					sender
						.send(SendMessage { content, from })
						.await
						.expect("TODO: panic message");
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
		use crate::client_management::client::ClientMessage::SendGlobalMessage;

		let client_addr: Vec<Addr<Client>> =
			self.clients.iter().map(|(_, v)| v).cloned().collect();

		if let Some(sender) = sender.upgrade() {
			let cm = self.chat_manager.clone();

			let snd1 = sender.clone();
			let snd2 = sender;

			let cont1 = content.clone();
			let cont2 = content;

			let fut = wrap_future(async move {
				let details: ClientDataResponse =
					snd1.send(ClientDataMessage::Details).await.unwrap();

				let from = if let Details(details) = details {
					details.uuid
				} else {
					ClientDetails::default().uuid
				};

				let collection = tokio_stream::iter(client_addr)
					.then(move |addr| {
						addr.send(SendGlobalMessage {
							content: cont1.clone(),
							from,
						})
					})
					.collect();
				let _: Vec<_> = collection.await;
			});

			let chat_manager_fut = wrap_future(async move {
				let details: ClientDataResponse =
					snd2.send(ClientDataMessage::Details).await.unwrap();

				let from = if let Details(details) = details {
					details.uuid
				} else {
					ClientDetails::default().uuid
				};

				let _ = cm.send(ChatManagerMessage::AddMessage(from, cont2)).await;
			});
			ctx.spawn(fut);
			ctx.spawn(chat_manager_fut);
		}
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

	fn started(&mut self, _ctx: &mut Self::Context) {
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
		use crate::client_management::client::ClientObservableMessage::{
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
		}
	}
}

impl Handler<ClientManagerDataMessage> for ClientManager {
	type Result = ClientManagerDataResponse;

	fn handle(
		&mut self,
		msg: ClientManagerDataMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		match msg {
			ClientManagerDataMessage::ClientCount => ClientCount(self.clients.values().count()),
			ClientManagerDataMessage::Clients => {
				Clients(self.clients.values().map(|a| a.downgrade()).collect())
			}
		}
	}
}
