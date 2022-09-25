use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::message::Message, ClientDetails};

/// This enum defined the message that the server will receive from a client
/// This uses the serde library to transform to and from json.
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientStreamIn {
	GetClients,
	GetMessages,

	SendMessage { to: Uuid, content: String },
	SendGlobalMessage { content: String },

	Disconnect,
}

/// This enum defined the message that the server will send to a client
/// This uses the serde library to transform to and from json.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ClientStreamOut {
	Connected,

	// get reequest messages
	ConnectedClients { clients: Vec<ClientDetails> },
	GlobalChatMessages { messages: Vec<Message> },

	// event messges
	UserMessage { from: Uuid, content: String },
	GlobalMessage { from: Uuid, content: String },

	ClientConnected { id: Uuid, username: String },
	ClientRemoved { id: Uuid },

	Disconnected,

	// error cases
	Error,
}
