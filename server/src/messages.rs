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
