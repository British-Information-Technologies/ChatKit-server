use std::sync::Arc;
use uuid::Uuid;

use crate::client::Client;

#[derive(Debug)]
pub enum ClientMessage {
	Message { from: Uuid, contents: String },

	Disconnect,
}

#[derive(Debug)]
pub enum ClientMgrMessage {
	Remove(Uuid),
	Add(Arc<Client>),
	SendMessage {
		from: Uuid,
		to: Uuid,
		contents: String,
	},
}

#[derive(Debug)]
pub enum ServerMessage {
	ClientConnected(Arc<Client>),
	ClientSendMessage {
		from: Uuid,
		to: Uuid,
		contents: String,
	},
	ClientDisconnected(Uuid),
}
