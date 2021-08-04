use crate::ClientDetails;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

/// # ClientMessage
/// This enum defined the message that a client can receive from the server
/// This uses the serde library to transform to and from json.
///
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientStreamIn {
	Connected,

	Update,
	SendMessage { to: Uuid, content: String },
	SendGlobalMessage { content: String },

	Disconnect,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ClientStreamOut {
	Connected,

	UserMessage { from: Uuid, content: String },
	GlobalMessage { content: String },

	ConnectedClients { clients: Vec<ClientDetails> },

	Disconnected,

	Error,
}
