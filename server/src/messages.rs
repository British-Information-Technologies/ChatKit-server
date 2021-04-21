use std::sync::Arc;
use uuid::Uuid;

use crate::client::Client;

#[derive(Debug)]
pub enum ClientMessage {
	Message { from: Uuid, content: String },

	SendClients { clients: Vec<Arc<Client>> },

	Disconnect,
}

#[derive(Debug)]
pub enum ClientMgrMessage {
	Remove(Uuid),
	Add(Arc<Client>),
	SendClients {
		to: Uuid,
	},
	SendMessage {
		from: Uuid,
		to: Uuid,
		content: String,
	},
}

#[derive(Debug)]
pub enum ServerMessage {
	ClientConnected {
		client: Arc<Client>,
	},
	ClientSendMessage {
		from: Uuid,
		to: Uuid,
		content: String,
	},
	ClientDisconnected {
		id: Uuid,
	},
	ClientUpdate {
		to: Uuid,
	},
}
