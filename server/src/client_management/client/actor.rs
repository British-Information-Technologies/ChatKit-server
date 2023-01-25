use actix::{Actor, Addr, AsyncContext, Context, Handler, WeakRecipient};
use foundation::{
	messages::client::{ClientStreamIn, ClientStreamOut},
	ClientDetails,
};
use uuid::Uuid;

use crate::{
	client_management::client::messages::{
		ClientDataMessage,
		ClientDataResponse,
		ClientMessage,
		ClientObservableMessage,
	},
	network::{Connection, ConnectionObservableOutput},
	prelude::messages::{ConnectionMessage, ObservableMessage},
};

/// # Client
/// This represents a connected client.
/// it will handle received message from a connection.
pub struct Client {
	connection: Addr<Connection>,
	details: ClientDetails,
	observers: Vec<WeakRecipient<ClientObservableMessage>>,
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

	#[inline]
	fn get_clients(&self, ctx: &mut Context<Client>) {
		println!("[Client] getting clients");
		use ClientObservableMessage::GetClients;
		self.broadcast(GetClients(ctx.address().downgrade()));
	}

	#[inline]
	fn get_messages(&self, ctx: &mut Context<Client>) {
		println!("[Client] getting messages");
		use ClientObservableMessage::GetGlobalMessages;
		self.broadcast(GetGlobalMessages(ctx.address().downgrade()));
	}

	#[inline]
	fn send_message(&self, ctx: &mut Context<Client>, to: Uuid, content: String) {
		println!("[Client] sending message");
		use ClientObservableMessage::Message;
		self.broadcast(Message(ctx.address().downgrade(), to, content));
	}

	#[inline]
	fn send_gloal_message(&self, ctx: &mut Context<Client>, content: String) {
		println!("[Client] sending global message");
		use ClientObservableMessage::GlobalMessage;
		self.broadcast(GlobalMessage(ctx.address().downgrade(), content));
	}

	#[inline]
	fn disconnect(&self, _ctx: &mut Context<Client>) {
		println!("[Client] disconnecting");
		use ClientObservableMessage::Disconnecting;
		self.broadcast(Disconnecting(self.details.uuid));
	}

	#[inline]
	fn broadcast(&self, message: ClientObservableMessage) {
		println!("[Client] broadcasting message");
		for recp in &self.observers {
			if let Some(upgraded) = recp.upgrade() {
				upgraded.do_send(message.clone());
			}
		}
	}

	pub(crate) fn error(&self, msg: String) {
		println!("[Client] sending error: {}", msg);
		use serde_json::to_string;
		use ConnectionMessage::SendData;

		let msg = to_string::<ClientStreamOut>(&ClientStreamOut::Error { msg })
			.expect("[Client] This should not fail");

		self.connection.do_send(SendData(msg));
	}
}

impl Actor for Client {
	type Context = Context<Self>;

	// tells the client that it has been connected.
	fn started(&mut self, ctx: &mut Self::Context) {
		use foundation::messages::client::ClientStreamOut::Connected;
		use serde_json::to_string;

		use crate::{
			network::ConnectionMessage::SendData,
			prelude::messages::ObservableMessage::Subscribe,
		};
		println!("[Client] started");
		self
			.connection
			.do_send::<ObservableMessage<ConnectionObservableOutput>>(Subscribe(
				ctx.address().recipient().downgrade(),
			));
		self
			.connection
			.do_send(SendData(to_string::<ClientStreamOut>(&Connected).unwrap()));
	}

	fn stopped(&mut self, ctx: &mut Self::Context) {
		use foundation::messages::client::ClientStreamOut::Disconnected;
		use serde_json::to_string;

		use crate::{
			network::ConnectionMessage::SendData,
			prelude::messages::ObservableMessage::Unsubscribe,
		};

		println!("[Client] stopped");

		self
			.connection
			.do_send::<ObservableMessage<ConnectionObservableOutput>>(Unsubscribe(
				ctx.address().recipient().downgrade(),
			));
		self.connection.do_send(SendData(
			to_string::<ClientStreamOut>(&Disconnected).unwrap(),
		));
	}
}

impl Handler<ClientDataMessage> for Client {
	type Result = ClientDataResponse;
	fn handle(
		&mut self,
		msg: ClientDataMessage,
		_ctx: &mut Self::Context,
	) -> Self::Result {
		match msg {
			ClientDataMessage::Details => {
				ClientDataResponse::Details(self.details.clone())
			}
			_ => todo!(),
		}
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
		use foundation::messages::client::ClientStreamOut::{
			ConnectedClients,
			GlobalChatMessages,
			GlobalMessage,
			UserMessage,
		};
		use serde_json::to_string;

		use crate::{
			client_management::client::messages::ClientMessage::{
				ClientList,
				ClientlySentMessage,
				GloballySentMessage,
				MessageList,
			},
			network::ConnectionMessage::SendData,
		};

		match msg {
			ClientList(clients) => self.connection.do_send(SendData(
				to_string::<ClientStreamOut>(&ConnectedClients { clients })
					.expect("[Client] Failed to encode string"),
			)),

			MessageList(messages) => self.connection.do_send(SendData(
				to_string::<ClientStreamOut>(&GlobalChatMessages { messages })
					.expect("[Client] Failed to encode string"),
			)),

			ClientlySentMessage { content, from } => {
				self.connection.do_send(SendData(
					to_string::<ClientStreamOut>(&UserMessage { from, content })
						.expect("[Client] Failed to encode string"),
				))
			}

			GloballySentMessage { from, content } => {
				self.connection.do_send(SendData(
					to_string::<ClientStreamOut>(&GlobalMessage { from, content })
						.expect("[Client] Failed to encode string"),
				))
			}
		}
	}
}

// Handles outputs from the connection.
impl Handler<ConnectionObservableOutput> for Client {
	type Result = ();

	fn handle(
		&mut self,
		msg: ConnectionObservableOutput,
		ctx: &mut Self::Context,
	) -> Self::Result {
		use crate::network::ConnectionObservableOutput::RecvData;
		if let RecvData(_sender, _addr, data) = msg {
			use foundation::messages::client::ClientStreamIn::{
				Disconnect,
				GetClients,
				GetMessages,
				SendGlobalMessage,
				SendMessage,
			};
			use serde_json::from_str;
			if let Ok(msg) = from_str::<ClientStreamIn>(data.as_str()) {
				match msg {
					GetClients => self.get_clients(ctx),
					GetMessages => self.get_messages(ctx),
					SendMessage { to, content } => self.send_message(ctx, to, content),
					SendGlobalMessage { content } => {
						self.send_gloal_message(ctx, content)
					}
					Disconnect => self.disconnect(ctx),
				}
			} else {
				self.error(format!("Failed to parse Message: {}", data));
			}
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
				let r = r.upgrade();
				self.observers = self
					.observers
					.clone()
					.into_iter()
					.filter(|a| a.upgrade() != r)
					.collect();
			}
		}
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		println!("[Client] Dropping value")
	}
}
