use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ClientDetails;

/// This enum defined the message that the server will receive from a client
/// This uses the serde library to transform to and from json.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientStreamIn {
	Connected,
	Update,

	SendMessage { to: Uuid, content: String },
	SendGlobalMessage { content: String },

	Disconnect,
}

/// This enum defined the message that the server will send to a client
/// This uses the serde library to transform to and from json.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientStreamOut {
	ConnectedClients { clients: Vec<ClientDetails> },
	UserMessage { from: Uuid, content: String },
	GlobalMessage { from: Uuid, content: String },
	Disconnected,

	Connected,

	// error cases
	Error,
}

impl PartialEq for ClientStreamOut {
	fn eq(&self, other: &Self) -> bool {
		use ClientStreamOut::{Connected, Disconnected};
		match (self, other) {
			(Connected, Connected) => true,
			(Disconnected, Disconnected) => true,
			_ => false,
		}
	}
}
