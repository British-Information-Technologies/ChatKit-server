use serde::{Deserialize, Serialize};

/// # ClientMessage
/// This enum defined the message that a client can receive from the server
/// This uses the serde library to transform to and from json.
///
#[derive(Serialize, Deserialize)]
pub enum ClientStreamIn {
	Connected,

	Update,
	SendMessage { to_uuid: String, contents: String },
	SendGlobalMessage { contents: String },

	Disconnect,
}

#[derive(Serialize, Deserialize)]
pub enum ClientStreamOut {
	Connected,

	UserMessage { from_uuid: String, contents: String },
	GlobalMessage { contents: String },

	Disconnected,
}
