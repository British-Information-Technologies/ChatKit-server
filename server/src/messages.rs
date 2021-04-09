use std::sync::Arc;
use uuid::Uuid;

use crate::client::Client;

#[derive(Debug)]
pub enum ClientMessage {
	Message { from: Uuid, content: String },

	Update {clients: Vec<Arc<Client>>},

	Disconnect,
}

#[derive(Debug)]
pub enum ClientMgrMessage {
	Remove(Uuid),
	Add(Arc<Client>),
	SendClients {to: Uuid},
	SendMessage {
		from: Uuid,
		to: Uuid,
		content: String,
	},
}

#[derive(Debug)]
pub enum ServerMessage {
	ClientConnected(Arc<Client>),
	ClientSendMessage {
		from: Uuid,
		to: Uuid,
		content: String,
	},
	ClientDisconnected(Uuid),
	ClientUpdate(Uuid),
}
