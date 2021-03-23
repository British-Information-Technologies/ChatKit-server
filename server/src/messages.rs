use std::sync::Arc;
use uuid::Uuid;

use crate::client::Client;

#[derive(Debug)]
pub enum ClientMessage {
	Message(Uuid, String),

	Disconnect,
}

#[derive(Debug)]
pub enum ClientMgrMessage {
	Remove(Uuid),
	Add(Arc<Client>),
	SendMessage(Uuid, Uuid, String),
}

#[derive(Debug)]
pub enum ServerMessage {
	ClientConnected(Arc<Client>),
	ClientDisconnected(Uuid),
}
