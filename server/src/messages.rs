use std::sync::{Arc};
use uuid::Uuid;

use foundation::connection::Connection;

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

	#[allow(dead_code)]
	Connected,

	#[allow(dead_code)]
	IncomingMessage { from: Uuid, to: Uuid, content: String },
	#[allow(dead_code)]
	IncomingGlobalMessage { from: Uuid, content: String },
	#[allow(dead_code)]
	RequestedUpdate { from: Uuid },

	Disconnect { id: Uuid },

	Error,
}

impl PartialEq for ClientMessage {
	fn eq(&self, other: &Self) -> bool {
		use ClientMessage::{Disconnect, Connected, Error};


		match (self,other) {
			(Connected, Connected) => true,
			(Error, Error) => true,
			(Disconnect {id, .. }, Disconnect {id: other_id, .. }) => id == other_id,
			_ => {
				false
			}
		}
	}
}
