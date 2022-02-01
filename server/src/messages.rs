use std::sync::{Arc, Weak};
use uuid::Uuid;

use crate::chat_manager::Message;
use crate::client::Client;

#[derive(Debug)]
pub enum ClientMessage {
	Message { from: Uuid, content: String },
	GlobalBroadcastMessage {from: Uuid, content:String},

	SendClients { clients: Vec<Arc<Client>> },

	Disconnect,

	Error,
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
	BroadcastGlobalMessage {sender: Uuid, content: String},
	SendError {
		to: Uuid,
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
	ClientError {
		to: Uuid,
	},
	
	BroadcastGlobalMessage {sender: Uuid, content: String}
}
