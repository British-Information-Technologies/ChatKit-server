use std::sync::{Arc};
use uuid::Uuid;
use foundation::ClientDetails;
use foundation::connection::Connection;

use crate::client::Client;

/// # ClientMessage
///
/// These messages are send from the client to a receiver
/// when events from the client happen that need to be delegated
///
/// ## Variants
///
///
/// ## Methods
///
#[derive(Debug)]
pub enum ClientMessage {

	Connected,

	IncomingMessage { from: Uuid, to: Uuid, content: String },
	IncomingGlobalMessage { from: Uuid, content: String },

	RequestedUpdate { from: Uuid },

	NewDisconnect { id: Uuid, connection: Arc<Connection> },

	Error,

	#[deprecated]
	Message { from: Uuid, content: String },

	#[deprecated]
	GlobalBroadcastMessage {from: Uuid, content:String},

	#[deprecated]
	SendClients { clients: Vec<ClientDetails> },

	#[deprecated]
	Disconnect,
}

impl PartialEq for ClientMessage {
	fn eq(&self, other: &Self) -> bool {
		use ClientMessage::{NewDisconnect, Connected, Error};


		match (self,other) {
			(Connected, Connected) => true,
			(Error, Error) => true,
			(NewDisconnect {id, .. }, NewDisconnect {id: other_id, .. }) => id == other_id,
			_ => {
				false
			}
		}
	}
}









#[derive(Debug)]
pub enum ClientMgrMessage {
	Remove(Uuid),
	Add(Arc<Client<Self>>),
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

impl From<ClientMessage> for ClientMgrMessage {
	fn from(_: ClientMessage) -> Self {
		todo!()
	}
}

#[derive(Debug)]
pub enum ServerMessage {
	ClientConnected {
		client: Arc<Client<Self>>,
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
	
	BroadcastGlobalMessage {sender: Uuid, content: String},

	Some
}

impl From<ClientMessage> for ServerMessage {
	fn from(_: ClientMessage) -> Self {
		todo!()
	}
}
